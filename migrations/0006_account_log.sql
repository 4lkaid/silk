-- Add migration script here
CREATE TABLE IF NOT EXISTS "public"."account_log" (
    "id" bigserial PRIMARY KEY,
    "account_id" int NOT NULL,
    "action_type_id" int NOT NULL,
    "amount_available_balance" DECIMAL(18, 6) NOT NULL,
    "amount_frozen_balance" DECIMAL(18, 6) NOT NULL,
    "amount_total_income" DECIMAL(18, 6) NOT NULL,
    "amount_total_expense" DECIMAL(18, 6) NOT NULL,
    "available_balance_after" DECIMAL(18, 6) NOT NULL,
    "frozen_balance_after" DECIMAL(18, 6) NOT NULL,
    "total_income_after" DECIMAL(18, 6) NOT NULL,
    "total_expense_after" DECIMAL(18, 6) NOT NULL,
    "order_number" text NOT NULL DEFAULT '',
    "description" text NOT NULL DEFAULT '',
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX account_log_account_id_action_type_id_idx ON "public"."account_log" ("account_id", "action_type_id");

COMMENT ON COLUMN "public"."account_log"."id" IS '主键自增id';

COMMENT ON COLUMN "public"."account_log"."account_id" IS '账户id';

COMMENT ON COLUMN "public"."account_log"."action_type_id" IS '操作类型id';

COMMENT ON COLUMN "public"."account_log"."amount_available_balance" IS '可用余额操作金额';

COMMENT ON COLUMN "public"."account_log"."amount_frozen_balance" IS '冻结余额操作金额';

COMMENT ON COLUMN "public"."account_log"."amount_total_income" IS '累计收入操作金额';

COMMENT ON COLUMN "public"."account_log"."amount_total_expense" IS '累计支出操作金额';

COMMENT ON COLUMN "public"."account_log"."available_balance_after" IS '操作后可用余额';

COMMENT ON COLUMN "public"."account_log"."frozen_balance_after" IS '操作后冻结余额';

COMMENT ON COLUMN "public"."account_log"."total_income_after" IS '操作后累计收入';

COMMENT ON COLUMN "public"."account_log"."total_expense_after" IS '操作后累计支出';

COMMENT ON COLUMN "public"."account_log"."order_number" IS '订单号';

COMMENT ON COLUMN "public"."account_log"."description" IS '操作描述';

COMMENT ON COLUMN "public"."account_log"."created_at" IS '日志创建时间';

COMMENT ON TABLE "public"."account_log" IS '账户操作日志表';
