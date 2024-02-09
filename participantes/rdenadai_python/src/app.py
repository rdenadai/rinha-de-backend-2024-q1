import asyncio
import gc

import uvloop
from asyncpg.exceptions import RaiseError
from pydantic import ValidationError
from starlette.applications import Starlette
from starlette.endpoints import HTTPEndpoint
from starlette.exceptions import HTTPException
from starlette.requests import Request
from starlette.routing import Route

from db import lifespan
from orjson_response import OrjsonResponse as JSONResponse
from schemas import BalanceOutput, TransactionInput
from stmt import READ_ACCOUNT_STATEMENT_SQL, READ_TRANSACTION_SQL, UPDATE_BALANCE_SQL

gc.disable()
asyncio.set_event_loop_policy(uvloop.EventLoopPolicy())


class ClientNotValidException(HTTPException):
    ...


class Validation:
    async def check_for_client_id(self, request):
        try:
            if not (client_id := request.path_params.get("id", None)):
                raise ClientNotValidException(status_code=404, detail="id is required")
            return int(client_id)
        except ValueError:
            raise ClientNotValidException(status_code=404, detail="id is required")


class Transaction(HTTPEndpoint, Validation):
    async def post(self, request):
        if client_id := await self.check_for_client_id(request):
            try:
                content = TransactionInput(**(await request.json()))
            except ValidationError as e:
                raise ClientNotValidException(status_code=422, detail=str(e.errors()))
            return await self.database_op(client_id, **content.dict())

    async def database_op(self, client_id, valor, tipo, descricao):
        async with app.state.connection_pool.acquire() as connection:
            try:
                record = dict(
                    (await connection.fetchrow(UPDATE_BALANCE_SQL, client_id, valor, tipo, descricao)).items()
                )
                return JSONResponse({"saldo": record.get("saldo", 0), "limite": record.get("limite", 0)})
            except RaiseError as e:
                raise HTTPException(status_code=422, detail=str(e))


class AccountStatement(HTTPEndpoint, Validation):
    async def get(self, request):
        if client_id := await self.check_for_client_id(request):
            return await self.database_op(client_id)

    async def database_op(self, client_id):
        async with app.state.connection_pool.acquire() as connection:
            async with connection.transaction():
                balance = BalanceOutput(
                    **dict((await connection.fetchrow(READ_ACCOUNT_STATEMENT_SQL, client_id) or {}).items())
                )

                transactions = (
                    dict(transaction.items())
                    for transaction in await connection.fetch(READ_TRANSACTION_SQL, client_id) or []
                )
                return JSONResponse({"saldo": balance.dict(), "ultimas_transacoes": tuple(transactions)})


async def http_exception(request: Request, exc: ClientNotValidException):
    return JSONResponse({"detail": exc.detail}, status_code=422)


app = Starlette(
    routes=[
        Route("/clientes/{id}/transacoes", Transaction, methods=["POST"]),
        Route("/clientes/{id}/extrato", AccountStatement, methods=["GET"]),
    ],
    lifespan=lifespan,
    exception_handlers={HTTPException: http_exception},  # type: ignore
)
