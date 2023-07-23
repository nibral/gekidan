use crate::domain::constants::APP_URL;
use crate::domain::models::app_config::AppConfig;
use crate::domain::services::app_config::AppConfigService;

pub struct AppConfigServiceImpl {
    app_config: AppConfig,
}

impl AppConfigServiceImpl {
    pub async fn new() -> Self {
        AppConfigServiceImpl {
            app_config: AppConfig {
                app_url: dotenv::var(APP_URL).expect(&*format!("{} must be set", APP_URL))
            }
        }
    }
}

impl AppConfigService for AppConfigServiceImpl {
    fn get_app_config(&self) -> &AppConfig {
        &self.app_config
    }
}
