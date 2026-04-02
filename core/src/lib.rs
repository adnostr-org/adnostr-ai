//! AI Nostr广告协议核心库
//!
//! 这个库提供了AI Nostr广告协议的核心实现，包括：
//! - Nostr协议扩展
//! - 广告事件定义
//! - 加密和安全
//! - 网络通信
//! - 数据存储

#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod error;
pub mod protocol;
pub mod crypto;
pub mod network;
pub mod storage;
pub mod consensus;

#[cfg(feature = "python")]
pub mod python;

#[cfg(feature = "wasm")]
pub mod wasm;

/// 重新导出常用类型
pub use error::{Error, Result};
pub use protocol::*;
pub use crypto::*;

/// 库版本
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 初始化库
///
/// # 示例
/// ```
/// use ai_nostr_ad_core::init;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     init().await?;
///     Ok(())
/// }
/// ```
pub async fn init() -> Result<(), Error> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    // 初始化加密库
    crypto::init()?;
    
    tracing::info!("AI Nostr广告协议核心库已初始化，版本: {}", VERSION);
    
    Ok(())
}

/// 库配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    /// 网络配置
    pub network: network::Config,
    /// 存储配置
    pub storage: storage::Config,
    /// 加密配置
    pub crypto: crypto::Config,
    /// 共识配置
    pub consensus: consensus::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: network::Config::default(),
            storage: storage::Config::default(),
            crypto: crypto::Config::default(),
            consensus: consensus::Config::default(),
        }
    }
}

/// 库状态
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Status {
    /// 是否已初始化
    pub initialized: bool,
    /// 网络状态
    pub network_status: network::Status,
    /// 存储状态
    pub storage_status: storage::Status,
    /// 内存使用情况
    pub memory_usage: MemoryUsage,
    /// 性能指标
    pub performance: PerformanceMetrics,
}

/// 内存使用情况
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryUsage {
    /// 总内存 (字节)
    pub total: u64,
    /// 已使用内存 (字节)
    pub used: u64,
    /// 峰值内存 (字节)
    pub peak: u64,
}

/// 性能指标
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PerformanceMetrics {
    /// 事件处理速度 (事件/秒)
    pub event_processing_rate: f64,
    /// 平均延迟 (毫秒)
    pub average_latency: f64,
    /// 错误率
    pub error_rate: f64,
    /// 连接数
    pub connections: u32,
}

/// 获取库状态
pub async fn get_status() -> Result<Status, Error> {
    let network_status = network::get_status().await?;
    let storage_status = storage::get_status().await?;
    
    Ok(Status {
        initialized: true,
        network_status,
        storage_status,
        memory_usage: MemoryUsage {
            total: 0,
            used: 0,
            peak: 0,
        },
        performance: PerformanceMetrics {
            event_processing_rate: 0.0,
            average_latency: 0.0,
            error_rate: 0.0,
            connections: 0,
        },
    })
}

/// 健康检查
pub async fn health_check() -> Result<bool, Error> {
    let network_healthy = network::health_check().await?;
    let storage_healthy = storage::health_check().await?;
    
    Ok(network_healthy && storage_healthy)
}

/// 重置库状态
pub async fn reset() -> Result<(), Error> {
    tracing::warn!("正在重置库状态...");
    
    network::reset().await?;
    storage::reset().await?;
    
    tracing::info!("库状态已重置");
    Ok(())
}