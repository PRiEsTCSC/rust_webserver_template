FROM rust:1.83 AS builder
WORKDIR /usr/src/{{project-name}}
COPY . .
RUN cargo build --release

FROM rust:1.83-slim
WORKDIR /usr/src/{{project-name}}
COPY --from=builder /usr/src/{{project-name}}/target/release/{{project-name}} .
COPY log4rs.yaml .
EXPOSE {{server_port}}
CMD ["./{{project-name}}"]