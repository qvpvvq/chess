-- CreateEnum
CREATE TYPE "moveRole" AS ENUM ('USER', 'AI');

-- AlterTable
ALTER TABLE "Move" ADD COLUMN     "role" "moveRole" NOT NULL DEFAULT 'USER';
