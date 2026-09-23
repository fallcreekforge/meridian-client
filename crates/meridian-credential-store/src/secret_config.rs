use serde::{
   Deserialize,
   Serialize,
};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SecretConfig {
   pub(crate) steam_ipartner_financials_service_key: String,
   pub(crate) meridian_cloud_key:                    String,
}
