ALTER TABLE "public"."cards" ADD COLUMN "tags" text[] NOT NULL DEFAULT array[]::text[];
CREATE INDEX "cards_tags" ON "public"."cards" USING BTREE ("tags");
