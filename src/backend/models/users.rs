use crate::backend::error::BackendError;
use crate::backend::schema::users;
use crate::backend::{AppState, DbPool};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::{Extension, RequestPartsExt};
use diesel::QueryDsl;
use diesel::{sql_query, ExpressionMethods};
use diesel::{Queryable, QueryableByName, Selectable, SelectableHelper};
use diesel_async::RunQueryDsl;
use rand::distr::Alphanumeric;
use rand::Rng;
use std::backtrace::Backtrace;
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
    pub fn verify_password(&self, password: impl AsRef<str>) -> Result<(), BackendError> {
        Argon2::default().verify_password(
            password.as_ref().as_bytes(),
            &PasswordHash::new(&self.password)?,
        )?;
        Ok(())
    }
}

impl Into<User> for LoginUser {
    fn into(self) -> User {
        User {
            uuid: self.uuid,
            username: self.username,
        }
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
        password: impl AsRef<str>,
    ) -> Result<Self, BackendError> {
        let mut conn = db.get().await?;

        let salt = SaltString::try_from_rng(&mut OsRng)?;
        let password = Argon2::default()
            .hash_password(password.as_ref().as_bytes(), &salt)?
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
        password: impl AsRef<str>,
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

    pub(crate) async fn assure_default_user(db: &DbPool) -> Result<(), BackendError> {
        let mut conn = db.get().await?;

        #[derive(QueryableByName)]
        struct UserExists {
            #[sql_type = "diesel::sql_types::Bool"]
            users_exist: bool,
        }

        let res: UserExists = sql_query("SELECT EXISTS(SELECT 1 FROM users) AS users_exist;")
            .get_result(&mut conn)
            .await?;

        if res.users_exist {
            return Ok(());
        }

        let password: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        let user = Self::create(db, "admin", &password).await?;

        println!(
            "Created default user: \"{}\" with password: \"{}\"",
            user.username, password
        );

        Ok(())
    }
}

pub const SESSION_USER_KEY: &str = "user";

impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    // type Rejection = (StatusCode, &'static str);
    type Rejection = BackendError;

    // TODO: change to return proper BackendErrors
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await?;
        let user_id: String = session
            .get(SESSION_USER_KEY)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "session not found"))?
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "not logged in"))?;

        let Extension(state) = parts
            .extract::<Extension<AppState>>()
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "extension state not set"))?;

        let user = Self::get_by_uuid(&state.db, &user_id)
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, "user not found"))?;

        Ok(user)
    }
}
