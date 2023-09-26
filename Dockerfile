FROM rustlang/rust:nightly

WORKDIR /usr/src/stashr
COPY . .

RUN cargo install --path .

CMD ["./test/test-cmd.sh"]

