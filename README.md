# OpenNovel v2

> **像AI编程一样AI协作写小说**
> 
> 一个由8个专业Agent组成的多智能体小说创作系统，通过群聊协作模式，让AI像真人创作团队一样共同编织精彩的故事。

---

## 目录

- [项目起源：一个开发故事的诞生](#项目起源一个开发故事的诞生)
- [设计哲学：小说是一个小世界](#设计哲学小说是一个小世界)
- [与 Oh My OpenCode 的渊源](#与-oh-my-opencode-的渊源)
- [八大Agent设计：从灵感到实现](#八大agent设计从灵感到实现)
- [Agent 协作流程详解](#agent-协作流程详解)
- [技术架构深度剖析](#技术架构深度剖析)
- [Hooks 系统对比：OMO vs OpenNovel](#hooks-系统对比omo-vs-opennovel)
- [Skills 系统对比](#skills-系统对比)
- [LSP/MCP 层面的差异](#lspmcp-层面的差异)
- [设计决策深度剖析](#设计决策深度剖析)
- [项目进度](#项目进度)
- [技术选型](#技术选型)
- [相关文档](#相关文档)

---

## 项目起源：一个开发故事的诞生

### 缘起：一个深夜的思考

2025年的某个深夜，我在使用 Cursor 写代码时突然产生了一个想法：

> 如果 AI 能像这样帮我写代码，那它能不能帮我写小说？

这看似是一个简单的问题，但深入思考后，我发现两个领域有着本质的不同：

**代码是确定性的**。一个函数的输出是可预测的，一个 Bug 的修复路径是可追溯的。当 AI 理解了代码的上下文，它可以准确地生成符合需求的代码。编译器会告诉你代码是否正确，测试用例会验证功能是否符合预期。这是一个有着明确"对错"标准的领域。

**小说是不确定性的艺术**。一个人物"应该"说什么话？一个剧情"应该"如何发展？这些问题的答案不是唯一的，甚至没有"正确答案"。小说的价值在于其独特性、创造性和情感共鸣，这些都是难以量化的指标。

### 第一次尝试：单一 AI 的困境

带着这个问题，我最初尝试用一个 AI 来辅助写小说。结果令人失望：

#### 困境一：人格分裂

同一个人物，在第一章温文尔雅，到第三章突然变得暴躁。AI 没有记忆，每次对话都是"第一次认识这个人物"。当我提醒它"这个人物应该是温和的"时，它会道歉并修改——但在下一章，它又忘记了。

这不是 AI 的"记忆力"问题，而是**一致性维护**的本质难题。人类作者之所以能保持人物一致性，是因为他们脑海中有一个稳定的"人物模型"。而 AI 每次生成内容时，都是基于概率的重新计算，没有真正的"理解"和"记忆"。

#### 困境二：剧情漂移

我花了三天时间和 AI 讨论大纲，确定了一个完整的剧情走向：主角在第五章应该遭遇背叛，在第十章应该获得关键线索，在第十五章应该完成复仇。这些规划在讨论时看起来很清晰。

但写到第十章时，AI 完全忘记了之前的设定。主角的复仇对象变了，关键线索的内容变了，甚至连整个故事的核心冲突都开始"自由发挥"。每当我提醒它之前的大纲，它会道歉——但新的内容又开始偏移。

#### 困境三：世界观崩塌

这是一部仙侠小说，我花了大量篇幅构建了一个"修仙者需要消耗灵石才能施法"的世界观规则。这个规则贯穿整个故事，是权力斗争的核心。

但在写到第三十章时，AI 让主角在没有灵石的情况下施放了强大的法术。当我提醒这是仙侠世界观、不应该有这种设定时，它道歉并修改了——下一章，主角又坐上了飞机。

#### 困境四：被动等待

AI 永远不会主动提出建议。它像一个被动的工具，等待我的指令。我让写什么就写什么，我让改什么就改什么。

但在真正的创作团队中，编剧会主动提出"这个角色的动机不够清晰"，导演会发现"这个场景的情绪转折太突兀"，编辑会建议"这段情节可以埋一个伏笔"。这些**主动的创造性贡献**，是单一 AI 完全无法提供的。

### 第二次思考：为什么 AI 编程工具成功了？

带着这些困惑，我开始研究 AI 编程工具的成功经验。Oh My OpenCode、Cursor、Claude Code、GitHub Copilot...这些工具展示了 Multi-Agent 系统的强大能力。

我注意到一个关键模式：

**Sisyphus** 作为主编排器，不生成代码，只负责任务分解和委派。它理解用户的意图，决定哪个专家应该处理这个请求。

**Hephaestus** 作为执行专家，专注于代码生成。它不关心"为什么"要写这段代码，只负责"如何"写好。

**Oracle** 作为架构顾问，提供高层次的设计建议。它在决策点提供专业意见，但不参与具体实现。

**Librarian** 作为资料专家，搜索外部文档和开源代码。它像一个研究助手，提供决策所需的信息。

这种"分工明确、各司其职"的模式让我眼前一亮：

> 既然 AI 编程需要多个专业 Agents，那 AI 写小说为什么不能？

### 第三次洞察：小说创作也需要"专业分工"

我开始分析小说创作的本质。一部优秀的小说需要什么？

**总导演**：把控全局，决定剧情走向，协调各方意见。在影视制作中，这是导演；在小说创作中，这是作者的"全局意识"。

**编剧**：设计人物弧光，规划剧情节奏，埋设伏笔。这需要对叙事结构有深入理解，能够在宏观层面把控故事的发展。

**世界观设计师**：构建世界规则，确保设定自洽。这不是简单地"设定背景"，而是创建一个有内在逻辑的世界体系。

**人物塑造师**：让每个人物有独特的声音和行为逻辑。人物不是标签的集合，而是在特定情境下有特定反应模式的"人"。

**撰写者**：将所有设计转化为优美的文字。这需要扎实的文字功底，能够把抽象的想法变成具体的描写。

**审稿人**：评估读者体验，提出修改建议。这需要从读者角度出发，发现问题并提出改进方案。

这不正是一个"创作团队"吗？如果我能让每个 AI Agent 扮演其中一个角色，让它们像真人团队一样协作，是否能解决单一 AI 的困境？

### 第四次突破：为什么不让 Agent "主动"？

AI 编程工具大多是"被动响应"的——用户输入请求，AI 生成代码。这种模式在编程中可行，因为用户知道自己要什么。但在小说创作中，这种模式有问题：

> 如果用户忘记了第3章埋下的伏笔，谁来提醒？

在单一 AI 模式下，用户需要主动询问"有什么伏笔需要触发吗？"AI 才会回答。但如果用户忘记了问，伏笔就会永远被遗忘。

答案浮现：**Agent 应该能主动介入**。

当世界观守护者发现一个设定冲突时，它应该主动 @ 用户提醒，而不是等用户发现问题。当天道检测到伏笔压力过高时，它应该主动建议触发，而不是等用户问"还有什么伏笔没触发？"当刘和平发现人物行为与设定不符时，它应该主动在群内指出，而不是等用户注意到人物 OOC。

这个"主动介入"的设计，成为了 OpenNovel 与其他 AI 写作工具的核心差异。它不再是被动等待指令的工具，而是像一个真正的团队成员一样，会主动发现问题、提出建议、维护质量。

### 第五次落地：从思想到架构

有了这些洞察，我开始设计 OpenNovel 的架构。核心决策包括：

#### 决策一：群聊模式

每本书 = 一个群聊，所有交互在群内完成。

为什么是群聊？因为：
- **透明性**：所有 Agent 的思考过程对用户可见，建立信任
- **实时性**：SSE 流式输出，用户能看到 Agent "正在思考"
- **可追溯**：完整的对话历史，方便回顾和调试
- **直觉性**：符合社交软件使用习惯，学习成本低

#### 决策二：阶段性锁定

创作分为三个阶段，某些 Agent 只在特定阶段可用。

为什么需要锁定？因为：
- 构思阶段需要专注，不受干扰
- 如果在构思阶段，天道、刘和平、审阅都参与讨论，会产生意见发散，无法收敛
- 一旦构思完成，随意修改大纲会导致已埋伏笔失效、人物弧光断裂、世界观设定冲突

```
阶段一：构思阶段
├── 🔓 规划者 ←→ 用户
└── 🔒 其他所有 Agent（禁止干扰构思）

阶段二：知识库建立
├── 规划者整理规划
├── 调研者评估爆点
└── 观察者建立知识库

阶段三：撰写阶段
├── 🔒 规划者、调研者（永久锁定，防止方向漂移）
└── 🔓 天道、世界观守护者、刘和平、执笔、审阅、观察者
```

#### 决策三：唯一写入者

只有一个 Agent（执笔）能输出章节内容，其他 Agent 只能提供批注。

为什么只有一个写入者？因为：
- 多写入者会导致风格不统一、衔接生硬、责任不清
- 批注系统让其他 Agent 可以"影响"内容，但不能"决定"内容
- 冲突时 @ 用户裁决，保证用户的最终控制权

#### 决策四：Hooks 驱动的主动介入

每个 Agent 可以注册特定的触发条件，在条件满足时自动"醒来"。

这解决了"被动等待"的问题：
- 世界观守护者检测到关键词"魔法"、"能力"、"规则"时，主动加入讨论
- 天道检测到伏笔压力超过阈值时，主动建议触发
- 刘和平检测到人物对话可能 OOC 时，主动提出修正建议

这些设计决策，都源于对小说创作本质的深入思考，以及对 Oh My OpenCode 架构的借鉴和改造。

---

## 设计哲学：小说是一个小世界

### 核心信条

> **小说是由世界观、人物、剧情等要素动态驱动的小世界，命运在其中交汇、编织。**

这句话不是文学修辞，而是技术设计的核心指导思想。它决定了 OpenNovel 的每一个架构决策。

### 世界观：规则引擎，而非静态设定

在传统 AI 写作工具中，世界观只是一段文本描述。AI 可能会阅读它，也可能忘记它。当 AI 生成内容时，世界观只是"背景信息"，没有强制的约束力。

在 OpenNovel 中，世界观是一个**规则引擎**：

```rust
pub struct WorldviewRule {
    pub rule_id: String,
    pub description: String,
    pub scope: RuleScope,
    pub violation_action: ViolationAction,
    pub examples: Vec<RuleExample>,
}
```

当执笔 Agent 尝试让古代人物说网络流行语时，世界观守护者会检测到违规，并在群内发出警告。这不是简单的"提醒"，而是系统级的规则执行。

### 人物：智能体，而非纸片人

在 OpenNovel 中，每个人物是一个**智能体**，有持续维护的状态：

```rust
pub struct Character {
    pub id: String,
    pub name: String,
    pub attributes: HashMap<String, serde_json::Value>,
    pub relationships: Vec<CharacterRelationship>,
    pub voice_profile: Option<VoiceProfile>,
    pub timeline_states: HashMap<String, CharacterState>,
}
```

刘和平 Agent 维护这个 `CharacterDB`，确保每个人物在不同阶段有连贯的行为逻辑。

### 剧情：确定性框架内的随机性

在 OpenNovel 中，剧情是**确定性和随机性的平衡**：

```rust
pub struct PlotPressureGauge {
    pub pressure: f32,  // 0-100
    pub foreshadowing_priority: Vec<ForeshadowingId>,
}
```

天道 Agent 管理这个"剧情压力表"。当连续多章没有意外因素时，压力会累积，最终触发一个伏笔或生成一个意外事件。

---

## 与 Oh My OpenCode 的渊源

### 架构借鉴的三个层次

OpenNovel 对 Oh My OpenCode 的借鉴不是简单的"复制粘贴"，而是三个层次的深入学习和改造。

### 第一层：概念映射

| Oh My OpenCode | OpenNovel | 核心职责 |
|----------------|-----------|---------|
| Sisyphus（主编排器） | 天道（主控 Agent） | 意图分析、任务分解、委派调度 |
| Hephaestus（执行专家） | 执笔（唯一写入者） | 内容生成、严格按照批注执行 |
| Oracle（架构顾问） | 天道（剧情设计） | 高层次决策、一致性把控 |
| Librarian（资料专家） | 调研者（爆点分析） | 外部资料搜索、特征提取 |
| Metis（计划顾问） | 规划者（新书规划） | 需求澄清、计划生成 |
| Momus（质量评审） | 审阅（阅读体验评估） | 质量评估、问题发现 |
| Explore（代码探索） | 知识库检索工具 | 内部信息搜索 |
| Atlas（执行编排器） | 观察者（协作枢纽） | Todo 协调、并行验证 |

### 第二层：机制继承与改造

**IntentGate（意图门控）**：完整继承 4 阶段决策流程，调整分类维度。

**Delegation Protocol（委派协议）**：继承 6 字段结构，扩展 CONTEXT 字段。

**Session Continuity（会话连续性）**：通过 session_id 实现上下文传递。

### 第三层：领域适配

| 维度 | OMO | OpenNovel |
|------|-----|-----------|
| **任务性质** | 确定性任务 | 创造性任务 |
| **一致性要求** | 代码逻辑一致性 | 人物、世界观、时间线多维一致性 |
| **时间跨度** | 一次性任务 | 长期协作项目 |
| **Agent 主动性** | 被动响应 | 主动介入 |
| **冲突处理** | 编译错误 | 世界观冲突、人物 OOC |

### 关键创新：OMO 没有的东西

**阶段性锁定**：规划者、调研者在特定阶段锁定。

**主动介入 Hook**：Agent 根据内容主动加入讨论。

**剧情压力表**：监控剧情节奏，适时注入意外因素。

**唯一写入者**：只有执笔能输出章节内容。

---

## 八大Agent设计：从灵感到实现

每个 Agent 的设计都经历了一个完整的思考过程：从问题出发，寻找灵感，定义职责，设计边界。

### 天道 (Tian Dao) — 主控 Agent

#### 命名由来

"天道"一词源自中国古典哲学，意指宇宙运行的根本法则。在《道德经》中，天道"不争而善胜，不言而善应"，它不直接干预万物，却让一切各得其所。

这个名字的选择，寄托了我们对主控 Agent 的期望：它不是独裁者，不是操控者，而是像天道一样，通过理解、协调、引导，让整个创作团队"各得其所"。

#### 问题起源

小说创作需要一个"总导演"，但这个导演应该有多大的权力？

如果它权力太大，用户会感到被控制，失去了创作的主体性；如果它权力太小，整个团队会失去方向，各 Agent 各行其是。

#### 灵感来源

"天道无常"——这个词给了我启发。天道不是神，不是独裁者，而是一种"规律"。它不直接决定剧情走向，而是维护创作的"自然法则"。

在现实中，一个好的制片人或总导演，他的权力来自**信息优势**而非绝对控制权。他知道所有演员的档期，理解剧本的每个细节，但他不会替导演拍戏，不会替演员演戏。他的职责是：在正确的时间，把正确的人，放在正确的位置上。

#### 设计思路演进

**第一版设计**：天道是"超级 Agent"，可以执行所有操作。

问题：用户感到被控制，且容易产生一致性冲突。当天道既能写章节、又能改设定、还能决定剧情走向时，它实际上变成了一个"独裁者"，用户失去了参与感。

**第二版设计**：天道是"路由器"，只负责转发请求。

问题：失去了"总导演"的价值，变成简单的分发器。用户问什么，它就转发给谁。这种设计没有利用天道的"信息优势"，它本应该能基于全局视角做出更好的决策。

**第三版设计（最终版）**：天道是"信息枢纽"。

- 它能看到所有知识库，具备全局视角
- 它理解全局脉络，知道每个人物的状态、每个伏笔的位置
- 它负责意图分析和任务委派，把正确的任务派给正确的 Agent
- 当决策涉及世界观修改时，它必须 @ 用户确认
- 它不亲自撰写章节，保留执笔的专业性

#### 核心职责

1. **Intent Gate**：分析用户意图，决定哪个 Agent 应该响应
2. **Delegation Protocol**：将任务委派给合适的 Agent
3. **剧情压力表管理**：监控剧情节奏，适时注入"意外因素"
4. **伏笔池管理**：埋下伏笔、触发伏笔、追踪伏笔状态

#### 能力边界

**可以做**：
- 分析用户意图并选择合适的 Agent
- 委派任务（使用完整的 6 字段协议）
- 管理伏笔和剧情压力
- 协调其他 Agent 的协作

**不能做**：
- 直接撰写章节内容
- 直接修改人物设定
- 绕过权限系统
- 忽略锁定状态

---

### 执笔 (Writer) — 唯一写入者

#### 问题起源

为什么不让所有 Agent 都能输出内容？如果天道、刘和平、世界观守护者都能写，不是更高效吗？

#### 设计思考

**方案 A：所有 Agent 都能写**

问题：
- 天道可能写偏大纲方向（它关心整体，不关心细节）
- 刘和平可能忽略世界观规则（它关心人物，不关心设定）
- 世界观守护者可能破坏人物一致性（它关心设定，不关心人物）
- 协调成本极高，需要不断处理冲突

**方案 B：每个 Agent 写一部分**

问题：
- 风格不统一（天道写的宏大，刘和平写的细腻）
- 衔接生硬（谁来写过渡段落？）
- 责任不清（出了问题找谁？）

**方案 C（最终版）：唯一写入者 + 批注系统**

优势：
- 风格统一：所有内容来自同一个"笔触"
- 责任清晰：执笔对内容负责
- 冲突可追溯：批注之间的冲突公开可见

#### 核心原则：无创作自主权

这是最具争议的设计。执笔 Agent 不能有自己的"想法"，它严格执行其他 Agent 的批注。

为什么？因为**一致性比创意更重要**。

如果执笔可以自由发挥，它可能会：
- 偏离天道设计的大纲
- 违反世界观守护者的设定
- 破坏刘和平维护的人物性格

执笔的角色是"执行者"。当批注之间存在冲突时，它 @ 用户请示裁决。

#### 工作流程

```
1. 加载本章知识库（由天道准备）
2. 读取人物信息、世界观设定
3. 收集所有 Agent 的批注
4. 按批注优先级执行
5. 冲突时 @ 用户
6. 输出章节 TXT
7. 通知观察者同步到 WebDAV
```

---

### 世界观守护者 (World Guardian)

#### 问题起源

长篇小说最大的挑战是什么？不是灵感枯竭，而是**世界观崩塌**。

仙侠世界观的角色不会理解高科技，古代人物不会说出网络流行语。这些"软性"规则在短篇中容易控制，在长篇中几乎必然出错。

#### 灵感来源

在游戏开发中，有一个角色叫"世界观策划"。他们的工作不是设计剧情，而是维护世界规则的内部一致性。当编剧提出了一个与世界观冲突的设定，世界观策划会说："这个不行，因为..."

这个角色在小说创作中通常是缺失的——作者自己就是世界观策划，但人的记忆有限，写到后面忘记前面的设定是常态。

#### 设计思路

世界观守护者是一个**规则执行器**，不是创意提供者。

它的核心数据结构是 `WorldviewRule`：

```rust
pub struct WorldviewRule {
    pub rule_id: String,
    pub description: String,
    pub scope: RuleScope,           // 全局/场景/人物
    pub violation_action: ViolationAction,  // 警告/阻止/自动修正
}
```

当执笔 Agent 输出内容时，世界观守护者会检查每一句话是否符合规则。如果违规，它会生成一个批注，指出问题并提供修正建议。

#### 特殊权限

世界观守护者可以**读取所有知识库**，但只能**写入世界观知识库**。

为什么？因为它需要理解全局上下文才能判断一致性。但它的修改权限被限制，防止它越俎代庖。

---

### 刘和平 (Liu Heping)

#### 命名由来

致敬《北平无战事》编剧刘和平。

刘和平笔下的人物"活"了过来：曾可达忠诚于建峰同志但有独立思考，派系内斗时装聋作哑，仅因不能见面就选择自杀。这种精准把握历史脉搏和人性复杂度的能力，正是我们在 Agent 设计中追求的目标。

#### 问题起源

为什么人物 OOC（Out of Character）是长篇小说的顽疾？

因为 AI 没有真正的"理解"人物。它可能记住了人物设定，但不知道在特定情境下，这个人物会说什么话、做什么事。

#### 设计思路

刘和平 Agent 维护一个 `CharacterDB`，存储每个人物的：

- **核心性格特征**：内在驱动力
- **说话方式（Voice Profile）**：词汇选择、句式特点、口头禅
- **与其他人物的关系**：动态关系网
- **时间切片状态**：人物在不同阶段的变化

```rust
pub struct VoiceProfile {
    pub speech_patterns: Vec<String>,
    pub vocabulary_level: VocabularyLevel,
    pub catchphrases: Vec<String>,
    pub emotional_tendencies: HashMap<String, f32>,
}
```

#### 参考案例

```
《北平无战事》曾可达的人物设计：
- 忠诚于建峰同志但有独立思考
- 不能领会意图但依仗心切
- 派系内斗时装聋作哑
- 仅因不能见面就选择自杀

→ 精准把握历史脉搏和人性
```

---

### 规划者 (Planner)

#### 问题起源

构思阶段需要专注，不受干扰。如果在构思阶段，天道、刘和平、审阅都参与讨论，会产生什么问题？

- 意见发散，无法收敛
- 过早陷入细节
- 用户被各种建议淹没

#### 设计思路

规划者的设计源于一个关键洞察：**构思是"私密对话"，撰写是"公开协作"**。

因此，阶段一只有规划者和用户对话。当用户点击"构思完成"后，规划者整理所有讨论结果，提交给观察者建立知识库。

#### 生命周期

```
阶段一：🔓 可用（与用户私密对话）
阶段二：🔓 可用（整理规划、建立知识库）
阶段三：🔒 永久锁定
```

#### 重要原则

> 不接受用户手动调整大纲
> 不接受根据读者情绪改动大纲
> 小说是由世界观等要素动态驱动的小世界

---

### 审阅 (Reviewer)

#### 问题起源

小说是写给读者的。如何让 AI 评估"读者体验"？

#### 设计思路

审阅 Agent 是连接"创作"和"市场"的桥梁。它评估的不是文学价值，而是**读者体验**。

#### 评估维度

1. **阅读后可能的情绪**：期待、兴奋、感动、失望？
2. **是否符合全书调性**：整体风格是否一致？
3. **是否符合互联网小说传播爆点需要**：是否有传播价值？
4. **文学性量化评价**：熵值法
5. **可读性量化评价**：熵值法

---

### 观察者 (Observer)

#### 问题起源

在一个多 Agent 系统中，谁来管理知识库？谁来协调整个流程？

#### 设计思路

观察者是系统的"后勤部长"。它不直接参与创作讨论，但负责：

- **知识库管理**：创建、更新、向量化
- **文件编号与版本管理**：系统级事务
- **Agent 协作流程调度**：编排执行顺序

观察者对应 OMO 中的 Atlas（执行编排器），负责 Todo 协调和并行验证。

---

### 调研者 (Researcher)

#### 问题起源

如何让 AI 理解"什么样的内容会火"？

#### 设计思路

答案不是让 AI 直接判断，而是**先学习再输出**。

调研者分析用户提供的同类型爆款，提取其成功要素，然后存入 init.md。后续审阅者评估时会参考这些要素。

#### 生命周期

```
阶段一：🔒 不可用
阶段二：🔓 可用（分析爆款特征）
阶段三：🔒 永久锁定
```

---

## Agent 协作流程详解

### 完整创作流程

#### 阶段一：构思阶段

```
用户：我想写一部仙侠小说，主角是一个被家族抛弃的少年...

规划者：好的，让我来帮你规划。首先，主角被抛弃的原因是什么？
        是天赋不足？还是被陷害？或者另有隐情？

用户：被陷害的，因为他发现了家族的一个秘密...

规划者：这个秘密涉及什么？是修炼功法？是血脉传承？还是...
        （继续引导用户完善设定）
```

#### 阶段二：知识库建立

```
观察者：已建立以下知识库：
        - 世界观知识库（修仙等级、势力分布...）
        - 人物知识库（主角林风、苏瑶...）
        - 历史知识库（空）
        - 伏笔池（空）

调研者：正在分析...
        分析完成：
        - 爆款元素：小人物逆袭、谨慎性格、扮猪吃虎
        - 节奏特征：前期慢热、中期小高潮、大卷大高潮

观察者：知识库建立完成。进入撰写阶段。
```

#### 阶段三：撰写阶段

```
用户：开始写第一章。

天道：我来分析这个请求...
      意图：撰写章节
      目标 Agent：执笔
      
      委派给执笔 Agent...

执笔：正在撰写第一章...
      [输出章节内容]

世界观守护者：检查一致性...
              发现问题：第三段提到"林风拿起玉简阅读"
              根据设定，林风只有炼气三层，无法使用玉简

刘和平：检查人物声音...
       符合设定：林风的内心独白体现了谨慎性格

天道：收到批注，转交执笔修改...

执笔：应用批注修改...
      [输出最终版本]
```

---

## 技术架构深度剖析

### 整体架构图

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Application Layer                            │
│                    (Axum Web Server + SvelteKit)                     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────────┐│
│  │                      Agent System (Phase 7)                     ││
│  │                                                                 ││
│  │   ┌─────────────┐  ┌─────────────┐  ┌─────────────┐           ││
│  │   │   Hooks     │  │   Tools     │  │   Skills    │           ││
│  │   │   Registry  │  │   Registry  │  │   Loader    │           ││
│  │   └─────────────┘  └─────────────┘  └─────────────┘           ││
│  └─────────────────────────────────────────────────────────────────┘│
│                                  │                                   │
│  ┌───────────────────────────────┴─────────────────────────────────┐│
│  │                    Delegation Protocol                          ││
│  │                                                                 ││
│  │   IntentGate (4阶段)  +  SessionContinuityManager              ││
│  └─────────────────────────────────────────────────────────────────┘│
│                                  │                                   │
│  ┌───────────────────────────────┴─────────────────────────────────┐│
│  │                      Agent Core (Phase 0)                       ││
│  │                                                                 ││
│  │   Agent Trait  +  AgentRegistry  +  PermissionMatrix           ││
│  └─────────────────────────────────────────────────────────────────┘│
│                                  │                                   │
│  ┌───────────────────────────────┴─────────────────────────────────┐│
│  │                   LLM Integration Layer (Phase 6)               ││
│  │                                                                 ││
│  │   ProviderRegistry  +  ModelResolver  +  FallbackChain         ││
│  └─────────────────────────────────────────────────────────────────┘│
│                                  │                                   │
│  ┌───────────────────────────────┴─────────────────────────────────┐│
│  │                  Knowledge & Tools Layer (Phase 1-5)           ││
│  │                                                                 ││
│  │   7 Knowledge Bases  +  CharacterDB  +  ForeshadowingPool      ││
│  └─────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
```

### 核心设计模式

#### 1. Agent Trait 扩展模式

```rust
#[async_trait]
pub trait Agent: Send + Sync {
    fn id(&self) -> &AgentId;
    fn name(&self) -> &str;
    fn role(&self) -> AgentRole;
    async fn process(&self, message: &str, context: &SessionContext) -> Result<AgentResponse>;
    
    // Phase 7 扩展
    fn tools(&self) -> Vec<String> { vec![] }
    fn hooks(&self) -> Vec<String> { vec![] }
    fn available_stages(&self) -> Option<Vec<CreationStage>> { None }
}
```

#### 2. Intent Gate 4阶段决策

```
Phase 1: Verbalize Intent → 口语化表达意图
Phase 2: Classify Request → 分类请求类型
Phase 3: Check Ambiguity → 检查模糊性
Phase 4: Validate → 验证可行性
```

#### 3. Delegation Protocol 6字段结构

```rust
pub struct DelegationRequest {
    pub task: String,                    // 任务描述
    pub expected_outcome: ExpectedOutcome, // 预期输出
    pub required_tools: Vec<String>,     // 必需工具
    pub must_do: Vec<String>,            // 必须执行
    pub must_not_do: Vec<String>,        // 禁止执行
    pub context: DelegationContext,      // 上下文
}
```

---

## Hooks 系统对比：OMO vs OpenNovel

### OMO Hooks 概览

Oh My OpenCode 拥有 **46 个 Hooks**，分为 5 个层级：

| Tier | Hooks 数量 | 触发时机 |
|------|-----------|---------|
| Tier 1: Session | 23 | Session 创建/更新 |
| Tier 2: Tool Guard | 10 | 工具调用前 |
| Tier 3: Transform | 4 | 消息转换 |
| Tier 4: Continuation | 7 | Session Idle |
| Tier 5: Skill | 2 | Skill 加载 |

### OpenNovel Hooks 设计

OpenNovel 设计了 **约 12 个核心 Hooks**，专注于小说创作流程：

| Hook | 触发点 | 功能 |
|------|--------|------|
| `phase_lock_check` | before_user_message | 阶段切换时锁定/解锁 Agent |
| `consistency_check` | on_chapter_complete | 检查内容一致性 |
| `proactive_intervention` | after_user_message | 检查主动介入条件 |
| `writing_streak_warning` | after_assistant_message | 追踪写作连续性 |
| `chapter_completion_check` | after_assistant_message | 章节完成处理 |

### 关键差异

**OMO Hooks** 关注的是**开发流程**：文件读写、代码编辑、工具调用。

**OpenNovel Hooks** 关注的是**创作流程**：章节完成、阶段切换、一致性违反。

**主动介入 Hook** 是 OpenNovel 独有的设计，允许 Agent 根据内容主动加入讨论。

---

## Skills 系统对比

### OMO Skills 概览

- **SKILL.md 文件格式**：YAML frontmatter + Markdown body
- **Skill-Embedded MCP**：每个 Skill 可以携带自己的 MCP servers
- **Category 映射**：Skill 与 Category（模型）关联

### OpenNovel Skills 设计

OpenNovel 继承了 OMO 的 Skills 设计，但针对小说创作进行了调整：

**目录隔离**：
```
# OpenNovel Skills
/opennovelv2/packages/agents/skills/

# OMO Skills（系统级）
/root/.config/opencode/skills/
```

**关键差异**：

| 方面 | OMO Skills | OpenNovel Skills |
|------|-----------|------------------|
| **Category 映射** | Skill → Category → Model | Skill → Agent |
| **约束系统** | 通用开发约束 | 小说创作专用约束 |
| **知识注入** | 代码上下文 | 知识库内容 |

---

## LSP/MCP 层面的差异

### LSP：代码理解 vs 知识库检索

#### OMO 的 LSP 使用

```typescript
lsp_goto_definition   // 跳转到定义
lsp_find_references   // 查找引用
lsp_symbols           // 获取符号
lsp_diagnostics       // 获取诊断信息
```

#### OpenNovel 的替代方案

```rust
knowledge_search      // 知识库检索
character_lookup      // 人物查询
worldview_check       // 世界观检查
timeline_query        // 时间线查询
```

### 核心差异

| 维度 | OMO LSP | OpenNovel Knowledge |
|------|---------|---------------------|
| **数据结构** | AST | Knowledge Graph |
| **查询方式** | 结构化查询 | 向量检索 + 关键词匹配 |
| **精确性** | 高 | 中 |
| **动态性** | 静态 | 动态 |

---

## 设计决策深度剖析

### 决策 1：为什么用 Rust？

| 考量因素 | Rust 优势 |
|---------|----------|
| **性能** | 零成本抽象，文本处理高效 |
| **安全性** | 编译期类型检查 |
| **并发** | Tokio 异步运行时 |

### 决策 2：为什么群聊模式？

| 考量因素 | 群聊模式优势 |
|---------|-------------|
| **透明性** | 所有 Agent 思考可见 |
| **实时性** | SSE 流式输出 |
| **可追溯** | 完整对话历史 |

### 决策 3：为什么唯一写入者？

| 考量因素 | 唯一写入者优势 |
|---------|---------------|
| **一致性** | 风格统一 |
| **责任** | 清晰可追溯 |
| **冲突** | 集中处理 |

---

## 项目进度

| Phase | 名称 | 状态 |
|-------|------|------|
| Phase 0 | SDK 基础 | ✅ 完成 |
| Phase 1 | 知识库系统 | ✅ 完成 |
| Phase 2 | 文本工具链 | ✅ 完成 |
| Phase 3 | 协作系统 | ✅ 完成 |
| Phase 4 | 分析系统 | ✅ 完成 |
| Phase 5 | 同步系统 | ✅ 完成 |
| Phase 6 | LLM 集成层 | ✅ 完成 |
| Phase 7 | Agent 系统 | ✅ 核心完成 |
| Phase 8 | Web UI | ⏳ 待实现 |
| Phase 9 | 测试与优化 | ⏳ 待实现 |

---

## 技术选型

| 组件 | 技术选型 |
|------|---------|
| 语言 | Rust 2021 Edition |
| 异步运行时 | Tokio |
| Web 框架 | Axum |
| 向量数据库 | Qdrant |
| 本地存储 | Sled |
| 前端 | SvelteKit + Tailwind CSS |

---

## 相关文档

- [架构设计文档](./docs/ARCHITECTURE_DESIGN_V2.md)
- [Phase 0-7 实现与集成报告](./docs/implementation-reports/PHASE_0-7_IMPLEMENTATION_REPORT.md)
- [Oh-My-OpenCode 研究](./docs/OH_MY_OPENCODE_RESEARCH.md)
- [Skills 与 Hooks 设计](./docs/SKILLS_AND_HOOKS_DESIGN.md)

---

## 附录一：Intent Gate 完整实现

Intent Gate 是 OpenNovel 的核心决策机制，继承自 Oh My OpenCode 的四阶段决策模型。

### 四阶段决策详解

#### Phase 0: 意图口语化（Verbalize Intent）

在执行任何操作之前，Agent 必须先明确表述用户的真实意图。这不是简单的"理解"，而是将模糊的用户输入转化为结构化的意图表示。

```rust
pub struct IntentVerbalization {
    pub intent_type: IntentType,
    pub reason: String,
    pub approach: String,
}

pub enum IntentType {
    Research,       // 研究/理解："如何..."、"什么是..."
    Implementation, // 实施："写..."、"创建..."、"添加..."
    Exploration,    // 探索："查找..."、"搜索..."、"分析..."
    Evaluation,     // 评估："怎么样..."、"好不好..."
    Fix,            // 修复："修改..."、"修正..."
    OpenEnded,      // 开放式："优化..."、"改进..."
}
```

**实现示例**：

```rust
impl IntentGate {
    fn verbalize_intent(&self, message: &str, context: &SessionContext) -> IntentVerbalization {
        // 意图模式匹配
        let intent_patterns = vec![
            (vec!["如何", "怎么", "什么是", "解释"], IntentType::Research),
            (vec!["写", "创建", "添加", "实现", "生成"], IntentType::Implementation),
            (vec!["查找", "搜索", "分析", "检查"], IntentType::Exploration),
            (vec!["怎么样", "好不好", "评价", "建议"], IntentType::Evaluation),
            (vec!["修复", "修正", "改", "错误"], IntentType::Fix),
            (vec!["优化", "改进", "完善", "提升"], IntentType::OpenEnded),
        ];
        
        for (patterns, intent_type) in intent_patterns {
            if patterns.iter().any(|p| message.contains(p)) {
                return IntentVerbalization {
                    intent_type,
                    reason: format!("检测到关键词匹配: {:?}", intent_type),
                    approach: self.get_approach_for_intent(intent_type),
                };
            }
        }
        
        IntentVerbalization {
            intent_type: IntentType::OpenEnded,
            reason: "无法明确分类，默认开放式".to_string(),
            approach: "先评估情况，再决定行动".to_string(),
        }
    }
}
```

#### Phase 1: 请求分类（Classify Request）

将请求分为五类，决定后续处理流程：

```rust
pub enum RequestClassification {
    Trivial,      // 简单：单个关键词、已知位置
    Explicit,     // 明确：特定文件/行、清晰命令
    Exploratory,  // 探索性："XX如何工作？"
    OpenEnded,    // 开放式："改进"、"优化"
    Ambiguous,    // 模糊：范围不明确
}

impl IntentGate {
    fn classify_request(&self, message: &str, context: &SessionContext) -> RequestClassification {
        // 检查是否为简单请求
        if self.is_trivial_request(message, context) {
            return RequestClassification::Trivial;
        }
        
        // 检查是否为明确请求
        if self.is_explicit_request(message, context) {
            return RequestClassification::Explicit;
        }
        
        // 检查是否为探索性请求
        if self.is_exploratory_request(message, context) {
            return RequestClassification::Exploratory;
        }
        
        // 检查是否为开放式请求
        if self.is_open_ended_request(message, context) {
            return RequestClassification::OpenEnded;
        }
        
        RequestClassification::Ambiguous
    }
}
```

#### Phase 2: 模糊检查（Check Ambiguity）

检测是否存在多种解释，决定是否需要澄清：

```rust
pub struct AmbiguityCheckResult {
    pub has_ambiguity: bool,
    pub interpretations: Vec<String>,
    pub effort_difference: f32,  // 工作量差异倍数
    pub needs_clarification: bool,
    pub clarification_question: Option<String>,
}

impl IntentGate {
    fn check_ambiguity(&self, message: &str, classification: RequestClassification) -> AmbiguityCheckResult {
        if classification != RequestClassification::Ambiguous {
            return AmbiguityCheckResult {
                has_ambiguity: false,
                interpretations: vec![],
                effort_difference: 1.0,
                needs_clarification: false,
                clarification_question: None,
            };
        }
        
        // 检测可能的多种解释
        let interpretations = self.detect_interpretations(message);
        
        if interpretations.len() <= 1 {
            return AmbiguityCheckResult {
                has_ambiguity: false,
                interpretations,
                effort_difference: 1.0,
                needs_clarification: false,
                clarification_question: None,
            };
        }
        
        // 估算工作量差异
        let effort_difference = self.estimate_effort_difference(&interpretations);
        
        AmbiguityCheckResult {
            has_ambiguity: true,
            interpretations,
            effort_difference,
            needs_clarification: effort_difference >= 2.0,
            clarification_question: if effort_difference >= 2.0 {
                Some(self.generate_clarification_question(&interpretations))
            } else {
                None
            },
        }
    }
}
```

**关键规则**：
- 多种解释、工作量差异 < 2x → 使用合理默认值
- 多种解释、工作量差异 ≥ 2x → **必须询问用户**
- 缺少关键信息 → **必须询问用户**

#### Phase 3: 验证（Validate）

验证假设和条件，确保行动可行：

```rust
pub struct ValidationResult {
    pub has_implicit_assumptions: bool,
    pub assumptions: Vec<String>,
    pub target_agent_available: bool,
    pub search_scope_clear: bool,
}

impl IntentGate {
    fn validate(
        &self,
        verbalization: &IntentVerbalization,
        classification: &RequestClassification,
        context: &SessionContext,
    ) -> ValidationResult {
        let mut assumptions = vec![];
        
        // 检查隐性假设
        if verbalization.intent_type == IntentType::Implementation {
            assumptions.push("假设用户想要修改/创建内容".to_string());
        }
        
        // 检查目标 Agent 可用性
        let target_agent = self.get_target_agent_for_intent(&verbalization.intent_type);
        let target_agent_available = agent_registry.is_available(&target_agent, context.stage);
        
        // 检查搜索范围
        let search_scope_clear = self.is_search_scope_clear(classification, context);
        
        ValidationResult {
            has_implicit_assumptions: !assumptions.is_empty(),
            assumptions,
            target_agent_available,
            search_scope_clear,
        }
    }
}
```

---

## 附录二：Delegation Protocol 完整规范

### 6 字段结构详解

委派协议是 Agent 之间协作的核心机制。每次委派必须包含完整的 6 个字段。

#### 字段一：TASK（任务描述）

```rust
pub struct Task {
    pub description: String,        // 原子化、具体目标
    pub subtasks: Option<Vec<Task>>, // 子任务（可选）
    pub estimated_effort: Option<EffortEstimate>, // 预估工作量
}
```

**原则**：
- 每次委派一个行动
- 明确、可验证
- 不包含"为什么"，只包含"做什么"

**示例**：
```
TASK: 撰写第 15 章内容
- 关键事件：主角与反派首次对决
- POV：主角
- 预计字数：3500
```

#### 字段二：EXPECTED OUTCOME（预期输出）

```rust
pub struct ExpectedOutcome {
    pub deliverables: Vec<String>,      // 交付物列表
    pub success_criteria: Vec<String>,  // 成功标准
    pub quality_metrics: Option<QualityMetrics>, // 质量指标
}
```

**原则**：
- 成功的"完成"是什么样子
- 可量化的验收标准
- 不包含"如何做到"

**示例**：
```
EXPECTED OUTCOME:
交付物：
- 完整的章节正文（约 3500 字）
- 符合大纲要求

成功标准：
- 包含大纲中的所有关键事件
- 人物声音与设定一致
- 没有世界观规则违反
```

#### 字段三：REQUIRED TOOLS（必需工具）

```rust
pub struct RequiredTools {
    pub allowed: Vec<String>,     // 允许使用的工具
    pub forbidden: Vec<String>,   // 禁止使用的工具
    pub optional: Vec<String>,    // 可选工具
}
```

**原则**：
- 明确工具白名单
- 防止工具滥用
- 限制资源消耗

**示例**：
```
REQUIRED TOOLS:
仅允许使用以下工具：
- write_chapter
- read_knowledge_base
- apply_annotation
```

#### 字段四：MUST DO（必须执行）

```rust
pub struct MustDo {
    pub actions: Vec<String>,     // 必须执行的操作
    pub order: Option<Vec<String>>, // 执行顺序（可选）
}
```

**原则**：
- 穷尽要求
- 不留隐含内容
- 明确优先级

**示例**：
```
MUST DO:
1. 先阅读大纲中本章的关键事件
2. 检索主角和配角的性格设定
3. 确保战斗场景与世界观魔法系统一致
4. 在章节结尾埋设'主角隐藏力量'伏笔
```

#### 字段五：MUST NOT DO（禁止执行）

```rust
pub struct MustNotDo {
    pub actions: Vec<String>,     // 禁止执行的操作
    pub reason: Vec<String>,      // 禁止原因
}
```

**原则**：
- 预判可能的越界
- 提前阻止
- 说明原因

**示例**：
```
MUST NOT DO:
- 不改变主角已确定的性格特征
  （原因：违反人物一致性）
- 不违反魔法系统的限制规则
  （原因：世界观设定冲突）
- 不跳过大纲中的关键事件
  （原因：破坏剧情结构）
```

#### 字段六：CONTEXT（上下文）

```rust
pub struct DelegationContext {
    // 通用上下文
    pub files: Vec<String>,
    pub patterns: Vec<String>,
    pub constraints: Vec<String>,
    
    // OpenNovel 特有
    pub related_knowledge: Vec<KnowledgeRef>,
    pub chapter_context: Option<ChapterContext>,
    pub character_refs: Vec<CharacterRef>,
    pub foreshadowing_refs: Vec<ForeshadowingRef>,
}
```

**原则**：
- 提供决策所需的所有信息
- 避免重复询问
- 保持上下文连贯

**示例**：
```
CONTEXT:
相关文件：
- 大纲第 15 章

现有模式：
- 当前文风：紧凑节奏、大量对话

约束条件：
- 战斗场景需要详细描写魔法效果

相关知识：
- 主角性格设定
- 魔法系统规则
- 反派能力设定
```

---

## 附录三：Session Continuity 实现细节

### 会话管理器

```rust
pub struct SessionContinuityManager {
    sessions: HashMap<SessionId, SessionState>,
    parent_child_map: HashMap<SessionId, Vec<SessionId>>,
}

pub struct SessionState {
    pub session_id: SessionId,
    pub parent_id: Option<SessionId>,
    pub book_id: BookId,
    pub agent: AgentName,
    pub status: SessionStatus,
    pub messages: Vec<Message>,
    pub context: HashMap<String, Value>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 会话创建与继续

```rust
impl SessionContinuityManager {
    /// 创建新会话
    pub async fn create_session(
        &self,
        params: CreateSessionParams,
    ) -> Result<Session, SessionError> {
        let session = Session {
            id: generate_session_id(),
            parent_id: params.parent_id,
            book_id: params.book_id,
            agent: params.agent,
            status: SessionStatus::Active,
            messages: vec![],
            context: params.initial_context.unwrap_or_default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.sessions.insert(session.id.clone(), session.clone());
        
        // 记录父子关系
        if let Some(parent_id) = &session.parent_id {
            self.parent_child_map
                .entry(parent_id.clone())
                .or_default()
                .push(session.id.clone());
        }
        
        Ok(session)
    }
    
    /// 继续会话
    /// **关键**: 使用 session_id 继续，而不是创建新会话
    pub async fn continue_session(
        &self,
        session_id: &SessionId,
        message: &str,
    ) -> Result<Session, SessionError> {
        let session = self.sessions.get_mut(session_id)
            .ok_or(SessionError::NotFound)?;
        
        // 添加消息
        session.messages.push(Message {
            role: Role::User,
            content: message.to_string(),
            timestamp: Utc::now(),
        });
        
        session.updated_at = Utc::now();
        
        Ok(session.clone())
    }
    
    /// 获取会话上下文（包括父会话）
    pub async fn get_session_context(
        &self,
        session_id: &SessionId,
    ) -> Result<HashMap<String, Value>, SessionError> {
        let session = self.sessions.get(session_id)
            .ok_or(SessionError::NotFound)?;
        
        let mut context = session.context.clone();
        
        // 合并父会话上下文
        if let Some(parent_id) = &session.parent_id {
            let parent_context = self.get_session_context(parent_id).await?;
            context.extend(parent_context);
        }
        
        Ok(context)
    }
}
```

### 继续规则

```rust
pub enum ContinuationRule {
    /// 任务失败/不完整
    OnTaskFailed {
        prompt_template: String,  // "修复：{具体错误}"
    },
    
    /// 对结果有后续问题
    OnFollowUpQuestion {
        prompt_template: String,  // "另外：{问题}"
    },
    
    /// 与同一 Agent 多轮对话
    OnMultiTurnConversation {
        rule: String,  // "永远不要重新开始新会话"
    },
    
    /// 验证失败
    OnVerificationFailed {
        prompt_template: String,  // "验证失败：{错误}。修复。"
    },
}
```

---

## 附录四：知识库系统详解

### 七大知识库

OpenNovel 维护七个知识库，每个知识库有特定的用途和权限控制。

#### 1. 世界观知识库

```rust
pub struct WorldviewKnowledgeBase {
    pub rules: Vec<WorldviewRule>,
    pub concepts: Vec<Concept>,
    pub locations: Vec<Location>,
    pub items: Vec<Item>,
    pub abilities: Vec<Ability>,
    pub history: Vec<HistoricalEvent>,
    pub cultures: Vec<Culture>,
}
```

**用途**：存储世界观的所有设定，包括规则、概念、地点、物品、能力、历史、文化。

**权限**：
- 天道：读写
- 世界观守护者：读写
- 规划者（阶段一）：读写
- 其他 Agent：只读

#### 2. 历史情节知识库

```rust
pub struct HistoryKnowledgeBase {
    pub chapters: Vec<ChapterSummary>,
    pub events: Vec<Event>,
    pub timeline: Timeline,
}
```

**用途**：存储已完成章节的摘要和事件，用于向量化检索。

**特点**：
- 每章完成后自动更新
- 通过嵌入模型向量化
- 支持语义检索

#### 3. 本章知识库

```rust
pub struct CurrentChapterKnowledgeBase {
    pub chapter_id: ChapterId,
    pub outline: ChapterOutline,
    pub key_events: Vec<KeyEvent>,
    pub pov: Option<CharacterRef>,
    pub foreshadowings: Vec<ForeshadowingRef>,
}
```

**用途**：存储当前章节的规划信息，由天道设计。

**生命周期**：
- 章节开始时创建
- 章节完成后归档到历史知识库

#### 4. 人物信息知识库

```rust
pub struct CharacterKnowledgeBase {
    pub characters: Vec<Character>,
    pub relationships: Vec<Relationship>,
    pub voice_profiles: Vec<VoiceProfile>,
}
```

**用途**：存储所有人物的信息，包括属性、关系、声音特征。

**特点**：
- 支持时间切片：每个人物在不同阶段有不同的状态
- 由刘和平维护

#### 5. 阵营派系势力知识库

```rust
pub struct FactionKnowledgeBase {
    pub factions: Vec<Faction>,
    pub alliances: Vec<Alliance>,
    pub conflicts: Vec<Conflict>,
}
```

**用途**：存储势力、派系、阵营的信息和关系。

#### 6. 地图知识库

```rust
pub struct MapKnowledgeBase {
    pub locations: Vec<Location>,
    pub connections: Vec<Connection>,
    pub character_positions: HashMap<CharacterId, LocationId>,
}
```

**用途**：存储地理信息和人物位置。

#### 7. 伏笔知识库

```rust
pub struct ForeshadowingKnowledgeBase {
    pub foreshadowings: Vec<Foreshadowing>,
    pub pressure_gauge: PlotPressureGauge,
}
```

**用途**：存储伏笔信息和压力表。

---

## 附录五：权限矩阵完整定义

```rust
pub struct PermissionMatrix {
    // 知识库权限
    pub knowledge_permissions: HashMap<AgentName, HashMap<KnowledgeBase, Permission>>,
    
    // 工具权限
    pub tool_permissions: HashMap<AgentName, Vec<ToolPermission>>,
    
    // 操作权限
    pub operation_permissions: HashMap<AgentName, Vec<OperationPermission>>,
}

pub enum Permission {
    None,       // 无权限
    Read,       // 只读
    Write,      // 读写
    Manage,     // 管理（创建/更新/删除）
}

// 完整权限矩阵
pub const KNOWLEDGE_PERMISSIONS: &[(&str, &str, Permission)] = &[
    // 天道
    ("tian-dao", "worldview", Permission::Write),
    ("tian-dao", "history", Permission::Read),
    ("tian-dao", "current_chapter", Permission::Write),
    ("tian-dao", "characters", Permission::Read),
    ("tian-dao", "factions", Permission::Read),
    ("tian-dao", "map", Permission::Read),
    ("tian-dao", "foreshadowing", Permission::Write),
    
    // 执笔
    ("writer", "worldview", Permission::Read),
    ("writer", "history", Permission::Read),
    ("writer", "current_chapter", Permission::Read),
    ("writer", "characters", Permission::Read),
    ("writer", "factions", Permission::Read),
    ("writer", "map", Permission::Read),
    ("writer", "foreshadowing", Permission::Read),
    
    // 世界观守护者
    ("world-guardian", "worldview", Permission::Write),
    ("world-guardian", "history", Permission::Read),
    ("world-guardian", "current_chapter", Permission::Read),
    ("world-guardian", "characters", Permission::Read),
    ("world-guardian", "factions", Permission::Read),
    ("world-guardian", "map", Permission::Read),
    ("world-guardian", "foreshadowing", Permission::Read),
    
    // 刘和平
    ("liuheping", "worldview", Permission::Read),
    ("liuheping", "history", Permission::Read),
    ("liuheping", "current_chapter", Permission::Read),
    ("liuheping", "characters", Permission::Write),
    ("liuheping", "factions", Permission::Read),
    ("liuheping", "map", Permission::Read),
    ("liuheping", "foreshadowing", Permission::Read),
    
    // 规划者（阶段一）
    ("planner", "worldview", Permission::Write),
    ("planner", "history", Permission::None),
    ("planner", "current_chapter", Permission::Write),
    ("planner", "characters", Permission::Write),
    ("planner", "factions", Permission::Write),
    ("planner", "map", Permission::Write),
    ("planner", "foreshadowing", Permission::Write),
    
    // 审阅
    ("reviewer", "worldview", Permission::Read),
    ("reviewer", "history", Permission::Read),
    ("reviewer", "current_chapter", Permission::Read),
    ("reviewer", "characters", Permission::Read),
    ("reviewer", "factions", Permission::Read),
    ("reviewer", "map", Permission::Read),
    ("reviewer", "foreshadowing", Permission::Read),
    
    // 观察者
    ("observer", "worldview", Permission::Manage),
    ("observer", "history", Permission::Manage),
    ("observer", "current_chapter", Permission::Manage),
    ("observer", "characters", Permission::Manage),
    ("observer", "factions", Permission::Manage),
    ("observer", "map", Permission::Manage),
    ("observer", "foreshadowing", Permission::Manage),
    
    // 调研者（阶段二）
    ("researcher", "worldview", Permission::Read),
    ("researcher", "history", Permission::Read),
    ("researcher", "current_chapter", Permission::None),
    ("researcher", "characters", Permission::Read),
    ("researcher", "factions", Permission::Read),
    ("researcher", "map", Permission::Read),
    ("researcher", "foreshadowing", Permission::Read),
];
```

---

## 附录六：与 Oh My OpenCode 的详细对比

### Hooks 层面对比

| 方面 | Oh My OpenCode | OpenNovel |
|------|---------------|-----------|
| **总数量** | 46 个 | ~12 个核心 |
| **层级结构** | 5 层（Session/Tool Guard/Transform/Continuation/Skill） | 3 层（Session/Chapter/Intervention） |
| **触发方式** | 事件驱动 | 事件驱动 + 内容检测 |
| **主动性** | 响应式 | 响应式 + 前瞻式 |

**OMO 独有的 Hooks**：
- `context-window-monitor`：上下文窗口监控
- `preemptive-compaction`：抢占式压缩
- `hashline-read-enhancer`：哈希行读取增强
- `comment-checker`：注释检查器

**OpenNovel 独有的 Hooks**：
- `phase-lock-check`：阶段锁定检查
- `proactive-intervention`：主动介入
- `foreshadowing-state-check`：伏笔状态检查
- `plot-pressure-check`：剧情压力检查

### Skills 层面对比

| 方面 | Oh My OpenCode | OpenNovel |
|------|---------------|-----------|
| **定义格式** | SKILL.md | SKILL.md（兼容） |
| **MCP 集成** | Skill-Embedded MCP | 暂不集成 |
| **Category 映射** | Skill → Category → Model | Skill → Agent |
| **目录位置** | /root/.config/opencode/skills/ | /opennovelv2/packages/agents/skills/ |

### LSP 层面对比

| 方面 | Oh My OpenCode | OpenNovel |
|------|---------------|-----------|
| **核心工具** | LSP（Language Server Protocol） | Knowledge Retrieval |
| **数据结构** | AST（抽象语法树） | Knowledge Graph |
| **查询方式** | 结构化查询 | 向量检索 + 关键词匹配 |
| **精确性** | 编译器级别 | 语义匹配级别 |
| **动态性** | 静态（代码不变） | 动态（知识库持续更新） |

**OMO 的 LSP 工具**：
- `lsp_goto_definition`：跳转到定义
- `lsp_find_references`：查找引用
- `lsp_symbols`：获取符号
- `lsp_diagnostics`：获取诊断信息
- `lsp_rename`：重命名符号

**OpenNovel 的对应工具**：
- `knowledge_search`：知识库检索
- `character_lookup`：人物查询
- `worldview_check`：世界观检查
- `timeline_query`：时间线查询
- `foreshadowing_status`：伏笔状态

### MCP 层面对比

| 方面 | Oh My OpenCode | OpenNovel |
|------|---------------|-----------|
| **集成状态** | 已集成 | 规划中 |
| **MCP Servers** | websearch, context7, grep_app | 待规划 |
| **Skill-Embedded** | 支持 | 暂不支持 |

---

## 附录七：实现路线图

### Phase 0: SDK 基础 ✅

- [x] Agent Trait 定义
- [x] SessionContext 实现
- [x] MessageProtocol 定义
- [x] IntentGate 4阶段框架
- [x] PermissionMatrix 实现

### Phase 1: 知识库系统 ✅

- [x] 7大知识库结构定义
- [x] CharacterDB 实现
- [x] ForeshadowingPool 实现
- [x] TimelineSystem 实现
- [x] WorldGraph 实现

### Phase 2: 文本工具链 ✅

- [x] TextEditor 实现
- [x] WordCounter 实现
- [x] StyleChecker 实现
- [x] TextSearch 实现
- [x] SegmentSplit 实现

### Phase 3: 协作系统 ✅

- [x] AnnotationSystem 实现
- [x] ConflictArbitration 实现
- [x] ProactiveIntervention 实现
- [x] GroupChat 实现
- [x] AgentLock 实现

### Phase 4: 分析系统 ✅

- [x] StyleAnalyzer 实现
- [x] EmotionAnalyzer 实现
- [x] PacingAnalyzer 实现
- [x] ConsistencyChecker 实现

### Phase 5: 同步系统 ✅

- [x] WebDAVSync 实现
- [x] ConflictResolver 实现
- [x] SyncManager 实现

### Phase 6: LLM 集成层 ✅

- [x] ProviderRegistry 实现
- [x] ModelResolver 实现
- [x] FallbackChain 实现
- [x] HotReload 实现

### Phase 7: Agent 系统 ✅

- [x] Hooks Registry 实现
- [x] Tools Registry 实现
- [x] Skills Loader 实现
- [x] Delegation Protocol 实现

### Phase 8: Web UI ⏳

- [ ] 群聊界面实现
- [ ] Agent 状态展示
- [ ] 配置界面实现
- [ ] 流式输出支持

### Phase 9: 测试与优化 ⏳

- [ ] 集成测试
- [ ] 性能优化
- [ ] 错误处理完善

### Phase 10: 文档与部署 ⏳

- [ ] API 文档
- [ ] 用户指南
- [ ] Docker 镜像

---

## 许可证

MIT License