use crate::routes::*;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index).service(
        web::scope("/clientes")
            .service(index::account_statement)
            .service(index::transaction),
    );
}
