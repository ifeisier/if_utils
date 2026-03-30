//! mysql 工具模块使用例子.
//!
//! 需要的依赖:
//!
//! ```toml
//! [dependencies]
//! flate2 = "1.0.13"
//! tokio = { version = "1.49.0", features = ["full"] }
//! chrono = "0.4.43"
//! mysql_async = { version = "0.36.1", default-features = false, features = ["rust_decimal", "bigdecimal", "chrono"] }
//! ```

use anyhow::anyhow;
use if_utils::extract_field;
use serde::Deserialize;

use mysql_async::prelude::*;
use mysql_async::*;

#[derive(Debug, Clone, Default)]
pub struct Example {
    date: chrono::NaiveDate,
}
impl FromRow for Example {
    fn from_row_opt(row: Row) -> anyhow::Result<Self, FromRowError>
    where
        Self: Sized,
    {
        let mut example = Example::default();

        for (i, column) in row.columns_ref().iter().enumerate() {
            match column.name_str().as_ref() {
                "date" => extract_field!(row, i, example, date, chrono::NaiveDate),
                v => {
                    log::error!("未知字段: {v}");
                    return Err(FromRowError(row));
                }
            }
        }

        Ok(example)
    }
}

/// `MySQL` 数据库配置.
#[derive(Debug, Deserialize)]
pub struct MysqlConfig {
    /// 数据库地址.
    pub host: String,
    /// 数据库端口.
    pub port: u16,
    /// 数据库用户名.
    pub user_name: String,
    /// 数据库密码.
    pub pass_word: String,
    /// 数据库名称.
    pub db_name: String,
    /// 缓存大小.
    pub stmt_cache_size: usize,
    /// 连接池最小连接数.
    pub pool_min: usize,
    /// 连接池最大连接数.
    pub pool_max: usize,
}

/// 创建 mysql 连接池
///
/// # 参数
/// * `mysql_config` - `MySQL` 数据库配置
///
/// # Errors
/// - 创建连接池失败时返回错误
pub fn connection(mysql_config: MysqlConfig) -> anyhow::Result<Pool> {
    let pc = PoolConstraints::new(mysql_config.pool_min, mysql_config.pool_max)
        .ok_or_else(|| anyhow!("创建 Mysql 连接池失败."))?;

    let opts_builder = OptsBuilder::default()
        .pool_opts(PoolOpts::default().with_constraints(pc))
        .ip_or_hostname(mysql_config.host)
        .tcp_port(mysql_config.port)
        .user(Some(mysql_config.user_name))
        .pass(Some(mysql_config.pass_word))
        .db_name(Some(mysql_config.db_name))
        .tcp_keepalive(Some(5000u32))
        .compression(Compression::best())
        .secure_auth(true)
        .stmt_cache_size(mysql_config.stmt_cache_size);

    Ok(Pool::new(Opts::from(opts_builder)))
}

/// 创建一个 可重复读(REPEATABLE READ) 级别的事务
///
/// # Errors
///
/// - 数据库连接异常断开
/// - 事务开启命令执行失败
/// - 权限不足无法开启事务
/// - `MySQL` 服务端返回错误
pub async fn start_repeatable_read_transaction(conn: &mut Conn) -> Result<Transaction<'_>> {
    let mut tx_opts = TxOpts::default();
    tx_opts.with_isolation_level(Some(IsolationLevel::RepeatableRead));

    conn.start_transaction(tx_opts).await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mysql_config = MysqlConfig {
        host: "127.0.0.1".to_string(),
        port: 31865,
        user_name: "root".to_string(),
        pass_word: "".to_string(),
        db_name: "".to_string(),
        stmt_cache_size: 30,
        pool_min: 5,
        pool_max: 20,
    };

    let pool = connection(mysql_config)?;
    let mut conn = pool.get_conn().await?;
    let r = conn
        .exec::<Example, _, _>("SELECT `date` FROM data LIMIT :l", params! { "l" => 2})
        .await;
    println!("{r:?}");

    Ok(())
}
