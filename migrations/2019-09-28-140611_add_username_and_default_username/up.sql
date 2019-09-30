ALTER TABLE "users" ADD COLUMN "username" VARCHAR;

UPDATE users
SET username = CONCAT('id', users.id)
FROM users comp
WHERE users.id = comp.id;

ALTER TABLE "users" ALTER COLUMN "username" SET NOT NULL;
