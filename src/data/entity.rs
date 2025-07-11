use crate::id::NumericId;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct UrlImage {
    pub url: Url,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct UrlEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<usize>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<UrlImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<usize>,
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AnnotationEntity {
    pub start: isize,
    pub end: isize,
    pub probability: f64,
    #[serde(rename = "type")]
    pub kind: String,
    pub normalized_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct CashtagEntity {
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
    pub id: Option<NumericId>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FullTextEntities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<UrlEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashtags: Option<Vec<HashtagEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<AnnotationEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashtags: Option<Vec<CashtagEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<MentionEntity>>,
}
