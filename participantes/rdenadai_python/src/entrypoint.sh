#!/bin/sh

cd /src

uvicorn app:app  --host 0.0.0.0 --port 8080 --loop uvloop --timeout-keep-alive 300
