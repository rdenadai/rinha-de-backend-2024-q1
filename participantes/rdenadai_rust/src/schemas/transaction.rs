use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionType {
    Credit,
    Debit,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TransactionType::Credit => write!(f, "c"),
            TransactionType::Debit => write!(f, "d"),
        }
    }
}

impl Serialize for TransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            TransactionType::Credit => serializer.serialize_str("c"),
            TransactionType::Debit => serializer.serialize_str("d"),
        }
    }
}

impl<'de> Deserialize<'de> for TransactionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "c" => Ok(TransactionType::Credit),
            "d" => Ok(TransactionType::Debit),
            _ => Err(D::Error::custom("tipo => 'c' ou 'd'")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TransactionInput {
    #[serde(deserialize_with = "greater_then_zero")]
    pub valor: usize,
    pub tipo: TransactionType,
    #[serde(deserialize_with = "more_than_zero_less_then_ten")]
    pub descricao: String,
}

fn greater_then_zero<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let num: usize = usize::deserialize(deserializer)?;

    if num < 0 {
        Err(D::Error::custom("valor must be greater then zero"))
    } else {
        Ok(num)
    }
}

fn more_than_zero_less_then_ten<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if s.len() < 1 || s.len() > 10 {
        Err(D::Error::custom("1 <= len(descricao) <= 10"))
    } else {
        Ok(s)
    }
}
