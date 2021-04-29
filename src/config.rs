use crate::database::DatabaseConnection;
use dotenv::dotenv;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub database: DatabaseConnection,
    pub database_url: String,
    pub rust_log: String,
    pub server: String,
    pub worker: usize,
}

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

fn get_config() -> Config {
    dotenv().ok();

    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}", error),
    }
}
