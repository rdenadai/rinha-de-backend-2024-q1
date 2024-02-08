import asyncio

import uvloop
from starlette.applications import Starlette
from starlette.endpoints import HTTPEndpoint
from starlette.routing import Route

from db import lifespan
from orjson_response import OrjsonResponse as JSONResponse
from stmt import (
    INSERT_TRANSACTION_SQL,
    READ_ACCOUNT_STATEMENT_SQL,
    READ_TRANSACTION_SQL,
)

asyncio.set_event_loop_policy(uvloop.EventLoopPolicy())


class Validation:
    async def check_for_client_id(self, request):
        try:
            return int(request.path_params.get("id", None))
        except ValueError:
            return JSONResponse({"error": "id is required"}, status_code=422)


class Transaction(HTTPEndpoint, Validation):
    async def post(self, request):
        if client_id := await self.check_for_client_id(request):
            content = await request.json()
            fields = (content.get(field, None) for field in ("valor", "tipo", "descricao"))
            if not all(fields) or not all(isinstance(field, (int, str)) for field in fields):
                return JSONResponse({"error": "missing fields"}, status_code=422)

            valor, tipo, descricao = fields
            async with app.state.connection_pool.acquire() as connection:
                await connection.execute(INSERT_TRANSACTION_SQL, client_id, valor, tipo, descricao)
                return JSONResponse({"limit": "ok", "saldo": ""})


class AccountStatement(HTTPEndpoint, Validation):
    async def get(self, request):
        if client_id := await self.check_for_client_id(request):
            async with app.state.connection_pool.acquire() as connection:
                balance = dict((await connection.fetchrow(READ_ACCOUNT_STATEMENT_SQL, client_id)).items())
                transactions = (
                    dict(transaction.items()) for transaction in await connection.fetch(READ_TRANSACTION_SQL, client_id)
                )
                return JSONResponse({"saldo": balance, "ultimas_transacoes": tuple(transactions)})


app = Starlette(
    routes=[
        Route("/clientes/{id}/transacoes", Transaction, methods=["POST"]),
        Route("/clientes/{id}/extrato", AccountStatement, methods=["GET"]),
    ],
    lifespan=lifespan,
)
