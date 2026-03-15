# OpenNovel v2 - 下一代多Agent小说创作协作系统

> **状态**: 🚧 设计阶段 - 本仓库为公开设计文档，核心实现在私有仓库

## ⚠️ 双仓库策略

本项目采用**公开设计 + 私有实现**的双仓库策略：

| 内容 | 本仓库 (公开) | 私有仓库 |
|------|-------------|---------|
| 设计文档 | ✅ 完整公开 | 同步 |
| 基础SDK框架 | ✅ 基础实现 | 完整实现 |
| 知识库算法 | ❌ 不包含 | 🔒 核心算法 |
| 协作逻辑 | ❌ 不包含 | 🔒 完整实现 |
| 分析引擎 | ❌ 不包含 | 🔒 完整实现 |
| Web UI | ✅ 基础框架 | 🔒 完整实现 |

本仓库包含完整的设计文档，供学习参考。核心实现代码仅限作者访问。

---

## 设计理念

### 核心思想

**像AI编程一样AI协作写小说**——用户通过群聊界面，像与真人协作一样与各Agent共同完成小说创作。

### 关键创新

1. **Agent主动性** - Agent不是被动等待呼叫，而是像真人一样主动介入讨论
2. **阶段性锁定** - 规划者和调研者在完成阶段任务后被锁定，确保创作方向稳定
3. **群聊协作模式** - 每本书一个群聊，所有交互都在群内完成
4. **流式思考展示** - 实时显示Agent的思考过程

## 八大Agent

| Agent | 角色 | 生命周期 | 核心职责 |
|-------|------|---------|---------|
| 规划者 | 新书规划 | 仅阶段一 | 确定世界观、人物、篇幅、文风 |
| 天道 | 剧情设计 | 阶段三起 | 大纲设计、剧情演化、伏笔管理 |
| 世界观守护者 | 一致性检查 | 阶段三起 | 维护世界观一致性 |
| 刘和平 | 角色设计 | 阶段三起 | 人物塑造、对话合理性 |
| 执笔 | 唯一写入者 | 阶段三起 | 实际撰写章节内容 |
| 审阅 | 阅读体验 | 阶段三起 | 文学性、可读性评估 |
| 观察者 | 协作枢纽 | 全程 | 知识库管理、多Agent调度 |
| 调研者 | 爆点分析 | 仅阶段二 | 分析同类爆款、评估爆点 |

### Agent生命周期

```
新建书籍
    │
    ▼
┌─────────────────────────────────────────┐
│  阶段一：构思阶段                        │
│  🔓 规划者 ←→ 用户                       │
│  🔒 其他所有Agent                        │
│  用户点击 [构思完成] 后结束              │
└─────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────┐
│  阶段二：知识库建立                      │
│  1. 规划者整理规划                       │
│  2. 用户上传参考小说                     │
│  3. 调研者评估爆点                       │
│  4. 观察者建立知识库                     │
│  完成后自动进入阶段三                    │
└─────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────┐
│  阶段三：撰写阶段                        │
│  🔒 规划者、调研者（永久锁定）            │
│  🔓 其他6个Agent                        │
│  ✓ 主动交互模式启用                      │
└─────────────────────────────────────────┘
```

## 七大知识库

每本书拥有独立的知识库系统：

| 知识库 | 用途 | 写入权限 |
|--------|------|---------|
| 世界观知识库 | 设定、规则、历史 | 天道、世界观守护者 |
| 历史情节知识库 | 已完成章节的向量化 | 观察者（自动） |
| 本章知识库 | 当前章节的规划 | 天道 |
| 人物信息知识库 | 角色属性、关系 | 刘和平 |
| 阵营派系势力知识库 | 势力分布、强弱 | 天道 |
| 地图知识库 | 人物位置、地理 | 天道 |
| 伏笔知识库 | 伏笔、悬念 | 天道 |

## 批注系统

批注 = 注释内容 + Agent签名

```
示例批注：
┌────────────────────────────────────────┐
│ 🟡 刘和平                               │
│ 主角此时的反应与其谨慎性格不符，建议    │
│ 改为先观察再行动，而非直接对话。        │
└────────────────────────────────────────┘
```

## 群聊界面

模仿钉钉的群聊UI：
- 每本书 = 一个群
- 群名 = 书名
- Agent消息包含思考过程（可折叠）
- Agent状态栏显示锁定/解锁状态
- 主动交互时Agent自动加入对话

## WebDAV同步

- 每章完成后自动同步到WebDAV云盘
- 增量同步，避免重复上传
- 版本管理由用户在云盘端自行处理

## 技术栈

| 组件 | 技术选型 |
|------|---------|
| 后端 | Rust + Axum |
| 前端 | SvelteKit + Tailwind CSS |
| 向量数据库 | Qdrant |
| LLM | 多供应商支持（OpenCode SDK兼容） |
| 嵌入模型 | bce-embedding-base_v1 |
| 缓存/队列 | Redis |
| 同步 | WebDAV |

## 模块化开发计划

> **开发原则**: 不追求快速MVP，而是每个模块完整实现并测试通过后再进入下一阶段。确保每个阶段都能稳定运行。

---

### Phase 0: SDK基础（预计2周）

**目标**: 建立整个系统的核心运行时框架

```
NovelSDK/
├── core/
│   ├── agent.rs          # Agent定义系统
│   ├── session.rs        # Session管理
│   ├── message.rs        # Message Protocol（SSE）
│   ├── intent_gate.rs    # Intent Gate
│   └── permission.rs     # Permission系统
└── config/
    ├── provider.rs       # Provider配置
    └── agent_config.rs   # Agent配置
```

| 模块 | 功能 | 依赖 |
|------|------|------|
| Agent定义系统 | 8个Agent注册、配置、生命周期管理 | - |
| Session管理 | 书籍级会话、消息历史、上下文窗口 | Agent系统 |
| Message Protocol | SSE流式输出、Thinking/Content分离 | Session |
| Intent Gate | 用户意图解析、Agent路由、锁定状态检查 | Agent系统 |
| Permission系统 | 知识库权限矩阵、操作授权 | Agent系统 |
| Provider配置 | 多LLM供应商支持、API密钥管理 | - |
| Agent配置 | 模型选择、温度、thinking预算、技能绑定 | Agent系统 |

**验收标准**:
- [ ] 可以通过API创建Agent并发送消息
- [ ] SSE流式输出正常工作（thinking + content）
- [ ] Intent Gate正确路由到目标Agent
- [ ] Permission系统可以阻止未授权操作
- [ ] 多Provider配置可切换

---

### Phase 1: 知识库系统（预计3周）

**目标**: 建立完整的知识与上下文层

```
NovelSDK/
├── knowledge/
│   ├── worldview.rs      # 世界观知识库
│   ├── character.rs      # 人物信息知识库 + CharacterDB
│   ├── plot.rs           # 历史情节知识库
│   ├── foreshadowing.rs  # 伏笔知识库 + ForeshadowingPool
│   ├── timeline.rs       # TimelineSystem
│   ├── factions.rs       # 阵营派系势力知识库
│   ├── map.rs            # 地图知识库
│   └── world_graph.rs    # WorldGraph
```

| 模块 | 功能 | 核心数据结构 |
|------|------|-------------|
| 7大知识库 | 向量化存储与检索 | Qdrant Collection |
| CharacterDB | 角色属性、关系、时间切片状态、VoiceProfile | `HashMap<String, Character>` |
| ForeshadowingPool | 伏笔生命周期管理、压力表、触发队列 | `HashMap<String, Foreshadowing>` |
| TimelineSystem | 事件时间线、冲突检测、时间尺度 | `Vec<TimelineEvent>` |
| WorldGraph | 世界观图谱、规则验证、一致性检查 | `HashMap<String, WorldNode>` |

**验收标准**:
- [ ] 所有知识库CRUD操作正常
- [ ] CharacterDB可检测角色行为一致性
- [ ] ForeshadowingPool可管理伏笔状态（已埋/已暗示/已触发/已放弃）
- [ ] TimelineSystem可检测时间冲突
- [ ] WorldGraph可验证世界观规则
- [ ] 权限控制矩阵生效

---

### Phase 2: 文本工具链（预计2周）

**目标**: 建立文本处理基础工具

```
NovelSDK/
├── text/
│   ├── editor.rs         # TextEditor
│   ├── counter.rs        # WordCounter
│   ├── style_checker.rs  # StyleChecker
│   ├── search.rs         # TextSearch
│   └── segment.rs        # SegmentSplit
```

| 模块 | 功能 | 关键方法 |
|------|------|---------|
| TextEditor | 章节编辑、撤销/重做、批注应用 | `insert()`, `delete()`, `apply_annotation()` |
| WordCounter | 中英文字数统计、阅读时间估算 | `count()`, `estimate_reading_time()` |
| StyleChecker | 风格规则检查、重复检测、句式分析 | `check()`, `check_repetition()` |
| TextSearch | 关键词搜索、语义搜索、角色/地点搜索 | `search_keyword()`, `search_semantic()` |
| SegmentSplit | 章节分割、场景检测 | `split_by_chapter()`, `split_by_scene()` |

**验收标准**:
- [ ] TextEditor支持完整编辑操作和撤销
- [ ] WordCounter准确统计中英文字数
- [ ] StyleChecker可检测常见风格问题
- [ ] TextSearch支持关键词和语义搜索
- [ ] SegmentSplit可智能分割章节

---

### Phase 3: 协作系统（预计3周）

**目标**: 建立Agent协作与群聊核心

```
NovelSDK/
├── collaboration/
│   ├── annotation.rs     # AnnotationSystem
│   ├── conflict.rs       # ConflictArbitration
│   ├── proactive.rs      # ProactiveIntervention
│   ├── group_chat.rs     # GroupChat
│   └── agent_lock.rs     # AgentLock
```

| 模块 | 功能 | 关键逻辑 |
|------|------|---------|
| AnnotationSystem | 批注添加、状态管理、冲突检测 | `add()`, `accept()`, `detect_conflicts()` |
| ConflictArbitration | 冲突报告生成、用户裁决 | `generate_report()`, `arbitrate()` |
| ProactiveIntervention | 介入条件注册、触发检查 | `check()`, `register_condition()` |
| GroupChat | 群聊消息、阶段管理、Agent状态 | `send()`, `change_stage()`, `lock_agent()` |
| AgentLock | 阶段切换时自动锁定/解锁 | `on_stage_change()`, `is_available()` |

**验收标准**:
- [ ] 批注系统完整工作流（添加→审核→接受/拒绝）
- [ ] 冲突检测正确识别重叠批注
- [ ] 主动介入机制可注册触发条件
- [ ] GroupChat支持三阶段切换
- [ ] AgentLock在阶段切换时正确锁定/解锁

---

### Phase 4: 分析系统（预计2周）

**目标**: 建立文本质量分析工具

```
NovelSDK/
├── analysis/
│   ├── style.rs          # StyleAnalyzer
│   ├── emotion.rs        # EmotionAnalyzer
│   ├── pacing.rs         # PacingAnalyzer
│   └── consistency.rs    # ConsistencyChecker
```

| 模块 | 功能 | 输出 |
|------|------|------|
| StyleAnalyzer | 词汇多样性、句式变化、修辞使用 | `StyleAnalysisResult` |
| EmotionAnalyzer | 情绪曲线、转折点检测、读者情绪预测 | `EmotionCurve` |
| PacingAnalyzer | 节奏分析、紧张度曲线、节奏建议 | `PacingReport` |
| ConsistencyChecker | 时间线一致性、角色行为一致性、世界观一致性 | `ConsistencyReport` |

**验收标准**:
- [ ] StyleAnalyzer可生成风格评分
- [ ] EmotionAnalyzer可绘制情绪曲线
- [ ] PacingAnalyzer可检测节奏问题
- [ ] ConsistencyChecker可检测常见不一致问题

---

### Phase 5: 同步系统（预计1周）

**目标**: 建立云同步与版本管理

```
NovelSDK/
├── sync/
│   ├── webdav.rs         # WebDAVSync
│   └── version.rs        # VersionManager（可选，用户自行管理）
```

| 模块 | 功能 | 关键操作 |
|------|------|---------|
| WebDAVSync | 章节同步、增量上传、失败通知 | `sync_chapter()`, `sync_book()` |

**验收标准**:
- [ ] WebDAV连接认证正常
- [ ] 章节完成时自动同步
- [ ] 同步失败时群内通知

---

### 开发顺序依赖图

```
Phase 0 (SDK基础)
    │
    ├── Agent定义系统 ──────────────────────────────────────────────┐
    │                                                               │
    ├── Session管理 ────────────────────────────────────────────────┤
    │                                                               │
    ├── Message Protocol ───────────────────────────────────────────┤
    │                                                               │
    ├── Intent Gate ────────────────────────────────────────────────┤
    │                                                               │
    └── Permission系统 ─────────────────────────────────────────────┤
                                                                    │
    ▼                                                               │
Phase 1 (知识库系统)                                                 │
    │                                                               │
    ├── 7大知识库 ←── Permission系统 ───────────────────────────────┤
    │                                                               │
    ├── CharacterDB ────────────────────────────────────────────────┤
    │                                                               │
    ├── ForeshadowingPool ──────────────────────────────────────────┤
    │                                                               │
    ├── TimelineSystem ─────────────────────────────────────────────┤
    │                                                               │
    └── WorldGraph ─────────────────────────────────────────────────┤
                                                                    │
    ▼                                                               │
Phase 2 (文本工具链)                                                 │
    │                                                               │
    ├── TextEditor ─────────────────────────────────────────────────┤
    │                                                               │
    ├── WordCounter ────────────────────────────────────────────────┤
    │                                                               │
    ├── StyleChecker ───────────────────────────────────────────────┤
    │                                                               │
    ├── TextSearch ←── 知识库向量索引 ───────────────────────────────┤
    │                                                               │
    └── SegmentSplit ───────────────────────────────────────────────┤
                                                                    │
    ▼                                                               │
Phase 3 (协作系统)                                                   │
    │                                                               │
    ├── AnnotationSystem ←── TextEditor ────────────────────────────┤
    │                                                               │
    ├── ConflictArbitration ←── AnnotationSystem ───────────────────┤
    │                                                               │
    ├── ProactiveIntervention ←── 知识库 ───────────────────────────┤
    │                                                               │
    ├── GroupChat ←── Agent系统 + Message Protocol ─────────────────┤
    │                                                               │
    └── AgentLock ←── GroupChat ────────────────────────────────────┤
                                                                    │
    ▼                                                               │
Phase 4 (分析系统)                                                   │
    │                                                               │
    ├── StyleAnalyzer ←── StyleChecker ─────────────────────────────┤
    │                                                               │
    ├── EmotionAnalyzer ────────────────────────────────────────────┤
    │                                                               │
    ├── PacingAnalyzer ─────────────────────────────────────────────┤
    │                                                               │
    └── ConsistencyChecker ←── CharacterDB + WorldGraph ────────────┤
                                                                    │
    ▼                                                               │
Phase 5 (同步系统)                                                   │
    │                                                               │
    └── WebDAVSync ←── TextEditor (输出TXT) ────────────────────────┘
```

---

### 总体时间线

| Phase | 预计时间 | 累计 |
|-------|---------|------|
| Phase 0: SDK基础 | 2周 | 2周 |
| Phase 1: 知识库系统 | 3周 | 5周 |
| Phase 2: 文本工具链 | 2周 | 7周 |
| Phase 3: 协作系统 | 3周 | 10周 |
| Phase 4: 分析系统 | 2周 | 12周 |
| Phase 5: 同步系统 | 1周 | **13周** |

**预计完整开发周期**: 约3个月

---

### 技术债务与注意事项

- **LLM供应商**: 优先支持OpenAI、Anthropic、阿里云、本地模型(Ollama)
- **性能目标**: 单章节生成响应时间 < 30秒
- **稳定性**: Agent失败自动重试、状态恢复
- **可扩展性**: 模块化设计，支持未来Agent数量扩展
- **测试覆盖**: 每个Phase完成后必须有单元测试和集成测试

## 与v1的区别

| 方面 | v1 | v2 |
|------|-----|-----|
| Agent数量 | 6个 | 8个 |
| Agent主动性 | 被动响应 | 主动介入 |
| 界面模式 | 简单聊天 | 群聊协作 |
| 输出方式 | 同步阻塞 | SSE流式 |
| 思考过程 | 不可见 | 实时展示 |
| 知识库 | 单一 | 7个专用库 |
| Agent锁定 | 无 | 有 |
| Skills/Hooks | 无 | 完整设计 |

## 相关链接

- [v1版本](https://github.com/jcy321/opennovelv1) - ⚠️ 已废弃，仅作参考
- [架构设计文档](./docs/ARCHITECTURE_DESIGN_V2.md) - 完整系统架构
- [设计补充文档](./docs/DESIGN_SUPPLEMENT.md) - Agent配置与权限系统
- [Oh-My-OpenCode研究](./docs/OH_MY_OPENCODE_RESEARCH.md) - 架构参考研究
- [Skills与Hooks设计](./docs/SKILLS_AND_HOOKS_DESIGN.md) - Agent部件设计
- [部件体系设计](./docs/COMPONENT_SYSTEM.md) - 完整部件体系

## 贡献

项目目前处于设计阶段，欢迎：
- 对设计文档提出建议
- 参与架构讨论
- 提交Issue反馈需求

## 许可证

MIT License

---

**设计哲学**: 小说是由世界观、人物、剧情等要素动态驱动的小世界，命运在其中交汇、编织。Agent像真人创作团队一样，共同编织精彩的故事。