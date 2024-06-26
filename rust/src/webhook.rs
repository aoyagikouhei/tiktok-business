use chrono::{prelude::*, Duration};
use hex;
use hmac_sha256::HMAC;
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TikTokSignature {
    t_str: String,
    t: DateTime<Utc>,
    s: String,
}

impl TikTokSignature {
    pub fn new(src: &str) -> Option<Self> {
        let v: Vec<&str> = src.split(',').collect();
        if v.len() <= 2 {
            return None;
        }
        let t_str = calc_signature_one(v[0])?;
        let t = str_to_timestamp(&t_str)?;
        let s = calc_signature_one(v[1])?;
        Some(TikTokSignature { t_str, t, s })
    }

    pub fn get_time(&self) -> DateTime<Utc> {
        self.t
    }

    pub fn check(&self, secret: &str, payload: &str, delay: &Option<Duration>) -> bool {
        // シグネチャーを計算
        let mut combinate = self.t_str.clone();
        combinate.push_str(payload);
        let key = secret.as_bytes();
        let data = combinate.as_bytes();
        let Ok(expected_signature) = hex::decode(&self.s) else {
            return false;
        };
        let calculated_signature = HMAC::mac(key, data);

        // シグネチャーの比較
        if expected_signature != calculated_signature {
            return false;
        }

        // 時間の比較
        if let Some(delay) = delay {
            let now = Utc::now();
            let diff = now - self.t;
            if diff > *delay {
                return false;
            }
        }
        true
    }
}

fn calc_signature_one(src: &str) -> Option<String> {
    let ary: Vec<&str> = src.split('=').collect();
    if ary.len() != 2 {
        return None;
    }
    Some(ary[1].to_string())
}

fn str_to_timestamp(src: &str) -> Option<DateTime<Utc>> {
    match src.parse::<i64>() {
        Ok(timestamp) => Utc.timestamp_opt(timestamp, 0).single(),
        Err(_) => None,
    }
}
