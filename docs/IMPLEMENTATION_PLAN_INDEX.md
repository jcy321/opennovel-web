# OpenNovel Phase 6-7 实施计划索引

**生成日期**: 2026-03-16
**最后更新**: 2026-03-16
**状态**: ✅ Phase 0-7 实现完成

---

## 实施报告

📄 **[Phase 0-7 实现与集成报告](./implementation-reports/PHASE_0-7_IMPLEMENTATION_REPORT.md)**

完整记录各 Phase 的实现状态、模块清单、核心类型、能力边界，以及 Phase 7 与 Phase 0-6 的集成详情。

---

## 概述

本文档集定义了 OpenNovel 项目的 Phase 6（LLM 集成层）和 Phase 7（Agent 系统）的详细实施计划。

### 设计决策确认

| 决策项 | 选择 | 说明 |
|--------|------|------|
| Provider 存储 | Redis 缓存 + PostgreSQL 持久化 | 读取快，数据安全 |
| 热重载 | WebSocket 主通道 + 轮询 Fallback | 实时推送，可靠降级 |
| Skills 定义 | SKILL.md 文件（独立路径） | 版本控制友好，与部署机 OpenCode 隔离 |
| MCP 管理 | 全局共享 + 按需加载 + 5分钟 idle 超时 | 资源效率高 |

### 核心参考架构

本计划像素级模仿 **Oh My OpenCode** 的架构设计：
- 46 个 Hooks 系统
- 26 个 Tools 注册
- 11 个 Agent 定义
- Category + Skills 委派协议
- Fallback Chain 模型解析

---

## 文档结构

```
/opennovelv2/docs/
├── IMPLEMENTATION_PLAN_INDEX.md    # 本文档（索引）
├── ARCHITECTURE_DESIGN_V2.md       # 原有架构设计
│
├── phase6/                          # Phase 6: LLM 集成层
│   ├── 01-overview.md               # 概述与目标
│   ├── 02-provider-system.md        # Provider 注册与管理
│   ├── 03-model-resolution.md       # 模型解析与 Fallback
│   ├── 04-hot-reload.md             # 热重载机制
│   └── 05-api-design.md             # API 设计
│
└── phase7/                          # Phase 7: Agent 系统
    ├── 01-overview.md               # 概述与目标
    ├── 02-agent-definitions.md      # 8个 Agent 定义
    ├── 03-hooks-system.md           # Hooks 系统
    ├── 04-tools-system.md           # Tools 系统
    ├── 05-skills-system.md          # Skills 系统
    └── 06-delegation-protocol.md    # 委派协议
```

---

## Phase 6: LLM 集成层

**目标**: 使用 Vercel AI SDK 实现多供应商支持，支持 Web 端配置、热重载。

| 文档 | 内容 |
|------|------|
| [01-overview.md](./phase6/01-overview.md) | 整体架构、技术选型、里程碑 |
| [02-provider-system.md](./phase6/02-provider-system.md) | Provider Registry、动态注册、存储方案 |
| [03-model-resolution.md](./phase6/03-model-resolution.md) | Fallback Chain、Category 映射、4步解析流水线 |
| [04-hot-reload.md](./phase6/04-hot-reload.md) | WebSocket 推送、轮询降级、配置更新流程 |
| [05-api-design.md](./phase6/05-api-design.md) | RESTful API、数据模型、接口规范 |

**预计工期**: 2-3 周

---

## Phase 7: Agent 系统

**目标**: 像素级模仿 Oh My OpenCode 的 Hooks/Tools/Skills 体系，定义 8 个小说创作 Agent 的行为。

| 文档 | 内容 |
|------|------|
| [01-overview.md](./phase7/01-overview.md) | 整体架构、Agent 列表、与 OMO 的对应关系 |
| [02-agent-definitions.md](./phase7/02-agent-definitions.md) | 8个 Agent 的完整定义（Factory、Metadata、Prompt） |
| [03-hooks-system.md](./phase7/03-hooks-system.md) | 小说创作专用 Hooks 设计与实现 |
| [04-tools-system.md](./phase7/04-tools-system.md) | 小说创作专用 Tools 设计与实现 |
| [05-skills-system.md](./phase7/05-skills-system.md) | SKILL.md 定义、加载机制、与部署机隔离 |
| [06-delegation-protocol.md](./phase7/06-delegation-protocol.md) | Intent Gate、Delegation Protocol、Session Continuity |

**预计工期**: 3-4 周

---

## 快速导航

### Phase 6 关键设计
- [Provider Registry 设计](./phase6/02-provider-system.md#provider-registry)
- [Fallback Chain 实现](./phase6/03-model-resolution.md#fallback-chain)
- [热重载架构](./phase6/04-hot-reload.md#架构设计)

### Phase 7 关键设计
- [天道 Agent 定义](./phase7/02-agent-definitions.md#天道-agent)
- [小说创作 Hooks](./phase7/03-hooks-system.md#专用-hooks)
- [Delegation Protocol](./phase7/06-delegation-protocol.md#6字段结构)

---

## 依赖关系

```
Phase 6 (LLM 集成层)
    │
    ├── Provider Registry ──────────────────┐
    ├── Model Resolution ───────────────────┤
    └── Hot Reload ─────────────────────────┤
                                            │
                                            ▼
Phase 7 (Agent 系统) ◄──────────────────────┘
    │
    ├── Agent Definitions ──────────────────┐
    ├── Hooks System ───────────────────────┤
    ├── Tools System ───────────────────────┤
    ├── Skills System ──────────────────────┤
    └── Delegation Protocol ────────────────┘
```

**Phase 6 必须先完成**，因为 Phase 7 的 Agent 需要调用 LLM。

---

## 后续 Phase 概览

### Phase 8: Web UI 集成（预计 2 周）
- 群聊界面
- Provider 配置界面
- Agent 状态展示

### Phase 9: 测试与优化（预计 1 周）
- 单元测试
- 集成测试
- 性能优化

### Phase 10: 文档与部署（预计 1 周）
- API 文档
- 用户指南
- Docker 部署

---

## 变更日志

| 日期 | 变更 | 作者 |
|------|------|------|
| 2026-03-16 | Phase 6 文档完成（5个文件） | Sisyphus |
| 2026-03-16 | Phase 7 文档完成（6个文件） | Sisyphus |
| 2026-03-16 | 初始版本 | Sisyphus |