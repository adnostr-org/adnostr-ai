//! 错误处理模块

use thiserror::Error;

/// 库错误类型
#[derive(Error, Debug)]
pub enum Error {
    /// 协议错误
    #[error("协议错误: {0}")]
    Protocol(String),
    
    /// 加密错误
    #[error("加密错误: {0}")]
    Crypto(String),
    
    /// 网络错误
    #[error("网络错误: {0}")]
    Network(String),
    
    /// 存储错误
    #[error("存储错误: {0}")]
    Storage(String),
    
    /// 共识错误
    #[error("共识错误: {0}")]
    Consensus(String),
    
    /// 配置错误
    #[error("配置错误: {0}")]
    Config(String),
    
    /// 验证错误
    #[error("验证错误: {0}")]
    Validation(String),
    
    /// 序列化错误
    #[error("序列化错误: {0}")]
    Serialization(String),
    
    /// 反序列化错误
    #[error("反序列化错误: {0}")]
    Deserialization(String),
    
    /// IO错误
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    
    /// JSON错误
    #[error("JSON错误: {0}")]
    Json(#[from] serde_json::Error),
    
    /// 数据库错误
    #[cfg(feature = "sqlx")]
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),
    
    /// Redis错误
    #[cfg(feature = "redis")]
    #[error("Redis错误: {0}")]
    Redis(#[from] redis::RedisError),
    
    /// 请求错误
    #[error("请求错误: {0}")]
    Request(#[from] reqwest::Error),
    
    /// WebSocket错误
    #[error("WebSocket错误: {0}")]
    WebSocket(String),
    
    /// 超时错误
    #[error("操作超时")]
    Timeout,
    
    /// 资源不足
    #[error("资源不足: {0}")]
    Resource(String),
    
    /// 权限错误
    #[error("权限错误: {0}")]
    Permission(String),
    
    /// 未知错误
    #[error("未知错误: {0}")]
    Unknown(String),
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// 创建协议错误
    pub fn protocol(msg: impl Into<String>) -> Self {
        Self::Protocol(msg.into())
    }
    
    /// 创建加密错误
    pub fn crypto(msg: impl Into<String>) -> Self {
        Self::Crypto(msg.into())
    }
    
    /// 创建网络错误
    pub fn network(msg: impl Into<String>) -> Self {
        Self::Network(msg.into())
    }
    
    /// 创建存储错误
    pub fn storage(msg: impl Into<String>) -> Self {
        Self::Storage(msg.into())
    }
    
    /// 创建验证错误
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }
    
    /// 创建WebSocket错误
    pub fn websocket(msg: impl Into<String>) -> Self {
        Self::WebSocket(msg.into())
    }
    
    /// 检查是否为超时错误
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout)
    }
    
    /// 检查是否为网络错误
    pub fn is_network(&self) -> bool {
        matches!(self, Self::Network(_))
    }
    
    /// 检查是否为协议错误
    pub fn is_protocol(&self) -> bool {
        matches!(self, Self::Protocol(_))
    }
}

/// 错误代码
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ErrorCode {
    /// 成功
    Success = 0,
    /// 未知错误
    Unknown = 1,
    /// 协议错误
    Protocol = 100,
    /// 加密错误
    Crypto = 200,
    /// 网络错误
    Network = 300,
    /// 存储错误
    Storage = 400,
    /// 共识错误
    Consensus = 500,
    /// 配置错误
    Config = 600,
    /// 验证错误
    Validation = 700,
    /// 权限错误
    Permission = 800,
    /// 资源不足
    Resource = 900,
    /// 超时错误
    Timeout = 1000,
}

impl ErrorCode {
    /// 获取错误代码的描述
    pub fn description(&self) -> &'static str {
        match self {
            Self::Success => "成功",
            Self::Unknown => "未知错误",
            Self::Protocol => "协议错误",
            Self::Crypto => "加密错误",
            Self::Network => "网络错误",
            Self::Storage => "存储错误",
            Self::Consensus => "共识错误",
            Self::Config => "配置错误",
            Self::Validation => "验证错误",
            Self::Permission => "权限错误",
            Self::Resource => "资源不足",
            Self::Timeout => "操作超时",
        }
    }
    
    /// 从错误获取错误代码
    pub fn from_error(error: &Error) -> Self {
        match error {
            Error::Protocol(_) => Self::Protocol,
            Error::Crypto(_) => Self::Crypto,
            Error::Network(_) => Self::Network,
            Error::Storage(_) => Self::Storage,
            Error::Consensus(_) => Self::Consensus,
            Error::Config(_) => Self::Config,
            Error::Validation(_) => Self::Validation,
            Error::Permission(_) => Self::Permission,
            Error::Resource(_) => Self::Resource,
            Error::Timeout => Self::Timeout,
            _ => Self::Unknown,
        }
    }
}

/// 错误响应
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorResponse {
    /// 错误代码
    pub code: ErrorCode,
    /// 错误消息
    pub message: String,
    /// 详细错误信息
    pub details: Option<String>,
    /// 时间戳
    pub timestamp: u64,
}

impl ErrorResponse {
    /// 创建错误响应
    pub fn new(code: ErrorCode, message: String) -> Self {
        Self {
            code,
            message,
            details: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
    
    /// 从错误创建错误响应
    pub fn from_error(error: &Error) -> Self {
        let code = ErrorCode::from_error(error);
        let message = error.to_string();
        
        Self::new(code, message)
    }
    
    /// 添加详细错误信息
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.code as u32,
            self.code.description(),
            self.message
        )
    }
}

/// 错误上下文
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// 错误
    pub error: Error,
    /// 上下文信息
    pub context: String,
    /// 位置
    pub location: Option<String>,
}

impl ErrorContext {
    /// 创建错误上下文
    pub fn new(error: Error, context: impl Into<String>) -> Self {
        Self {
            error,
            context: context.into(),
            location: None,
        }
    }
    
    /// 添加位置信息
    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }
}

impl std::fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(location) = &self.location {
            write!(f, "{} at {}: {}", self.error, location, self.context)
        } else {
            write!(f, "{}: {}", self.error, self.context)
        }
    }
}

impl std::error::Error for ErrorContext {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}