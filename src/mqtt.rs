//! mqtt 工具模块.

use anyhow::Result;
use rumqttc::v5::mqttbytes::Error;
use rumqttc::v5::{AsyncClient, mqttbytes::QoS};

/// 发送 MQTT 消息
///
/// 使用给定的客户端、主题、服务质量 (QoS) 和负载发送一条 MQTT 消息.
///
/// 此函数内部使用 `try_publish`, 如果客户端的内部有界队列已满, 会立即返回错误而非阻塞等待.
///
/// # Errors
///
/// - 如果输入的 `qos` 不是 0、1 或 2, 将通过 [`qos_v5`] 返回 [`Error::InvalidQoS`].
/// - 如果客户端的内部消息队列已满, `try_publish` 将返回错误.
pub async fn publish(client: &AsyncClient, topic: &str, qos: u8, payload: Vec<u8>) -> Result<()> {
    let qos = qos_v5(qos)?;

    // publish 和 try_publish 的区别
    // publish: 如果有界队列满了, 那么就会阻塞
    // try_publish: 如果有界队列满了, 那么就会返回错误
    client.try_publish(topic, qos, false, payload)?;
    Ok(())
}

/// 发送 MQTT 保留消息
///
/// 发送一条带有保留 (Retain) 标志的 MQTT 消息.
///
/// Broker 会保留该消息, 直到有新的订阅者连接并接收.
///
/// 此函数内部使用 `try_publish`, 如果客户端的内部有界队列已满, 会立即返回错误而非阻塞等待.
///
/// # Errors
///
/// - 如果输入的 `qos` 不是 0、1 或 2, 将通过 [`qos_v5`] 返回 [`Error::InvalidQoS`].
/// - 如果客户端的内部消息队列已满, `try_publish` 将返回错误.
pub async fn publish_retain(
    client: &AsyncClient,
    topic: &str,
    qos: u8,
    payload: Vec<u8>,
) -> Result<()> {
    let qos = qos_v5(qos)?;
    client.try_publish(topic, qos, true, payload)?;
    Ok(())
}

/// 判断和返回 v5.0 的 qos
///
/// 将一个 `u8` 类型的数值转换为 MQTT v5.0 协议下的 [`QoS`] 枚举.
///
/// # Errors
///
/// 如果输入的 `qos` 不是 0 (AtMostOnce)、1 (AtLeastOnce) 或 2 (ExactlyOnce),
/// 将返回 [`Error::InvalidQoS`].
pub fn qos_v5(qos: u8) -> Result<QoS> {
    Ok(match qos {
        0 => Ok(QoS::AtMostOnce),
        1 => Ok(QoS::AtLeastOnce),
        2 => Ok(QoS::ExactlyOnce),
        qos => Err(Error::InvalidQoS(qos)),
    }?)
}
