use serde::Deserialize;

#[derive(Deserialize, Getters)]
#[getset(get = "pub")]
pub struct AppConfig {
    yearly_template: Option<String>,
    monthly_template: Option<String>,
    daily_template: Option<String>,
}
