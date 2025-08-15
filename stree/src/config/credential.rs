use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Credential {
   access_key: Option<String>,
   secret_key: Option<String>,
}
