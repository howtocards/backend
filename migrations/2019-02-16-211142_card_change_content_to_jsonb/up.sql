DELETE FROM "useful_marks";
DELETE FROM "cards";
ALTER TABLE "cards" DROP COLUMN "content";
ALTER TABLE "cards" ADD COLUMN "content" jsonb NOT NULL;
