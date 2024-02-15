use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthCheck {
    status: bool,
    message: String,
}
