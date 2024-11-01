-- Add migration script here
CREATE TABLE IF NOT EXISTS "public"."change_log" (
    "log_id" serial PRIMARY KEY,
    "table_name" text NOT NULL,
    "operation_type" text NOT NULL,
    "old_data" jsonb,
    "new_data" jsonb,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON COLUMN "public"."change_log"."log_id" IS '主键自增id';

COMMENT ON COLUMN "public"."change_log"."table_name" IS '表名';

COMMENT ON COLUMN "public"."change_log"."operation_type" IS '操作类型';

COMMENT ON COLUMN "public"."change_log"."old_data" IS '旧数据';

COMMENT ON COLUMN "public"."change_log"."new_data" IS '新数据';

COMMENT ON COLUMN "public"."change_log"."created_at" IS '记录创建时间';

COMMENT ON TABLE "public"."change_log" IS '数据变更日志表';
