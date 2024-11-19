/*
 * Foxhole WarAPI
 *
 * The War API allows developers to query information about the state of the current Foxhole World Conquest.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShardStatus {
    #[serde(rename = "bShowColonialQueueWarning", skip_serializing_if = "Option::is_none")]
    pub b_show_colonial_queue_warning: Option<bool>,
    #[serde(rename = "bShowWardenQueueWarning", skip_serializing_if = "Option::is_none")]
    pub b_show_warden_queue_warning: Option<bool>,
    #[serde(rename = "normalizedGlobalPopulation", skip_serializing_if = "Option::is_none")]
    pub normalized_global_population: Option<f32>,
    #[serde(rename = "serverConnectionInfoList", skip_serializing_if = "Option::is_none")]
    pub server_connection_info_list: Option<Vec<models::ServerConnectionInfo>>,
    #[serde(rename = "warId", skip_serializing_if = "Option::is_none")]
    pub war_id: Option<uuid::Uuid>,
    #[serde(rename = "squadMaxSize", skip_serializing_if = "Option::is_none")]
    pub squad_max_size: Option<i32>,
    #[serde(rename = "secondsToPreConquest", skip_serializing_if = "Option::is_none")]
    pub seconds_to_pre_conquest: Option<i32>,
    #[serde(rename = "bIsPreConquest", skip_serializing_if = "Option::is_none")]
    pub b_is_pre_conquest: Option<bool>,
    #[serde(rename = "bIsVIPMode", skip_serializing_if = "Option::is_none")]
    pub b_is_vip_mode: Option<bool>,
}

impl ShardStatus {
    pub fn new() -> ShardStatus {
        ShardStatus {
            b_show_colonial_queue_warning: None,
            b_show_warden_queue_warning: None,
            normalized_global_population: None,
            server_connection_info_list: None,
            war_id: None,
            squad_max_size: None,
            seconds_to_pre_conquest: None,
            b_is_pre_conquest: None,
            b_is_vip_mode: None,
        }
    }
}
