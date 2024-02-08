import contextlib

import asyncpg


@contextlib.asynccontextmanager
async def lifespan(app):
    app.state.connection_pool = await asyncpg.create_pool(
        max_size=30,
        user="admin",
        password="123",
        database="rinha",
        host="db",
        port=5432,
    )
    yield
    await app.state.connection_pool.close()
