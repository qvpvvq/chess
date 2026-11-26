docker-compose up

cd chess-front
pnpm add .
npm run dev

cd chess-back
pnpm add .
npx prisma generate
npx prisma migrate deploy
npm run start:dev

cd chess-llm
python -m venv venv
venv\Scripts\activate
pip install -r requirements.txt
python -m app/worker.py

за env смотреть env.example в папках
