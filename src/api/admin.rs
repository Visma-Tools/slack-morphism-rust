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
            .http_post(
                "admin.roles.addAssignments",
                req,
                Some(&SLACK_TIER2_METHOD_CONFIG),
            )
            .await
    }

    pub async fn admin_roles_list_assignments(
        &self,
        req: &SlackApiAdminRolesListAssignmentsRequest,
    ) -> ClientResult<SlackApiAdminRolesListAssignmentsResponse> {
        let role_ids_str = req.role_ids.as_ref().map(|ids| {
            ids.iter()
                .map(|id| id.value().to_string())
                .collect::<Vec<String>>()
                .join(",")
        });

        let entity_ids_str = req.entity_ids.as_ref().map(|ids| ids.join(","));

        let limit_str = req.limit.map(|v| v.to_string());
        let limit_ref = limit_str.as_ref().map(|s| s as &String);

        let mut params = vec![
            ("cursor", req.cursor.as_ref().map(|v| v.value())),
            ("limit", limit_ref),
        ];

        if let Some(role_ids) = &role_ids_str {
            params.push(("role_ids", Some(role_ids)));
        }

        if let Some(entity_ids) = &entity_ids_str {
            params.push(("entity_ids", Some(entity_ids)));
        }

        if let Some(sort_dir) = &req.sort_dir {
            params.push(("sort_dir", Some(sort_dir)));
        }

        self.http_session_api
            .http_get(
                "admin.roles.listAssignments",
                &params,
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }

    pub async fn admin_roles_remove_assignments(
        &self,
        req: &SlackApiAdminRolesRemoveAssignmentsRequest,
    ) -> ClientResult<SlackApiAdminRolesRemoveAssignmentsResponse> {
        let entity_ids_str = req.entity_ids.join(",");
        let user_ids_str = req
            .user_ids
            .iter()
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
    pub role_ids: Option<Vec<SlackRoleId>>,
    pub entity_ids: Option<Vec<String>>,
    pub cursor: Option<SlackCursorId>,
    pub limit: Option<u16>,
    pub sort_dir: Option<String>,
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
