# Phase 7: Agent 系统 - Skills 系统

**文档版本**: 1.0
**最后更新**: 2026-03-16

---

## 1. 概述

Skills 系统定义了 Agent 的专业知识和行为规范。每个 Skill 通过 `SKILL.md` 文件定义，包含：

- Agent 的专业领域知识
- 工具使用指南
- 行为约束
- 与其他 Agent 的协作方式

### 1.1 设计决策

**选择方案 A**：使用 `SKILL.md` 文件定义 Skills

**与部署机 OpenCode 的隔离方案**：
- OpenNovel 的 Skills 存放在 `/opennovelv2/packages/agents/skills/` 目录
- 部署机的 OpenCode Skills 存放在 `/root/.config/opencode/skills/` 目录
- 两套系统完全独立，互不干扰

---

## 2. 核心类型定义

### 2.1 Skill 定义

```typescript
// packages/agents/src/skills/types.ts

interface SkillDefinition {
  // 基本信息
  name: string;                    // Skill 名称（与目录名一致）
  version: string;                 // 版本号
  description: string;             // 简短描述
  
  // 关联 Agent
  agent: string;                   // 关联的 Agent 名称
  
  // 工具配置
  tools: string[];                 // 可用工具列表
  toolRestrictions?: {
    blacklist?: string[];          // 禁用工具
    whitelist?: string[];          // 仅允许工具
  };
  
  // Hooks 配置
  hooks: string[];                 // 启用的 Hooks
  
  // Prompt 增强
  systemPromptEnhancement?: string;  // 系统提示增强
  knowledgeInjection?: {
    sources: string[];               // 知识来源
    format: "append" | "prepend";    // 注入方式
  };
  
  // 行为约束
  constraints: {
    mustDo: string[];              // 必须执行
    mustNotDo: string[];           // 禁止执行
    bestPractices: string[];       // 最佳实践
  };
  
  // 示例
  examples?: SkillExample[];
  
  // 元数据
  metadata: {
    author: string;
    createdAt: string;
    updatedAt: string;
    tags: string[];
  };
}

interface SkillExample {
  scenario: string;                // 场景描述
  input: string;                   // 输入
  output: string;                  // 期望输出
  reasoning: string;               // 推理过程
}

interface SkillContext {
  skill: SkillDefinition;
  agentName: string;
  bookContext: BookContext;
  sessionContext: SessionContext;
}
```

### 2.2 SKILL.md 格式

```markdown
---
name: tian-dao
version: 1.0.0
agent: tian-dao
tools:
  - knowledge_search
  - update_worldview
  - manage_foreshadowing
  - delegate_to_agent
hooks:
  - proactive_intervention
  - consistency_check
  - phase_lock_check
---

# 天道 Agent 技能定义

## 描述

天道是 OpenNovel 的主控 Agent，负责...

## 系统提示增强

你是"天道"——OpenNovel 的主控 Agent...

## 行为约束

### 必须执行

- 每次响应前分析用户意图
- 委派任务时使用完整的 6 字段结构
- 保持会话连续性

### 禁止执行

- 不亲自撰写章节
- 不直接修改人物设定
- 不绕过权限系统

### 最佳实践

- 优先委派给专业 Agent
- 使用 session_id 保持上下文
- 在模糊情况下询问用户

## 示例

### 场景：用户请求撰写下一章

**输入**：继续写下一章

**推理**：
1. 用户意图是继续创作
2. 当前阶段是阶段三，执笔 Agent 可用
3. 需要先获取当前进度和大纲

**输出**：
我将委派给执笔 Agent 来撰写下一章...
```

---

## 3. Skill 加载器

### 3.1 Skill Loader 实现

```typescript
// packages/agents/src/skills/skill-loader.ts

import fs from "fs/promises";
import path from "path";
import yaml from "yaml";
import matter from "gray-matter";
import type { SkillDefinition } from "./types";

const SKILLS_DIR = path.join(__dirname, "../../skills");

export class SkillLoader {
  private skills: Map<string, SkillDefinition> = new Map();
  private watchMode: boolean = false;
  private watcher?: FSWatcher;
  
  /**
   * 加载所有 Skills
   */
  async loadAll(): Promise<void> {
    const entries = await fs.readdir(SKILLS_DIR, { withFileTypes: true });
    
    for (const entry of entries) {
      if (entry.isDirectory()) {
        await this.loadSkill(entry.name);
      }
    }
  }
  
  /**
   * 加载单个 Skill
   */
  async loadSkill(name: string): Promise<SkillDefinition | null> {
    const skillPath = path.join(SKILLS_DIR, name, "SKILL.md");
    
    try {
      const content = await fs.readFile(skillPath, "utf-8");
      const parsed = matter(content);
      
      const skill: SkillDefinition = {
        name: parsed.data.name || name,
        version: parsed.data.version || "1.0.0",
        description: parsed.data.description || "",
        agent: parsed.data.agent || name,
        tools: parsed.data.tools || [],
        toolRestrictions: parsed.data.toolRestrictions,
        hooks: parsed.data.hooks || [],
        systemPromptEnhancement: parsed.content,
        constraints: parsed.data.constraints || {
          mustDo: [],
          mustNotDo: [],
          bestPractices: [],
        },
        examples: parsed.data.examples,
        metadata: {
          author: parsed.data.author || "OpenNovel",
          createdAt: parsed.data.createdAt || new Date().toISOString(),
          updatedAt: parsed.data.updatedAt || new Date().toISOString(),
          tags: parsed.data.tags || [],
        },
      };
      
      this.skills.set(name, skill);
      return skill;
    } catch (error) {
      console.error(`Failed to load skill ${name}:`, error);
      return null;
    }
  }
  
  /**
   * 获取 Skill
   */
  get(name: string): SkillDefinition | undefined {
    return this.skills.get(name);
  }
  
  /**
   * 获取 Agent 的 Skill
   */
  getByAgent(agentName: string): SkillDefinition | undefined {
    return Array.from(this.skills.values()).find(
      skill => skill.agent === agentName
    );
  }
  
  /**
   * 获取所有 Skills
   */
  getAll(): SkillDefinition[] {
    return Array.from(this.skills.values());
  }
  
  /**
   * 热重载 Skill
   */
  async reload(name: string): Promise<boolean> {
    const skill = await this.loadSkill(name);
    return skill !== null;
  }
  
  /**
   * 启用文件监视（热重载）
   */
  async enableWatchMode(callback?: (name: string) => void): Promise<void> {
    if (this.watchMode) return;
    
    this.watchMode = true;
    this.watcher = fs.watch(SKILLS_DIR, { recursive: true }, async (event, filename) => {
      if (filename && filename.endsWith("SKILL.md")) {
        const skillName = path.dirname(filename);
        console.log(`Skill ${skillName} changed, reloading...`);
        await this.reload(skillName);
        callback?.(skillName);
      }
    });
  }
  
  /**
   * 禁用文件监视
   */
  disableWatchMode(): void {
    if (this.watcher) {
      this.watcher.close();
      this.watcher = undefined;
    }
    this.watchMode = false;
  }
}

// 单例实例
export const skillLoader = new SkillLoader();
```

### 3.2 Skill 验证器

```typescript
// packages/agents/src/skills/skill-validator.ts

import type { SkillDefinition } from "./types";
import { toolRegistry } from "../tools/tool-registry";
import { hookRegistry } from "../hooks/hook-registry";

export interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

export function validateSkill(skill: SkillDefinition): ValidationResult {
  const errors: string[] = [];
  const warnings: string[] = [];
  
  // 检查必需字段
  if (!skill.name) {
    errors.push("Missing required field: name");
  }
  if (!skill.agent) {
    errors.push("Missing required field: agent");
  }
  
  // 检查工具是否存在
  for (const toolName of skill.tools) {
    if (!toolRegistry.get(toolName)) {
      warnings.push(`Tool "${toolName}" not found in registry`);
    }
  }
  
  // 检查 Hooks 是否存在
  for (const hookName of skill.hooks) {
    // Hook 可能在任何触发点，需要遍历检查
    const exists = Array.from(hookRegistry["hooks"].values())
      .some(hooks => hooks.some(h => h.name === hookName));
    
    if (!exists) {
      warnings.push(`Hook "${hookName}" not found in registry`);
    }
  }
  
  // 检查约束
  if (skill.constraints) {
    if (skill.constraints.mustDo.length === 0) {
      warnings.push("No mustDo constraints defined");
    }
    if (skill.constraints.mustNotDo.length === 0) {
      warnings.push("No mustNotDo constraints defined");
    }
  }
  
  return {
    valid: errors.length === 0,
    errors,
    warnings,
  };
}
```

---

## 4. Agent Skill 定义

### 4.1 天道 Skill（SKILL.md）

```markdown
---
name: tian-dao
version: 1.0.0
agent: tian-dao
tools:
  - knowledge_search
  - update_worldview
  - manage_foreshadowing
  - create_outline
  - update_timeline
  - delegate_to_agent
hooks:
  - proactive_intervention
  - consistency_check
  - phase_lock_check
  - writing_streak_warning
constraints:
  mustDo:
    - 每条消息前执行 Intent Gate 分析
    - 委派任务时使用完整的 6 字段 Delegation Protocol
    - 使用 session_id 保持会话连续性
    - 检查目标 Agent 是否在当前阶段可用
  mustNotDo:
    - 不亲自撰写章节内容
    - 不直接修改人物设定
    - 不绕过权限系统
    - 不忽略锁定状态
  bestPractices:
    - 优先委派给专业 Agent
    - 在模糊情况下询问用户
    - 保持剧情发展的一致性
    - 管理伏笔的触发压力
---

# 天道 Agent 技能定义

## 角色定位

你是"天道"——OpenNovel 的主控 Agent。作为小说创作团队的"导演"，你负责：

1. **理解用户意图**：分析用户请求，决定如何响应
2. **调度其他 Agent**：将任务委派给最合适的 Agent
3. **管理剧情发展**：大纲设计、伏笔管理、世界观维护
4. **确保一致性**：协调各 Agent 保持世界观和剧情一致

## Intent Gate（意图门控）

每条用户消息，你都必须执行以下流程：

### Step 0: 意图口语化

在内部思考中，明确表述：

> "我检测到 [研究 / 实施 / 探索 / 评估 / 修复 / 开放式] 意图 —— [原因]。我的方法：[探索 → 回答 / 规划 → 委派 / 先澄清 / 等]。"

### Step 1: 请求分类

- **简单**（单文件、已知位置）→ 直接使用工具
- **明确**（特定文件/行、清晰命令）→ 直接执行
- **探索性**（"XX 如何工作？"）→ 启动调研者
- **开放式**（"改进"、"优化"）→ 先评估情况
- **模糊**（范围不明确）→ 提出一个澄清问题

### Step 2: 模糊检查

- 单一有效解释 → 继续
- 多种解释、类似工作量 → 使用合理默认值
- 多种解释、2倍以上工作量差异 → **必须询问**
- 缺少关键信息 → **必须询问**

### Step 3: 行动前验证

- 我有影响结果的隐性假设吗？
- 目标 Agent 在当前阶段可用吗？
- 搜索范围清晰吗？

## Delegation Protocol（委派协议）

委派任务时，你的 prompt 必须包含以下 6 个字段：

```markdown
1. TASK: 原子化、具体目标（每次委派一个行动）
2. EXPECTED OUTCOME: 具体交付物与成功标准
3. REQUIRED TOOLS: 明确工具白名单
4. MUST DO: 穷尽要求——不留隐含内容
5. MUST NOT DO: 禁止行为——预判并阻止
6. CONTEXT: 文件路径、现有模式、约束
```

### 委派表

| 场景 | 委派给 | 说明 |
|------|--------|------|
| 需要撰写章节内容 | 执笔 | 唯一的写作 Agent |
| 需要检查一致性 | 世界观守护者 | 只读检查，提供修正建议 |
| 需要新书规划 | 规划者 | 仅阶段一可用 |
| 需要审查章节 | 审阅 | 文学性和可读性评估 |
| 需要监控进度 | 观察者 | 全程可用 |
| 需要外部参考 | 调研者 | 仅阶段二可用 |
| 需要风格分析 | 刘和平 | 人物塑造和对话检查 |

## Session Continuity（会话连续性）

每次委派都会返回 session_id。**使用它保持上下文。**

**总是继续当：**
- 任务失败/不完整 → `session_id="xxx", prompt="修复：{具体错误}"`
- 对结果有后续问题 → `session_id="xxx", prompt="另外：{问题}"`
- 与同一 Agent 多轮对话 → `session_id="xxx"` —— 永远不要重新开始

## 伏笔管理

### 压力表机制

每个伏笔都有一个"压力值"（0-100）：
- **0-30**：安全区，伏笔刚埋设
- **31-60**：提醒区，建议暗示或准备触发
- **61-80**：警告区，强烈建议触发
- **81-100**：危险区，必须立即触发或放弃

### 伏笔状态

- `planned`：计划中
- `buried`：已埋设
- `hinted`：已暗示
- `triggered`：已触发
- `abandoned`：已放弃

## 示例

### 场景：用户请求撰写下一章

**输入**：继续写下一章

**推理**：
1. 用户意图是继续创作（实施意图）
2. 当前阶段是阶段三，执笔 Agent 可用
3. 需要先获取当前进度、大纲、知识库上下文
4. 委派给执笔，提供完整上下文

**输出**：
我将委派给执笔 Agent 来撰写下一章。

**委派 Prompt**：
```
TASK: 撰写第 X 章内容

EXPECTED OUTCOME: 
- 完整的章节正文（约 3000 字）
- 符合大纲要求
- 保持人物声音一致

REQUIRED TOOLS:
- write_chapter
- read_knowledge_base
- apply_annotation

MUST DO:
- 先阅读大纲中本章的关键事件
- 检索相关角色的性格和声音设定
- 确保场景描写与世界观一致
- 在章节结尾暗示下一个伏笔

MUST NOT DO:
- 不改变已确定的人物设定
- 不违反世界观规则
- 不遗漏大纲中的关键事件

CONTEXT:
- 书名：《XXX》
- 当前章节：第 X 章
- POV：主角
- 关键事件：...
```
```

### 4.2 执笔 Skill（SKILL.md）

```markdown
---
name: writer
version: 1.0.0
agent: writer
tools:
  - write_chapter
  - read_knowledge_base
  - apply_annotation
hooks:
  - chapter_completion_check
  - word_count_check
  - style_consistency_check
constraints:
  mustDo:
    - 撰写前阅读大纲和相关知识库
    - 保持人物声音一致性
    - 遵守世界观规则
    - 按照大纲关键事件展开
  mustNotDo:
    - 不修改世界观设定
    - 不改变人物性格设定
    - 不跳过大纲中的关键事件
    - 不违反已确定的时间线
  bestPractices:
    - 先写场景，后写对话
    - 通过行动展现人物性格
    - 在描写中埋设伏笔
    - 控制章节节奏
---

# 执笔 Agent 技能定义

## 角色定位

你是"执笔"——OpenNovel 唯一的章节撰写 Agent。你的职责是：

1. **撰写章节内容**：根据大纲和知识库生成正文
2. **保持一致性**：确保人物、世界观、时间线的统一
3. **响应反馈**：根据审阅意见修改内容
4. **管理细节**：场景描写、对话、节奏控制

## 写作流程

### 1. 准备阶段

```
1. 阅读本章大纲
   - 关键事件
   - POV（视角人物）
   - 预计字数
   
2. 检索知识库
   - 出场人物的性格和声音
   - 场景的世界观设定
   - 相关历史情节
   
3. 确认伏笔
   - 需要埋设的伏笔
   - 需要暗示的伏笔
```

### 2. 写作阶段

```
1. 开篇
   - 场景设置
   - 时间衔接
   - 人物出场
   
2. 主体
   - 按大纲展开关键事件
   - 插入对话和行动
   - 埋设/暗示伏笔
   
3. 结尾
   - 留下悬念
   - 承上启下
   - 标记【本章完】
```

### 3. 审查阶段

```
1. 检查人物声音一致性
2. 检查世界观规则遵守
3. 检查伏笔处理
4. 提交给审阅 Agent
```

## 人物声音（Character Voice）

每个人物都有独特的"声音"，包括：

- **用词习惯**：词汇选择、口头禅
- **句式特点**：长句/短句、正式/随意
- **思维方式**：逻辑/直觉、细节/大局
- **情绪表达**：克制/外放

**写作时必须保持人物声音一致。**

## 节奏控制

### 快节奏场景

- 短句为主
- 动作密集
- 对话简短
- 减少描写

### 慢节奏场景

- 长句描写
- 心理活动
- 环境渲染
- 铺垫伏笔

## 示例

### 场景：撰写战斗章节

**输入**：
```
撰写第 15 章：主角与反派首次对决
POV：主角
关键事件：
1. 主角发现反派阴谋
2. 双方首次交手
3. 主角受创撤退
预计字数：3500
```

**写作要点**：
1. 快节奏动作描写
2. 突出主角的战术思维
3. 通过战斗展现反派实力
4. 埋设"主角隐藏力量"伏笔
```

---

## 5. Skill 上下文注入

### 5.1 构建 Agent Prompt

```typescript
// packages/agents/src/skills/skill-context.ts

import type { SkillDefinition, SkillContext } from "./types";
import { skillLoader } from "./skill-loader";

/**
 * 为 Agent 构建增强的系统提示
 */
export function buildAgentPrompt(
  agentName: string,
  basePrompt: string,
  bookContext: BookContext
): string {
  const skill = skillLoader.getByAgent(agentName);
  
  if (!skill) {
    return basePrompt;
  }
  
  const sections: string[] = [basePrompt];
  
  // 添加系统提示增强
  if (skill.systemPromptEnhancement) {
    sections.push("\n---\n\n" + skill.systemPromptEnhancement);
  }
  
  // 添加约束
  if (skill.constraints) {
    sections.push(buildConstraintsSection(skill.constraints));
  }
  
  // 添加知识注入
  if (skill.knowledgeInjection) {
    const injectedKnowledge = buildKnowledgeInjection(
      skill.knowledgeInjection,
      bookContext
    );
    
    if (skill.knowledgeInjection.format === "prepend") {
      sections.unshift(injectedKnowledge);
    } else {
      sections.push(injectedKnowledge);
    }
  }
  
  return sections.join("\n");
}

function buildConstraintsSection(constraints: SkillDefinition["constraints"]): string {
  const parts: string[] = ["\n---\n\n## 行为约束\n"];
  
  if (constraints.mustDo.length > 0) {
    parts.push("\n### 必须执行\n");
    constraints.mustDo.forEach(item => {
      parts.push(`- ${item}\n`);
    });
  }
  
  if (constraints.mustNotDo.length > 0) {
    parts.push("\n### 禁止执行\n");
    constraints.mustNotDo.forEach(item => {
      parts.push(`- ${item}\n`);
    });
  }
  
  if (constraints.bestPractices.length > 0) {
    parts.push("\n### 最佳实践\n");
    constraints.bestPractices.forEach(item => {
      parts.push(`- ${item}\n`);
    });
  }
  
  return parts.join("");
}

async function buildKnowledgeInjection(
  config: SkillDefinition["knowledgeInjection"],
  bookContext: BookContext
): Promise<string> {
  // 从知识库获取相关内容
  // 实际实现会调用知识库 API
  return "";
}
```

---

## 6. Skills 与部署机 OpenCode 隔离

### 6.1 目录结构隔离

```
# OpenNovel Skills（本项目）
/opennovelv2/packages/agents/skills/
├── tian-dao/
│   └── SKILL.md
├── writer/
│   └── SKILL.md
├── world-guardian/
│   └── SKILL.md
└── ...

# 部署机 OpenCode Skills（系统级）
/root/.config/opencode/skills/
├── mcp-builder/
│   └── SKILL.md
├── claude-api/
│   └── SKILL.md
└── ...
```

### 6.2 配置隔离

```typescript
// opennovel.config.ts

export const OPENNOVEL_CONFIG = {
  // OpenNovel Skills 目录（项目级）
  skillsDir: "./packages/agents/skills",
  
  // 不使用系统级 Skills
  useSystemSkills: false,
  
  // Skills 加载配置
  skillLoader: {
    watchMode: true,           // 启用热重载
    validateOnLoad: true,      // 加载时验证
  },
};
```

### 6.3 运行时隔离

```typescript
// Skill Loader 只扫描 OpenNovel 目录

export class SkillLoader {
  // 使用 OpenNovel 专用目录
  private skillsDir: string;
  
  constructor(skillsDir?: string) {
    // 默认使用 OpenNovel 目录，不扫描系统目录
    this.skillsDir = skillsDir || path.join(__dirname, "../../skills");
  }
  
  // 明确不扫描 /root/.config/opencode/skills
  // 所有 Skills 加载都在 OpenNovel 项目范围内
}
```

---

## 7. Skill 热重载

### 7.1 启用热重载

```typescript
// 在应用启动时
import { skillLoader } from "./skills/skill-loader";

async function initialize() {
  // 加载所有 Skills
  await skillLoader.loadAll();
  
  // 启用热重载
  await skillLoader.enableWatchMode((skillName) => {
    console.log(`Skill ${skillName} reloaded`);
    
    // 通知相关 Agent 更新
    notifyAgentsOfSkillUpdate(skillName);
  });
}
```

### 7.2 WebSocket 通知

当 Skill 更新时，通过 WebSocket 通知正在运行的会话：

```typescript
function notifyAgentsOfSkillUpdate(skillName: string) {
  const skill = skillLoader.get(skillName);
  if (!skill) return;
  
  // 获取使用该 Skill 的活跃会话
  const activeSessions = getActiveSessionsForAgent(skill.agent);
  
  for (const session of activeSessions) {
    // 发送 WebSocket 通知
    wsServer.send(session.id, {
      type: "skill_updated",
      skillName,
      message: `技能 ${skillName} 已更新，下一轮对话将使用新配置。`,
    });
  }
}
```

---

## 8. 相关文档

- [01-overview.md](./01-overview.md) - Agent 系统概述
- [02-agent-definitions.md](./02-agent-definitions.md) - Agent 定义
- [03-hooks-system.md](./03-hooks-system.md) - Hooks 系统
- [04-tools-system.md](./04-tools-system.md) - Tools 系统
- [06-delegation-protocol.md](./06-delegation-protocol.md) - 委派协议