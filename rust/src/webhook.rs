use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Debug, PartialEq, EnumString, Serialize, Deserialize, Clone)]
pub enum EventType {
    #[strum(serialize = "authorization.removed")]
    AuthorizationRemoved,

    #[strum(serialize = "post.publish.failed")]
    PostPublishFailed,

    #[strum(serialize = "post.publish.complete")]
    PostPublishComplete,

    #[strum(serialize = "post.publish.publicly_available")]
    PostPublishPubliclyAvailable,

    #[strum(serialize = "post.publish.no_longer_publicly_available")]
    PostPublishNoLongerPubliclyAvailable,

    #[strum(serialize = "comment.update")]
    CommentUpdate,
}

#[derive(Debug, PartialEq, EnumString, Serialize, Deserialize, Clone)]
pub enum CommentType {
    #[strum(serialize = "comment")]
    Comment,

    #[strum(serialize = "reply")]
    Reply,
}

#[derive(Debug, PartialEq, EnumString, Serialize, Deserialize, Clone)]
pub enum CommentAction {
    #[strum(serialize = "insert")]
    Insert,

    #[strum(serialize = "delete")]
    Delete,

    #[strum(serialize = "set_to_hidden")]
    SetToHidden,

    #[strum(serialize = "set_to_friends_only")]
    SetToFriendsOnly,

    #[strum(serialize = "set_to_public")]
    SetToPublic,
}

#[derive(Debug, PartialEq, EnumString, Deserialize, Serialize, Clone)]
pub enum PublishType {
    #[strum(serialize = "DIRECT_PUBLISH")]
    DirectPublish,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WebhookEvent {
    pub client_key: String,
    pub event: EventType,
    pub create_time: i64,
    pub user_openid: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthorizationRemoved {
    pub reason: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CommentEvent {
    pub comment_id: i64,
    pub video_id: i64,
    pub parent_comment_id: i64,
    pub comment_type: CommentType,
    pub comment_action: CommentAction,
    pub unique_identifier: String,
    pub timestamp: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostPublishFailedEvent {
    pub publish_id: String,
    pub reason: String,
    pub publish_type: PublishType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostPublishComplete {
    pub publish_id: String,
    pub publish_type: PublishType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostPublishPubliclyAvailable {
    pub publish_id: String,
    pub post_id: String,
    pub publish_type: PublishType,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostPublishNoLongerPubliclyAvailable {
    pub publish_id: String,
    pub post_id: String,
    pub publish_type: PublishType,
}
