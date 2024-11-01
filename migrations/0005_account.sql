-- Add migration script here
CREATE TABLE IF NOT EXISTS "public"."account" (
    "id" serial PRIMARY KEY,
    "user_id" int NOT NULL,
    "asset_type_id" int NOT NULL,
    "available_balance" DECIMAL(18, 6) NOT NULL DEFAULT 0,
    "frozen_balance" DECIMAL(18, 6) NOT NULL DEFAULT 0,
    "total_income" DECIMAL(18, 6) NOT NULL DEFAULT 0,
    "total_expense" DECIMAL(18, 6) NOT NULL DEFAULT 0,
    "is_active" boolean NOT NULL DEFAULT FALSE,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE ("user_id", "asset_type_id")
);

COMMENT ON COLUMN "public"."account"."id" IS '主键自增id';

COMMENT ON COLUMN "public"."account"."user_id" IS '用户id';

COMMENT ON COLUMN "public"."account"."asset_type_id" IS '资产类型id';

COMMENT ON COLUMN "public"."account"."available_balance" IS '可用余额';

COMMENT ON COLUMN "public"."account"."frozen_balance" IS '冻结余额';

COMMENT ON COLUMN "public"."account"."total_income" IS '累计收入';

COMMENT ON COLUMN "public"."account"."total_expense" IS '累计支出';

COMMENT ON COLUMN "public"."account"."is_active" IS '是否启用';

COMMENT ON COLUMN "public"."account"."created_at" IS '创建时间';

COMMENT ON COLUMN "public"."account"."updated_at" IS '更新时间';

COMMENT ON TABLE "public"."account" IS '用户账户表';
