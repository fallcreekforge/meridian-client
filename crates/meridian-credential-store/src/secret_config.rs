use secrecy::SecretString;
use serde::Deserialize;

#[expect(clippy::struct_field_names)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SecretConfig {
   pub(crate) meridian_cloud_key:          SecretString,
   pub(crate) steam_financial_web_api_key: SecretString,
   pub(crate) steam_publisher_web_api_key: SecretString,
}
