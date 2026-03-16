# Phase 7: Agent 系统 - 概述

**文档版本**: 1.0
**最后更新**: 2026-03-16

---

## 1. 目标

像素级模仿 **Oh My OpenCode** 的 Hooks/Tools/Skills 体系，为 OpenNovel 实现 8 个小说创作 Agent。

### 核心原则
- **Agent 行为通过工具链定义，而非简单 prompt**
- **Hooks 系统控制 Agent 生命周期**
- **Skills 系统定义 Agent 专业知识**
- **Delegation Protocol 规范 Agent 协作**

---

## 2. 与 Oh My OpenCode 的对应关系

### 2.1 Agent 映射

| OpenNovel Agent | Oh My OpenCode 对应 | 职责 |
|-----------------|---------------------|------|
| **天道** | Sisyphus | 主控 Agent，调度其他 Agent |
| **执笔** | Hephaestus | 自主写作，生成章节正文 |
| **世界观守护者** | Oracle | 只读检查，一致性验证 |
| **规划者** | Prometheus | 新书规划，世界观设计 |
| **审阅** | Momus | 审查章节，提出修改建议 |
| **观察者** | Atlas | 监控进度，管理待办 |
| **调研者** | Librarian | 外部参考，文学技巧查询 |
| **刘和平** | Metis | 风格专家，预规划咨询 |

### 2.2 架构映射

| Oh My OpenCode | OpenNovel | 说明 |
|----------------|-----------|------|
| 46 Hooks | 小说创作专用 Hooks | 阶段切换、一致性检查、写作惯性 |
| 26 Tools | 小说创作专用 Tools | 知识库检索、章节写入、批注添加 |
| 11 Agents | 8 Agents | 小说创作专用 Agent |
| Categories | Novel Categories | chapter-generation, outline-design 等 |
| Skills | Novel Skills | SKILL.md 定义 |

---

## 3. 整体架构

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        OpenNovel Agent System                            │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                     天道 (主控 Agent)                            │   │
│  │  - Intent Gate: 分析用户意图                                     │   │
│  │  - Delegation: 委派任务给其他 Agent                              │   │
│  │  - Session Continuity: 保持会话上下文                            │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                    │                                    │
│         ┌──────────────────────────┼──────────────────────────┐        │
│         │                          │                          │        │
│         ▼                          ▼                          ▼        │
│  ┌─────────────┐           ┌─────────────┐           ┌─────────────┐  │
│  │   执笔      │           │ 世界观守护者 │           │   规划者    │  │
│  │ (写作)      │           │ (一致性检查) │           │ (规划)      │  │
│  └─────────────┘           └─────────────┘           └─────────────┘  │
│         │                          │                          │        │
│         └──────────────────────────┼──────────────────────────┘        │
│                                    │                                    │
│         ┌──────────────────────────┼──────────────────────────┐        │
│         │                          │                          │        │
│         ▼                          ▼                          ▼        │
│  ┌─────────────┐           ┌─────────────┐           ┌─────────────┐  │
│  │   审阅      │           │   观察者    │           │   调研者    │  │
│  │ (审查)      │           │ (监控)      │           │ (研究)      │  │
│  └─────────────┘           └─────────────┘           └─────────────┘  │
│                                    │                                    │
│                                    ▼                                    │
│                            ┌─────────────┐                             │
│                            │   刘和平    │                             │
│                            │ (风格专家)  │                             │
│                            └─────────────┘                             │
│                                                                         │
├─────────────────────────────────────────────────────────────────────────┤
│                           Supporting Systems                            │
│                                                                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │
│  │   Hooks     │  │   Tools     │  │   Skills    │  │  Knowledge  │   │
│  │   System    │  │   System    │  │   System    │  │   Bases     │   │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 4. 目录结构

```
packages/agents/
├── src/
│   ├── index.ts                    # 主入口
│   │
│   ├── core/
│   │   ├── index.ts                # Core 模块入口
│   │   ├── agent-registry.ts       # Agent 注册中心
│   │   ├── agent-builder.ts        # Agent 工厂
│   │   ├── types.ts                # 核心类型定义
│   │   └── permission.ts           # 权限系统
│   │
│   ├── definitions/                # Agent 定义
│   │   ├── tian-dao.ts             # 天道
│   │   ├── writer.ts               # 执笔
│   │   ├── world-guardian.ts       # 世界观守护者
│   │   ├── planner.ts              # 规划者
│   │   ├── reviewer.ts             # 审阅
│   │   ├── observer.ts             # 观察者
│   │   ├── researcher.ts           # 调研者
│   │   └── liuheping.ts            # 刘和平
│   │
│   ├── hooks/                      # Hooks 系统
│   │   ├── index.ts                # Hooks 模块入口
│   │   ├── create-hooks.ts         # Hook 组合器
│   │   ├── types.ts                # Hook 类型定义
│   │   └── novel-hooks/            # 小说创作专用 Hooks
│   │       ├── phase-lock.ts       # 阶段锁定
│   │       ├── consistency-check.ts # 一致性检查
│   │       ├── writing-streak.ts   # 写作惯性提醒
│   │       ├── proactive-intervention.ts # 主动介入
│   │       └── chapter-completion.ts # 章节完成检查
│   │
│   ├── tools/                      # Tools 系统
│   │   ├── index.ts                # Tools 模块入口
│   │   ├── tool-registry.ts        # Tool 注册中心
│   │   ├── types.ts                # Tool 类型定义
│   │   └── novel-tools/            # 小说创作专用 Tools
│   │       ├── knowledge-search.ts # 知识库检索
│   │       ├── write-chapter.ts    # 章节写入
│   │       ├── add-annotation.ts   # 批注添加
│   │       ├── update-worldview.ts # 世界观更新
│   │       └── manage-foreshadowing.ts # 伏笔管理
│   │
│   ├── skills/                     # Skills 系统
│   │   ├── index.ts                # Skills 模块入口
│   │   ├── skill-loader.ts         # Skill 加载器
│   │   ├── skill-context.ts        # Skill 上下文
│   │   └── types.ts                # Skill 类型定义
│   │
│   └── delegation/                 # 委派协议
│       ├── index.ts                # Delegation 模块入口
│       ├── intent-gate.ts          # 意图门控
│       ├── protocol.ts             # 委派协议
│       ├── session-continuity.ts   # 会话连续性
│       └── types.ts                # Delegation 类型定义
│
├── skills/                         # SKILL.md 文件
│   ├── tian-dao/
│   │   └── SKILL.md
│   ├── writer/
│   │   └── SKILL.md
│   ├── world-guardian/
│   │   └── SKILL.md
│   └── ...
│
├── tests/
│   ├── definitions/
│   ├── hooks/
│   ├── tools/
│   └── skills/
│
├── package.json
└── tsconfig.json
```

---

## 5. 里程碑

### Milestone 1: 核心框架（Week 1）
- [ ] Agent Registry 实现
- [ ] Agent Builder 工厂模式
- [ ] 权限系统
- [ ] 类型定义

### Milestone 2: Agent 定义（Week 1-2）
- [ ] 8 个 Agent 的 Factory 实现
- [ ] AgentPromptMetadata 定义
- [ ] 动态 Prompt 构建
- [ ] Fallback Chain 集成

### Milestone 3: Hooks 系统（Week 2-3）
- [ ] Hook 组合器
- [ ] 小说创作专用 Hooks
- [ ] Hook 注册和执行

### Milestone 4: Tools 系统（Week 3）
- [ ] Tool Registry
- [ ] 小说创作专用 Tools
- [ ] Tool 权限控制

### Milestone 5: Skills 系统（Week 3-4）
- [ ] SKILL.md 定义
- [ ] Skill Loader
- [ ] 与部署机隔离

### Milestone 6: 委派协议（Week 4）
- [ ] Intent Gate
- [ ] Delegation Protocol
- [ ] Session Continuity

---

## 6. 依赖关系

### 6.1 内部依赖

```
packages/agents/
    │
    ├── @opennovel/llm (Phase 6)
    │   └── 模型调用、Fallback Chain
    │
    ├── @opennovel/core (Phase 0-5)
    │   └── Agent、Session、Message
    │
    └── @opennovel/knowledge (Phase 5)
        └── 知识库访问
```

### 6.2 外部依赖

```json
{
  "dependencies": {
    "zod": "^3.22.0",
    "yaml": "^2.3.0",
    "marked": "^12.0.0"
  }
}
```

---

## 7. 相关文档

- [02-agent-definitions.md](./02-agent-definitions.md) - 8个 Agent 定义
- [03-hooks-system.md](./03-hooks-system.md) - Hooks 系统
- [04-tools-system.md](./04-tools-system.md) - Tools 系统
- [05-skills-system.md](./05-skills-system.md) - Skills 系统
- [06-delegation-protocol.md](./06-delegation-protocol.md) - 委派协议