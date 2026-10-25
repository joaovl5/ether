CREATE TABLE "store_object_refs" (
    "lhs_id" BLOB NOT NULL,
    "rhs_id" BLOB NOT NULL,
    "ref_type" TEXT NOT NULL CHECK ("ref_type" IN ('produces')),
    PRIMARY KEY ("lhs_id", "rhs_id")
);
-- #[toasty::breakpoint]
CREATE INDEX "index_store_object_refs_by_lhs_id" ON "store_object_refs" ("lhs_id");
-- #[toasty::breakpoint]
CREATE INDEX "index_store_object_refs_by_rhs_id" ON "store_object_refs" ("rhs_id");
-- #[toasty::breakpoint]
CREATE INDEX "index_store_object_refs_by_ref_type" ON "store_object_refs" ("ref_type");
-- #[toasty::breakpoint]
CREATE TABLE "store_objects" (
    "id" BLOB NOT NULL,
    "hash" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
-- #[toasty::breakpoint]
CREATE INDEX "index_store_objects_by_hash" ON "store_objects" ("hash");
