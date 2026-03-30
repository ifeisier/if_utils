//! reqwest 工具

use anyhow::Result;
use bytes::Bytes;
use reqwest::header::HeaderMap;
use reqwest::{Client, ClientBuilder, Proxy, header::HeaderValue};
use std::time::Duration;

static USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

/// 创建 `ClientBuilder`.
fn create_client_builder() -> ClientBuilder {
    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert(
        "Accept-Encoding",
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert("Connection", HeaderValue::from_static("Keep-Alive"));

    Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(15))
        .user_agent(USER_AGENT)
}

/// 创建一个新的 Client 实例.
///
/// # Errors
/// `ClientBuilder::build` 构建客户端失败
#[allow(dead_code)]
pub fn create_client() -> Result<Client> {
    Ok(create_client_builder().build()?)
}

/// 创建一个新的代理 Client 实例.
///
/// # Arguments
/// * `proxy_scheme` - 代理服务器的 URL.
///
/// # Errors
/// - `proxy_scheme` 格式不合法
/// - 代理初始化失败
/// - `ClientBuilder::build` 构建客户端失败
#[allow(dead_code)]
pub fn create_proxy_client(proxy_scheme: &str) -> Result<Client> {
    let proxy = Proxy::all(proxy_scheme)?;
    Ok(create_client_builder().proxy(proxy).build()?)
}

/// 发送 GET 请求并返回响应体字节
///
/// # Arguments
///
/// * `client` - 已创建的 `reqwest::Client`
/// * `url` - 请求地址
///
/// # Errors
/// - 请求发送失败 (网络错误、DNS 解析失败等)
/// - 服务器返回异常导致 `send` 失败
/// - 读取响应体失败
#[allow(dead_code)]
pub async fn get(client: &Client, url: &str) -> Result<Bytes> {
    Ok(client.get(url).send().await?.bytes().await?)
}

/// 发送 POST JSON 请求并返回响应体字节
///
/// # Arguments
///
/// * `client` - 已创建的 `reqwest::Client`
/// * `url` - 请求地址
/// * `json_data` - 需要序列化为 JSON 的数据
///
/// # Errors
/// - JSON 序列化失败
/// - 请求发送失败 (网络错误等)
/// - 服务器响应异常
/// - 读取响应体失败
#[allow(dead_code)]
pub async fn post_json<T: serde::Serialize + Sync + ?Sized>(
    client: &Client,
    url: &str,
    json_data: &T,
) -> Result<Bytes> {
    let request = client.post(url).json(json_data);
    Ok(request.send().await?.bytes().await?)
}
