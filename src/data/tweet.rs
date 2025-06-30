use super::geo::GeoCoordinates;
use super::withheld::Withheld;
use crate::id::StringId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Tweet {
    pub id: StringId,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<StringId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<StringId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to_user_id: Option<StringId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_tweets: Option<Vec<ReferencedTweet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<TweetAttachments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<TweetGeo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_annotations: Option<Vec<ContextAnnotation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<TweetEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withheld: Option<Withheld>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_metrics: Option<TweetPublicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possibly_sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_settings: Option<ReplySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ReferencedTweet {
    #[serde(rename = "type")]
    pub reference_type: String,
    pub id: StringId,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct TweetAttachments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TweetGeo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<GeoCoordinates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<StringId>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ContextAnnotation {
    pub domain: ContextAnnotationDomain,
    pub entity: ContextAnnotationEntity,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ContextAnnotationDomain {
    pub id: StringId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ContextAnnotationEntity {
    pub id: StringId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TweetEntities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<UrlEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashtags: Option<Vec<HashtagEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<MentionEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashtags: Option<Vec<CashtagEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<AnnotationEntity>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct UrlEntity {
    pub start: usize,
    pub end: usize,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unwound_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct HashtagEntity {
    pub start: usize,
    pub end: usize,
    pub tag: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct MentionEntity {
    pub start: usize,
    pub end: usize,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<StringId>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct CashtagEntity {
    pub start: usize,
    pub end: usize,
    pub tag: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AnnotationEntity {
    pub start: usize,
    pub end: usize,
    pub probability: f64,
    #[serde(rename = "type")]
    pub annotation_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_text: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum ReplySettings {
    #[serde(rename = "everyone")]
    Everyone,
    #[serde(rename = "mentionedUsers")]
    MentionedUsers,
    #[serde(rename = "following")]
    Following,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct TweetPublicMetrics {
    pub retweet_count: usize,
    pub reply_count: usize,
    pub like_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_count: Option<usize>,
}
