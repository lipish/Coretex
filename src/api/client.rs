use crate::Result;
use bytes::Bytes;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// 一个简单的 TCP 客户端示例，实际生产环境建议用 gRPC/HTTP/自定义协议
pub struct Client {
    addr: SocketAddr,
}

impl Client {
    pub async fn new(addr: impl Into<SocketAddr>) -> Result<Self> {
        // 这里只是简单保存地址，不做连接池
        Ok(Self { addr: addr.into() })
    }

    pub async fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        let mut stream = TcpStream::connect(self.addr).await
            .map_err(|e| crate::error::Error::Storage(format!("连接失败: {}", e)))?;
        // 简单协议: "PUT key_len value_len key value"
        let mut buf = Vec::new();
        buf.extend_from_slice(b"PUT ");
        buf.extend_from_slice(&(key.len() as u32).to_be_bytes());
        buf.extend_from_slice(&(value.len() as u32).to_be_bytes());
        buf.extend_from_slice(key);
        buf.extend_from_slice(value);
        stream.write_all(&buf).await
            .map_err(|e| crate::error::Error::Storage(format!("写入失败: {}", e)))?;
        let mut resp = [0u8; 2];
        stream.read_exact(&mut resp).await
            .map_err(|e| crate::error::Error::Storage(format!("读取响应失败: {}", e)))?;
        if &resp == b"OK" {
            Ok(())
        } else {
            Err(crate::error::Error::Storage("PUT failed".into()))
        }
    }

    pub async fn get(&self, key: &[u8]) -> Result<Option<Bytes>> {
        let mut stream = TcpStream::connect(self.addr).await
            .map_err(|e| crate::error::Error::Storage(format!("连接失败: {}", e)))?;
        // 简单协议: "GET key_len key"
        let mut buf = Vec::new();
        buf.extend_from_slice(b"GET ");
        buf.extend_from_slice(&(key.len() as u32).to_be_bytes());
        buf.extend_from_slice(key);
        stream.write_all(&buf).await
            .map_err(|e| crate::error::Error::Storage(format!("写入失败: {}", e)))?;
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf).await
            .map_err(|e| crate::error::Error::Storage(format!("读取长度失败: {}", e)))?;
        let value_len = u32::from_be_bytes(len_buf);
        if value_len == 0 {
            return Ok(None);
        }
        let mut value = vec![0u8; value_len as usize];
        stream.read_exact(&mut value).await
            .map_err(|e| crate::error::Error::Storage(format!("读取值失败: {}", e)))?;
        Ok(Some(Bytes::from(value)))
    }
}
