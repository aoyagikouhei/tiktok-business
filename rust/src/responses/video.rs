use crate::responses::{audience_country::AudienceCountry, impression_source::ImpressionSource};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Video {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_views: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reach: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_video_watched_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_time_watched: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_time_watched: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impression_sources: Option<Vec<ImpressionSource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_countries: Option<Vec<AudienceCountry>>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Video {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .impression_sources
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true)
            && self
                .audience_countries
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true);
        if !res {
            println!("Video {:?}", self.extra);
        }
        res
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum VideoField {
    ItemId,
    CreateTime,
    ThumbnailUrl,
    ShareUrl,
    EmbedUrl,
    Caption,
    VideoViews,
    VideoDuration,
    Likes,
    Comments,
    Shares,
    Reach,
    FullVideoWatchedRate,
    TotalTimeWatched,
    AverageTimeWatched,
    ImpressionSources,
    AudienceCountries,
}

impl VideoField {
    pub fn all() -> HashSet<Self> {
        let mut set = HashSet::new();
        set.insert(VideoField::ItemId);
        set.insert(VideoField::CreateTime);
        set.insert(VideoField::ThumbnailUrl);
        set.insert(VideoField::ShareUrl);
        set.insert(VideoField::EmbedUrl);
        set.insert(VideoField::Caption);
        set.insert(VideoField::VideoViews);
        set.insert(VideoField::VideoDuration);
        set.insert(VideoField::Likes);
        set.insert(VideoField::Comments);
        set.insert(VideoField::Shares);
        set.insert(VideoField::Reach);
        set.insert(VideoField::FullVideoWatchedRate);
        set.insert(VideoField::TotalTimeWatched);
        set.insert(VideoField::AverageTimeWatched);
        set.insert(VideoField::ImpressionSources);
        set.insert(VideoField::AudienceCountries);
        set
    }
}

impl std::fmt::Display for VideoField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemId => write!(f, "item_id"),
            Self::CreateTime => write!(f, "create_time"),
            Self::ThumbnailUrl => write!(f, "thumbnail_url"),
            Self::ShareUrl => write!(f, "share_url"),
            Self::EmbedUrl => write!(f, "embed_url"),
            Self::Caption => write!(f, "caption"),
            Self::VideoViews => write!(f, "video_views"),
            Self::VideoDuration => write!(f, "video_duration"),
            Self::Likes => write!(f, "likes"),
            Self::Comments => write!(f, "comments"),
            Self::Shares => write!(f, "shares"),
            Self::Reach => write!(f, "reach"),
            Self::FullVideoWatchedRate => write!(f, "full_video_watched_rate"),
            Self::TotalTimeWatched => write!(f, "total_time_watched"),
            Self::AverageTimeWatched => write!(f, "average_time_watched"),
            Self::ImpressionSources => write!(f, "impression_sources"),
            Self::AudienceCountries => write!(f, "audience_countries"),
        }
    }
}
