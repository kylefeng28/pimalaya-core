pub mod config;
mod error;

use std::sync::Arc;

use async_trait::async_trait;
use graph_rs_sdk::Graph;

#[doc(inline)]
pub use self::error::{Error, Result};
use crate::{
    account::config::AccountConfig,
    backend::context::{BackendContext, BackendContextBuilder},
    folder::list::{outlook::ListOutlookFolders, ListFolders},
    AnyResult,
};

pub use self::config::OutlookConfig;

#[derive(Clone, Debug)]
pub struct OutlookContext {
    pub account_config: Arc<AccountConfig>,
    pub outlook_config: Arc<OutlookConfig>,
    pub(crate) client: Graph,
}

impl BackendContext for OutlookContext {}

#[derive(Clone, Debug)]
pub struct OutlookContextBuilder {
    pub account_config: Arc<AccountConfig>,
    pub outlook_config: Arc<OutlookConfig>,
    prebuilt_token: Option<String>,
}

impl OutlookContextBuilder {
    pub fn new(account_config: Arc<AccountConfig>, outlook_config: Arc<OutlookConfig>) -> Self {
        Self {
            account_config,
            outlook_config,
            prebuilt_token: None,
        }
    }

    pub async fn prebuild_token(&mut self) -> Result<()> {
        self.prebuilt_token = Some(self.outlook_config.build_access_token().await?);
        Ok(())
    }

    pub async fn with_prebuilt_token(mut self) -> Result<Self> {
        self.prebuild_token().await?;
        Ok(self)
    }
}

#[async_trait]
impl BackendContextBuilder for OutlookContextBuilder {
    type Context = OutlookContext;

    fn list_folders(&self) -> Option<Arc<dyn Fn(&Self::Context) -> Option<Box<dyn ListFolders>> + Send + Sync>> {
        Some(Arc::new(ListOutlookFolders::some_new_boxed))
    }

    async fn build(self) -> AnyResult<Self::Context> {
        let token = match self.prebuilt_token {
            Some(t) => t,
            None => return Err(Box::new(Error::MissingAccessToken)),
        };

        let client = Graph::new(&token);

        Ok(OutlookContext {
            account_config: self.account_config,
            outlook_config: self.outlook_config,
            client,
        })
    }
}
