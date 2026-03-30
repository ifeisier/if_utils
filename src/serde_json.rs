//! JSON 工具库
//!
//! 提供用于处理 JSON 数据的转换和提取功能,
//! 帮助开发者更方便地操作 [Value] 类型.

use anyhow::{Result, anyhow};
use serde_json::{Map, Value};

/// 提供将 [Value] 类型转换为目标类型的方法.
pub struct Convert;

/// 提供从 [Value] 类型中提取特定字段并转换为目标类型的方法.
pub struct Extract;

#[allow(dead_code)]
impl Convert {
    /// 尝试将 JSON 值转换为数组 (`Vec<Value>`).
    ///
    /// # 参数
    /// - `value`: 要转换的 JSON 值.
    ///
    /// # 返回
    /// 如果 `value` 是数组类型, 返回该数组的引用.
    ///
    /// # Errors
    /// 当 `value` 不是数组类型时返回错误.
    pub fn try_into_array(value: &Value) -> Result<&Vec<Value>> {
        value
            .as_array()
            .ok_or_else(|| anyhow!("字段类型错误,应为数组."))
    }

    /// 尝试将 JSON 值转换为对象 (`Map<String, Value>`).
    ///
    /// # 参数
    /// - `value`: 要转换的 JSON 值.
    ///
    /// # 返回
    /// 如果 `value` 是一个对象, 返回对该对象的引用.
    ///
    /// # Errors
    /// 当 `value` 不是对象类型时返回错误.
    pub fn try_into_map(value: &Value) -> Result<&Map<String, Value>> {
        value
            .as_object()
            .ok_or_else(|| anyhow!("字段类型错误,应为对象."))
    }

    /// 尝试将 JSON 值转换为字符串引用.
    ///
    /// # 参数
    /// - `value`: 要转换的 JSON 值.
    ///
    /// # 返回
    /// 如果 `value` 是字符串, 返回其引用.
    ///
    /// # Errors
    /// 当 `value` 不是字符串类型时返回错误.
    pub fn try_into_str(value: &Value) -> Result<&str> {
        value
            .as_str()
            .ok_or_else(|| anyhow!("字段类型错误,应为字符串."))
    }

    /// 尝试将 JSON 值转换为 `f64` 浮点数.
    ///
    /// # 参数
    /// - `value`: 要转换的 JSON 值.
    ///
    /// # 返回
    /// 如果 `value` 是数字类型, 返回其 `f64` 表示.
    ///
    /// # Errors
    /// 当 `value` 不是数字类型时返回错误.
    pub fn try_into_f64(value: &Value) -> Result<f64> {
        value
            .as_f64()
            .ok_or_else(|| anyhow!("字段类型错误,应为浮点数."))
    }

    /// 尝试将 JSON 值转换为 `u64` 整数.
    ///
    /// # 参数
    /// - `value`: 要转换的 JSON 值.
    ///
    /// # 返回
    /// 如果 `value` 是数字类型, 返回其 `u64` 表示.
    ///
    /// # Errors
    /// 当 `value` 不是无符号整数时返回错误.
    pub fn try_into_u64(value: &Value) -> Result<u64> {
        value
            .as_u64()
            .ok_or_else(|| anyhow!("字段类型错误,应为无符号整数."))
    }

    /// 尝试将 JSON 值转换为 `i64` 整数.
    ///
    /// # 参数
    /// - `value`: 要转换的 JSON 值.
    ///
    /// # 返回
    /// 如果 `value` 是数字类型, 返回其 `i64` 表示.
    ///
    /// # Errors
    /// 当 `value` 不是有符号整数时返回错误.
    pub fn try_into_i64(value: &Value) -> Result<i64> {
        value
            .as_i64()
            .ok_or_else(|| anyhow!("字段类型错误,应为有符号整数."))
    }

    /// 尝试将 JSON 值转换为布尔值.
    ///
    /// # 参数
    /// - `value`: 要转换的 JSON 值.
    ///
    /// # 返回
    /// 如果 `value` 是布尔类型, 返回其值.
    ///
    /// # Errors
    /// 当 `value` 不是布尔类型时返回错误.
    pub fn try_into_bool(value: &Value) -> Result<bool> {
        value
            .as_bool()
            .ok_or_else(|| anyhow!("字段类型错误,应为布尔值."))
    }
}

#[allow(dead_code)]
impl Extract {
    /// 从 JSON 对象中提取指定字段, 并尝试将其转换为数组 (`Vec<Value>`).
    ///
    /// # 参数
    /// - `value`: JSON 对象.
    /// - `key`: 要提取的字段名.
    ///
    /// # 返回
    /// 如果字段存在且类型为数组, 返回该数组的引用.
    ///
    /// # Errors
    /// 当字段缺失或字段类型不是数组时返回错误.
    pub fn get_array<'a>(value: &'a Value, key: &str) -> Result<&'a Vec<Value>> {
        value
            .get(key)
            .ok_or_else(|| anyhow!("缺失必要字段."))?
            .as_array()
            .ok_or_else(|| anyhow!("字段类型错误,应为数组."))
    }

    /// 从 JSON 对象中提取指定字段, 并尝试将其转换为对象 (`Map<String, Value>`).
    ///
    /// # 参数
    /// - `value`: JSON 对象.
    /// - `key`: 要提取的字段名.
    ///
    /// # 返回
    /// 如果字段存在且为对象, 返回引用.
    ///
    /// # Errors
    /// 当字段缺失或字段类型不是对象时返回错误.
    pub fn get_map<'a>(value: &'a Value, key: &str) -> Result<&'a Map<String, Value>> {
        value
            .get(key)
            .ok_or_else(|| anyhow!("缺失必要字段."))?
            .as_object()
            .ok_or_else(|| anyhow!("字段类型错误,应为对象."))
    }

    /// 从 JSON 对象中提取指定字段的值.
    ///
    /// # 参数
    /// - `value`: JSON 对象.
    /// - `key`: 要提取的字段名.
    ///
    /// # 返回
    /// 如果字段存在, 返回该字段的引用.
    ///
    /// # Errors
    /// 当字段缺失时返回错误.
    pub fn get_value<'a>(value: &'a Value, key: &str) -> Result<&'a Value> {
        value.get(key).ok_or_else(|| anyhow!("缺失必要字段."))
    }

    /// 从 JSON 对象中提取指定字段, 并尝试将其转换为字符串.
    ///
    /// # 参数
    /// - `value`: JSON 对象.
    /// - `key`: 要提取的字段名.
    ///
    /// # 返回
    /// 如果字段存在且为字符串, 返回引用.
    ///
    /// # Errors
    /// 当字段缺失或类型错误时返回错误.
    pub fn get_str<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
        value
            .get(key)
            .ok_or_else(|| anyhow!("缺失必要字段."))?
            .as_str()
            .ok_or_else(|| anyhow!("字段类型错误,应为字符串."))
    }

    /// 从 JSON 对象中提取指定字段, 并尝试将其转换为 `f64`.
    ///
    /// # 参数
    /// - `value`: JSON 对象.
    /// - `key`: 要提取的字段名.
    ///
    /// # 返回
    /// 如果字段存在且为数字, 返回 `f64`.
    ///
    /// # Errors
    /// 当字段缺失或类型错误时返回错误.
    pub fn get_f64(value: &Value, key: &str) -> Result<f64> {
        value
            .get(key)
            .ok_or_else(|| anyhow!("缺失必要字段."))?
            .as_f64()
            .ok_or_else(|| anyhow!("字段类型错误,应为浮点数."))
    }

    /// 从 JSON 对象中提取指定字段, 并尝试将其转换为 `u64`.
    ///
    /// # 参数
    /// - `value`: JSON 对象.
    /// - `key`: 要提取的字段名.
    ///
    /// # 返回
    /// 如果字段存在且为无符号整数, 返回 `u64`.
    ///
    /// # Errors
    /// 当字段缺失或类型错误时返回错误.
    pub fn get_u64(value: &Value, key: &str) -> Result<u64> {
        value
            .get(key)
            .ok_or_else(|| anyhow!("缺失必要字段."))?
            .as_u64()
            .ok_or_else(|| anyhow!("字段类型错误,应为无符号整数."))
    }

    /// 从 JSON 对象中提取指定字段, 并尝试将其转换为 `i64`.
    ///
    /// # 参数
    /// - `value`: JSON 对象.
    /// - `key`: 要提取的字段名.
    ///
    /// # 返回
    /// 如果字段存在且为有符号整数, 返回 `i64`.
    ///
    /// # Errors
    /// 当字段缺失或类型错误时返回错误.
    pub fn get_i64(value: &Value, key: &str) -> Result<i64> {
        value
            .get(key)
            .ok_or_else(|| anyhow!("缺失必要字段."))?
            .as_i64()
            .ok_or_else(|| anyhow!("字段类型错误,应为有符号整数."))
    }

    /// 从 JSON 对象中提取指定字段, 并尝试将其转换为布尔值.
    ///
    /// # 参数
    /// - `value`: JSON 对象.
    /// - `key`: 要提取的字段名.
    ///
    /// # 返回
    /// 如果字段存在且为布尔类型, 返回其值.
    ///
    /// # Errors
    /// 当字段缺失或类型错误时返回错误.
    pub fn get_bool(value: &Value, key: &str) -> Result<bool> {
        value
            .get(key)
            .ok_or_else(|| anyhow!("缺失必要字段."))?
            .as_bool()
            .ok_or_else(|| anyhow!("字段类型错误,应为布尔值."))
    }
}
