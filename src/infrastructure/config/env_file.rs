use std::env;
use url::Url;
use crate::domain::app_config::AppConfig;
use crate::domain::constants::{ADMIN_API_KEY, APP_URL, DATABASE_URL};

pub async fn load_app_config() -> AppConfig {
    let environment = match env::var("ENV") {
        Ok(val) => val,
        Err(_) => "local".to_string(),
    };
    dotenv::from_filename(format!(".env.{}", environment)).ok();

    let app_url = dotenv::var(APP_URL).expect(&*format!("{} must be set", APP_URL));
    let app_url_parsed = Url::parse(&app_url).expect("Invalid APP_URL format");

    AppConfig {
        environment,
        app_url,
        app_url_host: app_url_parsed.host().expect("Failed to extrace hostname from APP_URL").to_string(),
        admin_api_key: dotenv::var(ADMIN_API_KEY).expect(&*format!("{} must be set", ADMIN_API_KEY)),
        database_url: dotenv::var(DATABASE_URL).expect(&*format!("{} must be set", DATABASE_URL)),
    }
}
