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
pub struct MapTextItems {
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<f32>,
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<f32>,
    #[serde(rename = "mapMarkerType", skip_serializing_if = "Option::is_none")]
    pub map_marker_type: Option<MapMarkerType>,
}

impl MapTextItems {
    pub fn new() -> MapTextItems {
        MapTextItems {
            text: None,
            x: None,
            y: None,
            map_marker_type: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MapMarkerType {
    #[serde(rename = "Major")]
    Major,
    #[serde(rename = "Minor")]
    Minor,
}

impl Default for MapMarkerType {
    fn default() -> MapMarkerType {
        Self::Major
    }
}

