use crate::domain::models::app_config::AppConfig;

pub trait AppConfigService {
    fn get_app_config(&self) -> &AppConfig;
}
