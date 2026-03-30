//! redis 工具模块使用例子.
//!
//! 需要的依赖:
//!
//! ```toml
//! [dependencies]
//! anyhow = "1.0.100"
//! tokio = { version = "1.49.0", features = ["full"] }
//! serde = { version = "1.0.228", features = ["derive"] }
//! fred = { version = "10.1.0", default-features = false, features = ["dynamic-pool", "i-all", "transactions"] }
//! ```

use anyhow::Result;
use fred::prelude::*;
use fred::socket2::TcpKeepalive;
use serde::Deserialize;
use std::time::Duration;

/// Redis 配置.
#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    /// 连接 URL.
    pub url: String,
    /// TCP 保持活动时间.
    pub with_time: u64,
    /// TCP 保持活动间隔.
    pub with_interval: u64,
    /// 连接超时 (毫秒).
    pub connection_timeout: u64,
    /// 连接池大小.
    pub pool_size: usize,
}

/// 创建 Redis 连接池.
///
/// # 参数
///
/// - `redis_config`: Redis 配置.
///
/// # Errors
///
/// - 提供的 `url` 不是合法的 Redis 连接字符串;
/// - 无法建立到 Redis 服务器的连接;
/// - 构建连接池时发生配置或连接错误.
pub async fn connection(redis_config: RedisConfig) -> Result<Pool> {
    let tcp_config = TcpConfig {
        nodelay: Some(true),
        linger: Some(Duration::from_secs(0)),
        ttl: Some(64),
        keepalive: Some(
            TcpKeepalive::new()
                .with_time(Duration::from_secs(redis_config.with_time))
                .with_interval(Duration::from_secs(redis_config.with_interval)),
        ),
    };

    let url = redis_config.url;
    let pool = Builder::from_config(Config::from_url(&url)?)
        .with_connection_config(|config| {
            config.connection_timeout = Duration::from_secs(redis_config.connection_timeout);
            config.tcp = tcp_config;
        })
        .build_pool(redis_config.pool_size)?;
    pool.init().await?;

    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<()> {
    let redis_config = RedisConfig {
        url: "redis://:2BcyTEk2mxo3yGu4Vgs0GNlQ@192.168.200.196:46573/8".to_string(),
        with_time: 60,
        with_interval: 10,
        connection_timeout: 10,
        pool_size: 10,
    };
    let pool = connection(redis_config).await?;
    let r = pool.get::<String, _>("t").await?;
    println!("{r}");

    Ok(())
}
