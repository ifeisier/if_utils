//! mysql 工具模块.

pub use crate::extract_field;

/// 将 mysql 中的字段值转换为 rust 中的字段值.
#[macro_export]
macro_rules! extract_field {
    ($row:expr, $i:expr, $struct:ident, $field:ident, $type:ty) => {
        if let Some(result) = $row.get_opt::<$type, _>($i) {
            match result {
                Ok(v) => $struct.$field = v,
                Err(e) => {
                    log::error!("解析 {} 字段错误: {:?}", stringify!($field), e);
                    return Err(FromRowError($row));
                }
            }
        }
    };
}
