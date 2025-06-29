use super::entity::FullTextEntities;
use super::geo::GeoCoordinates;
use super::withheld::Withheld;
use crate::id::{NumericId, StringId};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct TweetPublicMetrics {
    pub retweet_count: usize,
    pub reply_count: usize,
    pub like_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_count: Option<usize>,
}
