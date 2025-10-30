use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateReply {
    pub comment_id: String,
    pub parent_comment_id: String,
    pub video_id: String,
    pub user_id: String,
    pub create_time: String,
    pub text: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl CreateReply {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("CreateReply {:?}", self.extra);
        }
        res
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum CreateReplyField {
    CommentId,
    ParentCommentId,
    VideoId,
    UserId,
    CreateTime,
    Text,
}

impl CreateReplyField {
    pub fn all() -> HashSet<Self> {
        let mut set = HashSet::new();
        set.insert(CreateReplyField::CommentId);
        set.insert(CreateReplyField::ParentCommentId);
        set.insert(CreateReplyField::VideoId);
        set.insert(CreateReplyField::UserId);
        set.insert(CreateReplyField::CreateTime);
        set.insert(CreateReplyField::Text);
        set
    }
}

impl std::fmt::Display for CreateReplyField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CommentId => write!(f, "comment_id"),
            Self::ParentCommentId => write!(f, "parent_comment_id"),
            Self::VideoId => write!(f, "video_id"),
            Self::UserId => write!(f, "user_id"),
            Self::CreateTime => write!(f, "create_time"),
            Self::Text => write!(f, "text"),
        }
    }
}
