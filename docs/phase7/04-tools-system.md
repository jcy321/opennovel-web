# Phase 7: Agent 系统 - Tools 系统

**文档版本**: 1.0
**最后更新**: 2026-03-16

---

## 1. 概述

Tools 系统定义了 Agent 可以调用的具体操作。每个 Tool 是一个独立的功能单元，可以被一个或多个 Agent 使用。

### 1.1 与 Oh My OpenCode 的对应

| Oh My OpenCode Tool | OpenNovel Tool | 功能 |
|---------------------|----------------|------|
| `bash` | - | 终端命令（不适用） |
| `read` | `read_knowledge_base` | 读取知识库 |
| `write` | `write_chapter` | 写入章节 |
| `edit` | `apply_annotation` | 应用批注 |
| `glob` | `search_knowledge` | 搜索知识库 |
| `grep` | `search_content` | 内容搜索 |
| `task` | `delegate_to_agent` | 委派任务 |
| `webfetch` | `search_web` | Web 搜索 |
| - | `update_worldview` | 更新世界观 |
| - | `manage_foreshadowing` | 伏笔管理 |
| - | `add_annotation` | 添加批注 |
| - | `create_outline` | 创建大纲 |

---

## 2. 核心类型定义

### 2.1 Tool 类型

```typescript
// packages/agents/src/tools/types.ts

import { z } from "zod";

interface ToolDefinition<TInput = unknown, TOutput = unknown> {
  // 基本信息
  name: string;
  displayName: string;
  description: string;
  
  // Schema
  inputSchema: z.ZodType<TInput>;
  outputSchema?: z.ZodType<TOutput>;
  
  // 执行器
  execute: (input: TInput, context: ToolContext) => Promise<TOutput>;
  
  // 权限要求
  requiredPermissions: string[];
  
  // 超时
  timeout?: number;
  
  // 重试
  retryConfig?: {
    maxRetries: number;
    delayMs: number;
  };
  
  // 是否流式
  streaming?: boolean;
}

interface ToolContext {
  bookId: string;
  sessionId: string;
  agentName: string;
  userId: string;
  
  // 知识库访问
  knowledgeBase: KnowledgeBaseClient;
  
  // 日志
  logger: Logger;
}

interface ToolResult<T = unknown> {
  success: boolean;
  data?: T;
  error?: {
    code: string;
    message: string;
    details?: unknown;
  };
}
```

### 2.2 Tool Registry

```typescript
// packages/agents/src/tools/tool-registry.ts

export class ToolRegistry {
  private tools: Map<string, ToolDefinition> = new Map();
  private permissions: PermissionChecker;
  
  constructor(permissions: PermissionChecker) {
    this.permissions = permissions;
  }
  
  register(tool: ToolDefinition): void {
    this.tools.set(tool.name, tool);
  }
  
  get(name: string): ToolDefinition | undefined {
    return this.tools.get(name);
  }
  
  async execute(
    name: string,
    input: unknown,
    context: ToolContext
  ): Promise<ToolResult> {
    const tool = this.tools.get(name);
    if (!tool) {
      return {
        success: false,
        error: {
          code: "TOOL_NOT_FOUND",
          message: `Tool not found: ${name}`,
        },
      };
    }
    
    // 权限检查
    const hasPermission = await this.permissions.check(
      context.agentName,
      tool.requiredPermissions
    );
    
    if (!hasPermission) {
      return {
        success: false,
        error: {
          code: "PERMISSION_DENIED",
          message: `Agent ${context.agentName} does not have permission to use ${name}`,
        },
      };
    }
    
    // 输入验证
    const validationResult = tool.inputSchema.safeParse(input);
    if (!validationResult.success) {
      return {
        success: false,
        error: {
          code: "INVALID_INPUT",
          message: "Invalid input parameters",
          details: validationResult.error.errors,
        },
      };
    }
    
    // 执行
    try {
      const result = await tool.execute(validationResult.data, context);
      return { success: true, data: result };
    } catch (error) {
      return {
        success: false,
        error: {
          code: "EXECUTION_ERROR",
          message: error instanceof Error ? error.message : "Unknown error",
        },
      };
    }
  }
  
  getToolsForAgent(agentName: string): ToolDefinition[] {
    return Array.from(this.tools.values()).filter(tool => 
      this.permissions.canUse(agentName, tool.requiredPermissions)
    );
  }
  
  getToolSchemasForAgent(agentName: string): ToolSchema[] {
    const tools = this.getToolsForAgent(agentName);
    
    return tools.map(tool => ({
      name: tool.name,
      description: tool.description,
      inputSchema: zodToJsonSchema(tool.inputSchema),
    }));
  }
}

// 单例实例
export const toolRegistry = new ToolRegistry(permissionChecker);
```

---

## 3. 小说创作专用 Tools

### 3.1 知识库检索 Tool（Knowledge Search）

```typescript
// packages/agents/src/tools/novel-tools/knowledge-search.ts

import { z } from "zod";
import type { ToolDefinition, ToolContext } from "../types";

const KnowledgeSearchInput = z.object({
  query: z.string().describe("搜索查询"),
  knowledgeBase: z.enum([
    "worldview",
    "history",
    "current_chapter",
    "characters",
    "factions",
    "map",
    "foreshadowing",
  ]).describe("要搜索的知识库"),
  topK: z.number().min(1).max(20).default(5).describe("返回结果数量"),
  filters: z.record(z.unknown()).optional().describe("过滤条件"),
});

const KnowledgeSearchOutput = z.object({
  results: z.array(z.object({
    id: z.string(),
    content: z.string(),
    score: z.number(),
    metadata: z.record(z.unknown()).optional(),
  })),
  total: z.number(),
});

export const knowledgeSearchTool: ToolDefinition<
  z.infer<typeof KnowledgeSearchInput>,
  z.infer<typeof KnowledgeSearchOutput>
> = {
  name: "knowledge_search",
  displayName: "知识库检索",
  description: "在指定知识库中搜索相关内容",
  
  inputSchema: KnowledgeSearchInput,
  outputSchema: KnowledgeSearchOutput,
  
  requiredPermissions: ["read:knowledge_base"],
  
  execute: async (input, context: ToolContext) => {
    const { query, knowledgeBase, topK, filters } = input;
    
    // 构建搜索请求
    const searchResult = await context.knowledgeBase.search({
      collection: knowledgeBase,
      query,
      topK,
      filters: {
        bookId: context.bookId,
        ...filters,
      },
    });
    
    return {
      results: searchResult.hits.map(hit => ({
        id: hit.id,
        content: hit.content,
        score: hit.score,
        metadata: hit.metadata,
      })),
      total: searchResult.total,
    };
  },
};
```

### 3.2 章节写入 Tool（Write Chapter）

```typescript
// packages/agents/src/tools/novel-tools/write-chapter.ts

import { z } from "zod";
import type { ToolDefinition, ToolContext } from "../types";

const WriteChapterInput = z.object({
  chapterId: z.string().describe("章节 ID"),
  content: z.string().describe("章节内容"),
  mode: z.enum(["append", "overwrite", "insert"]).default("append")
    .describe("写入模式：追加、覆盖、插入"),
  insertPosition: z.number().optional().describe("插入位置（仅 insert 模式）"),
  metadata: z.object({
    wordCount: z.number().optional(),
    sceneCount: z.number().optional(),
    pov: z.string().optional(),
  }).optional().describe("章节元数据"),
});

const WriteChapterOutput = z.object({
  chapterId: z.string(),
  wordCount: z.number(),
  timestamp: z.string(),
  version: z.number(),
});

export const writeChapterTool: ToolDefinition<
  z.infer<typeof WriteChapterInput>,
  z.infer<typeof WriteChapterOutput>
> = {
  name: "write_chapter",
  displayName: "章节写入",
  description: "写入或更新章节内容（仅执笔 Agent 可用）",
  
  inputSchema: WriteChapterInput,
  outputSchema: WriteChapterOutput,
  
  requiredPermissions: ["write:chapter_content"],
  
  // 仅执笔 Agent 可用
  agentRestriction: ["writer"],
  
  execute: async (input, context: ToolContext) => {
    const { chapterId, content, mode, insertPosition, metadata } = input;
    
    // 验证章节属于当前书籍
    const chapter = await context.knowledgeBase.getChapter(chapterId);
    if (!chapter || chapter.bookId !== context.bookId) {
      throw new Error(`Chapter ${chapterId} not found or does not belong to this book`);
    }
    
    // 执行写入
    const result = await context.knowledgeBase.writeChapter({
      chapterId,
      content,
      mode,
      insertPosition,
      metadata: {
        ...metadata,
        wordCount: metadata?.wordCount ?? countWords(content),
      },
      author: context.agentName,
      timestamp: new Date().toISOString(),
    });
    
    // 更新历史知识库（触发器）
    await context.knowledgeBase.updateHistory({
      bookId: context.bookId,
      chapterId,
      action: "content_update",
      content,
    });
    
    return {
      chapterId,
      wordCount: result.wordCount,
      timestamp: result.timestamp,
      version: result.version,
    };
  },
};
```

### 3.3 批注添加 Tool（Add Annotation）

```typescript
// packages/agents/src/tools/novel-tools/add-annotation.ts

import { z } from "zod";
import type { ToolDefinition, ToolContext } from "../types";

const AddAnnotationInput = z.object({
  chapterId: z.string().describe("章节 ID"),
  position: z.object({
    start: z.number().describe("起始位置"),
    end: z.number().describe("结束位置"),
  }).describe("批注位置"),
  
  content: z.string().describe("批注内容"),
  type: z.enum([
    "suggestion",      // 建议
    "warning",         // 警告
    "question",        // 问题
    "consistency",     // 一致性问题
    "style",           // 风格问题
  ]).describe("批注类型"),
  
  severity: z.enum(["low", "medium", "high"]).default("medium")
    .describe("严重程度"),
  
  relatedTo: z.object({
    type: z.enum(["character", "location", "event", "foreshadowing"]).optional(),
    id: z.string().optional(),
  }).optional().describe("关联元素"),
  
  autoApply: z.boolean().default(false)
    .describe("是否自动应用（仅审阅和刘和平可用）"),
});

const AddAnnotationOutput = z.object({
  annotationId: z.string(),
  status: z.enum(["pending", "applied", "rejected"]),
  createdAt: z.string(),
});

export const addAnnotationTool: ToolDefinition<
  z.infer<typeof AddAnnotationInput>,
  z.infer<typeof AddAnnotationOutput>
> = {
  name: "add_annotation",
  displayName: "添加批注",
  description: "在章节中添加批注（建议、警告、问题等）",
  
  inputSchema: AddAnnotationInput,
  outputSchema: AddAnnotationOutput,
  
  requiredPermissions: ["write:annotation"],
  
  execute: async (input, context: ToolContext) => {
    const {
      chapterId, position, content, type, severity,
      relatedTo, autoApply,
    } = input;
    
    // 创建批注
    const annotation = await context.knowledgeBase.addAnnotation({
      bookId: context.bookId,
      chapterId,
      position,
      content,
      type,
      severity,
      author: context.agentName,
      relatedTo,
      status: autoApply ? "applied" : "pending",
    });
    
    // 如果是自动应用，执行修改
    if (autoApply && hasAutoApplyPermission(context.agentName)) {
      await applyAnnotation(context, annotation);
    }
    
    return {
      annotationId: annotation.id,
      status: annotation.status,
      createdAt: annotation.createdAt,
    };
  },
};

function hasAutoApplyPermission(agentName: string): boolean {
  return ["reviewer", "liuheping", "world-guardian"].includes(agentName);
}

async function applyAnnotation(context: ToolContext, annotation: Annotation) {
  // 应用批注到章节内容
  // 实际实现会更复杂，需要处理冲突等
}
```

### 3.4 世界观更新 Tool（Update Worldview）

```typescript
// packages/agents/src/tools/novel-tools/update-worldview.ts

import { z } from "zod";
import type { ToolDefinition, ToolContext } from "../types";

const UpdateWorldviewInput = z.object({
  operation: z.enum(["create", "update", "delete"])
    .describe("操作类型"),
  
  entityType: z.enum([
    "rule",           // 规则
    "concept",        // 概念
    "location",       // 地点
    "item",           // 物品
    "ability",        // 能力
    "history",        // 历史
    "culture",        // 文化
  ]).describe("实体类型"),
  
  entity: z.object({
    id: z.string().optional().describe("实体 ID（update/delete 必填）"),
    name: z.string().describe("名称"),
    description: z.string().describe("描述"),
    rules: z.array(z.object({
      condition: z.string(),
      effect: z.string(),
    })).optional().describe("相关规则"),
    relationships: z.array(z.object({
      targetId: z.string(),
      type: z.string(),
    })).optional().describe("关联关系"),
  }).describe("实体数据"),
  
  reason: z.string().describe("修改原因（用于记录）"),
});

const UpdateWorldviewOutput = z.object({
  entityId: z.string(),
  operation: z.string(),
  timestamp: z.string(),
  version: z.number(),
});

export const updateWorldviewTool: ToolDefinition<
  z.infer<typeof UpdateWorldviewInput>,
  z.infer<typeof UpdateWorldviewOutput>
> = {
  name: "update_worldview",
  displayName: "更新世界观",
  description: "创建、更新或删除世界观实体",
  
  inputSchema: UpdateWorldviewInput,
  outputSchema: UpdateWorldviewOutput,
  
  requiredPermissions: ["write:worldview"],
  
  execute: async (input, context: ToolContext) => {
    const { operation, entityType, entity, reason } = input;
    
    // 验证权限
    if (operation === "delete") {
      // 删除操作需要更高权限
      const canDelete = await context.knowledgeBase.checkWorldviewUsage(
        context.bookId,
        entity.id!
      );
      
      if (canDelete.usedInChapters > 0) {
        throw new Error(
          `Cannot delete entity: used in ${canDelete.usedInChapters} chapters`
        );
      }
    }
    
    // 执行操作
    const result = await context.knowledgeBase.updateWorldview({
      bookId: context.bookId,
      operation,
      entityType,
      entity,
      author: context.agentName,
      reason,
    });
    
    // 记录变更历史
    await context.knowledgeBase.addWorldviewHistory({
      bookId: context.bookId,
      entityId: result.entityId,
      operation,
      author: context.agentName,
      reason,
      timestamp: new Date().toISOString(),
    });
    
    return {
      entityId: result.entityId,
      operation,
      timestamp: result.timestamp,
      version: result.version,
    };
  },
};
```

### 3.5 伏笔管理 Tool（Manage Foreshadowing）

```typescript
// packages/agents/src/tools/novel-tools/manage-foreshadowing.ts

import { z } from "zod";
import type { ToolDefinition, ToolContext } from "../types";

const ManageForeshadowingInput = z.object({
  operation: z.enum([
    "create",       // 创建伏笔
    "bury",         // 埋设伏笔
    "hint",         // 暗示伏笔
    "trigger",      // 触发伏笔
    "abandon",      // 放弃伏笔
    "update",       // 更新伏笔
  ]).describe("操作类型"),
  
  foreshadowing: z.object({
    id: z.string().optional().describe("伏笔 ID"),
    name: z.string().optional().describe("伏笔名称"),
    description: z.string().optional().describe("伏笔描述"),
    type: z.enum([
      "plot",         // 剧情伏笔
      "character",    // 人物伏笔
      "item",         // 物品伏笔
      "setting",      // 设定伏笔
    ]).optional().describe("伏笔类型"),
    
    // 埋设信息
    buriedIn: z.object({
      chapterId: z.string(),
      position: z.string().optional(),
    }).optional().describe("埋设位置"),
    
    // 暗示信息
    hintIn: z.array(z.object({
      chapterId: z.string(),
      hint: z.string(),
    })).optional().describe("暗示位置"),
    
    // 触发信息
    triggeredIn: z.object({
      chapterId: z.string(),
      resolution: z.string(),
    }).optional().describe("触发位置"),
    
    // 压力表
    pressure: z.number().min(0).max(100).optional()
      .describe("压力值（0-100，超过阈值需触发）"),
    
    // 预计触发
    expectedTrigger: z.object({
      chapterRange: z.tuple([z.number(), z.number()]).optional(),
      conditions: z.array(z.string()).optional(),
    }).optional().describe("预计触发条件"),
  }).describe("伏笔数据"),
  
  reason: z.string().optional().describe("操作原因"),
});

const ManageForeshadowingOutput = z.object({
  foreshadowingId: z.string(),
  status: z.enum(["planned", "buried", "hinted", "triggered", "abandoned"]),
  pressure: z.number(),
  warnings: z.array(z.string()).optional(),
});

export const manageForeshadowingTool: ToolDefinition<
  z.infer<typeof ManageForeshadowingInput>,
  z.infer<typeof ManageForeshadowingOutput>
> = {
  name: "manage_foreshadowing",
  displayName: "伏笔管理",
  description: "管理伏笔的完整生命周期",
  
  inputSchema: ManageForeshadowingInput,
  outputSchema: ManageForeshadowingOutput,
  
  requiredPermissions: ["write:foreshadowing"],
  
  execute: async (input, context: ToolContext) => {
    const { operation, foreshadowing, reason } = input;
    
    let result;
    const warnings: string[] = [];
    
    switch (operation) {
      case "create":
        result = await context.knowledgeBase.createForeshadowing({
          bookId: context.bookId,
          ...foreshadowing,
          status: "planned",
        });
        break;
        
      case "bury":
        if (!foreshadowing.buriedIn) {
          throw new Error("buriedIn is required for bury operation");
        }
        result = await context.knowledgeBase.updateForeshadowing(
          foreshadowing.id!,
          {
            status: "buried",
            buriedIn: foreshadowing.buriedIn,
          }
        );
        break;
        
      case "trigger":
        if (!foreshadowing.triggeredIn) {
          throw new Error("triggeredIn is required for trigger operation");
        }
        result = await context.knowledgeBase.updateForeshadowing(
          foreshadowing.id!,
          {
            status: "triggered",
            triggeredIn: foreshadowing.triggeredIn,
          }
        );
        break;
        
      case "abandon":
        result = await context.knowledgeBase.updateForeshadowing(
          foreshadowing.id!,
          {
            status: "abandoned",
            abandonReason: reason,
          }
        );
        warnings.push(`伏笔 "${result.name}" 已被放弃`);
        break;
        
      default:
        throw new Error(`Unknown operation: ${operation}`);
    }
    
    // 检查压力警告
    if (result.pressure > 80) {
      warnings.push(`伏笔 "${result.name}" 压力值过高 (${result.pressure})，建议尽快触发`);
    }
    
    return {
      foreshadowingId: result.id,
      status: result.status,
      pressure: result.pressure,
      warnings: warnings.length > 0 ? warnings : undefined,
    };
  },
};
```

### 3.6 大纲创建 Tool（Create Outline）

```typescript
// packages/agents/src/tools/novel-tools/create-outline.ts

import { z } from "zod";
import type { ToolDefinition, ToolContext } from "../types";

const CreateOutlineInput = z.object({
  outline: z.object({
    title: z.string().describe("小说标题"),
    genre: z.string().describe("类型"),
    targetWords: z.number().describe("目标字数"),
    structure: z.enum([
      "three_act",     // 三幕式
      "hero_journey",  // 英雄之旅
      "five_act",      // 五幕式
      "custom",        // 自定义
    ]).describe("结构类型"),
    
    // 卷/部结构
    volumes: z.array(z.object({
      name: z.string(),
      chapters: z.number(),
      summary: z.string(),
    })).optional().describe("分卷信息"),
    
    // 章节大纲
    chapters: z.array(z.object({
      number: z.number(),
      title: z.string().optional(),
      summary: z.string(),
      pov: z.string().optional(),
      keyEvents: z.array(z.string()),
      estimatedWords: z.number().optional(),
    })).describe("章节大纲"),
    
    // 关键节点
    keyPoints: z.object({
      incitingIncident: z.string().describe("激励事件"),
      firstPlotPoint: z.string().describe("第一情节点"),
      midpoint: z.string().describe("中点"),
      secondPlotPoint: z.string().describe("第二情节点"),
      climax: z.string().describe("高潮"),
      resolution: z.string().describe("结局"),
    }).optional().describe("关键剧情节点"),
  }).describe("大纲数据"),
});

const CreateOutlineOutput = z.object({
  outlineId: z.string(),
  chapterCount: z.number(),
  estimatedTotalWords: z.number(),
});

export const createOutlineTool: ToolDefinition<
  z.infer<typeof CreateOutlineInput>,
  z.infer<typeof CreateOutlineOutput>
> = {
  name: "create_outline",
  displayName: "创建大纲",
  description: "创建或更新小说大纲",
  
  inputSchema: CreateOutlineInput,
  outputSchema: CreateOutlineOutput,
  
  requiredPermissions: ["write:outline"],
  
  execute: async (input, context: ToolContext) => {
    const { outline } = input;
    
    // 计算预计总字数
    const estimatedTotalWords = outline.chapters.reduce(
      (sum, ch) => sum + (ch.estimatedWords || 3000),
      0
    );
    
    // 存储大纲
    const result = await context.knowledgeBase.createOutline({
      bookId: context.bookId,
      ...outline,
      estimatedTotalWords,
      author: context.agentName,
      version: 1,
      createdAt: new Date().toISOString(),
    });
    
    return {
      outlineId: result.id,
      chapterCount: outline.chapters.length,
      estimatedTotalWords,
    };
  },
};
```

### 3.7 委派 Tool（Delegate to Agent）

```typescript
// packages/agents/src/tools/novel-tools/delegate-to-agent.ts

import { z } from "zod";
import type { ToolDefinition, ToolContext } from "../types";

const DelegateToAgentInput = z.object({
  targetAgent: z.enum([
    "writer",
    "world-guardian",
    "planner",
    "reviewer",
    "observer",
    "researcher",
    "liuheping",
  ]).describe("目标 Agent"),
  
  task: z.string().describe("任务描述"),
  
  expectedOutcome: z.string().describe("预期输出"),
  
  context: z.object({
    chapterId: z.string().optional(),
    characterIds: z.array(z.string()).optional(),
    knowledgeBases: z.array(z.string()).optional(),
    constraints: z.array(z.string()).optional(),
  }).optional().describe("任务上下文"),
  
  priority: z.enum(["high", "medium", "low"]).default("medium")
    .describe("优先级"),
  
  sessionId: z.string().optional()
    .describe("会话 ID（用于连续对话）"),
});

const DelegateToAgentOutput = z.object({
  delegated: z.boolean(),
  sessionId: z.string(),
  status: z.enum(["queued", "running", "completed", "failed"]),
  message: z.string().optional(),
});

export const delegateToAgentTool: ToolDefinition<
  z.infer<typeof DelegateToAgentInput>,
  z.infer<typeof DelegateToAgentOutput>
> = {
  name: "delegate_to_agent",
  displayName: "委派任务",
  description: "将任务委派给其他 Agent",
  
  inputSchema: DelegateToAgentInput,
  outputSchema: DelegateToAgentOutput,
  
  requiredPermissions: ["delegate:agents"],
  
  execute: async (input, context: ToolContext) => {
    const { targetAgent, task, expectedOutcome, context: taskContext, priority, sessionId } = input;
    
    // 检查目标 Agent 是否可用
    const isAvailable = agentRegistry.isAvailable(targetAgent, context.stage);
    
    if (!isAvailable) {
      const meta = agentRegistry.getMetadata(targetAgent);
      return {
        delegated: false,
        sessionId: "",
        status: "failed",
        message: `Agent "${meta?.displayName || targetAgent}" 在当前阶段不可用`,
      };
    }
    
    // 创建委派任务
    const delegationResult = await createDelegation({
      fromAgent: context.agentName,
      toAgent: targetAgent,
      task,
      expectedOutcome,
      context: taskContext,
      priority,
      sessionId,
      bookId: context.bookId,
      parentSessionId: context.sessionId,
    });
    
    return {
      delegated: true,
      sessionId: delegationResult.sessionId,
      status: delegationResult.status,
    };
  },
};
```

---

## 4. Tool 注册

### 4.1 注册所有 Tools

```typescript
// packages/agents/src/tools/index.ts

import { toolRegistry } from "./tool-registry";
import { knowledgeSearchTool } from "./novel-tools/knowledge-search";
import { writeChapterTool } from "./novel-tools/write-chapter";
import { addAnnotationTool } from "./novel-tools/add-annotation";
import { updateWorldviewTool } from "./novel-tools/update-worldview";
import { manageForeshadowingTool } from "./novel-tools/manage-foreshadowing";
import { createOutlineTool } from "./novel-tools/create-outline";
import { delegateToAgentTool } from "./novel-tools/delegate-to-agent";

export function registerAllTools(): void {
  const tools = [
    knowledgeSearchTool,
    writeChapterTool,
    addAnnotationTool,
    updateWorldviewTool,
    manageForeshadowingTool,
    createOutlineTool,
    delegateToAgentTool,
  ];
  
  for (const tool of tools) {
    toolRegistry.register(tool);
  }
}

export { toolRegistry };
```

### 4.2 获取 Agent 可用 Tools

```typescript
// 在创建 Agent 时获取其可用的 Tools
export function getToolsForAgent(
  agentName: string
): ToolDefinition[] {
  return toolRegistry.getToolsForAgent(agentName);
}

// 获取 Tool Schemas（用于 LLM Function Calling）
export function getToolSchemasForAgent(
  agentName: string
): ToolSchema[] {
  return toolRegistry.getToolSchemasForAgent(agentName);
}
```

---

## 5. Tool 权限控制

### 5.1 权限矩阵

```typescript
// packages/agents/src/core/permission.ts

const AGENT_TOOL_PERMISSIONS: Record<string, string[]> = {
  "tian-dao": [
    "read:all_knowledge_bases",
    "write:worldview",
    "write:foreshadowing",
    "write:timeline",
    "write:factions",
    "delegate:agents",
  ],
  
  "writer": [
    "read:outline",
    "read:worldview",
    "read:characters",
    "read:history",
    "write:chapter_content",
    "apply:annotation",
  ],
  
  "world-guardian": [
    "read:all_knowledge_bases",
    "write:annotation",
    "report:conflict",
  ],
  
  "planner": [
    "write:worldview",
    "write:characters",
    "write:outline",
    "write:style_config",
  ],
  
  "reviewer": [
    "read:chapter_content",
    "read:all_knowledge_bases",
    "write:annotation",
  ],
  
  "observer": [
    "read:all_knowledge_bases",
    "write:progress",
    "write:todo",
    "sync:knowledge_base",
  ],
  
  "researcher": [
    "read:uploaded_novels",
    "write:analysis_report",
    "external_search",
  ],
  
  "liuheping": [
    "read:chapter_content",
    "read:characters",
    "write:annotation",
    "write:character_voice",
  ],
};

export class PermissionChecker {
  private permissions: Map<string, Set<string>> = new Map();
  
  constructor() {
    // 初始化权限
    for (const [agent, perms] of Object.entries(AGENT_TOOL_PERMISSIONS)) {
      this.permissions.set(agent, new Set(perms));
    }
  }
  
  hasPermission(agentName: string, permission: string): boolean {
    const agentPerms = this.permissions.get(agentName);
    if (!agentPerms) return false;
    
    // 检查通配符权限
    if (agentPerms.has("read:all_knowledge_bases") && 
        permission.startsWith("read:")) {
      return true;
    }
    
    return agentPerms.has(permission);
  }
  
  check(agentName: string, requiredPermissions: string[]): boolean {
    return requiredPermissions.every(perm => 
      this.hasPermission(agentName, perm)
    );
  }
  
  canUse(agentName: string, requiredPermissions: string[]): boolean {
    return this.check(agentName, requiredPermissions);
  }
}

export const permissionChecker = new PermissionChecker();
```

---

## 6. Tool 与 Vercel AI SDK 集成

### 6.1 转换为 AI SDK Tool

```typescript
// packages/agents/src/tools/ai-sdk-adapter.ts

import { tool } from "ai";
import { z } from "zod";
import type { ToolDefinition } from "./types";

export function toAITool(toolDef: ToolDefinition): ReturnType<typeof tool> {
  return tool({
    description: toolDef.description,
    parameters: toolDef.inputSchema,
    
    execute: async (input, options) => {
      // 构建执行上下文
      const context: ToolContext = {
        bookId: options.context?.bookId,
        sessionId: options.context?.sessionId,
        agentName: options.context?.agentName,
        userId: options.context?.userId,
        knowledgeBase: getKnowledgeBaseClient(options.context?.bookId),
        logger: getLogger(),
      };
      
      // 执行 Tool
      const result = await toolDef.execute(input, context);
      
      return result;
    },
  });
}

// 批量转换
export function toAITools(toolDefs: ToolDefinition[]): Record<string, ReturnType<typeof tool>> {
  const aiTools: Record<string, ReturnType<typeof tool>> = {};
  
  for (const toolDef of toolDefs) {
    aiTools[toolDef.name] = toAITool(toolDef);
  }
  
  return aiTools;
}
```

### 6.2 在 Agent 中使用

```typescript
// packages/agents/src/core/agent-executor.ts

import { generateText, streamText } from "ai";
import { toAITools } from "../tools/ai-sdk-adapter";

export async function executeAgent(
  agentName: string,
  messages: Message[],
  context: ExecutionContext
) {
  const agent = agentRegistry.get(agentName);
  if (!agent) throw new Error(`Agent not found: ${agentName}`);
  
  // 获取 Agent 可用的 Tools
  const tools = getToolsForAgent(agentName);
  const aiTools = toAITools(tools);
  
  // 获取模型
  const model = await resolveModel(agent.metadata.fallbackChain);
  
  // 调用 LLM
  const result = await streamText({
    model,
    system: agent.prompt,
    messages,
    tools: aiTools,
    maxTokens: agent.metadata.constraints?.maxTokens,
    temperature: agent.metadata.constraints?.temperature,
  });
  
  return result;
}
```

---

## 7. 相关文档

- [01-overview.md](./01-overview.md) - Agent 系统概述
- [02-agent-definitions.md](./02-agent-definitions.md) - Agent 定义
- [03-hooks-system.md](./03-hooks-system.md) - Hooks 系统
- [05-skills-system.md](./05-skills-system.md) - Skills 系统
- [06-delegation-protocol.md](./06-delegation-protocol.md) - 委派协议