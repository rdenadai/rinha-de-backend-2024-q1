use actix_web::{get, post, HttpResponse, Responder, Error};
use actix_web::web;
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::schemas::transaction::TransactionInput;
use sqlx::postgres::PgPool;
use sqlx::query;


#[derive(Serialize, Deserialize)]
struct HealthCheck {
    status: bool,
    message: String,
}

#[get("/")]
pub async fn index() -> impl Responder {
    let data: HealthCheck = serde_json::from_str(r#"{"status": true, "message": "Ok"}"#).unwrap();
    HttpResponse::Ok().json(data)
}

#[get("/{id}/extrato")]
pub async fn account_statement(path: web::Path<usize>, pool: web::Data<PgPool>) -> impl Responder {
    let cliente_id = path.into_inner();
    match pool.begin().await {
        Ok(transaction) => {
            let record = query!(
                r#"SELECT c.limite as limite, NOW() as data_extrato, s.valor as total 
                FROM clientes c 
                JOIN saldos s on c.id = s.cliente_id 
                WHERE c.id = $1;"#, cliente_id.to_string()
            )
            .fetch_one(&mut **transaction)
            .await;

            transaction.commit().await;

            if record {
                return HttpResponse::Ok().json(record)
            }

            return HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json("client not found")
        },
        Err(e) => {
            return HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(e.to_string())
        },
    }
    return HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json("client not found")
}

#[post("/{id}/transacoes")]
pub async fn transaction(
    path: web::Path<usize>,
    transaction: Result<web::Json<TransactionInput>, Error>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let cliente_id = path.into_inner();
    match transaction {
        Ok(transaction) => {
            HttpResponse::Ok().body(transaction.0.tipo.clone().to_string())
        },
        Err(e) => {
            HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(e.to_string())
        },
    }}
