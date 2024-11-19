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
pub struct War {
    #[serde(rename = "warId", skip_serializing_if = "Option::is_none")]
    pub war_id: Option<uuid::Uuid>,
    #[serde(rename = "warNumber", skip_serializing_if = "Option::is_none")]
    pub war_number: Option<i32>,
    #[serde(rename = "winner", skip_serializing_if = "Option::is_none")]
    pub winner: Option<models::FactionEnum>,
    #[serde(rename = "conquestStartTime", skip_serializing_if = "Option::is_none")]
    pub conquest_start_time: Option<i32>,
    #[serde(rename = "conquestEndTime", skip_serializing_if = "Option::is_none")]
    pub conquest_end_time: Option<i32>,
    #[serde(rename = "resistanceStartTime", skip_serializing_if = "Option::is_none")]
    pub resistance_start_time: Option<i32>,
    #[serde(rename = "requiredVictoryTowns", skip_serializing_if = "Option::is_none")]
    pub required_victory_towns: Option<i32>,
}

impl War {
    pub fn new() -> War {
        War {
            war_id: None,
            war_number: None,
            winner: None,
            conquest_start_time: None,
            conquest_end_time: None,
            resistance_start_time: None,
            required_victory_towns: None,
        }
    }
}
