-- AlterTable
ALTER TABLE "Game" ADD COLUMN     "comments" TEXT[] DEFAULT ARRAY[]::TEXT[],
ADD COLUMN     "persona" TEXT;

-- AlterTable
ALTER TABLE "Move" ADD COLUMN     "comment" TEXT,
ADD COLUMN     "evalCp" TEXT;
