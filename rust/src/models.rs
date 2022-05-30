use flutter_rust_bridge::frb;
use serde_derive::{Deserialize, Serialize};

pub struct ActionRequest {
    pub action: String,
    pub payload: String
}

pub struct ActionResponse {
    pub response: String,
    pub success: bool
}

// To mirror an external struct, you need to define a placeholder type with the same definition
pub struct AnotherResponse {
    pub response: String,
    pub success: bool,
    pub extra: String,
}

#[derive(Serialize, Deserialize)]
#[repr(C)]
#[frb]
pub struct Person {
    pub first_name: String,
    pub last_name: String
}