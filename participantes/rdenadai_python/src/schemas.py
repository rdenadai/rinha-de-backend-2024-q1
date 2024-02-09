from datetime import datetime
from enum import StrEnum

from pydantic import BaseModel, field_validator


class TransactionType(StrEnum):
    credito = "c"
    debito = "d"


class TransactionInput(BaseModel):
    valor: int
    tipo: TransactionType
    descricao: str

    @field_validator("valor")
    @classmethod
    def greater_then_zero(cls, v: int) -> int:
        if v <= 0:
            raise ValueError("valor must be greater then zero")
        return v

    @field_validator("descricao")
    @classmethod
    def more_than_zero_less_then_ten(cls, v: str) -> str:
        if not (1 <= len(v) <= 10):
            raise ValueError("1 <= len(descricao) <= 10")
        return v


class BalanceOutput(BaseModel):
    limite: int
    total: int
    data_extrato: datetime
