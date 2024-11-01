-- Add migration script here
CREATE TYPE change_enum AS ENUM (
    'INC',
    'DEC',
    'NONE'
);

CREATE TABLE IF NOT EXISTS "public"."action_type" (
    "id" serial PRIMARY KEY,
    "name" text UNIQUE NOT NULL,
    "description" text NOT NULL DEFAULT '',
    "available_balance_change" change_enum NOT NULL,
    "frozen_balance_change" change_enum NOT NULL,
    "total_income_change" change_enum NOT NULL,
    "total_expense_change" change_enum NOT NULL,
    "is_active" boolean NOT NULL DEFAULT FALSE,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON COLUMN "public"."action_type"."id" IS '主键自增id';

COMMENT ON COLUMN "public"."action_type"."name" IS '操作类型名称';

COMMENT ON COLUMN "public"."action_type"."description" IS '操作类型描述';

COMMENT ON COLUMN "public"."action_type"."available_balance_change" IS '账户可用余额变化';

COMMENT ON COLUMN "public"."action_type"."frozen_balance_change" IS '账户冻结余额变化';

COMMENT ON COLUMN "public"."action_type"."total_income_change" IS '账户累计收入变化';

COMMENT ON COLUMN "public"."action_type"."total_expense_change" IS '账户累计支出变化';

COMMENT ON COLUMN "public"."action_type"."is_active" IS '是否启用';

COMMENT ON COLUMN "public"."action_type"."created_at" IS '创建时间';

COMMENT ON COLUMN "public"."action_type"."updated_at" IS '更新时间';

COMMENT ON TABLE "public"."action_type" IS '账户操作类型表';

INSERT INTO "public"."action_type" ("name", "description", "available_balance_change", "frozen_balance_change", "total_income_change", "total_expense_change", "is_active")
    VALUES ('AB_INC', '收入(可用余额增加 累计收入增加)', 'INC', 'NONE', 'INC', 'NONE', 't'),
    ('AB_INC_RTN', '收入退还(可用余额减少 累计收入减少)', 'DEC', 'NONE', 'DEC', 'NONE', 't'),
    ('AB_EXP', '支出(可用余额减少 累计支出增加)', 'DEC', 'NONE', 'NONE', 'INC', 't'),
    ('AB_EXP_RTN', '支出退还(可用余额增加 累计支出减少)', 'INC', 'NONE', 'NONE', 'DEC', 't'),
    ('FB_INC', '收入(冻结余额增加 累计收入增加)', 'NONE', 'INC', 'INC', 'NONE', 't'),
    ('FB_INC_RTN', '收入退还(冻结余额减少 累计收入减少)', 'NONE', 'DEC', 'DEC', 'NONE', 't'),
    ('FB_EXP', '支出(冻结余额减少 累计支出增加)', 'NONE', 'DEC', 'NONE', 'INC', 't'),
    ('FB_EXP_RTN', '支出退还(冻结余额增加 累计支出减少)', 'NONE', 'INC', 'NONE', 'DEC', 't'),
    ('FRZ', '冻结(可用余额减少 冻结余额增加)', 'DEC', 'INC', 'NONE', 'NONE', 't'),
    ('UFZ', '解冻(可用余额增加 冻结余额减少)', 'INC', 'DEC', 'NONE', 'NONE', 't'),
    ('FIX_AB_INC', '修复可用余额(可用余额增加)', 'INC', 'NONE', 'NONE', 'NONE', 'f'),
    ('FIX_AB_DEC', '修复可用余额(可用余额减少)', 'DEC', 'NONE', 'NONE', 'NONE', 'f'),
    ('FIX_FB_INC', '修复冻结余额(冻结余额增加)', 'NONE', 'INC', 'NONE', 'NONE', 'f'),
    ('FIX_FB_DEC', '修复冻结余额(冻结余额减少)', 'NONE', 'DEC', 'NONE', 'NONE', 'f'),
    ('FIX_TI_INC', '修复累计收入(累计收入增加)', 'NONE', 'NONE', 'INC', 'NONE', 'f'),
    ('FIX_TI_DEC', '修复累计收入(累计收入减少)', 'NONE', 'NONE', 'DEC', 'NONE', 'f'),
    ('FIX_TE_INC', '修复累计支出(累计支出增加)', 'NONE', 'NONE', 'NONE', 'INC', 'f'),
    ('FIX_TE_DEC', '修复累计支出(累计支出减少)', 'NONE', 'NONE', 'NONE', 'DEC', 'f');
