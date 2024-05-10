use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AudienceActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl AudienceActivity {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("AudienceActivity {:?}", self.extra);
        }
        res
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum AudienceActivityField {
    Hour,
    Count,
}

impl AudienceActivityField {
    pub fn all() -> HashSet<Self> {
        let mut set = HashSet::new();
        set.insert(AudienceActivityField::Hour);
        set.insert(AudienceActivityField::Count);
        set
    }
}

impl std::fmt::Display for AudienceActivityField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Hour => write!(f, "hour"),
            Self::Count => write!(f, "count"),
        }
    }
}
