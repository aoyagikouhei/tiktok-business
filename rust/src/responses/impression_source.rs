use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ImpressionSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impression_source: Option<ImpressionSourceEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ImpressionSource {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("ImpressionSource {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ImpressionSourceEnum {
    #[serde(rename = "For You")]
    ForYou,
    #[serde(rename = "Follow")]
    Follow,
    #[serde(rename = "Hashtag")]
    Hashtag,
    #[serde(rename = "Sound")]
    Sound,
    #[serde(rename = "Personal Profile")]
    PersonalProfile,
    #[serde(rename = "other_profile_vv")]
    OtherProfileVv,
    #[serde(rename = "Search")]
    Search,
}

impl std::fmt::Display for ImpressionSourceEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ForYou => write!(f, "For You"),
            Self::Follow => write!(f, "Follow"),
            Self::Hashtag => write!(f, "Hashtag"),
            Self::Sound => write!(f, "Sound"),
            Self::PersonalProfile => write!(f, "Personal Profile"),
            Self::OtherProfileVv => write!(f, "other_profile_vv"),
            Self::Search => write!(f, "Search"),
        }
    }
}

impl Default for ImpressionSourceEnum {
    fn default() -> Self {
        Self::ForYou
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum ImpressionSourceField {
    ImpressionSource,
    Percentage,
}

impl ImpressionSourceField {
    pub fn all() -> HashSet<Self> {
        let mut set = HashSet::new();
        set.insert(ImpressionSourceField::ImpressionSource);
        set.insert(ImpressionSourceField::Percentage);
        set
    }
}

impl std::fmt::Display for ImpressionSourceField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ImpressionSource => write!(f, "impression_source"),
            Self::Percentage => write!(f, "percentage"),
        }
    }
}
