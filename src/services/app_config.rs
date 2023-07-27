use url::Url;
use crate::domain::constants::{ADMIN_API_KEY, APP_URL};
use crate::domain::models::app_config::AppConfig;
use crate::domain::services::app_config::AppConfigService;

pub struct AppConfigServiceImpl {
    app_config: AppConfig,
}

impl AppConfigServiceImpl {
    pub async fn new() -> Self {
        let app_url = dotenv::var(APP_URL).expect(&*format!("{} must be set", APP_URL));
        let parsed_url = Url::parse(&app_url).expect("Invalid APP_URL format");
        let admin_api_key = dotenv::var(ADMIN_API_KEY).expect(&*format!("{} must be set", ADMIN_API_KEY));

        AppConfigServiceImpl {
            app_config: AppConfig {
                app_url,
                app_url_host: parsed_url.host().expect("Failed to extract hostname from APP_URL").to_string(),
                admin_api_key,
            }
        }
    }
}

impl AppConfigService for AppConfigServiceImpl {
    fn get_app_config(&self) -> &AppConfig {
        &self.app_config
    }
}
