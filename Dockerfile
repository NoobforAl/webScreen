FROM rust:1.70.0-alpine

RUN apk update
RUN apk add --no-cache chromium

WORKDIR /app

COPY . /app/

CMD [ "cargo", "run" ]