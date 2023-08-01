FROM node:slim AS frontend-builder
WORKDIR /svelte
COPY ./svelte/package.json ./
COPY ./svelte/package-lock.json ./
RUN npm install
COPY ./svelte ./
RUN npm run build

FROM rust:1.70.0-slim-bookworm as backend-builder
WORKDIR /axum
COPY ./axum ./
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /marks
RUN apt update && apt install ripgrep
COPY --from=frontend-builder /svelte/dist /marks/static
COPY --from=backend-builder /axum/target/release/marks .
ENV RUST_LOG info
EXPOSE 8080
ENTRYPOINT [ "./marks" ]
