use crate::responses::audience_activity::AudienceActivity;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Metric {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_views: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_views: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_activity: Option<Vec<AudienceActivity>>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Metric {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .audience_activity
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true);
        if !res {
            println!("Metric {:?}", self.extra);
        }
        res
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum MetricField {
    Date,
    FollowersCount,
    ProfileViews,
    VideoViews,
    Likes,
    Comments,
    Shares,
    AudienceActivity,
}

impl MetricField {
    pub fn all() -> HashSet<Self> {
        let mut set = HashSet::new();
        set.insert(MetricField::Date);
        set.insert(MetricField::FollowersCount);
        set.insert(MetricField::ProfileViews);
        set.insert(MetricField::VideoViews);
        set.insert(MetricField::Likes);
        set.insert(MetricField::Comments);
        set.insert(MetricField::Shares);
        set.insert(MetricField::AudienceActivity);
        set
    }
}

impl std::fmt::Display for MetricField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Date => write!(f, "date"),
            Self::FollowersCount => write!(f, "followers_count"),
            Self::ProfileViews => write!(f, "profile_views"),
            Self::VideoViews => write!(f, "video_views"),
            Self::Likes => write!(f, "likes"),
            Self::Comments => write!(f, "comments"),
            Self::Shares => write!(f, "shares"),
            Self::AudienceActivity => write!(f, "audience_activity"),
        }
    }
}
