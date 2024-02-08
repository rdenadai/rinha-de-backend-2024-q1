#!/bin/sh

cd /src

sleep 5

if [ "$MODE" = "development" ]; then
    pip install debugpy -t /tmp && python /tmp/debugpy --wait-for-client --listen 0.0.0.0:5678 -m uvicorn app:app --host 0.0.0.0 --port 8080 --reload 
else
    gunicorn app:app -b 0.0.0.0:8080 -w 2 -k uvicorn.workers.UvicornWorker -t 300 --access-logfile - --error-logfile -
fi
