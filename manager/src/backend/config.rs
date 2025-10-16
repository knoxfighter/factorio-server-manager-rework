use crate::backend::error::BackendError;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub conf: PathBuf,
    pub manager_path: PathBuf,
    pub database_file: String,
}

#[derive(Serialize, Deserialize, Parser, Default)]
struct ConfigOption {
    #[arg(long)]
    conf: Option<PathBuf>,
    #[arg(long)]
    manager_path: Option<PathBuf>,
    #[arg(long)]
    database_file: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            conf: PathBuf::from("conf.json"),
            manager_path: PathBuf::from("./manager"),
            database_file: "./fsm.db".to_string(),
        }
    }
}

// default<-json<-.env<-env<-param

impl Config {
    pub fn init() -> Result<Self, BackendError> {
        println!("{:?}", std::env::args().collect::<Vec<_>>());

        let default = Config::default();
        let envs = ConfigOption::from_env();
        let args = ConfigOption::parse();

        let conf = args
            .conf
            .as_ref()
            .unwrap_or_else(|| envs.conf.as_ref().unwrap_or(&default.conf));

        let json = match conf.exists() {
            true => {
                let file = File::open(conf)?;
                serde_json::from_reader(file)?
            }
            false => Default::default(),
        };

        Ok(default.merge(json).merge(envs).merge(args))
    }

    fn merge(self, other: ConfigOption) -> Self {
        Self {
            conf: other.conf.unwrap_or(self.conf),
            manager_path: other.manager_path.unwrap_or(self.manager_path),
            database_file: other.database_file.unwrap_or(self.database_file),
        }
    }
}

impl ConfigOption {
    fn from_env() -> Self {
        Self {
            conf: env::var("FSM_CONF").map(|v| PathBuf::from(v)).ok(),
            manager_path: env::var("FSM_MANAGER_PATH").map(|v| PathBuf::from(v)).ok(),
            // TODO: add old env: SQ_LITE_DATABASE_FILE
            database_file: env::var("FSM_DATABASE_FILE").ok(),
        }
    }
}
