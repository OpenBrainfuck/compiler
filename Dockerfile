FROM rust:1.61.0

WORKDIR /usr/src/openbrainfuck
COPY . .

RUN cargo install --path .

CMD ["openbrainfuck"]
