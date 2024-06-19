use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackEventAuthorization {
    pub enterprise_id: Option<SlackEnterpriseId>,
    pub team_id: Option<SlackTeamId>,
    pub user_id: SlackUserId,
    pub is_bot: Option<bool>,
    pub is_enterprise_install: Option<bool>,
}
