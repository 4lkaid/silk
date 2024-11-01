-- Add migration script here
-- 创建触发器函数用于捕获数据变更并插入日志记录
CREATE OR REPLACE FUNCTION track_change ()
    RETURNS TRIGGER
    AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO change_log (TABLE_NAME, operation_type, new_data)
            VALUES (TG_TABLE_NAME, 'INSERT', ROW_TO_JSON(NEW)::jsonb - 'created_at' - 'updated_at');
    ELSIF TG_OP = 'UPDATE'
            AND NEW IS DISTINCT FROM OLD THEN
            INSERT INTO change_log (TABLE_NAME, operation_type, old_data, new_data)
                VALUES (TG_TABLE_NAME, 'UPDATE', ROW_TO_JSON(OLD)::jsonb - 'created_at' - 'updated_at', ROW_TO_JSON(NEW)::jsonb - 'created_at' - 'updated_at');
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO change_log (TABLE_NAME, operation_type, old_data)
            VALUES (TG_TABLE_NAME, 'DELETE', ROW_TO_JSON(OLD)::jsonb - 'created_at' - 'updated_at');
    END IF;
    RETURN NEW;
END;
$$
LANGUAGE plpgsql;

-- 创建触发器函数用于更新时间戳
CREATE OR REPLACE FUNCTION update_timestamp ()
    RETURNS TRIGGER
    AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$
LANGUAGE plpgsql;

-- "public"."asset_type"
CREATE TRIGGER update_asset_type_timestamp
    BEFORE UPDATE ON "public"."asset_type"
    FOR EACH ROW
    WHEN (NEW IS DISTINCT FROM OLD)
    EXECUTE FUNCTION update_timestamp ();

CREATE TRIGGER track_asset_type_change
    AFTER INSERT OR UPDATE OR DELETE ON "public"."asset_type"
    FOR EACH ROW
    EXECUTE FUNCTION track_change ();

-- "public"."action_type"
CREATE TRIGGER update_action_type_timestamp
    BEFORE UPDATE ON "public"."action_type"
    FOR EACH ROW
    WHEN (NEW IS DISTINCT FROM OLD)
    EXECUTE FUNCTION update_timestamp ();

CREATE TRIGGER track_action_type_change
    AFTER INSERT OR UPDATE OR DELETE ON "public"."action_type"
    FOR EACH ROW
    EXECUTE FUNCTION track_change ();
