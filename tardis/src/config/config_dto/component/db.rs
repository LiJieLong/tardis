use serde::{Deserialize, Serialize};

use typed_builder::TypedBuilder;

use crate::redact::Redact;

/// Database module configuration / 数据库模块配置
///
/// Database operations need to be enabled ```#[cfg(feature = "reldb")]``` .
///
/// 数据库的操作需要启用 ```#[cfg(feature = "reldb")]``` .
///
/// # Examples
/// ```ignore
/// use tardis::basic::config::DBConfig;
/// let config = DBConfig{
///    url: "mysql://root:123456@localhost:3306/test".to_string(),
///    ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, TypedBuilder)]
#[serde(default)]
pub struct DBModuleConfig {
    #[builder(setter(into))]
    /// Database access Url, Url with permission information / 数据库访问Url，Url带权限信息
    pub url: String,
    /// Maximum number of connections, default 20 / 最大连接数，默认 20
    #[builder(default = 20)]
    pub max_connections: u32,
    /// Minimum number of connections, default 5 / 最小连接数，默认 5
    #[builder(default = 5)]
    pub min_connections: u32,
    /// Connection timeout / 连接超时时间
    #[builder(default, setter(strip_option))]
    pub connect_timeout_sec: Option<u64>,
    /// Idle connection timeout / 空闲连接超时时间
    #[builder(default, setter(strip_option))]
    pub idle_timeout_sec: Option<u64>,
    /// Compatible database type / 兼容数据库类型
    #[builder(default)]
    pub compatible_type: CompatibleType,
}

impl Default for DBModuleConfig {
    fn default() -> Self {
        DBModuleConfig::builder().url("").build()
    }
}

impl std::fmt::Debug for DBModuleConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url_debug = if let Ok(url) = url::Url::parse(&self.url) {
            url.redact().to_string()
        } else {
            self.url.to_string()
        };

        f.debug_struct("DBModuleConfig")
            .field("url", &url_debug)
            .field("max_connections", &self.max_connections)
            .field("min_connections", &self.min_connections)
            .field("connect_timeout_sec", &self.connect_timeout_sec)
            .field("idle_timeout_sec", &self.idle_timeout_sec)
            .field("compatible_type", &self.compatible_type)
            .finish()
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompatibleType {
    #[default]
    None,
    Oracle,
}
