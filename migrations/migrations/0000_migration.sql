CREATE TABLE "store_objects" (
    "id" BLOB NOT NULL,
    "hash" TEXT NOT NULL,
    "comes_from_id" BLOB,
    PRIMARY KEY ("id")
);
-- #[toasty::breakpoint]
CREATE INDEX "index_store_objects_by_comes_from_id" ON "store_objects" ("comes_from_id");
