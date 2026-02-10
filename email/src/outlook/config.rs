use super::{Error, Result};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(
    feature = "derive",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub struct OutlookConfig {
    pub client_id: String,
    pub tenant_id: String,
    pub auth: OutlookAuthConfig,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "derive",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "kebab-case", tag = "type")
)]
pub enum OutlookAuthConfig {
    #[cfg(feature = "oauth2")]
    OAuth2(crate::account::config::oauth2::OAuth2Config),
}

impl Default for OutlookAuthConfig {
    fn default() -> Self {
        #[cfg(feature = "oauth2")]
        {
            Self::OAuth2(Default::default())
        }
        #[cfg(not(feature = "oauth2"))]
        {
            panic!("outlook backend requires oauth2 feature")
        }
    }
}

impl OutlookConfig {
    pub async fn build_access_token(&self) -> Result<String> {
        match &self.auth {
            #[cfg(feature = "oauth2")]
            OutlookAuthConfig::OAuth2(oauth2) => oauth2
                .access_token()
                .await
                .map_err(|err| Error::BuildAccessTokenError(Box::new(err))),
        }
    }
}
