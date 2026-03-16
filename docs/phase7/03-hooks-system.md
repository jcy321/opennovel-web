# Phase 7: Agent 系统 - Hooks 系统

**文档版本**: 1.0
**最后更新**: 2026-03-16

---

## 1. 概述

Hooks 系统是 Oh My OpenCode 架构的核心之一，用于在 Agent 生命周期的关键节点注入自定义逻辑。

### 1.1 与 Oh My OpenCode 的对应

| Oh My OpenCode Hook | OpenNovel Hook | 触发时机 |
|---------------------|----------------|---------|
| `hook_user_message` | `before_user_message` | 用户消息处理前 |
| `hook_assistant_message` | `after_assistant_message` | Agent 回复后 |
| `hook_tool_call` | `before_tool_call` | Tool 调用前 |
| `hook_tool_result` | `after_tool_result` | Tool 返回后 |
| `hook_session_start` | `on_session_start` | 会话开始 |
| `hook_session_end` | `on_session_end` | 会话结束 |
| `hook_agent_switch` | `on_agent_switch` | Agent 切换 |
| `hook_error` | `on_error` | 错误发生 |
| - | `on_phase_change` | 阶段切换 |
| - | `on_chapter_complete` | 章节完成 |
| - | `on_consistency_violation` | 一致性违反 |

---

## 2. 核心类型定义

### 2.1 Hook 类型

```typescript
// packages/agents/src/hooks/types.ts

type HookPoint = 
  | "before_user_message"
  | "after_user_message"
  | "before_assistant_message"
  | "after_assistant_message"
  | "before_tool_call"
  | "after_tool_result"
  | "on_session_start"
  | "on_session_end"
  | "on_agent_switch"
  | "on_phase_change"
  | "on_chapter_complete"
  | "on_consistency_violation"
  | "on_error"
  | "on_idle";

interface HookContext {
  // 会话信息
  sessionId: string;
  bookId: string;
  stage: BookStage;
  
  // Agent 信息
  agentName: string;
  agentConfig: AgentConfig;
  
  // 消息上下文
  message?: Message;
  messages?: Message[];
  
  // Tool 上下文
  toolName?: string;
  toolArgs?: Record<string, unknown>;
  toolResult?: unknown;
  
  // 自定义数据
  data?: Record<string, unknown>;
}

interface HookResult {
  // 是否继续执行
  continue: boolean;
  
  // 修改后的上下文
  modifiedContext?: Partial<HookContext>;
  
  // 修改后的消息
  modifiedMessage?: Message;
  
  // 附加操作
  actions?: HookAction[];
  
  // 通知用户
  notification?: {
    type: "info" | "warning" | "error";
    message: string;
  };
}

interface HookAction {
  type: "delegate" | "notify" | "modify" | "block";
  target?: string;
  payload?: unknown;
}

type HookHandler = (context: HookContext) => Promise<HookResult> | HookResult;

interface HookDefinition {
  name: string;
  point: HookPoint;
  priority: number;           // 执行优先级，数字越小越先执行
  enabled: boolean;
  handler: HookHandler;
  
  // 条件过滤
  condition?: (context: HookContext) => boolean;
  
  // Agent 绑定
  agents?: string[];          // 仅对特定 Agent 生效
  
  // 阶段过滤
  stages?: BookStage[];       // 仅在特定阶段生效
}
```

### 2.2 Hook 组合器

```typescript
// packages/agents/src/hooks/create-hooks.ts

import type { HookDefinition, HookHandler, HookContext, HookResult } from "./types";

/**
 * 创建 Hook 组合器
 * 类似 Oh My OpenCode 的 createHooks 函数
 */
export function createHooks(definitions: HookDefinition[]): HookDefinition[] {
  return definitions.map(def => ({
    ...def,
    priority: def.priority ?? 100,
    enabled: def.enabled ?? true,
  }));
}

/**
 * 组合多个 Hook Handler
 */
export function composeHooks(...handlers: HookHandler[]): HookHandler {
  return async (context: HookContext): Promise<HookResult> => {
    let currentContext = context;
    
    for (const handler of handlers) {
      const result = await handler(currentContext);
      
      if (!result.continue) {
        return result;
      }
      
      if (result.modifiedContext) {
        currentContext = { ...currentContext, ...result.modifiedContext };
      }
    }
    
    return { continue: true };
  };
}

/**
 * 条件 Hook
 */
export function conditionalHook(
  condition: (context: HookContext) => boolean,
  handler: HookHandler
): HookHandler {
  return async (context: HookContext) => {
    if (condition(context)) {
      return handler(context);
    }
    return { continue: true };
  };
}

/**
 * 创建 Hook 工厂
 */
export function createHookFactory<T extends Record<string, unknown>>(
  factory: (config: T) => HookDefinition
): (config: T) => HookDefinition {
  return factory;
}
```

---

## 3. 小说创作专用 Hooks

### 3.1 阶段锁定 Hook（Phase Lock Check）

检查 Agent 是否在当前阶段可用。

```typescript
// packages/agents/src/hooks/novel-hooks/phase-lock.ts

import type { HookDefinition, HookContext, HookResult } from "../types";
import { agentRegistry } from "../../core/agent-registry";

export const phaseLockHook: HookDefinition = {
  name: "phase_lock_check",
  point: "before_user_message",
  priority: 10,  // 高优先级
  enabled: true,
  
  handler: async (context: HookContext): Promise<HookResult> => {
    const { agentName, stage } = context;
    
    // 检查 Agent 是否可用
    const isAvailable = agentRegistry.isAvailable(agentName, stage);
    
    if (!isAvailable) {
      const meta = agentRegistry.getMetadata(agentName);
      
      return {
        continue: false,
        notification: {
          type: "info",
          message: `**${meta?.displayName || agentName}** 在当前阶段不可用。\n\n该 Agent 仅在以下阶段可用：${getAvailableStages(meta?.lifecycle)}`,
        },
        actions: [
          {
            type: "notify",
            payload: {
              alternativeAgents: getAlternativeAgents(agentName, stage),
            },
          },
        ],
      };
    }
    
    return { continue: true };
  },
};

function getAvailableStages(lifecycle?: AgentLifecycle): string {
  if (!lifecycle) return "未知";
  
  const stages: string[] = [];
  if (lifecycle.stage1 === "unlocked") stages.push("阶段一（构思）");
  if (lifecycle.stage2 === "unlocked") stages.push("阶段二（知识库建立）");
  if (lifecycle.stage3 === "unlocked") stages.push("阶段三（撰写）");
  
  return stages.join("、") || "无";
}

function getAlternativeAgents(agentName: string, stage: BookStage): string[] {
  // 返回当前阶段可用的替代 Agent
  const alternatives: Record<string, string[]> = {
    "planner": ["tian-dao"],
    "researcher": ["tian-dao", "observer"],
    "writer": [],
    "reviewer": ["tian-dao"],
  };
  
  return alternatives[agentName] || [];
}
```

### 3.2 一致性检查 Hook（Consistency Check）

在章节完成后自动检查一致性。

```typescript
// packages/agents/src/hooks/novel-hooks/consistency-check.ts

import type { HookDefinition, HookContext, HookResult } from "../types";

export const consistencyCheckHook: HookDefinition = {
  name: "consistency_check",
  point: "on_chapter_complete",
  priority: 50,
  enabled: true,
  
  handler: async (context: HookContext): Promise<HookResult> => {
    const { bookId, sessionId } = context;
    
    // 触发世界观守护者进行一致性检查
    return {
      continue: true,
      actions: [
        {
          type: "delegate",
          target: "world-guardian",
          payload: {
            task: "check_chapter_consistency",
            chapterId: context.data?.chapterId,
          },
        },
      ],
    };
  },
};

/**
 * 主动一致性检查 Hook
 * 在 Agent 回复后检查是否有一致性问题
 */
export const proactiveConsistencyHook: HookDefinition = {
  name: "proactive_consistency_check",
  point: "after_assistant_message",
  priority: 80,
  enabled: true,
  
  // 仅对执笔 Agent 生效
  agents: ["writer"],
  
  // 仅在阶段三生效
  stages: [3],
  
  handler: async (context: HookContext): Promise<HookResult> => {
    const { message, data } = context;
    
    // 检查消息中是否提到了可能影响一致性的内容
    const content = message?.content || "";
    const consistencyKeywords = [
      "忘记", "记错", "之前说过", "设定冲突",
      "时间线", "人物行为", "世界观规则",
    ];
    
    const hasPotentialIssue = consistencyKeywords.some(
      keyword => content.includes(keyword)
    );
    
    if (hasPotentialIssue) {
      return {
        continue: true,
        notification: {
          type: "warning",
          message: "检测到可能的一致性问题，建议进行详细检查。",
        },
        actions: [
          {
            type: "delegate",
            target: "world-guardian",
            payload: {
              task: "check_recent_content",
              sessionId,
            },
          },
        ],
      };
    }
    
    return { continue: true };
  },
};
```

### 3.3 写作惯性提醒 Hook（Writing Streak Warning）

检测连续写作时间，提醒用户休息。

```typescript
// packages/agents/src/hooks/novel-hooks/writing-streak.ts

import type { HookDefinition, HookContext, HookResult } from "../types";

const WRITING_STREAK_THRESHOLD = 60 * 60 * 1000; // 1小时
const WARNING_INTERVAL = 15 * 60 * 1000; // 15分钟

export const writingStreakHook: HookDefinition = {
  name: "writing_streak_warning",
  point: "after_assistant_message",
  priority: 100,
  enabled: true,
  
  agents: ["writer", "tian-dao"],
  stages: [3],
  
  handler: async (context: HookContext): Promise<HookResult> => {
    const { sessionId, data } = context;
    const writingStartTime = data?.writingStartTime as number | undefined;
    const lastWarningTime = data?.lastWarningTime as number | undefined;
    
    if (!writingStartTime) {
      return { continue: true };
    }
    
    const now = Date.now();
    const elapsed = now - writingStartTime;
    
    // 超过阈值且距上次警告超过间隔
    if (
      elapsed > WRITING_STREAK_THRESHOLD &&
      (!lastWarningTime || now - lastWarningTime > WARNING_INTERVAL)
    ) {
      const hours = Math.floor(elapsed / (60 * 60 * 1000));
      const minutes = Math.floor((elapsed % (60 * 60 * 1000)) / (60 * 1000));
      
      return {
        continue: true,
        notification: {
          type: "info",
          message: `📝 您已连续创作 **${hours}小时${minutes}分钟**，建议适当休息，保持创作状态。`,
        },
        modifiedContext: {
          data: {
            ...data,
            lastWarningTime: now,
          },
        },
      };
    }
    
    return { continue: true };
  },
};
```

### 3.4 主动介入 Hook（Proactive Intervention）

允许 Agent 在满足条件时主动加入对话。

```typescript
// packages/agents/src/hooks/novel-hooks/proactive-intervention.ts

import type { HookDefinition, HookContext, HookResult } from "../types";

interface InterventionCondition {
  agentName: string;
  condition: (context: HookContext) => boolean;
  message: string;
  priority: number;
}

/**
 * 主动介入条件注册表
 */
const INTERVENTION_CONDITIONS: InterventionCondition[] = [
  {
    agentName: "world-guardian",
    condition: (ctx) => {
      // 检测到世界观关键词
      const keywords = ["魔法", "能力", "规则", "设定"];
      return keywords.some(kw => ctx.message?.content?.includes(kw));
    },
    message: "检测到世界观相关讨论，我可以协助检查一致性。",
    priority: 50,
  },
  {
    agentName: "liuheping",
    condition: (ctx) => {
      // 检测到人物对话讨论
      const keywords = ["对话", "说话", "口吻", "性格"];
      return keywords.some(kw => ctx.message?.content?.includes(kw));
    },
    message: "检测到人物相关讨论，我可以协助分析人物声音。",
    priority: 60,
  },
  {
    agentName: "reviewer",
    condition: (ctx) => {
      // 章节完成
      return ctx.data?.chapterCompleted === true;
    },
    message: "章节已完成，我可以进行审阅。",
    priority: 40,
  },
];

export const proactiveInterventionHook: HookDefinition = {
  name: "proactive_intervention",
  point: "after_user_message",
  priority: 90,
  enabled: true,
  
  handler: async (context: HookContext): Promise<HookResult> => {
    const { stage, agentName } = context;
    
    // 仅在阶段三启用主动介入
    if (stage !== 3) {
      return { continue: true };
    }
    
    // 检查每个介入条件
    const triggeredInterventions: InterventionCondition[] = [];
    
    for (const condition of INTERVENTION_CONDITIONS) {
      // 跳过当前 Agent
      if (condition.agentName === agentName) continue;
      
      // 检查 Agent 是否可用
      if (!agentRegistry.isAvailable(condition.agentName, stage)) continue;
      
      // 检查条件是否满足
      if (condition.condition(context)) {
        triggeredInterventions.push(condition);
      }
    }
    
    if (triggeredInterventions.length === 0) {
      return { continue: true };
    }
    
    // 按优先级排序，取最高优先级的介入
    triggeredInterventions.sort((a, b) => a.priority - b.priority);
    const topIntervention = triggeredInterventions[0];
    
    return {
      continue: true,
      notification: {
        type: "info",
        message: `💡 **${getDisplayName(topIntervention.agentName)}**: ${topIntervention.message}`,
      },
      actions: [
        {
          type: "notify",
          payload: {
            intervention: {
              agent: topIntervention.agentName,
              available: true,
            },
          },
        },
      ],
    };
  },
};

function getDisplayName(agentName: string): string {
  const names: Record<string, string> = {
    "world-guardian": "世界观守护者",
    "liuheping": "刘和平",
    "reviewer": "审阅",
  };
  return names[agentName] || agentName;
}
```

### 3.5 章节完成检查 Hook（Chapter Completion Check）

检测章节是否完成，触发后续流程。

```typescript
// packages/agents/src/hooks/novel-hooks/chapter-completion.ts

import type { HookDefinition, HookContext, HookResult } from "../types";

export const chapterCompletionHook: HookDefinition = {
  name: "chapter_completion_check",
  point: "after_assistant_message",
  priority: 30,
  enabled: true,
  
  agents: ["writer"],
  stages: [3],
  
  handler: async (context: HookContext): Promise<HookResult> => {
    const { message, data } = context;
    const content = message?.content || "";
    
    // 检测章节完成标记
    const completionMarkers = [
      "【本章完】",
      "（本章完）",
      "--- 本章完 ---",
      "本章结束",
    ];
    
    const isChapterComplete = completionMarkers.some(
      marker => content.includes(marker)
    );
    
    if (!isChapterComplete) {
      return { continue: true };
    }
    
    // 章节完成，触发后续操作
    return {
      continue: true,
      notification: {
        type: "info",
        message: "🎉 章节已完成！正在启动后续流程...",
      },
      actions: [
        // 1. 触发一致性检查
        {
          type: "delegate",
          target: "world-guardian",
          payload: {
            task: "check_chapter_consistency",
            chapterId: data?.chapterId,
          },
        },
        // 2. 触发审阅
        {
          type: "delegate",
          target: "reviewer",
          payload: {
            task: "review_chapter",
            chapterId: data?.chapterId,
          },
        },
        // 3. 更新历史知识库
        {
          type: "notify",
          target: "observer",
          payload: {
            action: "update_history_kb",
            chapterId: data?.chapterId,
          },
        },
      ],
      modifiedContext: {
        data: {
          ...data,
          chapterCompleted: true,
        },
      },
    };
  },
};
```

### 3.6 伏笔状态检查 Hook（Foreshadowing State Check）

检查伏笔是否需要触发或更新。

```typescript
// packages/agents/src/hooks/novel-hooks/foreshadowing-check.ts

import type { HookDefinition, HookContext, HookResult } from "../types";

export const foreshadowingCheckHook: HookDefinition = {
  name: "foreshadowing_state_check",
  point: "after_assistant_message",
  priority: 70,
  enabled: true,
  
  agents: ["writer", "tian-dao"],
  stages: [3],
  
  handler: async (context: HookContext): Promise<HookResult> => {
    const { message, bookId, data } = context;
    const content = message?.content || "";
    
    // 获取当前活跃的伏笔
    const activeForeshadowings = data?.activeForeshadowings as string[] | undefined;
    
    if (!activeForeshadowings || activeForeshadowings.length === 0) {
      return { continue: true };
    }
    
    // 检查内容中是否触及任何伏笔
    const triggeredForeshadowings: string[] = [];
    
    for (const foreshadowingId of activeForeshadowings) {
      const foreshadowing = await getForeshadowing(bookId, foreshadowingId);
      
      if (foreshadowing && checkForeshadowingTrigger(content, foreshadowing)) {
        triggeredForeshadowings.push(foreshadowingId);
      }
    }
    
    if (triggeredForeshadowings.length === 0) {
      return { continue: true };
    }
    
    // 有伏笔被触发
    return {
      continue: true,
      notification: {
        type: "info",
        message: `📖 检测到 ${triggeredForeshadowings.length} 个伏笔可能被触发。\n\n伏笔列表：\n${triggeredForeshadowings.map(id => `- ${id}`).join("\n")}`,
      },
      actions: [
        {
          type: "delegate",
          target: "tian-dao",
          payload: {
            task: "update_foreshadowing_state",
            foreshadowingIds: triggeredForeshadowings,
            newState: "triggered",
          },
        },
      ],
    };
  },
};

async function getForeshadowing(bookId: string, id: string) {
  // 从知识库获取伏笔详情
  // 实际实现会调用 knowledge base
  return null;
}

function checkForeshadowingTrigger(content: string, foreshadowing: any): boolean {
  // 检查内容是否触发伏笔
  // 实际实现会进行语义匹配
  return false;
}
```

---

## 4. Hook 注册与执行

### 4.1 Hook Registry

```typescript
// packages/agents/src/hooks/hook-registry.ts

import type { HookDefinition, HookContext, HookResult, HookPoint } from "./types";

export class HookRegistry {
  private hooks: Map<HookPoint, HookDefinition[]> = new Map();
  
  register(hook: HookDefinition): void {
    const point = hook.point;
    
    if (!this.hooks.has(point)) {
      this.hooks.set(point, []);
    }
    
    const hooksAtPoint = this.hooks.get(point)!;
    hooksAtPoint.push(hook);
    
    // 按优先级排序
    hooksAtPoint.sort((a, b) => a.priority - b.priority);
  }
  
  unregister(name: string): boolean {
    for (const [point, hooks] of this.hooks.entries()) {
      const index = hooks.findIndex(h => h.name === name);
      if (index !== -1) {
        hooks.splice(index, 1);
        return true;
      }
    }
    return false;
  }
  
  getHooksAtPoint(point: HookPoint): HookDefinition[] {
    return this.hooks.get(point) || [];
  }
  
  async execute(point: HookPoint, context: HookContext): Promise<HookResult> {
    const hooks = this.getHooksAtPoint(point);
    
    let currentContext = context;
    
    for (const hook of hooks) {
      // 检查是否启用
      if (!hook.enabled) continue;
      
      // 检查 Agent 过滤
      if (hook.agents && !hook.agents.includes(context.agentName)) continue;
      
      // 检查阶段过滤
      if (hook.stages && !hook.stages.includes(context.stage)) continue;
      
      // 检查自定义条件
      if (hook.condition && !hook.condition(currentContext)) continue;
      
      // 执行 Hook
      const result = await hook.handler(currentContext);
      
      // 更新上下文
      if (result.modifiedContext) {
        currentContext = { ...currentContext, ...result.modifiedContext };
      }
      
      // 如果不继续，中断执行
      if (!result.continue) {
        return result;
      }
    }
    
    return { continue: true };
  }
}

// 单例实例
export const hookRegistry = new HookRegistry();

// 注册所有内置 Hooks
export function registerBuiltinHooks(): void {
  const builtinHooks: HookDefinition[] = [
    phaseLockHook,
    consistencyCheckHook,
    proactiveConsistencyHook,
    writingStreakHook,
    proactiveInterventionHook,
    chapterCompletionHook,
    foreshadowingCheckHook,
  ];
  
  for (const hook of builtinHooks) {
    hookRegistry.register(hook);
  }
}
```

### 4.2 Hook 执行流程

```typescript
// packages/agents/src/core/agent-executor.ts

import { hookRegistry } from "../hooks/hook-registry";

export class AgentExecutor {
  async executeAgent(agentName: string, context: ExecutionContext): Promise<AgentResult> {
    const agent = agentRegistry.get(agentName);
    if (!agent) {
      throw new Error(`Agent not found: ${agentName}`);
    }
    
    // 1. 创建 Hook 上下文
    const hookContext: HookContext = {
      sessionId: context.sessionId,
      bookId: context.bookId,
      stage: context.stage,
      agentName,
      agentConfig: agent,
      messages: context.messages,
      data: context.data,
    };
    
    // 2. 执行 before_user_message hooks
    const beforeResult = await hookRegistry.execute("before_user_message", hookContext);
    if (!beforeResult.continue) {
      return {
        blocked: true,
        notification: beforeResult.notification,
      };
    }
    
    // 3. 调用 LLM
    const response = await this.callLLM(agent, hookContext);
    
    // 4. 执行 after_assistant_message hooks
    hookContext.message = response.message;
    await hookRegistry.execute("after_assistant_message", hookContext);
    
    // 5. 处理 Tool 调用
    if (response.toolCalls) {
      for (const toolCall of response.toolCalls) {
        // before_tool_call
        hookContext.toolName = toolCall.name;
        hookContext.toolArgs = toolCall.args;
        await hookRegistry.execute("before_tool_call", hookContext);
        
        // 执行 Tool
        const toolResult = await this.executeTool(toolCall);
        
        // after_tool_result
        hookContext.toolResult = toolResult;
        await hookRegistry.execute("after_tool_result", hookContext);
      }
    }
    
    return {
      message: response.message,
      toolCalls: response.toolCalls,
    };
  }
}
```

---

## 5. Hook 配置

### 5.1 禁用特定 Hook

```typescript
// 用户可以通过配置禁用特定 Hook
const hookConfig = {
  disabledHooks: ["writing_streak_warning"],
  
  // 或者针对特定 Agent 禁用
  agentHookOverrides: {
    "writer": {
      disabledHooks: ["proactive_intervention"],
    },
  },
};
```

### 5.2 自定义 Hook 注册

```typescript
// 用户可以注册自定义 Hook
const customHook: HookDefinition = {
  name: "custom_word_count_check",
  point: "after_assistant_message",
  priority: 100,
  enabled: true,
  agents: ["writer"],
  
  handler: async (context: HookContext): Promise<HookResult> => {
    const content = context.message?.content || "";
    const wordCount = countWords(content);
    
    if (wordCount < 500) {
      return {
        continue: true,
        notification: {
          type: "warning",
          message: `章节字数较少（${wordCount}字），建议扩充内容。`,
        },
      };
    }
    
    return { continue: true };
  },
};

hookRegistry.register(customHook);
```

---

## 6. Hooks 与 Oh My OpenCode 的差异

| 方面 | Oh My OpenCode | OpenNovel |
|------|----------------|-----------|
| Hook 数量 | 46 个 | 约 12 个核心 Hook |
| 触发点 | 通用开发流程 | 小说创作流程 |
| 条件过滤 | 有 | 有（Agent + Stage） |
| 主动介入 | 无 | 有（proactive_intervention） |
| 阶段锁定 | 无 | 有（phase_lock_check） |
| 一致性检查 | 无 | 有（consistency_check） |

---

## 7. 相关文档

- [01-overview.md](./01-overview.md) - Agent 系统概述
- [02-agent-definitions.md](./02-agent-definitions.md) - Agent 定义
- [04-tools-system.md](./04-tools-system.md) - Tools 系统
- [05-skills-system.md](./05-skills-system.md) - Skills 系统
- [06-delegation-protocol.md](./06-delegation-protocol.md) - 委派协议