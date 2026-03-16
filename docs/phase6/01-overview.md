# Phase 6: LLM 集成层 - 概述

**文档版本**: 1.0
**最后更新**: 2026-03-16

---

## 1. 目标

使用 **Vercel AI SDK** 实现 OpenNovel 的 LLM 集成层，支持：

1. **多供应商支持**: 阿里云百炼、Anthropic、OpenAI 等
2. **Web 端配置**: 用户可自定义添加供应商和模型
3. **热重载**: 配置更新无需重启服务
4. **Fallback Chain**: 模型失败自动切换备用模型

---

## 2. 技术选型

### 2.1 Vercel AI SDK

**选择理由**:
- 统一的 API 接口，兼容 OpenAI/Anthropic/自定义供应商
- 内置流式响应支持
- 支持 Tool Calling
- 活跃的社区维护

**核心 API**:
```typescript
import { generateText, streamText } from 'ai'
import { createOpenAI } from '@ai-sdk/openai'
import { createAnthropic } from '@ai-sdk/anthropic'

// OpenAI 兼容接口（阿里云百炼）
const dashscope = createOpenAI({
  name: 'dashscope',
  baseURL: 'https://dashscope.aliyuncs.com/compatible-mode/v1',
  apiKey: process.env.DASHSCOPE_API_KEY,
})

// Anthropic
const anthropic = createAnthropic({
  apiKey: process.env.ANTHROPIC_API_KEY,
})

// 使用
const result = await generateText({
  model: dashscope('qwen-max'),
  prompt: 'Hello',
})
```

### 2.2 存储方案

**Redis + PostgreSQL 混合架构**:

```
┌─────────────────────────────────────────────────────────────┐
│                     Web 端配置请求                            │
│                  POST /api/providers                         │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    PostgreSQL (持久化)                       │
│  providers: id, name, type, config, created_at, updated_at  │
│  models: id, provider_id, name, config, is_active           │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Redis (缓存层)                            │
│  provider:{id} → JSON (TTL: 5分钟)                          │
│  model:{provider_id}:{model_name} → JSON                    │
│  config:version → int (配置版本号)                          │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    应用层读取                                │
│              优先 Redis，Miss 时回源 PostgreSQL             │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. 架构设计

### 3.1 整体架构

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           OpenNovel Application                          │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                     Agent Layer (Phase 7)                        │   │
│  │  天道、执笔、世界观守护者、规划者、审阅、观察者、调研者、刘和平       │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                    │                                    │
│                                    ▼                                    │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    LLM Integration Layer                         │   │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐                │   │
│  │  │  Provider   │ │   Model     │ │    Hot      │                │   │
│  │  │  Registry   │ │ Resolution  │ │   Reload    │                │   │
│  │  └─────────────┘ └─────────────┘ └─────────────┘                │   │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐                │   │
│  │  │  Fallback   │ │  Streaming  │ │   Config    │                │   │
│  │  │   Chain     │ │   Handler   │ │   Manager   │                │   │
│  │  └─────────────┘ └─────────────┘ └─────────────┘                │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                    │                                    │
│                                    ▼                                    │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    Vercel AI SDK                                 │   │
│  │  generateText, streamText, streamObject, tool calling            │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                    │                                    │
└────────────────────────────────────┼────────────────────────────────────┘
                                     │
                                     ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                          External Providers                              │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
│  │  阿里云百炼  │ │  Anthropic  │ │   OpenAI    │ │   Custom    │       │
│  │  (Qwen)     │ │  (Claude)   │ │  (GPT)      │ │  (用户定义)  │       │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘       │
└─────────────────────────────────────────────────────────────────────────┘
```

### 3.2 模块职责

| 模块 | 职责 | 核心文件 |
|------|------|---------|
| **Provider Registry** | 供应商注册、发现、配置管理 | `provider-registry.ts` |
| **Model Resolution** | 模型选择、Fallback Chain、Category 映射 | `model-resolution.ts` |
| **Hot Reload** | 配置更新、WebSocket 推送、版本管理 | `hot-reload.ts` |
| **Fallback Chain** | 模型失败自动切换 | `fallback-chain.ts` |
| **Streaming Handler** | 流式响应处理、SSE 事件 | `streaming-handler.ts` |
| **Config Manager** | 配置存储、缓存、读取 | `config-manager.ts` |

---

## 4. 目录结构

```
packages/llm/
├── src/
│   ├── index.ts                    # 主入口，导出所有模块
│   │
│   ├── providers/
│   │   ├── index.ts                # Provider 模块入口
│   │   ├── registry.ts             # Provider Registry
│   │   ├── dynamic-manager.ts      # 动态 Provider 管理
│   │   ├── types.ts                # Provider 类型定义
│   │   └── builtin/
│   │       ├── dashscope.ts        # 阿里云百炼
│   │       ├── anthropic.ts        # Anthropic
│   │       └── openai.ts           # OpenAI
│   │
│   ├── models/
│   │   ├── index.ts                # Model 模块入口
│   │   ├── resolution.ts           # 模型解析流水线
│   │   ├── fallback-chain.ts       # Fallback Chain 实现
│   │   ├── category-mapping.ts     # Category → Model 映射
│   │   └── types.ts                # Model 类型定义
│   │
│   ├── streaming/
│   │   ├── index.ts                # Streaming 模块入口
│   │   ├── handler.ts              # 流式响应处理
│   │   ├── events.ts               # SSE 事件定义
│   │   └── types.ts                # Streaming 类型定义
│   │
│   ├── config/
│   │   ├── index.ts                # Config 模块入口
│   │   ├── manager.ts              # 配置管理器
│   │   ├── storage.ts              # 存储抽象层
│   │   ├── schema.ts               # Zod 配置 Schema
│   │   └── hot-reload.ts           # 热重载实现
│   │
│   └── utils/
│       ├── logger.ts               # 日志工具
│       └── errors.ts               # 错误定义
│
├── tests/
│   ├── providers/
│   ├── models/
│   ├── streaming/
│   └── config/
│
├── package.json
└── tsconfig.json
```

---

## 5. 里程碑

### Milestone 1: 基础设施（Week 1）
- [ ] 项目结构搭建
- [ ] Provider Registry 实现
- [ ] 内置供应商（DashScope、Anthropic、OpenAI）
- [ ] 基础配置存储（PostgreSQL）

### Milestone 2: 模型解析（Week 1-2）
- [ ] Fallback Chain 实现
- [ ] Category 映射系统
- [ ] 4步解析流水线
- [ ] 模型可用性检查

### Milestone 3: 热重载与流式（Week 2）
- [ ] Redis 缓存层
- [ ] WebSocket 推送
- [ ] 轮询 Fallback
- [ ] 流式响应处理

### Milestone 4: API 与集成（Week 2-3）
- [ ] RESTful API 设计
- [ ] Web 端配置接口
- [ ] 与 Phase 5 SDK 集成
- [ ] 测试覆盖

---

## 6. 依赖关系

### 6.1 外部依赖

```json
{
  "dependencies": {
    "ai": "^4.0.0",                    // Vercel AI SDK
    "@ai-sdk/openai": "^1.0.0",        // OpenAI 兼容
    "@ai-sdk/anthropic": "^1.0.0",     // Anthropic
    "ioredis": "^5.3.0",               // Redis 客户端
    "pg": "^8.11.0",                   // PostgreSQL
    "zod": "^3.22.0",                  // Schema 验证
    "ws": "^8.16.0"                    // WebSocket
  }
}
```

### 6.2 内部依赖

- `@opennovel/core` - 核心类型定义
- `@opennovel/sync` - 配置同步（WebDAV）

---

## 7. 风险与缓解

| 风险 | 影响 | 缓解措施 |
|------|------|---------|
| Vercel AI SDK 版本更新 | API 变更 | 锁定版本，渐进升级 |
| 供应商 API 限流 | 请求失败 | 实现重试机制，Fallback Chain |
| Redis 连接中断 | 缓存失效 | 降级到直接查询 PostgreSQL |
| WebSocket 断连 | 实时推送失败 | 轮询 Fallback 机制 |

---

## 8. 相关文档

- [02-provider-system.md](./02-provider-system.md) - Provider 注册与管理
- [03-model-resolution.md](./03-model-resolution.md) - 模型解析与 Fallback
- [04-hot-reload.md](./04-hot-reload.md) - 热重载机制
- [05-api-design.md](./05-api-design.md) - API 设计