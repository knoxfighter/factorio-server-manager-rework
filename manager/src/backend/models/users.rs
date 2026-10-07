use crate::backend::error::BackendError;
use crate::backend::schema::users;
use crate::backend::{AppState, DbPool};
use crate::password::Password;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::{Extension, RequestPartsExt};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Queryable, QueryableByName, Selectable, SelectableHelper};
use diesel_async::RunQueryDsl;
use dioxus::server::ServerFnError;
use rand::distr::{Alphanumeric, SampleString};
use tower_sessions_core::Session;
use uuid::Uuid;

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = users)]
struct LoginUser {
    pub uuid: String,
    pub username: String,
    pub password: String,
}

impl LoginUser {
    pub fn verify_password(&self, password: Password) -> Result<(), BackendError> {
        Argon2::default().verify_password(
            password.as_ref().as_bytes(),
            &PasswordHash::new(&self.password)?,
        )?;
        Ok(())
    }
}

#[derive(Queryable, Selectable, QueryableByName)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = users)]
pub struct User {
    pub uuid: String,
    pub username: String,
}

impl User {
    pub async fn get_by_uuid(db: &DbPool, user_id: &str) -> Result<Self, BackendError> {
        let mut conn = db.get().await?;

        let r = users::table
            .filter(users::uuid.eq(user_id))
            .select(User::as_select())
            .first(&mut conn)
            .await?;

        Ok(r)
    }

    pub async fn create(
        db: &DbPool,
        username: impl AsRef<str>,
        password: Password,
    ) -> Result<Self, BackendError> {
        let mut conn = db.get().await?;

        let password = Argon2::default()
            .hash_password(password.as_ref().as_bytes())?
            .to_string();

        let uuid = Uuid::new_v4().to_string();

        let res = diesel::insert_into(users::table)
            .values((
                users::uuid.eq(uuid),
                users::username.eq(username.as_ref()),
                users::password.eq(password),
            ))
            .returning(Self::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(res)
    }

    pub async fn login(
        db: &DbPool,
        username: impl AsRef<str>,
        password: Password,
    ) -> Result<Self, BackendError> {
        let mut conn = db.get().await?;

        let user: LoginUser = users::table
            .filter(users::username.eq(username.as_ref()))
            .select(LoginUser::as_select())
            .first(&mut conn)
            .await?;

        user.verify_password(password)?;

        Ok(user.into())
    }

    pub(crate) async fn assure_admin_user(db: &DbPool) -> Result<(), BackendError> {
        let mut conn = db.get().await?;

        let count: i64 = users::table.count().first(&mut conn).await?;

        if count > 0 {
            return Ok(());
        }

        let password: Password = Alphanumeric.sample_string(&mut rand::rng(), 16).into();

        let user = Self::create(db, "admin", password.clone()).await?;

        println!(
            "Created default user: \"{}\" with password: \"{}\"",
            user.username,
            password.as_ref()
        );

        Ok(())
    }
}

impl From<LoginUser> for User {
    fn from(user: LoginUser) -> Self {
        Self {
            uuid: user.uuid,
            username: user.username,
        }
    }
}

pub const SESSION_USER_KEY: &str = "user";

impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    // type Rejection = (StatusCode, &'static str);
    type Rejection = ServerFnError;

    // TODO: change to return proper BackendErrors
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(BackendError::from)?;
        let user_id: String = session
            .get(SESSION_USER_KEY)
            .await
            .map_err(BackendError::from)?
            .ok_or((StatusCode::UNAUTHORIZED, "not logged in"))
            .map_err(BackendError::from)?;

        let Extension(state) = parts
            .extract::<Extension<AppState>>()
            .await
            .map_err(BackendError::from)?;

        let user = Self::get_by_uuid(&state.db, &user_id)
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, "user not found"))
            .map_err(BackendError::from)?;

        Ok(user)
    }
}
