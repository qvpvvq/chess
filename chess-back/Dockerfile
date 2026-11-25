# syntax=docker/dockerfile:1.7

# ============================
# Stage 1: Builder
# ============================
FROM node:22-alpine AS builder

WORKDIR /app

RUN corepack enable && corepack prepare pnpm@10.34.8 --activate

# Устойчивость к плохой сети
RUN pnpm config set fetch-retries 5 \
 && pnpm config set fetch-retry-mintimeout 20000 \
 && pnpm config set fetch-retry-maxtimeout 120000 \
 && pnpm config set network-concurrency 4

COPY package.json pnpm-lock.yaml ./
COPY prisma ./prisma
COPY prisma7.config.ts ./

# Кэш pnpm-store сохраняется между попытками сборки
RUN --mount=type=cache,id=pnpm-store,target=/pnpm/store \
    pnpm config set store-dir /pnpm/store \
 && pnpm install --frozen-lockfile

COPY . .

RUN pnpm exec prisma generate
RUN pnpm run build

RUN pnpm prune --prod

# ============================
# Stage 2: Runtime
# ============================
FROM node:22-alpine AS runtime

WORKDIR /app

RUN corepack enable && corepack prepare pnpm@10.34.8 --activate

COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/prisma ./prisma
COPY --from=builder /app/prisma7.config.ts ./
COPY --from=builder /app/package.json ./

COPY docker-entrypoint.sh ./
RUN chmod +x ./docker-entrypoint.sh

EXPOSE 3000

ENTRYPOINT ["./docker-entrypoint.sh"]
CMD ["node", "dist/main.js"]