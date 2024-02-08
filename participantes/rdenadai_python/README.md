```bash
$> docker build . -f Dockerfile -t rdenadai/rinha-backend-2024q1 && docker run -dp 8080:8080 rdenadai/rinha-backend-2024q1

$> poetry export -f requirements.txt --without-hashes --without-urls --output requirements.txt

$> ab -n 5000 -c 100 "http://localhost:9999/clientes/1/extrato"
$> ab -n 5000 -c 100 "http://localhost:9999/clientes/1/extrato"
```
