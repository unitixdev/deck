FROM oven/bun:1.1.27 AS builder
WORKDIR /app

COPY package.json bun.lockb . /app/
RUN bun install --production --frozen-lockfile

RUN bun run build

FROM oven/bun:1.1.27-distroless
WORKDIR /app

COPY --from=builder /app/.next/standalone /app
COPY --from=builder /app/.next/static /app/.next/static

CMD ["/app/server.js"]