use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AudienceCountry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<isocountry::CountryCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl AudienceCountry {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("AudienceCountry {:?}", self.extra);
        }
        res
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum AudienceCountryField {
    Country,
    Percentage,
}

impl AudienceCountryField {
    pub fn all() -> HashSet<Self> {
        let mut set = HashSet::new();
        set.insert(AudienceCountryField::Country);
        set.insert(AudienceCountryField::Percentage);
        set
    }
}

impl std::fmt::Display for AudienceCountryField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Country => write!(f, "country"),
            Self::Percentage => write!(f, "percentage"),
        }
    }
}
