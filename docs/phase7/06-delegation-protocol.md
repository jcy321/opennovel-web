# Phase 7: Agent 系统 - 委派协议

**文档版本**: 1.0
**最后更新**: 2026-03-16

---

## 1. 概述

委派协议（Delegation Protocol）定义了 Agent 之间如何协作和通信。核心组件包括：

- **Intent Gate**：意图门控，分析用户意图
- **Delegation Protocol**：委派协议，规范任务委派
- **Session Continuity**：会话连续性，保持上下文

---

## 2. Intent Gate（意图门控）

### 2.1 四阶段决策模型

Intent Gate 采用 Oh My OpenCode 的四阶段决策模型：

```
┌─────────────────────────────────────────────────────────────┐
│                    Intent Gate 流程                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Step 0: 意图口语化                                         │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ 在内部思考中明确表述：                                 │   │
│  │ "我检测到 [意图类型] —— [原因]                         │   │
│  │  我的方法：[处理方式]"                                 │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                 │
│                           ▼                                 │
│  Step 1: 请求分类                                           │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ 分类请求类型：                                         │   │
│  │ - 简单：直接工具                                       │   │
│  │ - 明确：直接执行                                       │   │
│  │ - 探索性：启动调研者                                   │   │
│  │ - 开放式：先评估                                       │   │
│  │ - 模糊：提问澄清                                       │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                 │
│                           ▼                                 │
│  Step 2: 模糊检查                                           │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ 检查是否存在多种解释：                                 │   │
│  │ - 单一解释 → 继续                                      │   │
│  │ - 多种解释、类似工作量 → 使用默认                      │   │
│  │ - 多种解释、2倍+工作量差异 → 必须询问                  │   │
│  │ - 缺少关键信息 → 必须询问                              │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                 │
│                           ▼                                 │
│  Step 3: 行动前验证                                         │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ 验证假设和条件：                                       │   │
│  │ - 我有隐性假设吗？                                     │   │
│  │ - 目标 Agent 可用吗？                                  │   │
│  │ - 搜索范围清晰吗？                                     │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 意图类型映射

```typescript
// packages/agents/src/delegation/intent-gate.ts

type IntentType = 
  | "research"      // 研究/理解
  | "implementation" // 实施
  | "exploration"   // 探索
  | "evaluation"    // 评估
  | "fix"           // 修复
  | "open-ended";   // 开放式

interface IntentVerbalization {
  type: IntentType;
  reason: string;
  approach: string;
}

interface IntentGateResult {
  verbalization: IntentVerbalization;
  classification: RequestClassification;
  ambiguityCheck: AmbiguityCheckResult;
  validation: ValidationResult;
  action: IntentAction;
}

type RequestClassification = 
  | "trivial"       // 简单
  | "explicit"      // 明确
  | "exploratory"   // 探索性
  | "open-ended"    // 开放式
  | "ambiguous";    // 模糊

interface AmbiguityCheckResult {
  hasAmbiguity: boolean;
  interpretations: string[];
  effortDifference: number;  // 工作量差异倍数
  needsClarification: boolean;
  clarificationQuestion?: string;
}

interface ValidationResult {
  hasImplicitAssumptions: boolean;
  assumptions: string[];
  targetAgentAvailable: boolean;
  searchScopeClear: boolean;
}

type IntentAction = 
  | { type: "direct_tool"; tool: string }
  | { type: "direct_execute"; task: string }
  | { type: "delegate"; agent: string; task: string }
  | { type: "clarify"; question: string }
  | { type: "assess_first"; scope: string };

export class IntentGate {
  /**
   * 分析用户意图
   */
  async analyze(userMessage: string, context: SessionContext): Promise<IntentGateResult> {
    // Step 0: 意图口语化
    const verbalization = this.verbalizeIntent(userMessage, context);
    
    // Step 1: 请求分类
    const classification = this.classifyRequest(userMessage, context);
    
    // Step 2: 模糊检查
    const ambiguityCheck = this.checkAmbiguity(userMessage, classification);
    
    // Step 3: 验证
    const validation = this.validate(verbalization, classification, context);
    
    // 确定行动
    const action = this.determineAction(
      verbalization,
      classification,
      ambiguityCheck,
      validation
    );
    
    return {
      verbalization,
      classification,
      ambiguityCheck,
      validation,
      action,
    };
  }
  
  /**
   * Step 0: 意图口语化
   */
  private verbalizeIntent(message: string, context: SessionContext): IntentVerbalization {
    const intentPatterns: Array<{
      patterns: RegExp[];
      type: IntentType;
    }> = [
      {
        patterns: [/如何/, /怎么/, /什么是/, /解释/],
        type: "research",
      },
      {
        patterns: [/写/, /创建/, /添加/, /实现/, /生成/],
        type: "implementation",
      },
      {
        patterns: [/查找/, /搜索/, /分析/, /检查/],
        type: "exploration",
      },
      {
        patterns: [/怎么样/, /好不好/, /评价/, /建议/],
        type: "evaluation",
      },
      {
        patterns: [/修复/, /修正/, /改/, /错误/],
        type: "fix",
      },
      {
        patterns: [/优化/, /改进/, /完善/, /提升/],
        type: "open-ended",
      },
    ];
    
    for (const { patterns, type } of intentPatterns) {
      if (patterns.some(p => p.test(message))) {
        return {
          type,
          reason: `检测到关键词匹配: ${type}`,
          approach: this.getApproachForIntent(type),
        };
      }
    }
    
    return {
      type: "open-ended",
      reason: "无法明确分类，默认开放式",
      approach: "先评估情况，再决定行动",
    };
  }
  
  private getApproachForIntent(type: IntentType): string {
    const approaches: Record<IntentType, string> = {
      research: "调研者 → 综合回答",
      implementation: "规划 → 委派或执行",
      exploration: "调研者/观察者 → 报告发现",
      evaluation: "评估 → 提议 → 等待确认",
      fix: "诊断 → 最小修复",
      open-ended: "评估代码库 → 提议方案",
    };
    return approaches[type];
  }
  
  /**
   * Step 1: 请求分类
   */
  private classifyRequest(message: string, context: SessionContext): RequestClassification {
    // 简单请求：单个关键词、已知位置
    if (this.isTrivialRequest(message, context)) {
      return "trivial";
    }
    
    // 明确请求：特定文件/行、清晰命令
    if (this.isExplicitRequest(message, context)) {
      return "explicit";
    }
    
    // 探索性请求
    if (this.isExploratoryRequest(message, context)) {
      return "exploratory";
    }
    
    // 开放式请求
    if (this.isOpenEndedRequest(message, context)) {
      return "open-ended";
    }
    
    return "ambiguous";
  }
  
  /**
   * Step 2: 模糊检查
   */
  private checkAmbiguity(message: string, classification: RequestClassification): AmbiguityCheckResult {
    if (classification !== "ambiguous") {
      return {
        hasAmbiguity: false,
        interpretations: [],
        effortDifference: 1,
        needsClarification: false,
      };
    }
    
    // 检测可能的多种解释
    const interpretations = this.detectInterpretations(message);
    
    if (interpretations.length <= 1) {
      return {
        hasAmbiguity: false,
        interpretations,
        effortDifference: 1,
        needsClarification: false,
      };
    }
    
    // 估算工作量差异
    const effortDifference = this.estimateEffortDifference(interpretations);
    
    return {
      hasAmbiguity: true,
      interpretations,
      effortDifference,
      needsClarification: effortDifference >= 2,
      clarificationQuestion: effortDifference >= 2 
        ? this.generateClarificationQuestion(interpretations)
        : undefined,
    };
  }
  
  /**
   * Step 3: 验证
   */
  private validate(
    verbalization: IntentVerbalization,
    classification: RequestClassification,
    context: SessionContext
  ): ValidationResult {
    const assumptions: string[] = [];
    
    // 检查隐性假设
    if (verbalization.type === "implementation") {
      // 假设用户想要创建/修改内容
      assumptions.push("假设用户想要修改/创建内容");
    }
    
    // 检查目标 Agent 可用性
    const targetAgent = this.getTargetAgentForIntent(verbalization.type);
    const targetAgentAvailable = agentRegistry.isAvailable(targetAgent, context.stage);
    
    // 检查搜索范围
    const searchScopeClear = this.isSearchScopeClear(classification, context);
    
    return {
      hasImplicitAssumptions: assumptions.length > 0,
      assumptions,
      targetAgentAvailable,
      searchScopeClear,
    };
  }
  
  /**
   * 确定行动
   */
  private determineAction(
    verbalization: IntentVerbalization,
    classification: RequestClassification,
    ambiguityCheck: AmbiguityCheckResult,
    validation: ValidationResult
  ): IntentAction {
    // 需要澄清
    if (ambiguityCheck.needsClarification && ambiguityCheck.clarificationQuestion) {
      return {
        type: "clarify",
        question: ambiguityCheck.clarificationQuestion,
      };
    }
    
    // 目标 Agent 不可用
    if (!validation.targetAgentAvailable) {
      return {
        type: "clarify",
        question: "当前阶段该 Agent 不可用，请尝试其他方式或等待阶段切换。",
      };
    }
    
    // 根据分类决定行动
    switch (classification) {
      case "trivial":
        return {
          type: "direct_tool",
          tool: this.getToolForTrivialRequest(verbalization),
        };
        
      case "explicit":
        return {
          type: "direct_execute",
          task: verbalization.reason,
        };
        
      case "exploratory":
        return {
          type: "delegate",
          agent: "researcher",
          task: verbalization.reason,
        };
        
      case "open-ended":
        return {
          type: "assess_first",
          scope: verbalization.reason,
        };
        
      default:
        return {
          type: "clarify",
          question: "请提供更多细节以便我更好地帮助您。",
        };
    }
  }
}
```

---

## 3. Delegation Protocol（委派协议）

### 3.1 6 字段结构

委派任务时，必须使用完整的 6 字段结构：

```typescript
// packages/agents/src/delegation/protocol.ts

interface DelegationRequest {
  // 1. TASK: 原子化、具体目标
  task: string;
  
  // 2. EXPECTED OUTCOME: 具体交付物与成功标准
  expectedOutcome: {
    deliverables: string[];
    successCriteria: string[];
  };
  
  // 3. REQUIRED TOOLS: 明确工具白名单
  requiredTools: string[];
  
  // 4. MUST DO: 穷尽要求
  mustDo: string[];
  
  // 5. MUST NOT DO: 禁止行为
  mustNotDo: string[];
  
  // 6. CONTEXT: 文件路径、现有模式、约束
  context: {
    files?: string[];
    patterns?: string[];
    constraints?: string[];
    relatedKnowledge?: string[];
  };
  
  // 元数据
  metadata: {
    fromAgent: string;
    toAgent: string;
    priority: "high" | "medium" | "low";
    sessionId?: string;
    parentSessionId?: string;
  };
}

interface DelegationResult {
  success: boolean;
  sessionId: string;
  output?: unknown;
  error?: {
    code: string;
    message: string;
  };
  followUpRequired: boolean;
  followUpSuggestions?: string[];
}

export class DelegationProtocol {
  /**
   * 创建委派请求
   */
  createRequest(params: Partial<DelegationRequest>): DelegationRequest {
    // 验证必需字段
    if (!params.task) {
      throw new Error("TASK is required");
    }
    if (!params.expectedOutcome) {
      throw new Error("EXPECTED OUTCOME is required");
    }
    if (!params.requiredTools) {
      throw new Error("REQUIRED TOOLS is required");
    }
    if (!params.mustDo) {
      throw new Error("MUST DO is required");
    }
    if (!params.mustNotDo) {
      throw new Error("MUST NOT DO is required");
    }
    
    return {
      task: params.task,
      expectedOutcome: params.expectedOutcome,
      requiredTools: params.requiredTools,
      mustDo: params.mustDo,
      mustNotDo: params.mustNotDo,
      context: params.context || {},
      metadata: params.metadata || {
        fromAgent: "unknown",
        toAgent: "unknown",
        priority: "medium",
      },
    };
  }
  
  /**
   * 执行委派
   */
  async delegate(request: DelegationRequest): Promise<DelegationResult> {
    const { metadata } = request;
    
    // 检查目标 Agent 是否可用
    const targetAgent = agentRegistry.get(metadata.toAgent);
    if (!targetAgent) {
      return {
        success: false,
        sessionId: "",
        error: {
          code: "AGENT_NOT_FOUND",
          message: `Agent ${metadata.toAgent} not found`,
        },
        followUpRequired: false,
      };
    }
    
    // 检查权限
    const hasPermission = await this.checkDelegationPermission(
      metadata.fromAgent,
      metadata.toAgent
    );
    
    if (!hasPermission) {
      return {
        success: false,
        sessionId: "",
        error: {
          code: "PERMISSION_DENIED",
          message: `${metadata.fromAgent} cannot delegate to ${metadata.toAgent}`,
        },
        followUpRequired: false,
      };
    }
    
    // 创建会话
    const session = await this.createDelegationSession(request);
    
    // 构建 Agent Prompt
    const agentPrompt = this.buildDelegationPrompt(request);
    
    // 执行 Agent
    try {
      const result = await this.executeAgent(metadata.toAgent, agentPrompt, session);
      
      return {
        success: true,
        sessionId: session.id,
        output: result,
        followUpRequired: this.needsFollowUp(result),
        followUpSuggestions: this.generateFollowUpSuggestions(result),
      };
    } catch (error) {
      return {
        success: false,
        sessionId: session.id,
        error: {
          code: "EXECUTION_ERROR",
          message: error instanceof Error ? error.message : "Unknown error",
        },
        followUpRequired: true,
        followUpSuggestions: ["重试任务", "简化任务范围", "请求用户澄清"],
      };
    }
  }
  
  /**
   * 构建委派 Prompt
   */
  private buildDelegationPrompt(request: DelegationRequest): string {
    const sections: string[] = [];
    
    // Task
    sections.push(`## TASK\n\n${request.task}`);
    
    // Expected Outcome
    sections.push(`## EXPECTED OUTCOME\n\n**交付物:**\n${request.expectedOutcome.deliverables.map(d => `- ${d}`).join("\n")}\n\n**成功标准:**\n${request.expectedOutcome.successCriteria.map(c => `- ${c}`).join("\n")}`);
    
    // Required Tools
    sections.push(`## REQUIRED TOOLS\n\n仅允许使用以下工具:\n${request.requiredTools.map(t => `- ${t}`).join("\n")}`);
    
    // Must Do
    sections.push(`## MUST DO\n\n${request.mustDo.map(d => `- ${d}`).join("\n")}`);
    
    // Must Not Do
    sections.push(`## MUST NOT DO\n\n${request.mustNotDo.map(d => `- ${d}`).join("\n")}`);
    
    // Context
    if (Object.keys(request.context).length > 0) {
      const contextSections: string[] = [];
      
      if (request.context.files) {
        contextSections.push(`**相关文件:**\n${request.context.files.map(f => `- ${f}`).join("\n")}`);
      }
      if (request.context.patterns) {
        contextSections.push(`**现有模式:**\n${request.context.patterns.map(p => `- ${p}`).join("\n")}`);
      }
      if (request.context.constraints) {
        contextSections.push(`**约束条件:**\n${request.context.constraints.map(c => `- ${c}`).join("\n")}`);
      }
      if (request.context.relatedKnowledge) {
        contextSections.push(`**相关知识:**\n${request.context.relatedKnowledge.map(k => `- ${k}`).join("\n")}`);
      }
      
      sections.push(`## CONTEXT\n\n${contextSections.join("\n\n")}`);
    }
    
    return sections.join("\n\n---\n\n");
  }
}
```

### 3.2 委派表示例

```typescript
// 委派表定义
const DELEGATION_TABLE: Record<string, DelegationTarget[]> = {
  // 天道的委派目标
  "tian-dao": [
    {
      condition: "需要撰写章节内容",
      target: "writer",
      priority: "high",
    },
    {
      condition: "需要检查一致性",
      target: "world-guardian",
      priority: "medium",
    },
    {
      condition: "需要新书规划",
      target: "planner",
      priority: "high",
      stageRestriction: [1],  // 仅阶段一
    },
    {
      condition: "需要审查章节",
      target: "reviewer",
      priority: "medium",
    },
    {
      condition: "需要监控进度",
      target: "observer",
      priority: "low",
    },
    {
      condition: "需要外部参考",
      target: "researcher",
      priority: "medium",
      stageRestriction: [2],  // 仅阶段二
    },
    {
      condition: "需要风格分析",
      target: "liuheping",
      priority: "medium",
    },
  ],
};

interface DelegationTarget {
  condition: string;
  target: string;
  priority: "high" | "medium" | "low";
  stageRestriction?: number[];
}
```

---

## 4. Session Continuity（会话连续性）

### 4.1 会话管理

```typescript
// packages/agents/src/delegation/session-continuity.ts

interface DelegationSession {
  id: string;
  parentId?: string;          // 父会话 ID
  bookId: string;
  agent: string;
  status: "active" | "completed" | "failed" | "waiting";
  
  // 对话历史
  messages: Message[];
  
  // 上下文
  context: Record<string, unknown>;
  
  // 时间戳
  createdAt: string;
  updatedAt: string;
  completedAt?: string;
}

export class SessionContinuityManager {
  private sessions: Map<string, DelegationSession> = new Map();
  
  /**
   * 创建新会话
   */
  async createSession(params: {
    bookId: string;
    agent: string;
    parentId?: string;
    initialContext?: Record<string, unknown>;
  }): Promise<DelegationSession> {
    const session: DelegationSession = {
      id: generateSessionId(),
      parentId: params.parentId,
      bookId: params.bookId,
      agent: params.agent,
      status: "active",
      messages: [],
      context: params.initialContext || {},
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };
    
    this.sessions.set(session.id, session);
    return session;
  }
  
  /**
   * 获取会话
   */
  getSession(sessionId: string): DelegationSession | undefined {
    return this.sessions.get(sessionId);
  }
  
  /**
   * 继续会话
   * 
   * **关键**: 使用 session_id 继续，而不是创建新会话
   */
  async continueSession(
    sessionId: string,
    message: string | Message
  ): Promise<DelegationSession> {
    const session = this.sessions.get(sessionId);
    if (!session) {
      throw new Error(`Session ${sessionId} not found`);
    }
    
    // 添加消息
    const msg: Message = typeof message === "string" 
      ? { role: "user", content: message }
      : message;
    
    session.messages.push(msg);
    session.updatedAt = new Date().toISOString();
    
    return session;
  }
  
  /**
   * 获取会话上下文
   * 
   * 包括父会话的上下文（如果有）
   */
  async getSessionContext(sessionId: string): Promise<Record<string, unknown>> {
    const session = this.sessions.get(sessionId);
    if (!session) return {};
    
    let context = { ...session.context };
    
    // 合并父会话上下文
    if (session.parentId) {
      const parentContext = await this.getSessionContext(session.parentId);
      context = { ...parentContext, ...context };
    }
    
    return context;
  }
  
  /**
   * 获取会话历史
   */
  getSessionHistory(sessionId: string): Message[] {
    const session = this.sessions.get(sessionId);
    return session?.messages || [];
  }
  
  /**
   * 完成会话
   */
  async completeSession(sessionId: string): Promise<void> {
    const session = this.sessions.get(sessionId);
    if (session) {
      session.status = "completed";
      session.completedAt = new Date().toISOString();
    }
  }
  
  /**
   * 标记会话失败
   */
  async failSession(sessionId: string, error: string): Promise<void> {
    const session = this.sessions.get(sessionId);
    if (session) {
      session.status = "failed";
      session.completedAt = new Date().toISOString();
      session.context.error = error;
    }
  }
}

// 单例实例
export const sessionContinuityManager = new SessionContinuityManager();
```

### 4.2 Session Continuity 规则

```typescript
/**
 * Session Continuity 规则
 * 
 * **总是继续当：**
 */
const SESSION_CONTINUITY_RULES = {
  // 任务失败/不完整
  onTaskFailed: {
    action: "continue",
    promptTemplate: "修复：{具体错误}",
    useParentSession: false,  // 使用当前 session
  },
  
  // 对结果有后续问题
  onFollowUpQuestion: {
    action: "continue",
    promptTemplate: "另外：{问题}",
    useParentSession: false,
  },
  
  // 与同一 Agent 多轮对话
  onMultiTurnConversation: {
    action: "continue",
    promptTemplate: "{消息内容}",
    useParentSession: false,
    rule: "永远不要重新开始新会话",
  },
  
  // 验证失败
  onVerificationFailed: {
    action: "continue",
    promptTemplate: "验证失败：{错误}。修复。",
    useParentSession: false,
  },
};

/**
 * 使用示例
 */
async function exampleUsage() {
  // ❌ 错误：每次都创建新会话
  // 这样会丢失所有上下文
  async function wrongWay() {
    const result1 = await delegate({ task: "写第一章" });
    const result2 = await delegate({ task: "继续写" });  // 新会话，丢失上下文
  }
  
  // ✅ 正确：使用 session_id 继续
  async function rightWay() {
    const result1 = await delegate({ task: "写第一章" });
    const sessionId = result1.sessionId;
    
    // 继续使用同一个会话
    const result2 = await delegate({
      task: "继续写",
      sessionId,  // 使用上一个会话的 ID
    });
  }
}
```

---

## 5. 委派流程图

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        委派流程完整流程                                   │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  用户消息                                                                │
│      │                                                                  │
│      ▼                                                                  │
│  ┌─────────────┐                                                        │
│  │ Intent Gate │                                                        │
│  │             │                                                        │
│  │ Step 0-3    │                                                        │
│  └─────────────┘                                                        │
│      │                                                                  │
│      ▼                                                                  │
│  ┌─────────────┐     ┌─────────────┐                                    │
│  │ 检查 Agent  │────▶│ Agent 可用? │                                    │
│  │ 可用性      │     └─────────────┘                                    │
│      │                    │                                             │
│      │              ┌─────┴─────┐                                       │
│      │              │           │                                       │
│      │              ▼           ▼                                       │
│      │           是           否                                        │
│      │              │           │                                       │
│      │              │           ▼                                       │
│      │              │     ┌─────────────┐                               │
│      │              │     │ 通知用户    │                               │
│      │              │     │ 提供替代方案 │                               │
│      │              │     └─────────────┘                               │
│      │              │                                                   │
│      ▼              ▼                                                   │
│  ┌─────────────────────────┐                                            │
│  │ 构建 6 字段委派请求     │                                            │
│  │                         │                                            │
│  │ 1. TASK                 │                                            │
│  │ 2. EXPECTED OUTCOME     │                                            │
│  │ 3. REQUIRED TOOLS       │                                            │
│  │ 4. MUST DO              │                                            │
│  │ 5. MUST NOT DO          │                                            │
│  │ 6. CONTEXT              │                                            │
│  └─────────────────────────┘                                            │
│      │                                                                  │
│      ▼                                                                  │
│  ┌─────────────────────────┐                                            │
│  │ 创建/继续会话           │                                            │
│  │                         │                                            │
│  │ session_id?             │                                            │
│  │ ├─ 是 → 继续现有会话    │                                            │
│  │ └─ 否 → 创建新会话      │                                            │
│  └─────────────────────────┘                                            │
│      │                                                                  │
│      ▼                                                                  │
│  ┌─────────────────────────┐                                            │
│  │ 执行目标 Agent          │                                            │
│  │                         │                                            │
│  │ - 调用 LLM              │                                            │
│  │ - 执行 Tools            │                                            │
│  │ - 触发 Hooks            │                                            │
│  └─────────────────────────┘                                            │
│      │                                                                  │
│      ▼                                                                  │
│  ┌─────────────────────────┐                                            │
│  │ 验证结果                │                                            │
│  │                         │                                            │
│  │ - 符合预期?             │                                            │
│  │ - 遵守约束?             │                                            │
│  │ - 需要后续?             │                                            │
│  └─────────────────────────┘                                            │
│      │                                                                  │
│      ├──────────────────────────────┐                                   │
│      │                              │                                   │
│      ▼                              ▼                                   │
│  ┌─────────────┐              ┌─────────────┐                           │
│  │ 返回结果    │              │ 继续会话    │                           │
│  │             │              │             │                           │
│  │ success     │              │ 使用相同    │                           │
│  │ sessionId   │              │ session_id  │                           │
│  └─────────────┘              └─────────────┘                           │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 6. 完整示例

### 6.1 天道委派给执笔

```typescript
// 用户请求："继续写下一章"

async function handleUserRequest(userMessage: string, context: SessionContext) {
  // 1. Intent Gate 分析
  const intentGate = new IntentGate();
  const intentResult = await intentGate.analyze(userMessage, context);
  
  console.log("意图口语化:", intentResult.verbalization);
  // 输出: "我检测到实施意图 —— 用户想要继续创作。我的方法：委派给执笔 Agent。"
  
  // 2. 构建委派请求
  const delegationProtocol = new DelegationProtocol();
  
  const delegationRequest = delegationProtocol.createRequest({
    task: "撰写第 15 章内容",
    
    expectedOutcome: {
      deliverables: [
        "完整的章节正文（约 3500 字）",
        "符合大纲要求",
      ],
      successCriteria: [
        "包含大纲中的所有关键事件",
        "人物声音与设定一致",
        "没有世界观规则违反",
      ],
    },
    
    requiredTools: [
      "write_chapter",
      "read_knowledge_base",
      "apply_annotation",
    ],
    
    mustDo: [
      "先阅读大纲中本章的关键事件",
      "检索主角和配角的性格设定",
      "确保战斗场景与世界观魔法系统一致",
      "在章节结尾埋设'主角隐藏力量'伏笔",
    ],
    
    mustNotDo: [
      "不改变主角已确定的性格特征",
      "不违反魔法系统的限制规则",
      "不跳过大纲中的关键事件",
    ],
    
    context: {
      files: ["大纲第 15 章"],
      patterns: ["当前文风：紧凑节奏、大量对话"],
      constraints: ["战斗场景需要详细描写魔法效果"],
      relatedKnowledge: ["主角性格设定", "魔法系统规则", "反派能力设定"],
    },
    
    metadata: {
      fromAgent: "tian-dao",
      toAgent: "writer",
      priority: "high",
      parentSessionId: context.sessionId,
    },
  });
  
  // 3. 执行委派
  const result = await delegationProtocol.delegate(delegationRequest);
  
  // 4. 处理结果
  if (result.success) {
    console.log("章节撰写成功");
    console.log("会话 ID:", result.sessionId);
    
    if (result.followUpRequired) {
      console.log("建议后续:", result.followUpSuggestions);
    }
  } else {
    console.log("委派失败:", result.error);
    
    // 使用 session_id 继续修复
    if (result.sessionId) {
      const fixResult = await delegationProtocol.delegate({
        ...delegationRequest,
        task: `修复错误: ${result.error!.message}`,
        metadata: {
          ...delegationRequest.metadata,
          sessionId: result.sessionId,  // 继续使用同一个会话
        },
      });
    }
  }
}
```

---

## 7. 相关文档

- [01-overview.md](./01-overview.md) - Agent 系统概述
- [02-agent-definitions.md](./02-agent-definitions.md) - Agent 定义
- [03-hooks-system.md](./03-hooks-system.md) - Hooks 系统
- [04-tools-system.md](./04-tools-system.md) - Tools 系统
- [05-skills-system.md](./05-skills-system.md) - Skills 系统