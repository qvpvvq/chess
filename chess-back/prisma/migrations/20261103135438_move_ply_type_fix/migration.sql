/*
  Warnings:

  - Changed the type of `ply` on the `Move` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- AlterTable
ALTER TABLE "Move" DROP COLUMN "ply",
ADD COLUMN     "ply" INTEGER NOT NULL;

-- CreateIndex
CREATE UNIQUE INDEX "Move_gameId_ply_key" ON "Move"("gameId", "ply");
