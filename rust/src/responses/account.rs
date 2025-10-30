use crate::responses::{audience_country::AudienceCountry, audience_gender::AudienceGender, metric::Metric};
use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Account {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_business_account: Option<bool>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_count: Option<i64>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_countries: Option<Vec<AudienceCountry>>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_genders: Option<Vec<AudienceGender>>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<Metric>>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Account {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.audience_countries.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.audience_genders.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.metrics.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true);
        if !res {
          println!("Account {:?}", self.extra);
        }
        res
    }
}



#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum AccountField {
    IsBusinessAccount,
    Username,
    DisplayName,
    ProfileImage,
    FollowersCount,
    AudienceCountries,
    AudienceGenders,
}

impl AccountField {
    pub fn all() -> HashSet<Self> {
        let mut set = HashSet::new();
        set.insert(AccountField::IsBusinessAccount);
        set.insert(AccountField::Username);
        set.insert(AccountField::DisplayName);
        set.insert(AccountField::ProfileImage);
        set.insert(AccountField::FollowersCount);
        set.insert(AccountField::AudienceCountries);
        set.insert(AccountField::AudienceGenders);
        set
    }
}

impl std::fmt::Display for AccountField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IsBusinessAccount => write!(f, "is_business_account"),
            Self::Username => write!(f, "username"),
            Self::DisplayName => write!(f, "display_name"),
            Self::ProfileImage => write!(f, "profile_image"),
            Self::FollowersCount => write!(f, "followers_count"),
            Self::AudienceCountries => write!(f, "audience_countries"),
            Self::AudienceGenders => write!(f, "audience_genders"),
        }
    }
}


