use crate::responses::{reply::Reply};
use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Comment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_id: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>, 
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_list: Option<Vec<Reply>>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Comment {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.reply_list.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true);
        if !res {
          println!("Comment {:?}", self.extra);
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
    fn default() -> Self { Self::Hidden }
}



#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum CommentField {
    CommentId,
    VideoId,
    UserId,
    UniqueIdentifier,
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
    ReplyList,
}

impl CommentField {
    pub fn all() -> HashSet<Self> {
        let mut set = HashSet::new();
        set.insert(CommentField::CommentId);
        set.insert(CommentField::VideoId);
        set.insert(CommentField::UserId);
        set.insert(CommentField::UniqueIdentifier);
        set.insert(CommentField::CreateTime);
        set.insert(CommentField::Text);
        set.insert(CommentField::Likes);
        set.insert(CommentField::Replies);
        set.insert(CommentField::Owner);
        set.insert(CommentField::Liked);
        set.insert(CommentField::Pinned);
        set.insert(CommentField::Status);
        set.insert(CommentField::Username);
        set.insert(CommentField::ProfileImage);
        set.insert(CommentField::ParentCommentId);
        set.insert(CommentField::ReplyList);
        set
    }
}

impl std::fmt::Display for CommentField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CommentId => write!(f, "comment_id"),
            Self::VideoId => write!(f, "video_id"),
            Self::UserId => write!(f, "user_id"),
            Self::UniqueIdentifier => write!(f, "unique_identifier"),
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
            Self::ReplyList => write!(f, "reply_list"),
        }
    }
}


