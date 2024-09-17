FROM rust:1.78 AS server-builder
WORKDIR /app
COPY server .
RUN cargo build --release

FROM node:22.2 AS client-builder
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
WORKDIR /app
COPY client .
RUN pnpm install --frozen-lockfile
RUN NODE_ENV=production ./node_modules/.bin/rspack build

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=server-builder /app/target/release/queue /app/queue
COPY --from=client-builder /app/dist /app/dist
# ENTRYPOINT ["/usr/local/bin/queue"]
