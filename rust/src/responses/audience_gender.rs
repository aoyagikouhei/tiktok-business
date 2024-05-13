use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AudienceGender {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl AudienceGender {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("AudienceGender {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Gender {
    #[serde(rename = "Female")]
    Female,
    #[serde(rename = "Male")]
    Male,
    #[serde(rename = "Other")]
    Other,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Female => write!(f, "Female"),
            Self::Male => write!(f, "Male"),
            Self::Other => write!(f, "Other"),
        }
    }
}

impl Default for Gender {
    fn default() -> Self {
        Self::Female
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum AudienceGenderField {
    Gender,
    Percentage,
}

impl AudienceGenderField {
    pub fn all() -> HashSet<Self> {
        let mut set = HashSet::new();
        set.insert(AudienceGenderField::Gender);
        set.insert(AudienceGenderField::Percentage);
        set
    }
}

impl std::fmt::Display for AudienceGenderField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Gender => write!(f, "gender"),
            Self::Percentage => write!(f, "percentage"),
        }
    }
}
