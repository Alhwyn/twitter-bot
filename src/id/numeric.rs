use serde::de::Visitor;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NumericId(u64);

impl NumericId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl From<NumericId> for u64 {
    fn from(id: NumericId) -> Self {
        id.0
    }
}

impl<'a> From<&'a u64> for NumericId {
    fn from(id: &'a u64) -> Self {
        NumericId(*id)
    }
}

impl FromStr for NumericId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self)
    }
}
