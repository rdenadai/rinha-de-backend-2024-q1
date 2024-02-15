use crate::schemas::transaction::Transaction;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::TimestampSeconds;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct Balance {
    pub limite: i32,
    #[serde_as(as = "TimestampSeconds<String>")]
    pub data_extrato: DateTime<Utc>,
    pub total: i32,
}

#[derive(Serialize, Deserialize)]
pub struct AccountStatement {
    pub saldo: Balance,
    pub ultimas_transacoes: Vec<Transaction>,
}
