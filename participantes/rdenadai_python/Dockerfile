FROM python:3.12.2-alpine3.19

WORKDIR /src

ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1

COPY ./requirements.txt .
ADD ./src .

RUN pip install setuptools==44.1.1 --upgrade && \
    pip install --no-cache-dir -r requirements.txt

ENTRYPOINT ["/bin/sh", "/src/entrypoint.sh"]