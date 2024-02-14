from typing import Any

from orjson import dumps
from starlette.responses import JSONResponse


class OrjsonResponse(JSONResponse):
    def render(self, content: Any) -> bytes:
        return dumps(content)
