# OpenNovel Web

> **OpenNovel Web 前端** - 基于 SvelteKit 构建的 Web 客户端

OpenNovel Web 是 OpenNovel 项目的 Web 前端，提供浏览器访问方式，与后端服务 (opennovel-core) 配合使用。

## ✨ 特性

- **浏览器访问**: 无需安装客户端，直接通过浏览器使用
- **群聊界面**: 类似即时通讯的群聊 UI，与多个 AI Agent 协作创作
- **Agent 状态面板**: 实时查看 8 个 Agent 的状态和任务
- **知识库浏览**: 查看人物、世界观、剧情、伏笔等数据
- **流式输出**: SSE 实时显示 Agent 回复

## 🚀 快速开始

### 前置要求

- Node.js 18+
- 后端服务运行中 (opennovel-core)

### 安装

```bash
# 克隆仓库
git clone https://github.com/jcy321/opennovel-web.git
cd opennovel-web

# 安装依赖
cd apps/web/frontend
npm install
```

### 开发

```bash
# 启动开发服务器
npm run dev

# 访问 http://localhost:5173
```

### 构建

```bash
npm run build
```

## 📁 项目结构

```
opennovel-web/
├── apps/
│   └── web/
│       └── frontend/           # SvelteKit 前端
│           ├── src/
│           │   ├── lib/
│           │   │   ├── components/  # UI 组件
│           │   │   │   ├── IdeLayout.svelte
│           │   │   │   ├── ActivityBar.svelte
│           │   │   │   ├── Sidebar.svelte
│           │   │   │   ├── ChatPage.svelte
│           │   │   │   └── ...
│           │   │   ├── stores/      # 状态管理
│           │   │   └── api/         # API 客户端
│           │   └── routes/          # 页面路由
│           └── package.json
│
└── docs/                       # 设计文档
    ├── ARCHITECTURE_DESIGN_V2.md
    ├── COMPONENT_SYSTEM.md
    └── ...
```

## 🛠️ 技术栈

| 组件 | 技术 |
|------|------|
| 框架 | SvelteKit |
| 样式 | Tailwind CSS v4 |
| 状态管理 | Svelte Stores |
| HTTP 客户端 | fetch API |
| 实时通信 | SSE (Server-Sent Events) |

## 🔗 相关仓库

| 仓库 | 说明 |
|------|------|
| [opennovel-core](https://github.com/jcy321/opennovel-core) | 后端服务 (Rust + Axum) |
| [opennovel-ide](https://github.com/jcy321/opennovel-ide) | 桌面客户端 (Electron) |
| [opennovel-hub](https://github.com/jcy321/opennovel-hub) | 项目主页 |

## 📦 部署

### Web 模式

后端服务需要以 Web 模式启动：

```bash
# 启动后端 (Web 模式，端口 80)
opennovel --web
```

### 配置

修改 `src/lib/api/config.ts` 设置后端地址：

```typescript
export const API_BASE_URL = 'http://your-server:80';
```

## 🎨 UI 组件

### 主要组件

| 组件 | 说明 |
|------|------|
| `IdeLayout.svelte` | 主布局 |
| `ActivityBar.svelte` | 活动栏 |
| `Sidebar.svelte` | 侧边栏 |
| `ChatPage.svelte` | 群聊页面 |
| `MessageList.svelte` | 消息列表 |
| `AgentStatusPanel.svelte` | Agent 状态 |
| `KnowledgePanel.svelte` | 知识库浏览 |

### Agent 头像

| Agent | 头像 | 颜色 |
|-------|------|------|
| 天道 | [道] | 紫色 |
| 刘和平 | [刘] | 蓝色 |
| 世界观守护者 | [世] | 绿色 |
| 执笔 | [笔] | 橙色 |
| 审阅 | [阅] | 青色 |
| 观察者 | [观] | 灰色 |
| 规划者 | [规] | 黄色 |
| 调研者 | [研] | 粉色 |

## 📄 许可证

本项目采用 **AGPL-3.0** 许可证开源。

---

**注意**: 本仓库仅包含前端代码，后端服务请访问 [opennovel-core](https://github.com/jcy321/opennovel-core)。