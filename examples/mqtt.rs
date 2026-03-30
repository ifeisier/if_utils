//! 用来创建 MQTT 客户端.
//!
//! 需要的依赖:
//!
//! ```toml
//! anyhow = "1.0.100"
//! tokio = { version = "1.49.0", features = ["full"] }
//! serde = { version = "1.0.228", features = ["derive"] }
//! rumqttc = { version = "0.25.1", default-features = false }
//! ```

use anyhow::Result;
use if_utils::mqtt::qos_v5;
use rumqttc::v5::{AsyncClient, Event, MqttOptions, mqttbytes::v5::*};
use serde::Deserialize;
use std::time::Duration;
use tokio::{sync::mpsc, time::sleep};

/// MQTT 客户端配置.
#[derive(Debug, Deserialize)]
pub struct MqttConfig {
    /// 客户端 ID.
    pub id: String,
    /// 服务器地址.
    pub host: String,
    /// 服务器端口.
    pub port: u16,
    /// 用户名和密码.
    pub username_password: Option<(String, String)>,
    /// 通道最大数量.
    pub channel_cap: usize,
    /// 订阅的主题列表.
    pub subscribes: Vec<(u8, String)>,
}

/// MQTT v5.0 客户端
pub struct MQTTV5Client;
#[allow(dead_code)]
impl MQTTV5Client {
    /// 连接到 MQTT 服务器并返回异步客户端和事件接收器
    ///
    /// # 参数
    /// * `mqtt_config` - MQTT 客户端配置
    ///
    /// # Errors
    /// - 连接服务器失败时返回错误
    /// - 初始订阅主题失败时返回错误
    /// - 创建异步通道失败时返回错误
    pub fn connect(mqtt_config: MqttConfig) -> Result<(AsyncClient, mpsc::Receiver<Event>)> {
        let mut options = MqttOptions::new(&mqtt_config.id, &mqtt_config.host, mqtt_config.port);
        options.set_keep_alive(Duration::from_secs(10));
        options.set_clean_start(true);
        options.set_connection_timeout(30);
        options.set_max_packet_size(Some(1_048_576)); // 1048576Byte = 1MB

        if let Some(v) = mqtt_config.username_password {
            options.set_credentials(&v.0, &v.1);
        }

        let (client, mut event_loop) = AsyncClient::new(options, mqtt_config.channel_cap);
        // let restore_subs = mqtt_config.subscribes.clone();
        let restore_client = client.clone();

        let (tx, event_rx) = mpsc::channel::<Event>(mqtt_config.channel_cap);
        tokio::spawn(async move {
            loop {
                match event_loop.poll().await {
                    Ok(event) => {
                        if let Event::Incoming(Packet::ConnAck(_ack)) = &event {
                            log::info!("MQTT 已连接, 开始订阅:{:?}.", mqtt_config.subscribes);

                            let r = subscribe(&restore_client, &mqtt_config.subscribes).await;
                            if let Err(e) = r {
                                log::error!("重连后订阅 {:?} 失败: {e}", mqtt_config.subscribes);
                                break;
                            }
                        }

                        if let Err(e) = tx.send(event).await {
                            log::error!("将MQTT事件发送到通道错误:{e:?}");
                            break;
                        }
                    }
                    Err(e) => {
                        log::error!("接收MQTT事件错误:{e:?}");
                        sleep(Duration::from_secs(10)).await;
                    }
                }
            }
        });

        Ok((client, event_rx))
    }
}

/// 订阅
async fn subscribe(client: &AsyncClient, subscribes: &[(u8, String)]) -> Result<()> {
    for (q, topic) in subscribes {
        let qos = qos_v5(*q)?;
        client.subscribe(topic, qos).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mqtt_config = MqttConfig {
        id: "".to_string(),
        host: "127.0.0.1".to_string(),
        port: 2883,
        username_password: None,
        channel_cap: 1024,
        subscribes: vec![(0, "".to_string())],
    };
    let (_, mut mqtt_event) = MQTTV5Client::connect(mqtt_config).unwrap();
    tokio::spawn(async move {
        loop {
            if let Some(event) = mqtt_event.recv().await {
                if let Event::Incoming(Packet::Publish(publish)) = event {}
            } else {
                println!("MQTT event channel closed");
                break;
            }
        }
    });

    Ok(())
}
