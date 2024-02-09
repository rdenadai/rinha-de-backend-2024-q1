## Rinha Backend 2024Q1

Nome: _Rodolfo De Nadai_

### Tecnologias

- Application:
  - Python 3.12.2
  - starlette
  - pydantic
  - orjson
  - asyncpg
  - uvicorn && gunicorn
- Database
  - PostgreSQL 16.1

### Comandos úteis

- Construir a imagem docker usada na aplicação

```bash
$> docker build . -f Dockerfile -t rdenadai/rinha-backend-2024q1 && docker run -dp 8080:8080 rdenadai/rinha-backend-2024q1
```

- Exportar as dependências de código do projeto

```bash
$> poetry export -f requirements.txt --without-hashes --without-urls --output requirements.txt
```

- Benchmark com ApacheBench

```bash
$> ab -n 5000 -c 100 "http://localhost:9999/clientes/1/extrato"
$> ab -n 5000 -c 100 "http://localhost:9999/clientes/2/extrato"

$> ab -n 1000 -c 100 -T 'application/json' -p transacao.json "http://localhost:9999/clientes/1/transacoes"
```
