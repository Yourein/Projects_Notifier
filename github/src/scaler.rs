use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct DateTime(String);

impl From<&chrono::DateTime<chrono::Utc>> for DateTime {
    fn from(value: &chrono::DateTime<chrono::Utc>) -> Self {
        DateTime(value.to_rfc3339())
    }
}

impl TryFrom<DateTime> for chrono::DateTime<chrono::Utc> {
    type Error = chrono::ParseError;

    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        chrono::DateTime::parse_from_rfc3339(value.0.as_str()).map(Into::into)
    }
}