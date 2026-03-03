use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credential {
   pub access_key: Option<String>,
   pub secret_key: Option<String>,
}
