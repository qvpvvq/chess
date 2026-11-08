docker-compose up

cd chess-front
pnpm add .
npm run dev

cd chess-back
pnpm add .
npx prisma generate
npx prisma migrate deploy
npm run start:dev

за env смотреть env.example в проекте
