FROM rustlang/rust:nightly

WORKDIR /usr/src/stashr
COPY . .

RUN cargo install --path .

WORKDIR /usr/src/stashr/test-dir

RUN mkdir subdir
RUN touch a
RUN touch subdir/b

CMD ["../test/test-cmd.sh"]

