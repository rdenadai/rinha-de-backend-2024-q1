import contextlib

import asyncpg


@contextlib.asynccontextmanager
async def setup_database(app):
    app.state.connection_pool = await asyncpg.create_pool(
        user="admin",
        password="123",
        database="rinha",
        host="db",
        port=5432,
        max_size=15,
        max_queries=150000,
    )
    yield
    await app.state.connection_pool.close()
