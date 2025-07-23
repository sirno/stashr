FROM rust:1.88

WORKDIR /usr/src/stashr
COPY . .

RUN cargo install --path .

CMD ["./test/test-cmd.sh"]

