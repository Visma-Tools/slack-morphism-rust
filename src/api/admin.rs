
use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::*;
use crate::ratectl::*;
use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    pub async fn admin_roles_add_assignments(
        &self,
        req: &SlackApiAdminRolesAddAssignmentsRequest,
    ) -> ClientResult<SlackApiAdminRolesAddAssignmentsResponse> {
        self.http_session_api
            .http_post("admin.roles.addAssignments", req, Some(&SLACK_TIER2_METHOD_CONFIG))
            .await
    }

    pub async fn admin_roles_list_assignments(
        &self,
        req: &SlackApiAdminRolesListAssignmentsRequest,
    ) -> ClientResult<SlackApiAdminRolesListAssignmentsResponse> {
        self.http_session_api
            .http_get(
                "admin.roles.listAssignments",
                &vec![
                    ("role_id", Some(req.role_id.value())),
                    ("cursor", req.cursor.as_ref().map(|v| v.value())),
                    ("limit", req.limit.map(|v| v.to_string()).as_ref()),
                ],
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }

    pub async fn admin_roles_remove_assignments(
        &self,
        req: &SlackApiAdminRolesRemoveAssignmentsRequest,
    ) -> ClientResult<SlackApiAdminRolesRemoveAssignmentsResponse> {
        let entity_ids_str = req.entity_ids.join(",");
        let user_ids_str = req.user_ids.iter()
            .map(|id| id.value().to_string())
            .collect::<Vec<String>>()
            .join(",");
            
        self.http_session_api
            .http_get(
                "admin.roles.removeAssignments",
                &vec![
                    ("role_id", Some(req.role_id.value())),
                    ("entity_ids", Some(&entity_ids_str)),
                    ("user_ids", Some(&user_ids_str)),
                ],
                Some(&SLACK_TIER2_METHOD_CONFIG),
            )
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAdminRolesAddAssignmentsRequest {
    pub role_id: SlackRoleId,
    pub user_ids: Vec<SlackUserId>,
    pub entity_ids: Vec<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAdminRolesAddAssignmentsResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAdminRolesListAssignmentsRequest {
    pub role_id: SlackRoleId,
    pub cursor: Option<SlackCursorId>,
    pub limit: Option<u16>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAdminRolesListAssignmentsResponse {
    pub role_assignments: Vec<SlackRoleAssignment>,
    pub response_metadata: Option<SlackResponseMetadata>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRoleAssignment {
    pub role_id: SlackRoleId,
    pub user_id: SlackUserId,
    pub entity_id: String,
    pub date_create: Option<SlackDateTime>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAdminRolesRemoveAssignmentsRequest {
    pub role_id: SlackRoleId,
    pub user_ids: Vec<SlackUserId>,
    pub entity_ids: Vec<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAdminRolesRemoveAssignmentsResponse {}
