# Migrations

配置相关表`asset_type`和`action_type`均使用了`PostgreSQL`提供的高级功能，包括外键、触发器和约束等，以确保数据的完整性和一致性。每当对这些表进行操作时，例如插入、更新或删除数据，系统会自动将操作记录写入`change_log`表中，从而实现对数据变更的完整追踪和记录。

## action_type

### `change_enum`枚举释义

- `INC` 增加
- `DEC` 减少
- `NONE` 无变化

## account

可用余额 + 冻结余额 = 余额

## account_log

当`amount_x<0`时，表示`account.x`字段扣减

当`amount_x>0`时，表示`account.x`字段增加

当`amount_x=0`时，表示`account.x`字段无变化
