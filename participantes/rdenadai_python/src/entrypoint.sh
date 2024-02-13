#!/bin/sh

cd /src

gunicorn app:app -b 0.0.0.0:8080 -w 1 -k uvicorn.workers.UvicornWorker -t 300 --access-logfile - --error-logfile -
