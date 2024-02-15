use actix_web::{get, post, HttpResponse, Responder, Error};
use actix_web::web;
use actix_web::http::StatusCode;
use crate::schemas::transaction::{TransactionInput, Transaction, TransactionOutput};
use crate::schemas::account_statement::{Balance, AccountStatement};
use crate::schemas::health_check::{HealthCheck};
use sqlx::postgres::PgPool;
use sqlx::Row;



#[get("/")]
pub async fn index() -> impl Responder {
    let data: HealthCheck = serde_json::from_str(r#"{"status": true, "message": "Ok"}"#).unwrap();
    HttpResponse::Ok().json(data)
}

async fn read_account_statement(cliente_id: i32, pool: web::Data<PgPool>) -> Result<HttpResponse, sqlx::Error> {
    let mut db_transaction = pool.begin().await?;
    let balance_record = sqlx::query(
        "SELECT c.limite as limite, NOW() as data_extrato, s.valor as total 
        FROM clientes c 
        JOIN saldos s on c.id = s.cliente_id 
        WHERE c.id = $1;"
    )
    .bind(cliente_id)
    .fetch_one(&mut *db_transaction)
    .await;

    match balance_record {
        Ok(balance_record) => {
            let get_transactions = sqlx::query(
                "SELECT valor, tipo, descricao, realizada_em
                FROM transacoes t
                JOIN clientes c on c.id = t.cliente_id
                WHERE c.id = $1
                ORDER BY t.realizada_em DESC
                LIMIT 10;"
            )
            .bind(cliente_id)
            .fetch_all(&mut *db_transaction)
            .await?;
        
            let balance = Balance {
                limite: balance_record.try_get("limite")?,
                data_extrato: balance_record.try_get("data_extrato")?,
                total: balance_record.try_get("total")?,
            };
        
            let mut vtransactions_ = Vec::new();
            for trans in get_transactions {
                vtransactions_.push(Transaction {
                    valor: trans.try_get("valor")?,
                    tipo: trans.try_get("tipo")?,
                    descricao: trans.try_get("descricao")?,
                    realizada_em: trans.try_get("realizada_em")?,
                });
            }
        
            let account_statement_ = AccountStatement {
                saldo: balance, 
                ultimas_transacoes: vtransactions_
            };
        
            db_transaction.commit().await?; 
            return Ok(HttpResponse::Ok().json(&account_statement_));
        },
        Err(_) => {
            db_transaction.commit().await?;
            return Ok(HttpResponse::build(StatusCode::NOT_FOUND).json("client not found"));
        },
    }
}

#[get("/{id}/extrato")]
pub async fn account_statement(path: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    let cliente_id = path.into_inner();
    match read_account_statement(cliente_id, pool).await {
        Ok(response) => {
            return response;
        },
        Err(e) => {
            return HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(e.to_string());
        },
    }
}

async fn write_transaction(cliente_id: i32, input_transaction: web::Json<TransactionInput>, pool: web::Data<PgPool>) -> Result<HttpResponse, sqlx::Error> {
    let mut db_transaction = pool.begin().await?;
    let client = sqlx::query("SELECT 1 FROM clientes WHERE id = $1;")
                    .bind(cliente_id)
                    .fetch_one(&mut *db_transaction)
                    .await;
    match client {
        Ok(_) => {
            let saldo_atualizado = sqlx::query("SELECT saldo, limite, efetuado FROM atualiza_saldo($1, $2, $3, $4);")
                        .bind(cliente_id)
                        .bind(input_transaction.valor)
                        .bind(input_transaction.tipo.to_string())
                        .bind(&input_transaction.descricao)
                        .fetch_one(&mut *db_transaction)
                        .await?;
            db_transaction.commit().await?;

            let transaction_output = TransactionOutput {
                saldo: saldo_atualizado.try_get("saldo")?,
                limite: saldo_atualizado.try_get("limite")?,
            };
            let efetuado: bool = saldo_atualizado.try_get("efetuado")?;
            if !efetuado {
                return Ok(HttpResponse::Ok().json(transaction_output));
            } else {
                return Ok(HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(transaction_output));
            }
        },
        Err(_) => {
            db_transaction.commit().await?;
        },
    }
    return Ok(HttpResponse::build(StatusCode::NOT_FOUND).json("client not found"));
}


#[post("/{id}/transacoes")]
pub async fn transaction(
    path: web::Path<i32>,
    input_transaction: Result<web::Json<TransactionInput>, Error>,
    pool: web::Data<PgPool>
) -> impl Responder {
    match input_transaction {
        Ok(input_transaction) => {
            let cliente_id = path.into_inner();
            match write_transaction(cliente_id, input_transaction, pool).await {
                Ok(response) => {
                    return response;
                },
                Err(e) => {
                    return HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(e.to_string());
                },
            }
        },
        Err(e) => {
            return HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(e.to_string());
        },
    }
    return HttpResponse::build(StatusCode::NOT_FOUND).json("client not found");
}
