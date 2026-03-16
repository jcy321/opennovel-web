//! OpenNovel SDK Core - 基础 SDK 框架
//!
//! 提供Agent系统、Session管理、Message Protocol和Intent Gate等核心功能。
//!
//! # 模块结构
//!
//! - [`agent`] - Agent定义和生命周期管理
//! - [`session`] - 会话管理
//! - [`message`] - 消息协议和SSE流式输出
//! - [`intent`] - 意图解析和路由
//! - [`permission`] - 权限系统
//! - [`provider`] - LLM供应商抽象
//! - [`hooks`] - Agent生命周期钩子系统
//! - [`tools`] - Agent工具系统
//! - [`skills`] - Agent技能系统
//! - [`delegation`] - Agent委派协议
//!
//! # 开源说明
//!
//! 本模块是 OpenNovel 基础 SDK 框架，提供 Agent 系统的核心抽象和基础设施。
//! 完整的小说创作系统（天道推演引擎、刘和平人物塑造算法、世界观守护者规则引擎等）
//! 暂不开源，后期计划推出社区版。

pub mod agent;
pub mod session;
pub mod message;
pub mod intent;
pub mod permission;
pub mod provider;
pub mod hooks;
pub mod tools;
pub mod skills;
pub mod delegation;

pub use agent::{Agent, AgentCapability, AgentConfig, AgentError, AgentId, AgentRegistry, AgentResponse, AgentRole};
pub use permission::{Permission, PermissionCheckResult, PermissionChecker, PermissionMatrix, PermissionScope, PermissionSet};
pub use session::SessionContext;
pub use intent::{IntentGate, IntentType, ParsedIntent, RouteTarget};

/// SDK版本
pub const VERSION: &str = env!("CARGO_PKG_VERSION");