# small workaround for building linux target on windows
# build with docker build -t <some tag>
# execute docker run --rm -v <pwd>:/usr/src/todo_cmd -w /usr/src/todo_cmd <some tag>:latest cargo build --target x86_64-unknown-linux-gnu
FROM rust:1.82 as builder
RUN apt update && apt install -y build-essential && rm -rf /var/lib/apt/lists/*
