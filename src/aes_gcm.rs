//! AES (Advanced Encryption Standard, 先进加密标准) 是一种现代对称加密算法,
//! 用来安全地加密数据, 是目前全球最广泛使用的加密标准之一.

use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
use anyhow::{Result, anyhow, bail};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;

/// 使用 AES-256-GCM 对数据进行加密, 并将结果编码为 Base64 字符串
///
/// # 参数
/// - `data`: 要加密的原始数据字节切片
/// - `key`: 32 字节的 AES 密钥
///
/// # 返回值
/// - `Ok(String)`：包含 nonce 和密文的 Base64 编码字符串
/// - `Err`：加密失败或其他错误
///
/// # 实现细节
/// - 生成一个随机 12 字节的 nonce (GCM 标准长度)
/// - 使用 `Aes256Gcm` 进行加密
/// - 输出为 `[nonce | ciphertext]` 的组合, 然后进行 Base64 编码
///
/// # Errors
/// - 如果 AES-GCM 加密失败, 返回错误信息
/// - 如果随机数生成失败, 返回错误信息
/// - 如果 Base64 编码失败, 返回错误信息
pub fn encrypt_aes256_gcm_to_base64(data: &[u8], key: &[u8; 32]) -> Result<String> {
    let cipher = Aes256Gcm::new(key.into());

    let mut nonce_bytes = [0u8; 12];
    rand::fill(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|e| anyhow!("AES-GCM encrypt error: {e:?}"))?;

    let mut out = Vec::with_capacity(12 + ciphertext.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);

    Ok(BASE64_STANDARD.encode(out))
}

/// 从 Base64 字符串解码并使用 AES-256-GCM 解密数据
///
/// # 参数
/// - `encoded`: 包含 nonce 和密文的 Base64 编码字符串
/// - `key`: 32 字节的 AES 密钥
///
/// # 返回值
/// - `Ok(Vec<u8>)`：解密后的原始数据
/// - `Err`：Base64 解码错误或 AES-GCM 解密失败
///
/// # 实现细节
/// - 解码 Base64 得到 `[nonce | ciphertext]`
/// - 分离前 12 字节作为 nonce, 剩余部分作为密文
/// - 使用 `Aes256Gcm` 解密得到原始数据
///
/// # Errors
/// - 如果 Base64 解码失败, 返回错误信息
/// - 如果数据长度不足以包含 nonce, 返回错误信息
/// - 如果 AES-GCM 解密失败, 返回错误信息
pub fn decrypt_aes256_gcm_from_base64(encoded: &str, key: &[u8; 32]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(key.into());

    let data = BASE64_STANDARD
        .decode(encoded)
        .map_err(|e| anyhow!("Base64 decode error: {e:?}"))?;

    if data.len() < 12 {
        bail!("Data too short to contain nonce")
    }

    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow!("AES-GCM decrypt error: {e:?}"))?;

    Ok(plaintext)
}
