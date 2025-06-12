
use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::ratectl::*;
use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    pub async fn canvases_edit(
        &self,
        req: &SlackApiCanvasesEditRequest,
    ) -> ClientResult<SlackApiCanvasesEditResponse> {
        self.http_session_api
            .http_post("canvases.edit", req, Some(&SLACK_TIER3_METHOD_CONFIG))
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiCanvasesEditRequest {
    pub canvas_id: String,
    pub changes: Vec<SlackCanvasChange>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiCanvasesEditResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackCanvasChange {
    pub operation: SlackCanvasOperation,
    pub document_content: Option<SlackCanvasDocumentContent>,
    pub section_id: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackCanvasOperation {
    InsertAfter,
    InsertBefore,
    InsertAtStart,
    InsertAtEnd,
    Replace,
    Delete,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackCanvasDocumentContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub markdown: Option<String>,
}
