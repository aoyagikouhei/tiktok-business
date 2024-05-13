use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Reply {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_comment_id: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Reply {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Reply {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    #[serde(rename = "HIDDEN")]
    Hidden,
    #[serde(rename = "PUBLIC")]
    Public,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Hidden => write!(f, "HIDDEN"),
            Self::Public => write!(f, "PUBLIC"),
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::Hidden
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum ReplyField {
    CommentId,
    VideoId,
    UserId,
    CreateTime,
    Text,
    Likes,
    Replies,
    Owner,
    Liked,
    Pinned,
    Status,
    Username,
    ProfileImage,
    ParentCommentId,
}

impl ReplyField {
    pub fn all() -> HashSet<Self> {
        let mut set = HashSet::new();
        set.insert(ReplyField::CommentId);
        set.insert(ReplyField::VideoId);
        set.insert(ReplyField::UserId);
        set.insert(ReplyField::CreateTime);
        set.insert(ReplyField::Text);
        set.insert(ReplyField::Likes);
        set.insert(ReplyField::Replies);
        set.insert(ReplyField::Owner);
        set.insert(ReplyField::Liked);
        set.insert(ReplyField::Pinned);
        set.insert(ReplyField::Status);
        set.insert(ReplyField::Username);
        set.insert(ReplyField::ProfileImage);
        set.insert(ReplyField::ParentCommentId);
        set
    }
}

impl std::fmt::Display for ReplyField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CommentId => write!(f, "comment_id"),
            Self::VideoId => write!(f, "video_id"),
            Self::UserId => write!(f, "user_id"),
            Self::CreateTime => write!(f, "create_time"),
            Self::Text => write!(f, "text"),
            Self::Likes => write!(f, "likes"),
            Self::Replies => write!(f, "replies"),
            Self::Owner => write!(f, "owner"),
            Self::Liked => write!(f, "liked"),
            Self::Pinned => write!(f, "pinned"),
            Self::Status => write!(f, "status"),
            Self::Username => write!(f, "username"),
            Self::ProfileImage => write!(f, "profile_image"),
            Self::ParentCommentId => write!(f, "parent_comment_id"),
        }
    }
}
