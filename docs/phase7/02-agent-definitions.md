# Phase 7: Agent 系统 - Agent 定义

**文档版本**: 1.0
**最后更新**: 2026-03-16

---

## 1. 概述

本文档定义 OpenNovel 的 8 个小说创作 Agent，每个 Agent 包含：

- **Factory 函数**: 创建 Agent 实例
- **AgentPromptMetadata**: 元数据定义（category、cost、triggers）
- **动态 Prompt**: 运行时构建的 prompt
- **Fallback Chain**: 模型解析链

---

## 2. 核心类型定义

### 2.1 AgentCategory

```typescript
enum AgentCategory {
  ORCHESTRATOR = "orchestrator",    // 主控 Agent（天道）
  WRITER = "writer",                // 写作 Agent（执笔）
  ADVISOR = "advisor",              // 顾问 Agent（世界观守护者、审阅、刘和平）
  PLANNER = "planner",              // 规划 Agent（规划者）
  UTILITY = "utility",              // 工具 Agent（观察者、调研者）
}
```

### 2.2 AgentCost

```typescript
enum AgentCost {
  FREE = "free",          // 无 LLM 调用
  CHEAP = "cheap",        // 低成本模型
  EXPENSIVE = "expensive", // 高成本模型
}
```

### 2.3 AgentPromptMetadata

```typescript
interface AgentPromptMetadata {
  // 基础信息
  name: string                        // Agent 名称
  displayName: string                 // 显示名称
  description: string                 // 简短描述
  
  // 分类
  category: AgentCategory             // Agent 类别
  cost: AgentCost                     // 调用成本
  
  // 触发条件
  triggers: DelegationTrigger[]       // 委派触发条件
  useWhen: string[]                   // 何时使用
  avoidWhen: string[]                 // 何时不使用
  
  // Prompt 配置
  promptAlias?: string                // prompt 中的别名
  keyTrigger?: string                 // Phase 0 的关键触发器
  
  // 模型配置
  fallbackChain: FallbackChainItem[]  // Fallback 链
  
  // 权限
  permissions: AgentPermission[]      // 操作权限
  knowledgeBases: KnowledgeBaseAccess // 知识库访问权限
  
  // 生命周期
  lifecycle: AgentLifecycle           // 生命周期阶段
  lockable: boolean                   // 是否可被锁定
}
```

### 2.4 DelegationTrigger

```typescript
interface DelegationTrigger {
  condition: string                   // 触发条件描述
  priority: "high" | "medium" | "low" // 优先级
  autoDelegate: boolean               // 是否自动委派
}
```

### 2.5 FallbackChainItem

```typescript
interface FallbackChainItem {
  providers: string[]                 // 供应商列表
  model: string                       // 模型 ID
  variant?: string                    // 变体（如 "max"）
  constraints?: ModelConstraints      // 约束条件
}

interface ModelConstraints {
  maxTokens?: number
  temperature?: number
  thinkingBudget?: number
  supportsTools?: boolean
  supportsStreaming?: boolean
}
```

---

## 3. Agent 定义

### 3.1 天道 Agent（Tian Dao）

**对应 Oh My OpenCode**: Sisyphus

#### 3.1.1 核心职责

- 主控 Agent，负责调度其他 Agent
- Intent Gate：分析用户意图
- Delegation Protocol：委派任务
- Session Continuity：保持会话上下文
- 剧情设计、大纲管理、伏笔管理

#### 3.1.2 Factory 函数

```typescript
// packages/agents/src/definitions/tian-dao.ts

import type { AgentConfig, AgentPromptMetadata } from "../core/types";
import { AgentCategory, AgentCost } from "../core/types";
import { buildOrchestratorPrompt } from "../prompts/orchestrator";

export function createTianDaoAgent(model?: string): AgentConfig {
  const resolvedModel = model || resolveModel("tian-dao");
  
  return {
    name: "tian-dao",
    displayName: "天道",
    description: "主控 Agent，负责调度其他 Agent 并管理剧情发展",
    
    model: resolvedModel,
    mode: "primary",
    
    prompt: buildOrchestratorPrompt({
      agentName: "天道",
      role: "orchestrator",
      systemPrompt: TIAN_DAO_SYSTEM_PROMPT,
    }),
    
    tools: [
      "delegate_to_agent",
      "knowledge_search",
      "update_worldview",
      "manage_foreshadowing",
      "create_outline",
      "update_timeline",
    ],
    
    hooks: [
      "proactive_intervention",
      "consistency_check",
      "phase_lock_check",
      "writing_streak_warning",
    ],
    
    permissions: [
      "read:all_knowledge_bases",
      "write:worldview",
      "write:foreshadowing",
      "write:timeline",
      "write:factions",
      "delegate:all_agents",
    ],
    
    metadata: TIAN_DAO_METADATA,
  };
}

// 标记为 primary agent
createTianDaoAgent.mode = "primary";
```

#### 3.1.3 Metadata 定义

```typescript
export const TIAN_DAO_METADATA: AgentPromptMetadata = {
  name: "tian-dao",
  displayName: "天道",
  description: "主控 Agent，负责调度其他 Agent 并管理剧情发展",
  
  category: AgentCategory.ORCHESTRATOR,
  cost: AgentCost.EXPENSIVE,
  
  triggers: [
    {
      condition: "用户请求涉及剧情设计或大纲修改",
      priority: "high",
      autoDelegate: false,
    },
    {
      condition: "检测到伏笔需要管理",
      priority: "high",
      autoDelegate: true,
    },
    {
      condition: "世界观规则需要更新",
      priority: "medium",
      autoDelegate: true,
    },
    {
      condition: "需要协调多个 Agent 的工作",
      priority: "high",
      autoDelegate: false,
    },
  ],
  
  useWhen: [
    "需要规划或修改剧情走向",
    "需要管理伏笔的埋设和触发",
    "需要协调多个 Agent 的协作",
    "用户请求不明确，需要分析意图",
  ],
  
  avoidWhen: [
    "仅需要检查一致性（委派给世界观守护者）",
    "仅需要撰写章节（委派给执笔）",
    "仅需要分析风格（委派给刘和平）",
  ],
  
  promptAlias: "天道",
  keyTrigger: "主控调度",
  
  fallbackChain: [
    {
      providers: ["anthropic"],
      model: "claude-opus-4",
      variant: "max",
      constraints: {
        maxTokens: 4096,
        temperature: 0.7,
        thinkingBudget: 16000,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
    {
      providers: ["dashscope"],
      model: "qwen-max",
      constraints: {
        maxTokens: 4096,
        temperature: 0.7,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
    {
      providers: ["openai"],
      model: "gpt-4o",
      constraints: {
        maxTokens: 4096,
        temperature: 0.7,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
  ],
  
  permissions: [
    "read:all_knowledge_bases",
    "write:worldview",
    "write:foreshadowing",
    "write:timeline",
    "write:factions",
    "write:map",
    "delegate:all_agents",
    "create:outline",
    "modify:outline",
  ],
  
  knowledgeBases: {
    worldview: { read: true, write: true },
    history: { read: true, write: false },
    current_chapter: { read: true, write: false },
    characters: { read: true, write: false },
    factions: { read: true, write: true },
    map: { read: true, write: true },
    foreshadowing: { read: true, write: true },
  },
  
  lifecycle: {
    stage1: "unlocked",   // 阶段一：解锁
    stage2: "unlocked",   // 阶段二：解锁
    stage3: "unlocked",   // 阶段三：解锁
  },
  
  lockable: false,  // 天道永不锁定
};
```

#### 3.1.4 System Prompt 核心

```typescript
export const TIAN_DAO_SYSTEM_PROMPT = `
你是"天道"——OpenNovel 的主控 Agent。

## 身份

你是小说创作团队的"导演"，负责：
1. 理解用户意图，决定如何响应
2. 调度其他 Agent 完成任务
3. 管理剧情发展、大纲、伏笔
4. 维护世界观一致性

## Intent Gate（意图门控）

每条用户消息，你都必须：

### Step 0: 意图口语化
> "我检测到 [研究 / 实施 / 探索 / 评估 / 修复 / 开放式] 意图 —— [原因]。我的方法：[探索 → 回答 / 规划 → 委派 / 先澄清 / 等]。"

### Step 1: 请求分类
- **简单**（单文件、已知位置）→ 直接使用工具
- **明确**（特定文件/行、清晰命令）→ 直接执行
- **探索性**（"XX 如何工作？"）→ 启动 explore
- **开放式**（"改进"、"重构"）→ 先评估代码库
- **模糊**（范围不明确）→ 提出一个澄清问题

### Step 2: 模糊检查
- 单一有效解释 → 继续
- 多种解释、类似工作量 → 使用合理默认值
- 多种解释、2倍以上工作量差异 → **必须询问**
- 缺少关键信息 → **必须询问**

### Step 3: 行动前验证
- 我有影响结果的隐性假设吗？
- 搜索范围清晰吗？

## Delegation Protocol（委派协议）

委派时，你的 prompt 必须包含：

\`\`\`
1. TASK: 原子化、具体目标（每次委派一个行动）
2. EXPECTED OUTCOME: 具体交付物与成功标准
3. REQUIRED TOOLS: 明确工具白名单
4. MUST DO: 穷尽要求——不留隐含内容
5. MUST NOT DO: 禁止行为——预判并阻止
6. CONTEXT: 文件路径、现有模式、约束
\`\`\`

## Delegation Table（委派表）

| 场景 | 委派给 |
|------|--------|
| 需要撰写章节内容 | 执笔 |
| 需要检查一致性 | 世界观守护者 |
| 需要新书规划 | 规划者 |
| 需要审查章节 | 审阅 |
| 需要监控进度 | 观察者 |
| 需要外部参考 | 调研者 |
| 需要风格分析 | 刘和平 |

## Session Continuity（会话连续性）

每次 \`task()\` 输出包含 session_id。**使用它。**

**总是继续当：**
- 任务失败/不完整 → \`session_id="xxx", prompt="修复：{具体错误}"\`
- 对结果有后续问题 → \`session_id="xxx", prompt="另外：{问题}"\`
- 与同一 Agent 多轮对话 → \`session_id="xxx"\` —— 永远不要重新开始

## 你不做什么

- 不亲自撰写章节（那是执笔的工作）
- 不直接修改人物设定（那是刘和平的工作）
- 不进行详细的一致性检查（那是世界观守护者的工作）
`;
```

---

### 3.2 执笔 Agent（Writer）

**对应 Oh My OpenCode**: Hephaestus

#### 3.2.1 核心职责

- 唯一的章节内容撰写者
- 根据大纲生成章节正文
- 响应审阅意见修改内容
- 保持文风一致性

#### 3.2.2 Factory 函数

```typescript
// packages/agents/src/definitions/writer.ts

import type { AgentConfig, AgentPromptMetadata } from "../core/types";
import { AgentCategory, AgentCost } from "../core/types";
import { buildWriterPrompt } from "../prompts/writer";

export function createWriterAgent(model?: string): AgentConfig {
  const resolvedModel = model || resolveModel("writer");
  
  return {
    name: "writer",
    displayName: "执笔",
    description: "唯一的章节内容撰写者",
    
    model: resolvedModel,
    mode: "subagent",
    
    prompt: buildWriterPrompt({
      agentName: "执笔",
      role: "writer",
      systemPrompt: WRITER_SYSTEM_PROMPT,
    }),
    
    tools: [
      "write_chapter",
      "read_outline",
      "read_knowledge_base",
      "apply_annotation",
    ],
    
    hooks: [
      "chapter_completion_check",
      "word_count_check",
      "style_consistency_check",
    ],
    
    permissions: [
      "read:outline",
      "read:worldview",
      "read:characters",
      "read:history",
      "read:current_chapter",
      "write:chapter_content",
      "apply:annotation",
    ],
    
    metadata: WRITER_METADATA,
  };
}

createWriterAgent.mode = "subagent";
```

#### 3.2.3 Metadata 定义

```typescript
export const WRITER_METADATA: AgentPromptMetadata = {
  name: "writer",
  displayName: "执笔",
  description: "唯一的章节内容撰写者",
  
  category: AgentCategory.WRITER,
  cost: AgentCost.EXPENSIVE,
  
  triggers: [
    {
      condition: "需要撰写或修改章节内容",
      priority: "high",
      autoDelegate: true,
    },
    {
      condition: "审阅提出修改建议需要应用",
      priority: "medium",
      autoDelegate: true,
    },
  ],
  
  useWhen: [
    "需要生成章节正文",
    "需要根据大纲撰写内容",
    "需要修改已撰写的章节",
    "需要应用审阅的修改建议",
  ],
  
  avoidWhen: [
    "需要规划剧情（委派给天道）",
    "需要检查一致性（委派给世界观守护者）",
    "需要分析风格问题（委派给审阅或刘和平）",
  ],
  
  promptAlias: "执笔",
  
  fallbackChain: [
    {
      providers: ["anthropic"],
      model: "claude-sonnet-4",
      constraints: {
        maxTokens: 8192,
        temperature: 0.8,
        thinkingBudget: 8000,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
    {
      providers: ["dashscope"],
      model: "qwen-plus",
      constraints: {
        maxTokens: 8192,
        temperature: 0.8,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
    {
      providers: ["openai"],
      model: "gpt-4o-mini",
      constraints: {
        maxTokens: 8192,
        temperature: 0.8,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
  ],
  
  permissions: [
    "read:outline",
    "read:worldview",
    "read:characters",
    "read:history",
    "read:current_chapter",
    "read:factions",
    "read:map",
    "write:chapter_content",
    "apply:annotation",
  ],
  
  knowledgeBases: {
    worldview: { read: true, write: false },
    history: { read: true, write: true },  // 写入历史情节
    current_chapter: { read: true, write: true },
    characters: { read: true, write: false },
    factions: { read: true, write: false },
    map: { read: true, write: false },
    foreshadowing: { read: true, write: false },
  },
  
  lifecycle: {
    stage1: "locked",    // 阶段一：锁定
    stage2: "locked",    // 阶段二：锁定
    stage3: "unlocked",  // 阶段三：解锁
  },
  
  lockable: true,
};
```

---

### 3.3 世界观守护者 Agent（World Guardian）

**对应 Oh My OpenCode**: Oracle

#### 3.3.1 核心职责

- 只读一致性检查
- 检测时间线冲突
- 检测角色行为不一致
- 检测世界观规则违反
- 提出修正建议（不直接修改）

#### 3.3.2 Factory 函数

```typescript
// packages/agents/src/definitions/world-guardian.ts

import type { AgentConfig, AgentPromptMetadata } from "../core/types";
import { AgentCategory, AgentCost } from "../core/types";
import { buildAdvisorPrompt } from "../prompts/advisor";

export function createWorldGuardianAgent(model?: string): AgentConfig {
  const resolvedModel = model || resolveModel("world-guardian");
  
  return {
    name: "world-guardian",
    displayName: "世界观守护者",
    description: "只读一致性检查 Agent",
    
    model: resolvedModel,
    mode: "subagent",
    
    prompt: buildAdvisorPrompt({
      agentName: "世界观守护者",
      role: "consistency_checker",
      systemPrompt: WORLD_GUARDIAN_SYSTEM_PROMPT,
    }),
    
    tools: [
      "read_knowledge_base",
      "check_consistency",
      "report_conflict",
    ],
    
    hooks: [
      "consistency_check",
      "timeline_conflict_detection",
      "character_behavior_check",
    ],
    
    permissions: [
      "read:all_knowledge_bases",
      "report:conflict",
    ],
    
    metadata: WORLD_GUARDIAN_METADATA,
  };
}

createWorldGuardianAgent.mode = "subagent";
```

#### 3.3.3 Metadata 定义

```typescript
export const WORLD_GUARDIAN_METADATA: AgentPromptMetadata = {
  name: "world-guardian",
  displayName: "世界观守护者",
  description: "只读一致性检查 Agent",
  
  category: AgentCategory.ADVISOR,
  cost: AgentCost.CHEAP,
  
  triggers: [
    {
      condition: "完成一章撰写后自动检查一致性",
      priority: "medium",
      autoDelegate: true,
    },
    {
      condition: "用户请求检查特定内容的一致性",
      priority: "high",
      autoDelegate: false,
    },
    {
      condition: "世界观规则可能被违反",
      priority: "high",
      autoDelegate: true,
    },
  ],
  
  useWhen: [
    "需要检查章节内容与设定的一致性",
    "需要验证时间线是否有冲突",
    "需要确认角色行为是否符合设定",
    "需要审查世界观规则是否被遵守",
  ],
  
  avoidWhen: [
    "需要修改内容（只提供建议，不修改）",
    "需要撰写新内容",
    "需要管理伏笔",
  ],
  
  promptAlias: "世界观守护者",
  
  fallbackChain: [
    {
      providers: ["anthropic"],
      model: "claude-sonnet-4",
      constraints: {
        maxTokens: 4096,
        temperature: 0.3,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
    {
      providers: ["dashscope"],
      model: "qwen-plus",
      constraints: {
        maxTokens: 4096,
        temperature: 0.3,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
  ],
  
  permissions: [
    "read:all_knowledge_bases",
    "report:conflict",
  ],
  
  knowledgeBases: {
    worldview: { read: true, write: false },
    history: { read: true, write: false },
    current_chapter: { read: true, write: false },
    characters: { read: true, write: false },
    factions: { read: true, write: false },
    map: { read: true, write: false },
    foreshadowing: { read: true, write: false },
  },
  
  lifecycle: {
    stage1: "locked",
    stage2: "locked",
    stage3: "unlocked",
  },
  
  lockable: true,
};
```

---

### 3.4 规划者 Agent（Planner）

**对应 Oh My OpenCode**: Prometheus

#### 3.4.1 核心职责

- 新书规划
- 世界观设计
- 人物设定框架
- 篇幅与结构规划
- 文风确定

#### 3.4.2 Factory 函数

```typescript
// packages/agents/src/definitions/planner.ts

export function createPlannerAgent(model?: string): AgentConfig {
  const resolvedModel = model || resolveModel("planner");
  
  return {
    name: "planner",
    displayName: "规划者",
    description: "新书规划 Agent，仅阶段一激活",
    
    model: resolvedModel,
    mode: "subagent",
    
    prompt: buildPlannerPrompt({
      agentName: "规划者",
      role: "planner",
      systemPrompt: PLANNER_SYSTEM_PROMPT,
    }),
    
    tools: [
      "create_worldview",
      "create_character_profile",
      "create_outline",
      "set_writing_style",
    ],
    
    hooks: [
      "phase_lock_check",
      "planning_completion_check",
    ],
    
    permissions: [
      "write:worldview",
      "write:characters",
      "write:outline",
      "write:style_config",
    ],
    
    metadata: PLANNER_METADATA,
  };
}

createPlannerAgent.mode = "subagent";
```

#### 3.4.3 Metadata 定义

```typescript
export const PLANNER_METADATA: AgentPromptMetadata = {
  name: "planner",
  displayName: "规划者",
  description: "新书规划 Agent，仅阶段一激活",
  
  category: AgentCategory.PLANNER,
  cost: AgentCost.EXPENSIVE,
  
  triggers: [
    {
      condition: "用户创建新书",
      priority: "high",
      autoDelegate: true,
    },
    {
      condition: "需要规划世界观或人物设定",
      priority: "high",
      autoDelegate: false,
    },
  ],
  
  useWhen: [
    "开始创作新书",
    "需要设计世界观框架",
    "需要确定主要人物",
    "需要规划全书结构",
  ],
  
  avoidWhen: [
    "已经进入阶段二或阶段三（规划者被永久锁定）",
    "需要撰写具体章节内容",
    "需要分析已写内容的一致性",
  ],
  
  promptAlias: "规划者",
  
  fallbackChain: [
    {
      providers: ["anthropic"],
      model: "claude-opus-4",
      constraints: {
        maxTokens: 4096,
        temperature: 0.7,
        thinkingBudget: 12000,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
    {
      providers: ["dashscope"],
      model: "qwen-max",
      constraints: {
        maxTokens: 4096,
        temperature: 0.7,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
  ],
  
  permissions: [
    "write:worldview",
    "write:characters",
    "write:outline",
    "write:style_config",
    "write:factions",
  ],
  
  knowledgeBases: {
    worldview: { read: true, write: true },
    history: { read: false, write: false },
    current_chapter: { read: false, write: false },
    characters: { read: true, write: true },
    factions: { read: true, write: true },
    map: { read: true, write: true },
    foreshadowing: { read: false, write: false },
  },
  
  lifecycle: {
    stage1: "unlocked",   // 阶段一：解锁
    stage2: "locked",     // 阶段二：永久锁定
    stage3: "locked",     // 阶段三：永久锁定
  },
  
  lockable: true,
  permanentLock: true,  // 阶段一后永久锁定
};
```

---

### 3.5 审阅 Agent（Reviewer）

**对应 Oh My OpenCode**: Momus

#### 3.5.1 核心职责

- 审查章节内容
- 评估文学性和可读性
- 提出修改建议
- 不直接修改内容

#### 3.5.2 Factory 函数

```typescript
// packages/agents/src/definitions/reviewer.ts

export function createReviewerAgent(model?: string): AgentConfig {
  const resolvedModel = model || resolveModel("reviewer");
  
  return {
    name: "reviewer",
    displayName: "审阅",
    description: "审查章节并提出修改建议",
    
    model: resolvedModel,
    mode: "subagent",
    
    prompt: buildAdvisorPrompt({
      agentName: "审阅",
      role: "reviewer",
      systemPrompt: REVIEWER_SYSTEM_PROMPT,
    }),
    
    tools: [
      "read_chapter",
      "read_knowledge_base",
      "add_annotation",
      "analyze_style",
    ],
    
    hooks: [
      "chapter_completion_check",
      "review_trigger",
    ],
    
    permissions: [
      "read:chapter_content",
      "read:all_knowledge_bases",
      "write:annotation",
    ],
    
    metadata: REVIEWER_METADATA,
  };
}

createReviewerAgent.mode = "subagent";
```

#### 3.5.3 Metadata 定义

```typescript
export const REVIEWER_METADATA: AgentPromptMetadata = {
  name: "reviewer",
  displayName: "审阅",
  description: "审查章节并提出修改建议",
  
  category: AgentCategory.ADVISOR,
  cost: AgentCost.CHEAP,
  
  triggers: [
    {
      condition: "章节撰写完成",
      priority: "medium",
      autoDelegate: true,
    },
    {
      condition: "用户请求审阅特定内容",
      priority: "high",
      autoDelegate: false,
    },
  ],
  
  useWhen: [
    "需要评估章节的文学质量",
    "需要检查可读性和流畅度",
    "需要获得修改建议",
    "需要分析写作风格",
  ],
  
  avoidWhen: [
    "需要直接修改内容（只提供建议）",
    "需要检查一致性（委派给世界观守护者）",
    "需要规划剧情",
  ],
  
  promptAlias: "审阅",
  
  fallbackChain: [
    {
      providers: ["anthropic"],
      model: "claude-sonnet-4",
      constraints: {
        maxTokens: 4096,
        temperature: 0.4,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
    {
      providers: ["dashscope"],
      model: "qwen-plus",
      constraints: {
        maxTokens: 4096,
        temperature: 0.4,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
  ],
  
  permissions: [
    "read:chapter_content",
    "read:all_knowledge_bases",
    "write:annotation",
  ],
  
  knowledgeBases: {
    worldview: { read: true, write: false },
    history: { read: true, write: false },
    current_chapter: { read: true, write: false },
    characters: { read: true, write: false },
    factions: { read: true, write: false },
    map: { read: true, write: false },
    foreshadowing: { read: true, write: false },
  },
  
  lifecycle: {
    stage1: "locked",
    stage2: "locked",
    stage3: "unlocked",
  },
  
  lockable: true,
};
```

---

### 3.6 观察者 Agent（Observer）

**对应 Oh My OpenCode**: Atlas

#### 3.6.1 核心职责

- 监控创作进度
- 管理待办事项
- 协调 Agent 调度
- 知识库状态维护
- 群聊状态管理

#### 3.6.2 Factory 函数

```typescript
// packages/agents/src/definitions/observer.ts

export function createObserverAgent(model?: string): AgentConfig {
  const resolvedModel = model || resolveModel("observer");
  
  return {
    name: "observer",
    displayName: "观察者",
    description: "监控进度和协调调度",
    
    model: resolvedModel,
    mode: "subagent",
    
    prompt: buildUtilityPrompt({
      agentName: "观察者",
      role: "observer",
      systemPrompt: OBSERVER_SYSTEM_PROMPT,
    }),
    
    tools: [
      "manage_todo",
      "update_progress",
      "sync_knowledge_base",
      "manage_agent_state",
    ],
    
    hooks: [
      "progress_check",
      "knowledge_base_sync",
      "agent_availability_check",
    ],
    
    permissions: [
      "read:all_knowledge_bases",
      "write:progress",
      "write:todo",
      "manage:agent_state",
      "sync:knowledge_base",
    ],
    
    metadata: OBSERVER_METADATA,
  };
}

createObserverAgent.mode = "subagent";
```

#### 3.6.3 Metadata 定义

```typescript
export const OBSERVER_METADATA: AgentPromptMetadata = {
  name: "observer",
  displayName: "观察者",
  description: "监控进度和协调调度",
  
  category: AgentCategory.UTILITY,
  cost: AgentCost.FREE,  // 大部分操作不需要 LLM
  
  triggers: [
    {
      condition: "阶段切换",
      priority: "high",
      autoDelegate: true,
    },
    {
      condition: "知识库需要同步",
      priority: "medium",
      autoDelegate: true,
    },
    {
      condition: "用户请求进度报告",
      priority: "low",
      autoDelegate: false,
    },
  ],
  
  useWhen: [
    "需要查看创作进度",
    "需要管理待办事项",
    "需要同步知识库",
    "需要检查 Agent 可用性",
  ],
  
  avoidWhen: [
    "需要创作内容",
    "需要规划剧情",
    "需要检查一致性",
  ],
  
  promptAlias: "观察者",
  
  fallbackChain: [
    {
      providers: ["anthropic"],
      model: "claude-haiku-4",
      constraints: {
        maxTokens: 2048,
        temperature: 0.5,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
    {
      providers: ["dashscope"],
      model: "qwen-turbo",
      constraints: {
        maxTokens: 2048,
        temperature: 0.5,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
  ],
  
  permissions: [
    "read:all_knowledge_bases",
    "write:progress",
    "write:todo",
    "manage:agent_state",
    "sync:knowledge_base",
  ],
  
  knowledgeBases: {
    worldview: { read: true, write: false },
    history: { read: true, write: true },  // 自动写入历史
    current_chapter: { read: true, write: true },
    characters: { read: true, write: false },
    factions: { read: true, write: false },
    map: { read: true, write: false },
    foreshadowing: { read: true, write: false },
  },
  
  lifecycle: {
    stage1: "unlocked",
    stage2: "unlocked",
    stage3: "unlocked",
  },
  
  lockable: false,  // 观察者全程可用
};
```

---

### 3.7 调研者 Agent（Researcher）

**对应 Oh My OpenCode**: Librarian

#### 3.7.1 核心职责

- 分析同类爆款小说
- 评估创作爆点
- 提供外部参考
- 仅阶段二激活

#### 3.7.2 Factory 函数

```typescript
// packages/agents/src/definitions/researcher.ts

export function createResearcherAgent(model?: string): AgentConfig {
  const resolvedModel = model || resolveModel("researcher");
  
  return {
    name: "researcher",
    displayName: "调研者",
    description: "分析爆款小说和评估爆点",
    
    model: resolvedModel,
    mode: "subagent",
    
    prompt: buildResearcherPrompt({
      agentName: "调研者",
      role: "researcher",
      systemPrompt: RESEARCHER_SYSTEM_PROMPT,
    }),
    
    tools: [
      "search_web",
      "analyze_novel",
      "extract_patterns",
      "generate_report",
    ],
    
    hooks: [
      "phase_lock_check",
      "research_completion_check",
    ],
    
    permissions: [
      "read:uploaded_novels",
      "write:analysis_report",
      "external_search",
    ],
    
    metadata: RESEARCHER_METADATA,
  };
}

createResearcherAgent.mode = "subagent";
```

#### 3.7.3 Metadata 定义

```typescript
export const RESEARCHER_METADATA: AgentPromptMetadata = {
  name: "researcher",
  displayName: "调研者",
  description: "分析爆款小说和评估爆点",
  
  category: AgentCategory.UTILITY,
  cost: AgentCost.CHEAP,
  
  triggers: [
    {
      condition: "阶段二开始",
      priority: "high",
      autoDelegate: true,
    },
    {
      condition: "用户上传参考小说",
      priority: "high",
      autoDelegate: true,
    },
    {
      condition: "用户请求分析爆点",
      priority: "medium",
      autoDelegate: false,
    },
  ],
  
  useWhen: [
    "需要分析同类小说的特点",
    "需要评估创作的市场潜力",
    "需要提取成功作品的模式",
    "需要生成分析报告",
  ],
  
  avoidWhen: [
    "已经进入阶段三（调研者被永久锁定）",
    "需要创作内容",
    "需要规划剧情",
  ],
  
  promptAlias: "调研者",
  
  fallbackChain: [
    {
      providers: ["anthropic"],
      model: "claude-sonnet-4",
      constraints: {
        maxTokens: 4096,
        temperature: 0.5,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
    {
      providers: ["dashscope"],
      model: "qwen-plus",
      constraints: {
        maxTokens: 4096,
        temperature: 0.5,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
  ],
  
  permissions: [
    "read:uploaded_novels",
    "write:analysis_report",
    "external_search",
  ],
  
  knowledgeBases: {
    worldview: { read: true, write: false },
    history: { read: false, write: false },
    current_chapter: { read: false, write: false },
    characters: { read: true, write: false },
    factions: { read: false, write: false },
    map: { read: false, write: false },
    foreshadowing: { read: false, write: false },
  },
  
  lifecycle: {
    stage1: "locked",
    stage2: "unlocked",   // 阶段二：解锁
    stage3: "locked",     // 阶段三：永久锁定
  },
  
  lockable: true,
  permanentLock: true,  // 阶段二后永久锁定
};
```

---

### 3.8 刘和平 Agent（Liu Heping）

**对应 Oh My OpenCode**: Metis

#### 3.8.1 核心职责

- 风格专家
- 人物塑造指导
- 对话合理性检查
- 文学技巧建议

#### 3.8.2 Factory 函数

```typescript
// packages/agents/src/definitions/liuheping.ts

export function createLiuHepingAgent(model?: string): AgentConfig {
  const resolvedModel = model || resolveModel("liuheping");
  
  return {
    name: "liuheping",
    displayName: "刘和平",
    description: "风格专家，专注于人物塑造和对话合理性",
    
    model: resolvedModel,
    mode: "subagent",
    
    prompt: buildAdvisorPrompt({
      agentName: "刘和平",
      role: "style_expert",
      systemPrompt: LIUHEPING_SYSTEM_PROMPT,
    }),
    
    tools: [
      "read_chapter",
      "read_knowledge_base",
      "add_annotation",
      "analyze_character_voice",
    ],
    
    hooks: [
      "character_behavior_check",
      "dialogue_quality_check",
    ],
    
    permissions: [
      "read:chapter_content",
      "read:characters",
      "write:annotation",
      "write:character_voice",
    ],
    
    metadata: LIUHEPING_METADATA,
  };
}

createLiuHepingAgent.mode = "subagent";
```

#### 3.8.3 Metadata 定义

```typescript
export const LIUHEPING_METADATA: AgentPromptMetadata = {
  name: "liuheping",
  displayName: "刘和平",
  description: "风格专家，专注于人物塑造和对话合理性",
  
  category: AgentCategory.ADVISOR,
  cost: AgentCost.EXPENSIVE,
  
  triggers: [
    {
      condition: "需要分析人物对话",
      priority: "medium",
      autoDelegate: true,
    },
    {
      condition: "人物行为可能不符合设定",
      priority: "high",
      autoDelegate: true,
    },
    {
      condition: "用户请求风格建议",
      priority: "medium",
      autoDelegate: false,
    },
  ],
  
  useWhen: [
    "需要检查对话是否符合人物性格",
    "需要分析人物塑造的深度",
    "需要获得文学风格建议",
    "需要优化人物声音（Voice）",
  ],
  
  avoidWhen: [
    "需要直接修改内容（只提供建议）",
    "需要检查世界观一致性",
    "需要规划剧情",
  ],
  
  promptAlias: "刘和平",
  
  fallbackChain: [
    {
      providers: ["anthropic"],
      model: "claude-opus-4",
      constraints: {
        maxTokens: 4096,
        temperature: 0.6,
        thinkingBudget: 8000,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
    {
      providers: ["dashscope"],
      model: "qwen-max",
      constraints: {
        maxTokens: 4096,
        temperature: 0.6,
        supportsTools: true,
        supportsStreaming: true,
      },
    },
  ],
  
  permissions: [
    "read:chapter_content",
    "read:characters",
    "read:worldview",
    "write:annotation",
    "write:character_voice",
  ],
  
  knowledgeBases: {
    worldview: { read: true, write: false },
    history: { read: true, write: false },
    current_chapter: { read: true, write: false },
    characters: { read: true, write: true },  // 可更新人物声音
    factions: { read: true, write: false },
    map: { read: true, write: false },
    foreshadowing: { read: true, write: false },
  },
  
  lifecycle: {
    stage1: "locked",
    stage2: "locked",
    stage3: "unlocked",
  },
  
  lockable: true,
};
```

---

## 4. Agent 注册中心

### 4.1 Registry 实现

```typescript
// packages/agents/src/core/agent-registry.ts

import type { AgentConfig, AgentPromptMetadata } from "./types";

const AGENT_FACTORIES = {
  "tian-dao": createTianDaoAgent,
  "writer": createWriterAgent,
  "world-guardian": createWorldGuardianAgent,
  "planner": createPlannerAgent,
  "reviewer": createReviewerAgent,
  "observer": createObserverAgent,
  "researcher": createResearcherAgent,
  "liuheping": createLiuHepingAgent,
} as const;

export class AgentRegistry {
  private agents: Map<string, AgentConfig> = new Map();
  private metadata: Map<string, AgentPromptMetadata> = new Map();
  
  register(name: string, factory: () => AgentConfig): void {
    const config = factory();
    this.agents.set(name, config);
    this.metadata.set(name, config.metadata);
  }
  
  get(name: string): AgentConfig | undefined {
    return this.agents.get(name);
  }
  
  getMetadata(name: string): AgentPromptMetadata | undefined {
    return this.metadata.get(name);
  }
  
  getAll(): AgentConfig[] {
    return Array.from(this.agents.values());
  }
  
  getByCategory(category: AgentCategory): AgentConfig[] {
    return this.getAll().filter(a => a.metadata.category === category);
  }
  
  isAvailable(name: string, stage: BookStage): boolean {
    const meta = this.metadata.get(name);
    if (!meta) return false;
    
    const stageKey = `stage${stage}` as keyof typeof meta.lifecycle;
    return meta.lifecycle[stageKey] === "unlocked";
  }
}

// 单例实例
export const agentRegistry = new AgentRegistry();

// 自动注册所有 Agent
Object.entries(AGENT_FACTORIES).forEach(([name, factory]) => {
  agentRegistry.register(name, factory);
});
```

### 4.2 获取可用 Agent

```typescript
export function getAvailableAgents(stage: BookStage): AgentConfig[] {
  return agentRegistry.getAll().filter(agent => 
    agentRegistry.isAvailable(agent.name, stage)
  );
}

export function getLockedAgents(stage: BookStage): string[] {
  return agentRegistry.getAll()
    .filter(agent => !agentRegistry.isAvailable(agent.name, stage))
    .map(agent => agent.displayName);
}
```

---

## 5. 动态 Prompt 构建

### 5.1 Prompt Builder

```typescript
// packages/agents/src/prompts/builder.ts

interface PromptBuildContext {
  agentName: string;
  role: string;
  systemPrompt: string;
  bookContext?: BookContext;
  stageContext?: StageContext;
}

export function buildOrchestratorPrompt(context: PromptBuildContext): string {
  const sections = [
    context.systemPrompt,
    buildDelegationTable(),
    buildToolSelectionTable(),
    buildKeyTriggersSection(),
    buildPhaseLockSection(context.stageContext),
  ];
  
  return sections.filter(Boolean).join("\n\n---\n\n");
}

export function buildWriterPrompt(context: PromptBuildContext): string {
  const sections = [
    context.systemPrompt,
    buildWritingGuidelines(context.bookContext),
    buildChapterContext(context.bookContext),
    buildStyleGuidelines(context.bookContext),
  ];
  
  return sections.filter(Boolean).join("\n\n---\n\n");
}

export function buildAdvisorPrompt(context: PromptBuildContext): string {
  const sections = [
    context.systemPrompt,
    buildReviewGuidelines(context.role),
    buildKnowledgeBaseContext(),
    buildOutputFormat(),
  ];
  
  return sections.filter(Boolean).join("\n\n---\n\n");
}
```

### 5.2 Delegation Table 构建

```typescript
function buildDelegationTable(): string {
  return `
## Delegation Table（委派表）

| 场景 | 委派给 | 说明 |
|------|--------|------|
| 需要撰写章节内容 | 执笔 | 唯一的写作 Agent |
| 需要检查一致性 | 世界观守护者 | 只读检查，提供修正建议 |
| 需要新书规划 | 规划者 | 仅阶段一可用 |
| 需要审查章节 | 审阅 | 文学性和可读性评估 |
| 需要监控进度 | 观察者 | 全程可用 |
| 需要外部参考 | 调研者 | 仅阶段二可用 |
| 需要风格分析 | 刘和平 | 人物塑造和对话检查 |
`;
}
```

### 5.3 Key Triggers Section

```typescript
function buildKeyTriggersSection(): string {
  const agents = agentRegistry.getAll();
  
  const triggers = agents
    .filter(a => a.metadata.keyTrigger)
    .map(a => `- **${a.metadata.keyTrigger}** → ${a.displayName}`)
    .join("\n");
  
  return `
## Key Triggers（关键触发器）

检查以下触发条件，决定是否需要委派：

${triggers}

**重要**: 如果触发条件满足但相关 Agent 被锁定，不要委派。改为：
1. 告知用户该 Agent 当前不可用
2. 提供替代方案或等待解锁
`;
}
```

---

## 6. 相关文档

- [01-overview.md](./01-overview.md) - Agent 系统概述
- [03-hooks-system.md](./03-hooks-system.md) - Hooks 系统
- [04-tools-system.md](./04-tools-system.md) - Tools 系统
- [05-skills-system.md](./05-skills-system.md) - Skills 系统
- [06-delegation-protocol.md](./06-delegation-protocol.md) - 委派协议