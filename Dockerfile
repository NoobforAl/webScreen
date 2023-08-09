FROM rust:latest

WORKDIR /app

RUN apt update
RUN apt install -y chromium
RUN apt upgrade -y

COPY . /app

RUN cargo build --release

EXPOSE 8080

CMD [ "cargo", "run", "--", "--debug", "--run-server" ]