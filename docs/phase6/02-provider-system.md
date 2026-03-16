# Phase 6: Provider 系统设计

**文档版本**: 1.0
**最后更新**: 2026-03-16

---

## 1. 概述

Provider 系统负责管理和注册所有 LLM 供应商，支持：
- 内置供应商（阿里云百炼、Anthropic、OpenAI）
- 用户自定义供应商（Web 端配置）
- 动态注册和发现
- 配置验证和持久化

---

## 2. 类型定义

### 2.1 Provider 类型

```typescript
// packages/llm/src/providers/types.ts

/**
 * Provider 类型
 */
export type ProviderType = 
  | 'openai-compatible'  // OpenAI 兼容接口（阿里云百炼、DeepSeek 等）
  | 'anthropic'          // Anthropic 原生接口
  | 'openai'             // OpenAI 原生接口
  | 'custom'             // 用户自定义

/**
 * Provider 配置
 */
export interface ProviderConfig {
  /** Provider ID（唯一标识） */
  id: string
  
  /** 显示名称 */
  name: string
  
  /** Provider 类型 */
  type: ProviderType
  
  /** API 基础 URL */
  baseURL: string
  
  /** API Key（加密存储） */
  apiKey: string
  
  /** 请求头配置 */
  headers?: Record<string, string>
  
  /** 默认模型 */
  defaultModel?: string
  
  /** 可用模型列表 */
  models: ModelConfig[]
  
  /** 是否启用 */
  enabled: boolean
  
  /** 创建时间 */
  createdAt: Date
  
  /** 更新时间 */
  updatedAt: Date
}

/**
 * 模型配置
 */
export interface ModelConfig {
  /** 模型 ID */
  id: string
  
  /** 显示名称 */
  name: string
  
  /** 模型标识符（供应商侧） */
  modelId: string
  
  /** 是否支持思考模式 */
  supportsThinking?: boolean
  
  /** 是否支持流式输出 */
  supportsStreaming?: boolean
  
  /** 是否支持工具调用 */
  supportsToolCalling?: boolean
  
  /** 最大上下文长度 */
  maxContextLength?: number
  
  /** 最大输出长度 */
  maxOutputTokens?: number
  
  /** 默认温度 */
  defaultTemperature?: number
  
  /** 成本分类 */
  costTier?: 'low' | 'medium' | 'high'
  
  /** 是否启用 */
  enabled: boolean
}
```

### 2.2 Provider Registry 接口

```typescript
/**
 * Provider Registry 接口
 */
export interface IProviderRegistry {
  /** 注册 Provider */
  register(config: ProviderConfig): Promise<void>
  
  /** 注销 Provider */
  unregister(providerId: string): Promise<void>
  
  /** 获取 Provider */
  get(providerId: string): Promise<ProviderConfig | null>
  
  /** 获取所有 Provider */
  list(): Promise<ProviderConfig[]>
  
  /** 获取启用的 Provider */
  listEnabled(): Promise<ProviderConfig[]>
  
  /** 更新 Provider */
  update(providerId: string, config: Partial<ProviderConfig>): Promise<void>
  
  /** 检查 Provider 是否可用 */
  checkAvailability(providerId: string): Promise<boolean>
  
  /** 创建 SDK Provider 实例 */
  createSDKProvider(providerId: string): Promise<SDKProvider>
}

/**
 * Vercel AI SDK Provider 类型
 */
export type SDKProvider = ReturnType<typeof createOpenAI> | ReturnType<typeof createAnthropic>
```

---

## 3. Provider Registry 实现

### 3.1 核心实现

```typescript
// packages/llm/src/providers/registry.ts

import { createOpenAI, type OpenAIProvider } from '@ai-sdk/openai'
import { createAnthropic, type AnthropicProvider } from '@ai-sdk/anthropic'
import type { ProviderConfig, ModelConfig, IProviderRegistry, SDKProvider } from './types'
import { ConfigStorage } from '../config/storage'
import { Logger } from '../utils/logger'

const log = new Logger('ProviderRegistry')

export class ProviderRegistry implements IProviderRegistry {
  private storage: ConfigStorage
  private cache: Map<string, ProviderConfig> = new Map()
  private sdkInstances: Map<string, SDKProvider> = new Map()
  
  constructor(storage: ConfigStorage) {
    this.storage = storage
  }
  
  async register(config: ProviderConfig): Promise<void> {
    // 验证配置
    this.validateConfig(config)
    
    // 存储
    await this.storage.saveProvider(config)
    
    // 更新缓存
    this.cache.set(config.id, config)
    
    // 清除 SDK 实例缓存（下次使用时重新创建）
    this.sdkInstances.delete(config.id)
    
    log.info(`Provider registered: ${config.id}`)
  }
  
  async unregister(providerId: string): Promise<void> {
    await this.storage.deleteProvider(providerId)
    this.cache.delete(providerId)
    this.sdkInstances.delete(providerId)
    
    log.info(`Provider unregistered: ${providerId}`)
  }
  
  async get(providerId: string): Promise<ProviderConfig | null> {
    // 先查缓存
    if (this.cache.has(providerId)) {
      return this.cache.get(providerId)!
    }
    
    // 回源存储
    const config = await this.storage.getProvider(providerId)
    if (config) {
      this.cache.set(providerId, config)
    }
    
    return config
  }
  
  async list(): Promise<ProviderConfig[]> {
    const providers = await this.storage.listProviders()
    
    // 更新缓存
    for (const config of providers) {
      this.cache.set(config.id, config)
    }
    
    return providers
  }
  
  async listEnabled(): Promise<ProviderConfig[]> {
    const all = await this.list()
    return all.filter(p => p.enabled)
  }
  
  async update(providerId: string, updates: Partial<ProviderConfig>): Promise<void> {
    const existing = await this.get(providerId)
    if (!existing) {
      throw new Error(`Provider not found: ${providerId}`)
    }
    
    const updated: ProviderConfig = {
      ...existing,
      ...updates,
      id: existing.id, // ID 不可修改
      updatedAt: new Date(),
    }
    
    await this.register(updated)
  }
  
  async checkAvailability(providerId: string): Promise<boolean> {
    try {
      const sdk = await this.createSDKProvider(providerId)
      const config = await this.get(providerId)
      
      if (!config) return false
      
      // 发送一个简单的测试请求
      // 实际实现中可能需要更智能的检测
      return true
    } catch (error) {
      log.warn(`Provider ${providerId} not available:`, error)
      return false
    }
  }
  
  async createSDKProvider(providerId: string): Promise<SDKProvider> {
    // 检查缓存
    if (this.sdkInstances.has(providerId)) {
      return this.sdkInstances.get(providerId)!
    }
    
    const config = await this.get(providerId)
    if (!config) {
      throw new Error(`Provider not found: ${providerId}`)
    }
    
    // 根据 Provider 类型创建 SDK 实例
    let sdk: SDKProvider
    
    switch (config.type) {
      case 'openai-compatible':
      case 'openai':
        sdk = createOpenAI({
          name: config.id,
          baseURL: config.baseURL,
          apiKey: config.apiKey,
          headers: config.headers,
        })
        break
        
      case 'anthropic':
        sdk = createAnthropic({
          name: config.id,
          baseURL: config.baseURL,
          apiKey: config.apiKey,
          headers: config.headers,
        })
        break
        
      default:
        throw new Error(`Unsupported provider type: ${config.type}`)
    }
    
    // 缓存 SDK 实例
    this.sdkInstances.set(providerId, sdk)
    
    return sdk
  }
  
  private validateConfig(config: ProviderConfig): void {
    if (!config.id || !config.name) {
      throw new Error('Provider ID and name are required')
    }
    
    if (!config.baseURL) {
      throw new Error('Provider baseURL is required')
    }
    
    if (!config.apiKey) {
      throw new Error('Provider apiKey is required')
    }
    
    // 验证模型配置
    for (const model of config.models) {
      if (!model.id || !model.modelId) {
        throw new Error('Model ID and modelId are required')
      }
    }
  }
}
```

---

## 4. 内置 Provider

### 4.1 阿里云百炼（DashScope）

```typescript
// packages/llm/src/providers/builtin/dashscope.ts

import type { ProviderConfig, ModelConfig } from '../types'

export const DASHSCOPE_MODELS: ModelConfig[] = [
  {
    id: 'qwen-max',
    name: 'Qwen Max',
    modelId: 'qwen-max',
    supportsThinking: true,
    supportsStreaming: true,
    supportsToolCalling: true,
    maxContextLength: 32768,
    maxOutputTokens: 8192,
    defaultTemperature: 0.7,
    costTier: 'high',
    enabled: true,
  },
  {
    id: 'qwen-plus',
    name: 'Qwen Plus',
    modelId: 'qwen-plus',
    supportsThinking: false,
    supportsStreaming: true,
    supportsToolCalling: true,
    maxContextLength: 16384,
    maxOutputTokens: 4096,
    defaultTemperature: 0.7,
    costTier: 'medium',
    enabled: true,
  },
  {
    id: 'qwen-turbo',
    name: 'Qwen Turbo',
    modelId: 'qwen-turbo',
    supportsThinking: false,
    supportsStreaming: true,
    supportsToolCalling: true,
    maxContextLength: 8192,
    maxOutputTokens: 2048,
    defaultTemperature: 0.7,
    costTier: 'low',
    enabled: true,
  },
]

export function createDashScopeConfig(apiKey: string): ProviderConfig {
  return {
    id: 'dashscope',
    name: '阿里云百炼',
    type: 'openai-compatible',
    baseURL: 'https://dashscope.aliyuncs.com/compatible-mode/v1',
    apiKey,
    headers: {
      // 启用思考模式（可选）
      // 'X-DashScope-Enable-Thinking': 'enabled',
    },
    defaultModel: 'qwen-max',
    models: DASHSCOPE_MODELS,
    enabled: true,
    createdAt: new Date(),
    updatedAt: new Date(),
  }
}
```

### 4.2 Anthropic

```typescript
// packages/llm/src/providers/builtin/anthropic.ts

import type { ProviderConfig, ModelConfig } from '../types'

export const ANTHROPIC_MODELS: ModelConfig[] = [
  {
    id: 'claude-opus-4',
    name: 'Claude Opus 4',
    modelId: 'claude-opus-4-20250514',
    supportsThinking: true,
    supportsStreaming: true,
    supportsToolCalling: true,
    maxContextLength: 200000,
    maxOutputTokens: 32000,
    defaultTemperature: 0.7,
    costTier: 'high',
    enabled: true,
  },
  {
    id: 'claude-sonnet-4',
    name: 'Claude Sonnet 4',
    modelId: 'claude-sonnet-4-20250514',
    supportsThinking: true,
    supportsStreaming: true,
    supportsToolCalling: true,
    maxContextLength: 200000,
    maxOutputTokens: 16000,
    defaultTemperature: 0.7,
    costTier: 'medium',
    enabled: true,
  },
  {
    id: 'claude-haiku-4',
    name: 'Claude Haiku 4',
    modelId: 'claude-haiku-4-20250514',
    supportsThinking: false,
    supportsStreaming: true,
    supportsToolCalling: true,
    maxContextLength: 200000,
    maxOutputTokens: 8192,
    defaultTemperature: 0.7,
    costTier: 'low',
    enabled: true,
  },
]

export function createAnthropicConfig(apiKey: string): ProviderConfig {
  return {
    id: 'anthropic',
    name: 'Anthropic',
    type: 'anthropic',
    baseURL: 'https://api.anthropic.com',
    apiKey,
    defaultModel: 'claude-sonnet-4',
    models: ANTHROPIC_MODELS,
    enabled: true,
    createdAt: new Date(),
    updatedAt: new Date(),
  }
}
```

### 4.3 OpenAI

```typescript
// packages/llm/src/providers/builtin/openai.ts

import type { ProviderConfig, ModelConfig } from '../types'

export const OPENAI_MODELS: ModelConfig[] = [
  {
    id: 'gpt-4o',
    name: 'GPT-4o',
    modelId: 'gpt-4o',
    supportsThinking: false,
    supportsStreaming: true,
    supportsToolCalling: true,
    maxContextLength: 128000,
    maxOutputTokens: 16384,
    defaultTemperature: 0.7,
    costTier: 'high',
    enabled: true,
  },
  {
    id: 'gpt-4o-mini',
    name: 'GPT-4o Mini',
    modelId: 'gpt-4o-mini',
    supportsThinking: false,
    supportsStreaming: true,
    supportsToolCalling: true,
    maxContextLength: 128000,
    maxOutputTokens: 16384,
    defaultTemperature: 0.7,
    costTier: 'low',
    enabled: true,
  },
]

export function createOpenAIConfig(apiKey: string): ProviderConfig {
  return {
    id: 'openai',
    name: 'OpenAI',
    type: 'openai',
    baseURL: 'https://api.openai.com/v1',
    apiKey,
    defaultModel: 'gpt-4o',
    models: OPENAI_MODELS,
    enabled: true,
    createdAt: new Date(),
    updatedAt: new Date(),
  }
}
```

---

## 5. 动态 Provider 管理

### 5.1 Web 端添加 Provider

```typescript
// packages/llm/src/providers/dynamic-manager.ts

import type { ProviderConfig } from './types'
import { ProviderRegistry } from './registry'

export class DynamicProviderManager {
  constructor(private registry: ProviderRegistry) {}
  
  /**
   * 添加自定义 Provider
   */
  async addCustomProvider(input: {
    name: string
    type: 'openai-compatible' | 'anthropic' | 'openai'
    baseURL: string
    apiKey: string
    headers?: Record<string, string>
    models: Array<{
      name: string
      modelId: string
      supportsThinking?: boolean
      maxContextLength?: number
    }>
  }): Promise<ProviderConfig> {
    // 生成唯一 ID
    const id = `custom-${Date.now()}`
    
    const config: ProviderConfig = {
      id,
      name: input.name,
      type: input.type,
      baseURL: input.baseURL,
      apiKey: input.apiKey,
      headers: input.headers,
      models: input.models.map((m, i) => ({
        id: `${id}-model-${i}`,
        name: m.name,
        modelId: m.modelId,
        supportsThinking: m.supportsThinking ?? false,
        supportsStreaming: true,
        supportsToolCalling: true,
        maxContextLength: m.maxContextLength ?? 8192,
        enabled: true,
      })),
      enabled: true,
      createdAt: new Date(),
      updatedAt: new Date(),
    }
    
    await this.registry.register(config)
    
    return config
  }
  
  /**
   * 测试 Provider 连接
   */
  async testConnection(providerId: string): Promise<{
    success: boolean
    error?: string
    availableModels?: string[]
  }> {
    try {
      const available = await this.registry.checkAvailability(providerId)
      
      if (!available) {
        return { success: false, error: 'Provider not available' }
      }
      
      const config = await this.registry.get(providerId)
      
      return {
        success: true,
        availableModels: config?.models.map(m => m.modelId),
      }
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      }
    }
  }
}
```

---

## 6. 存储层

### 6.1 PostgreSQL Schema

```sql
-- providers 表
CREATE TABLE providers (
  id VARCHAR(64) PRIMARY KEY,
  name VARCHAR(128) NOT NULL,
  type VARCHAR(32) NOT NULL,
  base_url VARCHAR(512) NOT NULL,
  api_key TEXT NOT NULL,  -- 加密存储
  headers JSONB,
  default_model VARCHAR(64),
  enabled BOOLEAN DEFAULT true,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- models 表
CREATE TABLE models (
  id VARCHAR(64) PRIMARY KEY,
  provider_id VARCHAR(64) NOT NULL REFERENCES providers(id) ON DELETE CASCADE,
  name VARCHAR(128) NOT NULL,
  model_id VARCHAR(128) NOT NULL,
  supports_thinking BOOLEAN DEFAULT false,
  supports_streaming BOOLEAN DEFAULT true,
  supports_tool_calling BOOLEAN DEFAULT true,
  max_context_length INTEGER,
  max_output_tokens INTEGER,
  default_temperature REAL,
  cost_tier VARCHAR(16),
  enabled BOOLEAN DEFAULT true,
  config JSONB,
  
  UNIQUE(provider_id, model_id)
);

-- 索引
CREATE INDEX idx_models_provider_id ON models(provider_id);
CREATE INDEX idx_providers_enabled ON providers(enabled);
```

### 6.2 存储实现

```typescript
// packages/llm/src/config/storage.ts

import type { ProviderConfig, ModelConfig } from '../providers/types'
import { Pool } from 'pg'
import Redis from 'ioredis'

export class ConfigStorage {
  private pg: Pool
  private redis: Redis
  private encryptionKey: string
  
  constructor(pg: Pool, redis: Redis, encryptionKey: string) {
    this.pg = pg
    this.redis = redis
    this.encryptionKey = encryptionKey
  }
  
  async saveProvider(config: ProviderConfig): Promise<void> {
    const client = await this.pg.connect()
    
    try {
      await client.query('BEGIN')
      
      // 加密 API Key
      const encryptedKey = this.encrypt(config.apiKey)
      
      // 保存 Provider
      await client.query(`
        INSERT INTO providers (id, name, type, base_url, api_key, headers, default_model, enabled, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())
        ON CONFLICT (id) DO UPDATE SET
          name = $2, type = $3, base_url = $4, api_key = $5, headers = $6,
          default_model = $7, enabled = $8, updated_at = NOW()
      `, [
        config.id,
        config.name,
        config.type,
        config.baseURL,
        encryptedKey,
        config.headers || null,
        config.defaultModel || null,
        config.enabled,
      ])
      
      // 删除旧模型
      await client.query('DELETE FROM models WHERE provider_id = $1', [config.id])
      
      // 保存模型
      for (const model of config.models) {
        await client.query(`
          INSERT INTO models (id, provider_id, name, model_id, supports_thinking,
            supports_streaming, supports_tool_calling, max_context_length,
            max_output_tokens, default_temperature, cost_tier, enabled)
          VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        `, [
          model.id,
          config.id,
          model.name,
          model.modelId,
          model.supportsThinking || false,
          model.supportsStreaming !== false,
          model.supportsToolCalling !== false,
          model.maxContextLength || null,
          model.maxOutputTokens || null,
          model.defaultTemperature || null,
          model.costTier || null,
          model.enabled,
        ])
      }
      
      await client.query('COMMIT')
      
      // 更新 Redis 缓存
      await this.cacheProvider(config)
      
      // 发布配置更新事件
      await this.publishConfigUpdate(config.id)
      
    } catch (error) {
      await client.query('ROLLBACK')
      throw error
    } finally {
      client.release()
    }
  }
  
  async getProvider(id: string): Promise<ProviderConfig | null> {
    // 先查 Redis
    const cached = await this.redis.get(`provider:${id}`)
    if (cached) {
      return JSON.parse(cached)
    }
    
    // 回源 PostgreSQL
    const result = await this.pg.query(`
      SELECT p.*, 
        json_agg(m.*) FILTER (WHERE m.id IS NOT NULL) as models
      FROM providers p
      LEFT JOIN models m ON p.id = m.provider_id
      WHERE p.id = $1
      GROUP BY p.id
    `, [id])
    
    if (result.rows.length === 0) {
      return null
    }
    
    const row = result.rows[0]
    const config = this.rowToConfig(row)
    
    // 缓存到 Redis
    await this.cacheProvider(config)
    
    return config
  }
  
  async listProviders(): Promise<ProviderConfig[]> {
    const result = await this.pg.query(`
      SELECT p.*, 
        json_agg(m.*) FILTER (WHERE m.id IS NOT NULL) as models
      FROM providers p
      LEFT JOIN models m ON p.id = m.provider_id
      GROUP BY p.id
      ORDER BY p.created_at
    `)
    
    return result.rows.map(row => this.rowToConfig(row))
  }
  
  async deleteProvider(id: string): Promise<void> {
    await this.pg.query('DELETE FROM providers WHERE id = $1', [id])
    await this.redis.del(`provider:${id}`)
    await this.publishConfigUpdate(id)
  }
  
  private encrypt(value: string): string {
    // TODO: 实现加密
    return value
  }
  
  private decrypt(value: string): string {
    // TODO: 实现解密
    return value
  }
  
  private async cacheProvider(config: ProviderConfig): Promise<void> {
    await this.redis.setex(
      `provider:${config.id}`,
      300, // 5分钟 TTL
      JSON.stringify(config)
    )
  }
  
  private async publishConfigUpdate(providerId: string): Promise<void> {
    // 发布到 Redis channel，触发 WebSocket 推送
    await this.redis.publish('config:update', JSON.stringify({
      type: 'provider',
      id: providerId,
      timestamp: Date.now(),
    }))
  }
  
  private rowToConfig(row: any): ProviderConfig {
    return {
      id: row.id,
      name: row.name,
      type: row.type,
      baseURL: row.base_url,
      apiKey: this.decrypt(row.api_key),
      headers: row.headers || undefined,
      defaultModel: row.default_model || undefined,
      models: (row.models || []).map((m: any) => ({
        id: m.id,
        name: m.name,
        modelId: m.model_id,
        supportsThinking: m.supports_thinking,
        supportsStreaming: m.supports_streaming,
        supportsToolCalling: m.supports_tool_calling,
        maxContextLength: m.max_context_length,
        maxOutputTokens: m.max_output_tokens,
        defaultTemperature: m.default_temperature,
        costTier: m.cost_tier,
        enabled: m.enabled,
      })),
      enabled: row.enabled,
      createdAt: row.created_at,
      updatedAt: row.updated_at,
    }
  }
}
```

---

## 7. 相关文档

- [01-overview.md](./01-overview.md) - 概述与目标
- [03-model-resolution.md](./03-model-resolution.md) - 模型解析与 Fallback
- [04-hot-reload.md](./04-hot-reload.md) - 热重载机制