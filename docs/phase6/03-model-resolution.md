# Phase 6: 模型解析与 Fallback Chain

**文档版本**: 1.0
**最后更新**: 2026-03-16

---

## 1. 概述

模型解析系统负责根据 Agent 需求选择最优模型，当首选模型不可用时自动切换到备用模型。

### 核心功能
- **4步解析流水线**: Override → Category → Fallback → System Default
- **Fallback Chain**: 多级备用模型链
- **Category 映射**: 任务类型 → 最优模型
- **可用性检测**: 模型在线状态检查

---

## 2. 类型定义

```typescript
// packages/llm/src/models/types.ts

/**
 * Fallback 条目
 */
export interface FallbackEntry {
  /** 允许的供应商列表 */
  providers: string[]
  
  /** 模型标识符 */
  model: string
  
  /** 变体（如 "max", "high", "medium"） */
  variant?: string
}

/**
 * 模型需求定义
 */
export interface ModelRequirement {
  /** Fallback Chain */
  fallbackChain: FallbackEntry[]
  
  /** 默认变体 */
  variant?: string
  
  /** 需要特定模型（模糊匹配） */
  requiresModel?: string
  
  /** 需要至少一个模型可用 */
  requiresAnyModel?: boolean
  
  /** 需要特定供应商 */
  requiresProvider?: string[]
}

/**
 * Agent 模型需求映射
 */
export type AgentModelRequirements = Record<string, ModelRequirement>

/**
 * Category 模型需求映射
 */
export type CategoryModelRequirements = Record<string, ModelRequirement>

/**
 * 模型解析结果
 */
export interface ModelResolution {
  /** 选中的供应商 ID */
  providerId: string
  
  /** 选中的模型 ID */
  modelId: string
  
  /** 变体 */
  variant?: string
  
  /** 供应商配置 */
  providerConfig: ProviderConfig
  
  /** 模型配置 */
  modelConfig: ModelConfig
  
  /** 是否来自 Fallback */
  isFallback: boolean
}
```

---

## 3. Fallback Chain 实现

### 3.1 OpenNovel Agent Fallback Chain

参考 Oh My OpenCode 的设计，为 OpenNovel 的 8 个 Agent 定义 Fallback Chain：

```typescript
// packages/llm/src/models/agent-requirements.ts

import type { AgentModelRequirements } from './types'

/**
 * OpenNovel Agent 模型需求
 * 
 * 设计原则：
 * - 天道（主控）：首选强推理模型（Claude Opus / Qwen Max）
 * - 执笔（写作）：首选高质量长文本模型（Claude Opus / Qwen Max）
 * - 世界观守护者：中等模型即可（Claude Sonnet / Qwen Plus）
 * - 审阅：中等模型（Claude Sonnet / Qwen Plus）
 * - 调研者：快速模型（Claude Haiku / Qwen Turbo）
 */
export const NOVEL_AGENT_MODEL_REQUIREMENTS: AgentModelRequirements = {
  // 天道 - 主控 Agent
  'tian-dao': {
    fallbackChain: [
      { providers: ['anthropic'], model: 'claude-opus-4', variant: 'max' },
      { providers: ['dashscope'], model: 'qwen-max' },
      { providers: ['openai'], model: 'gpt-4o' },
    ],
    requiresAnyModel: true,
  },
  
  // 执笔 - 写作 Agent
  'writer': {
    fallbackChain: [
      { providers: ['anthropic'], model: 'claude-opus-4', variant: 'max' },
      { providers: ['dashscope'], model: 'qwen-max' },
      { providers: ['openai'], model: 'gpt-4o' },
    ],
    requiresAnyModel: true,
  },
  
  // 世界观守护者 - 一致性检查
  'world-guardian': {
    fallbackChain: [
      { providers: ['anthropic'], model: 'claude-sonnet-4' },
      { providers: ['dashscope'], model: 'qwen-plus' },
      { providers: ['openai'], model: 'gpt-4o-mini' },
    ],
    requiresAnyModel: true,
  },
  
  // 规划者
  'planner': {
    fallbackChain: [
      { providers: ['anthropic'], model: 'claude-opus-4', variant: 'high' },
      { providers: ['dashscope'], model: 'qwen-max' },
      { providers: ['openai'], model: 'gpt-4o' },
    ],
    requiresAnyModel: true,
  },
  
  // 审阅
  'reviewer': {
    fallbackChain: [
      { providers: ['anthropic'], model: 'claude-sonnet-4' },
      { providers: ['dashscope'], model: 'qwen-plus' },
    ],
    requiresAnyModel: true,
  },
  
  // 观察者
  'observer': {
    fallbackChain: [
      { providers: ['anthropic'], model: 'claude-haiku-4' },
      { providers: ['dashscope'], model: 'qwen-turbo' },
    ],
    requiresAnyModel: true,
  },
  
  // 调研者
  'researcher': {
    fallbackChain: [
      { providers: ['anthropic'], model: 'claude-haiku-4' },
      { providers: ['dashscope'], model: 'qwen-turbo' },
      { providers: ['openai'], model: 'gpt-4o-mini' },
    ],
    requiresAnyModel: true,
  },
  
  // 刘和平 - 风格专家
  'liuheping': {
    fallbackChain: [
      { providers: ['anthropic'], model: 'claude-opus-4', variant: 'high' },
      { providers: ['dashscope'], model: 'qwen-max' },
    ],
    requiresAnyModel: true,
  },
}
```

### 3.2 Category Fallback Chain

```typescript
// packages/llm/src/models/category-requirements.ts

import type { CategoryModelRequirements } from './types'

/**
 * OpenNovel Category 模型需求
 * 
 * Category 系统允许通过任务类型自动选择最优模型：
 * - chapter-generation: 章节正文生成（高质量长文本）
 * - outline-design: 大纲设计（强推理）
 * - consistency-check: 一致性检查（中等模型）
 * - quick-edit: 快速编辑（低延迟）
 */
export const NOVEL_CATEGORY_MODEL_REQUIREMENTS: CategoryModelRequirements = {
  // 章节生成 - 高质量长文本
  'chapter-generation': {
    fallbackChain: [
      { providers: ['anthropic'], model: 'claude-opus-4', variant: 'max' },
      { providers: ['dashscope'], model: 'qwen-max' },
      { providers: ['openai'], model: 'gpt-4o' },
    ],
  },
  
  // 大纲设计 - 强推理
  'outline-design': {
    fallbackChain: [
      { providers: ['anthropic'], model: 'claude-opus-4', variant: 'max' },
      { providers: ['dashscope'], model: 'qwen-max' },
    ],
  },
  
  // 一致性检查 - 中等模型
  'consistency-check': {
    fallbackChain: [
      { providers: ['anthropic'], model: 'claude-sonnet-4' },
      { providers: ['dashscope'], model: 'qwen-plus' },
    ],
  },
  
  // 快速编辑 - 低延迟
  'quick-edit': {
    fallbackChain: [
      { providers: ['anthropic'], model: 'claude-haiku-4' },
      { providers: ['dashscope'], model: 'qwen-turbo' },
      { providers: ['openai'], model: 'gpt-4o-mini' },
    ],
  },
  
  // 研究分析
  'research': {
    fallbackChain: [
      { providers: ['anthropic'], model: 'claude-sonnet-4' },
      { providers: ['dashscope'], model: 'qwen-plus' },
    ],
  },
  
  // 风格模仿
  'style-imitation': {
    fallbackChain: [
      { providers: ['anthropic'], model: 'claude-opus-4', variant: 'high' },
      { providers: ['dashscope'], model: 'qwen-max' },
    ],
  },
}
```

---

## 4. 模型解析流水线

### 4.1 4步解析流程

```
┌─────────────────────────────────────────────────────────────────────────┐
│                      Model Resolution Pipeline                           │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Input: { agentName, category, userOverride? }                         │
│                                                                         │
│  Step 1: User Override                                                  │
│  ├─ 用户是否指定了特定模型？                                             │
│  └─ 如果是 → 直接使用用户指定的模型                                       │
│                                                                         │
│  Step 2: Category Default                                               │
│  ├─ 是否指定了 category？                                               │
│  └─ 如果是 → 使用 category 对应的默认模型                                │
│                                                                         │
│  Step 3: Agent Fallback Chain                                          │
│  ├─ Agent 是否有定义 Fallback Chain？                                   │
│  ├─ 遍历 Fallback Chain，找到第一个可用的模型                            │
│  └─ 检查 Provider 是否连接、模型是否可用                                 │
│                                                                         │
│  Step 4: System Default                                                 │
│  ├─ 使用系统默认模型（配置文件指定）                                      │
│  └─ 如果仍然失败 → 抛出错误                                              │
│                                                                         │
│  Output: { providerId, modelId, variant, ... }                         │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 4.2 解析器实现

```typescript
// packages/llm/src/models/resolution.ts

import type { 
  ModelRequirement, 
  FallbackEntry, 
  ModelResolution,
  AgentModelRequirements,
  CategoryModelRequirements 
} from './types'
import type { ProviderRegistry } from '../providers/registry'
import type { ProviderConfig, ModelConfig } from '../providers/types'
import { Logger } from '../utils/logger'

const log = new Logger('ModelResolution')

export interface ResolutionInput {
  /** Agent 名称 */
  agentName?: string
  
  /** Category 名称 */
  category?: string
  
  /** 用户指定的模型 */
  userOverride?: {
    providerId: string
    modelId: string
    variant?: string
  }
  
  /** Agent 模型需求映射 */
  agentRequirements?: AgentModelRequirements
  
  /** Category 模型需求映射 */
  categoryRequirements?: CategoryModelRequirements
}

export interface ResolutionContext {
  /** 可用的供应商 ID 列表 */
  availableProviders: Set<string>
  
  /** 可用的模型列表（providerId:modelId） */
  availableModels: Set<string>
  
  /** 系统默认模型 */
  systemDefault?: {
    providerId: string
    modelId: string
  }
}

export class ModelResolver {
  constructor(
    private providerRegistry: ProviderRegistry,
    private defaultAgentRequirements: AgentModelRequirements,
    private defaultCategoryRequirements: CategoryModelRequirements
  ) {}
  
  /**
   * 解析模型
   */
  async resolve(input: ResolutionInput): Promise<ModelResolution> {
    const context = await this.buildContext()
    
    // Step 1: User Override
    if (input.userOverride) {
      const resolution = await this.tryResolveOverride(input.userOverride, context)
      if (resolution) {
        log.debug(`Resolved via user override: ${resolution.providerId}/${resolution.modelId}`)
        return resolution
      }
    }
    
    // Step 2: Category Default
    if (input.category) {
      const resolution = await this.tryResolveCategory(
        input.category,
        input.categoryRequirements ?? this.defaultCategoryRequirements,
        context
      )
      if (resolution) {
        log.debug(`Resolved via category: ${resolution.providerId}/${resolution.modelId}`)
        return resolution
      }
    }
    
    // Step 3: Agent Fallback Chain
    if (input.agentName) {
      const resolution = await this.tryResolveAgent(
        input.agentName,
        input.agentRequirements ?? this.defaultAgentRequirements,
        context
      )
      if (resolution) {
        log.debug(`Resolved via agent fallback: ${resolution.providerId}/${resolution.modelId}`)
        return resolution
      }
    }
    
    // Step 4: System Default
    if (context.systemDefault) {
      const resolution = await this.tryResolveOverride(context.systemDefault, context)
      if (resolution) {
        log.debug(`Resolved via system default: ${resolution.providerId}/${resolution.modelId}`)
        return resolution
      }
    }
    
    throw new Error('No available model found')
  }
  
  /**
   * 构建解析上下文
   */
  private async buildContext(): Promise<ResolutionContext> {
    const providers = await this.providerRegistry.listEnabled()
    
    const availableProviders = new Set<string>()
    const availableModels = new Set<string>()
    
    for (const provider of providers) {
      availableProviders.add(provider.id)
      
      for (const model of provider.models) {
        if (model.enabled) {
          availableModels.add(`${provider.id}:${model.modelId}`)
        }
      }
    }
    
    return {
      availableProviders,
      availableModels,
      systemDefault: {
        providerId: 'dashscope',
        modelId: 'qwen-max',
      },
    }
  }
  
  /**
   * 尝试使用用户指定的模型
   */
  private async tryResolveOverride(
    override: { providerId: string; modelId: string; variant?: string },
    context: ResolutionContext
  ): Promise<ModelResolution | null> {
    // 检查供应商是否可用
    if (!context.availableProviders.has(override.providerId)) {
      return null
    }
    
    // 检查模型是否可用
    const modelKey = `${override.providerId}:${override.modelId}`
    if (!context.availableModels.has(modelKey)) {
      return null
    }
    
    // 获取配置
    const providerConfig = await this.providerRegistry.get(override.providerId)
    if (!providerConfig) return null
    
    const modelConfig = providerConfig.models.find(m => m.modelId === override.modelId)
    if (!modelConfig) return null
    
    return {
      providerId: override.providerId,
      modelId: override.modelId,
      variant: override.variant,
      providerConfig,
      modelConfig,
      isFallback: false,
    }
  }
  
  /**
   * 尝试使用 Category 默认模型
   */
  private async tryResolveCategory(
    category: string,
    requirements: CategoryModelRequirements,
    context: ResolutionContext
  ): Promise<ModelResolution | null> {
    const requirement = requirements[category]
    if (!requirement) return null
    
    return this.resolveFallbackChain(requirement.fallbackChain, context)
  }
  
  /**
   * 尝试使用 Agent Fallback Chain
   */
  private async tryResolveAgent(
    agentName: string,
    requirements: AgentModelRequirements,
    context: ResolutionContext
  ): Promise<ModelResolution | null> {
    const requirement = requirements[agentName]
    if (!requirement) return null
    
    return this.resolveFallbackChain(requirement.fallbackChain, context)
  }
  
  /**
   * 解析 Fallback Chain
   */
  private async resolveFallbackChain(
    chain: FallbackEntry[],
    context: ResolutionContext
  ): Promise<ModelResolution | null> {
    for (const entry of chain) {
      // 检查是否有可用的供应商
      for (const providerId of entry.providers) {
        if (!context.availableProviders.has(providerId)) continue
        
        // 检查模型是否可用
        const modelKey = `${providerId}:${entry.model}`
        if (!context.availableModels.has(modelKey)) continue
        
        // 获取配置
        const providerConfig = await this.providerRegistry.get(providerId)
        if (!providerConfig) continue
        
        const modelConfig = providerConfig.models.find(m => m.modelId === entry.model)
        if (!modelConfig) continue
        
        return {
          providerId,
          modelId: entry.model,
          variant: entry.variant,
          providerConfig,
          modelConfig,
          isFallback: true,
        }
      }
    }
    
    return null
  }
}
```

---

## 5. Category 映射系统

### 5.1 Category 定义

```typescript
// packages/llm/src/models/category-mapping.ts

import type { CategoryModelRequirements } from './types'

/**
 * Category 定义
 */
export interface CategoryDefinition {
  /** Category 名称 */
  name: string
  
  /** 描述 */
  description: string
  
  /** 领域 */
  domain: string
  
  /** 推荐场景 */
  useCases: string[]
  
  /** 模型需求 */
  modelRequirement: CategoryModelRequirements[string]
}

/**
 * OpenNovel 内置 Category
 */
export const NOVEL_CATEGORIES: CategoryDefinition[] = [
  {
    name: 'chapter-generation',
    description: '章节正文生成',
    domain: '写作',
    useCases: [
      '生成章节正文',
      '扩写场景描写',
      '人物对话生成',
    ],
    modelRequirement: {
      fallbackChain: [
        { providers: ['anthropic'], model: 'claude-opus-4', variant: 'max' },
        { providers: ['dashscope'], model: 'qwen-max' },
      ],
    },
  },
  {
    name: 'outline-design',
    description: '大纲设计与剧情演化',
    domain: '规划',
    useCases: [
      '设计章节大纲',
      '埋设伏笔',
      '剧情转折设计',
    ],
    modelRequirement: {
      fallbackChain: [
        { providers: ['anthropic'], model: 'claude-opus-4', variant: 'max' },
        { providers: ['dashscope'], model: 'qwen-max' },
      ],
    },
  },
  {
    name: 'consistency-check',
    description: '一致性检查',
    domain: '审核',
    useCases: [
      '检查人物设定一致性',
      '检查时间线一致性',
      '检查世界观一致性',
    ],
    modelRequirement: {
      fallbackChain: [
        { providers: ['anthropic'], model: 'claude-sonnet-4' },
        { providers: ['dashscope'], model: 'qwen-plus' },
      ],
    },
  },
  {
    name: 'quick-edit',
    description: '快速编辑',
    domain: '编辑',
    useCases: [
      '修正错别字',
      '调整语句流畅度',
      '简单格式修改',
    ],
    modelRequirement: {
      fallbackChain: [
        { providers: ['anthropic'], model: 'claude-haiku-4' },
        { providers: ['dashscope'], model: 'qwen-turbo' },
      ],
    },
  },
]

/**
 * 根据 Agent 任务类型推荐 Category
 */
export function recommendCategory(taskType: string): string | null {
  const mapping: Record<string, string> = {
    'write_chapter': 'chapter-generation',
    'design_outline': 'outline-design',
    'check_consistency': 'consistency-check',
    'quick_fix': 'quick-edit',
    'research': 'research',
    'style_analysis': 'style-imitation',
  }
  
  return mapping[taskType] ?? null
}
```

---

## 6. 使用示例

### 6.1 Agent 调用

```typescript
import { ModelResolver } from './models/resolution'
import { ProviderRegistry } from './providers/registry'
import { NOVEL_AGENT_MODEL_REQUIREMENTS, NOVEL_CATEGORY_MODEL_REQUIREMENTS } from './models'

// 初始化
const registry = new ProviderRegistry(storage)
const resolver = new ModelResolver(
  registry,
  NOVEL_AGENT_MODEL_REQUIREMENTS,
  NOVEL_CATEGORY_MODEL_REQUIREMENTS
)

// Agent 调用时解析模型
async function callAgent(agentName: string, prompt: string) {
  const resolution = await resolver.resolve({ agentName })
  
  // 创建 SDK Provider
  const sdk = await registry.createSDKProvider(resolution.providerId)
  
  // 调用模型
  const result = await generateText({
    model: sdk(resolution.modelId),
    prompt,
  })
  
  return result
}

// Category 调用
async function callByCategory(category: string, prompt: string) {
  const resolution = await resolver.resolve({ category })
  
  const sdk = await registry.createSDKProvider(resolution.providerId)
  
  const result = await generateText({
    model: sdk(resolution.modelId),
    prompt,
  })
  
  return result
}
```

### 6.2 带用户覆盖的调用

```typescript
// 用户指定了特定模型
async function callWithOverride(
  agentName: string,
  userSelectedModel: { providerId: string; modelId: string },
  prompt: string
) {
  const resolution = await resolver.resolve({
    agentName,
    userOverride: userSelectedModel,
  })
  
  // ... 使用 resolution
}
```

---

## 7. 相关文档

- [02-provider-system.md](./02-provider-system.md) - Provider 注册与管理
- [04-hot-reload.md](./04-hot-reload.md) - 热重载机制