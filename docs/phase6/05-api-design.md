# Phase 6: API 设计

**文档版本**: 1.0
**最后更新**: 2026-03-16

---

## 1. 概述

本文档定义 OpenNovel LLM 集成层的 RESTful API 接口。

### 设计原则
- RESTful 风格
- JSON 请求/响应
- 版本化 API
- 统一错误格式

---

## 2. API 端点

### 2.1 Provider 管理

#### GET /api/providers

获取所有 Provider 列表

**响应**:
```json
{
  "providers": [
    {
      "id": "dashscope",
      "name": "阿里云百炼",
      "type": "openai-compatible",
      "enabled": true,
      "models": [
        {
          "id": "qwen-max",
          "name": "Qwen Max",
          "enabled": true
        }
      ]
    }
  ]
}
```

---

#### GET /api/providers/:id

获取单个 Provider 详情

**响应**:
```json
{
  "id": "dashscope",
  "name": "阿里云百炼",
  "type": "openai-compatible",
  "baseURL": "https://dashscope.aliyuncs.com/compatible-mode/v1",
  "enabled": true,
  "models": [
    {
      "id": "qwen-max",
      "name": "Qwen Max",
      "modelId": "qwen-max",
      "supportsThinking": true,
      "supportsStreaming": true,
      "supportsToolCalling": true,
      "maxContextLength": 32768,
      "enabled": true
    }
  ],
  "createdAt": "2026-01-01T00:00:00Z",
  "updatedAt": "2026-01-01T00:00:00Z"
}
```

---

#### POST /api/providers

创建新的 Provider

**请求**:
```json
{
  "name": "自定义供应商",
  "type": "openai-compatible",
  "baseURL": "https://api.example.com/v1",
  "apiKey": "sk-xxx",
  "headers": {
    "X-Custom-Header": "value"
  },
  "models": [
    {
      "name": "Model A",
      "modelId": "model-a",
      "supportsThinking": false,
      "maxContextLength": 8192
    }
  ]
}
```

**响应**:
```json
{
  "id": "custom-1700000000000",
  "name": "自定义供应商",
  "type": "openai-compatible",
  "baseURL": "https://api.example.com/v1",
  "enabled": true,
  "models": [
    {
      "id": "custom-1700000000000-model-0",
      "name": "Model A",
      "modelId": "model-a",
      "enabled": true
    }
  ],
  "createdAt": "2026-01-01T00:00:00Z",
  "updatedAt": "2026-01-01T00:00:00Z"
}
```

---

#### PATCH /api/providers/:id

更新 Provider

**请求**:
```json
{
  "name": "新名称",
  "enabled": false
}
```

**响应**: 同 GET /api/providers/:id

---

#### DELETE /api/providers/:id

删除 Provider

**响应**:
```json
{
  "success": true
}
```

---

#### POST /api/providers/:id/test

测试 Provider 连接

**响应**:
```json
{
  "success": true,
  "availableModels": ["qwen-max", "qwen-plus"]
}
```

---

### 2.2 Model 管理

#### GET /api/providers/:providerId/models

获取 Provider 下的所有模型

---

#### PATCH /api/providers/:providerId/models/:modelId

更新模型配置

**请求**:
```json
{
  "enabled": false,
  "defaultTemperature": 0.5
}
```

---

### 2.3 配置版本

#### GET /api/config/version

获取当前配置版本号

**响应**:
```json
{
  "version": 1700000000000
}
```

---

#### GET /api/config

获取完整配置（用于轮询降级）

**响应**:
```json
{
  "version": 1700000000000,
  "providers": [...]
}
```

---

### 2.4 模型调用

#### POST /api/llm/generate

同步文本生成

**请求**:
```json
{
  "providerId": "dashscope",
  "modelId": "qwen-max",
  "messages": [
    { "role": "user", "content": "Hello" }
  ],
  "temperature": 0.7,
  "maxTokens": 1000
}
```

**响应**:
```json
{
  "content": "Hello! How can I help you?",
  "usage": {
    "promptTokens": 10,
    "completionTokens": 20,
    "totalTokens": 30
  }
}
```

---

#### POST /api/llm/stream

流式文本生成（SSE）

**请求**: 同 POST /api/llm/generate

**响应**: Server-Sent Events

```
event: text
data: {"delta": "Hello"}

event: text
data: {"delta": "!"}

event: done
data: {"usage": {"promptTokens": 10, "completionTokens": 20}}
```

---

#### POST /api/llm/resolve

解析模型（获取推荐模型）

**请求**:
```json
{
  "agentName": "tian-dao",
  "category": "outline-design"
}
```

**响应**:
```json
{
  "providerId": "anthropic",
  "modelId": "claude-opus-4",
  "variant": "max",
  "isFallback": false
}
```

---

## 3. 数据模型

### 3.1 Provider

```typescript
interface Provider {
  id: string
  name: string
  type: 'openai-compatible' | 'anthropic' | 'openai' | 'custom'
  baseURL: string
  enabled: boolean
  models: Model[]
  createdAt: string
  updatedAt: string
}
```

### 3.2 Model

```typescript
interface Model {
  id: string
  name: string
  modelId: string
  supportsThinking: boolean
  supportsStreaming: boolean
  supportsToolCalling: boolean
  maxContextLength: number
  maxOutputTokens: number
  defaultTemperature: number
  costTier: 'low' | 'medium' | 'high'
  enabled: boolean
}
```

### 3.3 错误响应

```typescript
interface ErrorResponse {
  error: {
    code: string
    message: string
    details?: Record<string, any>
  }
}
```

**错误码**:
- `PROVIDER_NOT_FOUND`: Provider 不存在
- `MODEL_NOT_FOUND`: Model 不存在
- `INVALID_CONFIG`: 配置验证失败
- `PROVIDER_UNAVAILABLE`: Provider 不可用
- `MODEL_UNAVAILABLE`: 模型不可用
- `RATE_LIMITED`: 请求限流
- `INTERNAL_ERROR`: 内部错误

---

## 4. 认证与授权

### 4.1 API Key 认证

```
Authorization: Bearer <api_key>
```

### 4.2 权限级别

| 权限 | 描述 |
|------|------|
| `read:providers` | 查看 Provider |
| `write:providers` | 创建/修改/删除 Provider |
| `read:models` | 查看模型 |
| `write:models` | 修改模型配置 |
| `call:llm` | 调用 LLM |

---

## 5. 限流

### 5.1 限流规则

| 端点 | 限制 |
|------|------|
| POST /api/llm/* | 100 次/分钟 |
| POST /api/providers | 10 次/分钟 |
| GET /api/* | 1000 次/分钟 |

### 5.2 响应头

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1700000000
```

---

## 6. WebSocket 协议

### 6.1 连接

```
ws://localhost:3000/ws
```

### 6.2 消息格式

#### 客户端 → 服务端

```json
{
  "type": "subscribe",
  "channel": "config"
}
```

#### 服务端 → 客户端

```json
{
  "type": "config_update",
  "version": 1700000000000,
  "data": {
    "type": "provider",
    "id": "dashscope"
  },
  "timestamp": 1700000000000
}
```

---

## 7. 相关文档

- [02-provider-system.md](./02-provider-system.md) - Provider 注册与管理
- [03-model-resolution.md](./03-model-resolution.md) - 模型解析与 Fallback
- [04-hot-reload.md](./04-hot-reload.md) - 热重载机制