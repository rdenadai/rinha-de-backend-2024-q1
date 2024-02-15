from typing import Optional

from db import setup_database
from orjson_response import OrjsonResponse as JSONResponse
from pydantic import ValidationError
from schemas import BalanceOutput, TransactionInput
from starlette.applications import Starlette
from starlette.requests import Request
from starlette.routing import Route
from stmt import (
    CHECK_IF_USER_EXISTS,
    READ_ACCOUNT_STATEMENT_SQL,
    READ_TRANSACTION_SQL,
    UPDATE_BALANCE_SQL,
)

DEFAULT_ERROR_MSG = "client not found"


def check_for_client_id(request: Request) -> Optional[int]:
    if (client_id := request.path_params.get("id", None)) and client_id.isdigit():
        return int(client_id)
    return None


async def account_statement(request: Request) -> JSONResponse:
    if client_id := check_for_client_id(request):
        async with app.state.connection_pool.acquire() as connection, connection.transaction():
            if record := (await connection.fetchrow(READ_ACCOUNT_STATEMENT_SQL, client_id) or {}):
                balance = BalanceOutput(**dict(record.items()))
                transactions = [
                    dict(transaction.items())
                    for transaction in await connection.fetch(READ_TRANSACTION_SQL, client_id) or []
                ]
                return JSONResponse(
                    {"saldo": balance.dict(), "ultimas_transacoes": transactions},
                    status_code=200,
                )
            return JSONResponse({"detail": DEFAULT_ERROR_MSG}, status_code=404)
    return JSONResponse({"detail": DEFAULT_ERROR_MSG}, status_code=404)


async def transaction(request: Request) -> JSONResponse:
    if client_id := check_for_client_id(request):
        try:
            content = TransactionInput(**(await request.json()))
            valor, tipo, descricao = content.valor, content.tipo, content.descricao
            async with app.state.connection_pool.acquire() as connection, connection.transaction():
                if record := await connection.fetchval(CHECK_IF_USER_EXISTS, client_id):
                    record = dict(
                        (await connection.fetchrow(UPDATE_BALANCE_SQL, client_id, valor, tipo, descricao)).items()
                        or {"efetuado": True}
                    )
                    status_code = 200 if not record.get("efetuado", False) else 422
                    return JSONResponse(
                        {"saldo": record.get("saldo", 0), "limite": record.get("limite", 0)},
                        status_code=status_code,
                    )
                return JSONResponse({"detail": DEFAULT_ERROR_MSG}, status_code=404)
        except ValidationError as e:
            return JSONResponse({"detail": str(e.errors())}, status_code=422)
    return JSONResponse({"detail": DEFAULT_ERROR_MSG}, status_code=404)


app = Starlette(
    routes=[
        Route("/clientes/{id}/extrato", account_statement, methods=["GET"]),
        Route("/clientes/{id}/transacoes", transaction, methods=["POST"]),
    ],
    lifespan=setup_database,
)
