# OpenNovel v2 补充设计文档

> 本文档补充 ARCHITECTURE_DESIGN_V2.md 中未完成的设计细节

---

## 一、Agent配置选项

### 1.1 完整配置结构

```json
{
  "agents": {
    "tian_dao": {
      "model": "bailian-coding-plan/glm-5",
      "variant": "max",
      "category": "planning",
      "temperature": 0.3,
      "top_p": 0.9,
      "prompt": "可选：完全覆盖系统提示词",
      "prompt_append": "可选：在系统提示词后追加",
      "tools": {
        "knowledge_search": true,
        "vector_query": true,
        "web_search": false
      },
      "disable": false,
      "maxTokens": 16384,
      "thinking": {
        "type": "enabled",
        "budgetTokens": 8192
      },
      "reasoningEffort": "high",
      "permissions": {
        "edit_knowledge": "allow",
        "write_chapter": "deny",
        "annotate": "allow"
      }
    }
  }
}
```

### 1.2 配置选项详解

| 选项 | 类型 | 默认值 | 说明 |
|-----|------|-------|------|
| `model` | string | - | 模型标识符（格式：provider/model） |
| `variant` | string | - | 模型变体：max/high/medium/low |
| `category` | string | - | 从分类继承配置 |
| `temperature` | number | 0.7 | 采样温度（0-2） |
| `top_p` | number | 0.9 | Top-p 采样（0-1） |
| `prompt` | string | - | 完全覆盖系统提示词 |
| `prompt_append` | string | - | 在系统提示词后追加文本 |
| `tools` | Record | - | 启用或禁用特定工具 |
| `disable` | boolean | false | 禁用此 Agent |
| `maxTokens` | number | 4096 | 响应的最大 token 数 |
| `thinking` | object | - | 扩展思考配置 |
| `reasoningEffort` | string | - | 推理强度：low/medium/high/xhigh |

### 1.3 八大Agent推荐配置

```json
{
  "agents": {
    "planner": {
      "model": "bailian-coding-plan/kimi-k2.5",
      "temperature": 0.3,
      "thinking": { "type": "enabled", "budgetTokens": 8192 },
      "maxTokens": 16384,
      "permissions": {
        "edit_worldview": "allow",
        "edit_characters": "allow",
        "write_chapter": "deny"
      }
    },
    "tian_dao": {
      "model": "bailian-coding-plan/glm-5",
      "temperature": 0.3,
      "thinking": { "type": "enabled", "budgetTokens": 8192 },
      "reasoningEffort": "xhigh",
      "maxTokens": 16384,
      "permissions": {
        "edit_worldview": "allow",
        "edit_plot": "allow",
        "edit_foreshadowing": "allow"
      }
    },
    "world_guardian": {
      "model": "bailian-coding-plan/qwen3-max-2026-01-23",
      "temperature": 0.2,
      "thinking": { "type": "enabled", "budgetTokens": 4096 },
      "permissions": {
        "edit_worldview": "allow",
        "annotate": "allow"
      }
    },
    "liu_heping": {
      "model": "bailian-coding-plan/qwen3-coder-next",
      "temperature": 0.4,
      "thinking": { "type": "enabled", "budgetTokens": 4096 },
      "permissions": {
        "edit_characters": "allow",
        "annotate": "allow"
      }
    },
    "writer": {
      "model": "bailian-coding-plan/glm-5",
      "temperature": 0.7,
      "thinking": { "type": "enabled", "budgetTokens": 8192 },
      "maxTokens": 32768,
      "permissions": {
        "write_chapter": "allow",
        "edit_chapter": "ask"
      }
    },
    "reviewer": {
      "model": "bailian-coding-plan/qwen3-max-2026-01-23",
      "temperature": 0.2,
      "thinking": { "type": "enabled", "budgetTokens": 4096 },
      "permissions": {
        "annotate": "allow",
        "edit_evaluation": "allow"
      }
    },
    "observer": {
      "model": "bailian-coding-plan/qwen3-coder-next",
      "temperature": 0.3,
      "permissions": {
        "manage_knowledge": "allow",
        "orchestrate_agents": "allow"
      }
    },
    "researcher": {
      "model": "bailian-coding-plan/kimi-k2.5",
      "temperature": 0.3,
      "thinking": { "type": "enabled", "budgetTokens": 8192 },
      "permissions": {
        "read_references": "allow",
        "write_init": "allow"
      }
    }
  }
}
```

---

## 二、权限系统设计

### 2.1 权限类型

| 权限 | 值 | 说明 |
|-----|-----|------|
| `read_worldview` | allow/deny | 读取世界观知识库 |
| `edit_worldview` | allow/deny/ask | 编辑世界观知识库 |
| `read_characters` | allow/deny | 读取人物信息知识库 |
| `edit_characters` | allow/deny/ask | 编辑人物信息知识库 |
| `read_plot` | allow/deny | 读取历史情节/本章知识库 |
| `edit_plot` | allow/deny/ask | 编辑本章知识库 |
| `read_foreshadowing` | allow/deny | 读取伏笔知识库 |
| `edit_foreshadowing` | allow/deny/ask | 编辑伏笔知识库 |
| `write_chapter` | allow/deny/ask | 撰写章节 |
| `edit_chapter` | allow/deny/ask | 修改章节 |
| `annotate` | allow/deny | 添加批注 |
| `manage_knowledge` | allow/deny | 管理知识库（创建/更新/删除） |
| `orchestrate_agents` | allow/deny | 协调其他Agent |
| `read_references` | allow/deny | 读取参考小说库 |
| `write_init` | allow/deny | 写入init.md |
| `edit_evaluation` | allow/deny | 编辑本书意见.md |

### 2.2 权限值含义

- `allow`: 直接允许，无需确认
- `deny`: 直接拒绝
- `ask`: 需要用户确认后在群内弹出确认对话框

### 2.3 权限继承

```
默认权限（所有Agent）
    │
    ├── 规划者覆盖（阶段一：全部allow；阶段三后：全部deny）
    │
    ├── 天道覆盖（世界观、剧情、伏笔：allow）
    │
    ├── 世界观守护者覆盖（世界观：allow）
    │
    ├── 刘和平覆盖（人物：allow）
    │
    ├── 执笔覆盖（章节写入：allow）
    │
    ├── 审阅覆盖（批注、评估：allow）
    │
    ├── 观察者覆盖（知识库管理、协调：allow）
    │
    └── 调研者覆盖（阶段二：参考库读、init写；阶段三后：全部deny）
```

### 2.4 权限检查流程

```rust
pub fn check_permission(
    agent: &Agent,
    permission: &Permission,
    book_stage: BookStage,
) -> PermissionResult {
    // 1. 检查Agent是否被锁定
    if agent.is_locked(book_stage) {
        return PermissionResult::Denied("Agent is locked in current stage");
    }
    
    // 2. 检查权限配置
    match agent.permissions.get(permission) {
        Some(PermissionValue::Allow) => PermissionResult::Allowed,
        Some(PermissionValue::Deny) => PermissionResult::Denied("Permission denied"),
        Some(PermissionValue::Ask) => PermissionResult::NeedConfirmation,
        None => PermissionResult::Denied("Permission not configured"),
    }
}
```

---

## 三、批注系统详细设计

### 3.1 批注结构

```rust
pub struct Annotation {
    pub id: String,                    // 批注唯一ID
    pub agent_id: String,              // Agent签名（哪个Agent发出的）
    pub agent_name: String,            // Agent显示名称
    pub chapter_id: String,            // 关联章节
    pub position: AnnotationPosition,  // 批注位置
    pub content: String,               // 批注内容
    pub severity: AnnotationSeverity,  // 严重程度
    pub status: AnnotationStatus,      // 状态
    pub created_at: DateTime<Utc>,     // 创建时间
    pub resolved_at: Option<DateTime<Utc>>, // 解决时间
    pub resolved_by: Option<String>,   // 解决者
}

pub struct AnnotationPosition {
    pub start_offset: usize,    // 起始字符位置
    pub end_offset: usize,      // 结束字符位置
    pub selected_text: String,  // 选中的文本
}

pub enum AnnotationSeverity {
    Critical,   // 严重问题，必须修改
    Warning,    // 警告，建议修改
    Suggestion, // 建议，可选修改
    Info,       // 信息，仅供参考
}

pub enum AnnotationStatus {
    Pending,    // 待处理
    Accepted,   // 已接受
    Rejected,   // 已拒绝
    Modified,   // 已修改
}
```

### 3.2 批注UI展示

```
┌─────────────────────────────────────────────────────────────────────┐
│ 章节3：初入星港                                                      │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│ 李明站在观景窗前，望着浩瀚的星海，心中涌起莫名的激动。              │
│ ┌────────────────────────────────────────────────────────────────┐ │
│ │ 🟡 刘和平 · Warning · 待处理                                    │ │
│ │ ─────────────────────────────────────────────────────────────── │ │
│ │ 根据主角性格（谨慎、内敛），此时应该更偏向观察而非直接表达      │ │
│ │ 激动。建议改为："李明站在观景窗前，默默注视着浩瀚的星海，      │ │
│ │ 眼神中闪过一丝难以察觉的光芒。"                                 │ │
│ │ ─────────────────────────────────────────────────────────────── │ │
│ │ [接受] [拒绝] [修改后接受]                                      │ │
│ └────────────────────────────────────────────────────────────────┘ │
│                                                                      │
│ 他想起了师父曾经说过的话："星辰大海，才是我们的归宿。"             │
│ ┌────────────────────────────────────────────────────────────────┐ │
│ │ 🔵 天道 · Info · 已接受                                         │ │
│ │ ─────────────────────────────────────────────────────────────── │ │
│ │ 此处呼应第一章的伏笔，师父的遗言将在第十章揭晓真相。            │ │
│ └────────────────────────────────────────────────────────────────┘ │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 3.3 批注冲突处理

当多个Agent给出冲突的批注时：

```
┌────────────────────────────────────────────────────────────────┐
│ ⚠️ 批注冲突 detected                                            │
│ ─────────────────────────────────────────────────────────────── │
│                                                                  │
│ 🔵 天道：                                                        │
│ "建议让主角此时表现出好奇，主动询问星港的功能"                  │
│                                                                  │
│ 🟡 刘和平：                                                      │
│ "主角性格谨慎，不应该主动询问，而应该通过观察了解"              │
│                                                                  │
│ 🟢 世界观守护者：                                                │
│ "根据设定，此时主角对'星港'概念尚不了解，需要以其他方式描述"    │
│                                                                  │
│ ─────────────────────────────────────────────────────────────── │
│ 请@用户 裁决：选择一个方案或提供新方向                          │
│ [采纳天道] [采纳刘和平] [采纳世界观守护者] [自定义方案]         │
└────────────────────────────────────────────────────────────────┘
```

---

## 四、后台任务管理

### 4.1 任务配置

```json
{
  "background_task": {
    "defaultConcurrency": 5,
    "staleTimeoutMs": 180000,
    "providerConcurrency": {
      "bailian-coding-plan": 5,
      "xiaomubiao": 3,
      "zai": 5
    }
  }
}
```

### 4.2 任务类型

| 任务类型 | 描述 | 触发时机 |
|---------|------|---------|
| `knowledge_update` | 更新知识库向量 | 章节完成后 |
| `consistency_check` | 一致性检查 | 执笔撰写后 |
| `annotation_collect` | 收集批注 | 初稿完成后 |
| `webdav_sync` | WebDAV同步 | 章节输出后 |
| `research_analysis` | 爆点分析 | 阶段二 |

### 4.3 任务优先级

```rust
pub enum TaskPriority {
    Critical,   // 必须完成才能继续
    High,       // 高优先级
    Normal,     // 正常优先级
    Low,        // 低优先级，可以等待
}
```

---

## 五、运行时回退机制

### 5.1 配置

```json
{
  "runtime_fallback": {
    "enabled": true,
    "retry_on_errors": [400, 429, 503, 529],
    "max_fallback_attempts": 3,
    "cooldown_seconds": 60,
    "notify_on_fallback": true,
    "fallback_order": [
      "bailian-coding-plan",
      "xiaomubiao",
      "zai"
    ]
  }
}
```

### 5.2 回退流程

```
主Provider失败（429/503等）
    │
    ▼
检查 fallback_enabled
    │
    ├── true ──► 尝试 fallback_order 中下一个Provider
    │              │
    │              ├── 成功 ──► 继续执行，通知用户
    │              │
    │              └── 失败 ──► 尝试下一个
    │                            │
    │                            └── 全部失败 ──► 返回错误
    │
    └── false ──► 直接返回错误
```

---

## 六、Categories（任务分类）

### 6.1 分类定义

| 分类 | 温度 | 适用场景 | 对应Agent |
|-----|------|---------|----------|
| `planning` | 0.3 | 规划类任务 | 规划者、天道 |
| `creation` | 0.7 | 创作类任务 | 执笔、刘和平 |
| `review` | 0.2 | 审核类任务 | 审阅、世界观守护者 |
| `analysis` | 0.3 | 分析类任务 | 调研者、观察者 |
| `coordination` | 0.3 | 协调类任务 | 观察者 |

### 6.2 分类继承

Agent可以设置 `category` 字段来继承分类配置：

```json
{
  "agents": {
    "writer": {
      "category": "creation",
      "model": "bailian-coding-plan/glm-5"
    }
  }
}
```

继承后，temperature等参数从category继承，除非Agent级别显式覆盖。

---

## 七、下一步补充内容

待后台调查任务完成后，补充：

1. **Skills系统详细设计**
   - 每个Agent的Skills列表
   - Skill触发条件
   - Skill执行流程

2. **Hooks系统详细设计**
   - Hook类型和触发时机
   - 每个Agent的Hooks配置
   - Hook执行顺序

3. **工具系统设计**
   - 向量搜索工具
   - 知识库查询工具
   - MCP服务器集成

4. **Agent协作流程图**
   - Sisyphus式编排模式
   - Session传递机制
   - 结果收集与验证

---

**文档版本**: v2.2
**更新时间**: 2026-03-15