use async_trait::async_trait;
use tracing::info;

use super::{Folders, ListFolders};
use crate::{folder::Folder, outlook::OutlookContext, AnyResult};

#[derive(Debug, Clone)]
pub struct ListOutlookFolders {
    ctx: OutlookContext,
}

impl ListOutlookFolders {
    pub fn new(ctx: &OutlookContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    pub fn new_boxed(ctx: &OutlookContext) -> Box<dyn ListFolders> {
        Box::new(Self::new(ctx))
    }

    pub fn some_new_boxed(ctx: &OutlookContext) -> Option<Box<dyn ListFolders>> {
        Some(Self::new_boxed(ctx))
    }
}

#[async_trait]
impl ListFolders for ListOutlookFolders {
    async fn list_folders(&self) -> AnyResult<Folders> {
        info!("listing outlook folders");

        let response = self
            .ctx
            .client
            .me()
            .mail_folders()
            .list_mail_folders()
            .send()
            .await
            .map_err(|e| -> crate::AnyBoxedError { Box::new(crate::outlook::Error::from(e)) })?;

        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| -> crate::AnyBoxedError { 
                Box::new(crate::outlook::Error::GraphApiError(
                    graph_rs_sdk::GraphFailure::from(e)
                ))
            })?;

        let folders: Folders = body["value"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|folder| {
                folder["displayName"].as_str().map(|name| {
                    let kind = self.ctx.account_config.find_folder_kind_from_alias(name);
                    Folder {
                        kind,
                        name: name.to_string(),
                        desc: String::new(),
                    }
                })
            })
            .collect();

        Ok(folders)
    }
}
