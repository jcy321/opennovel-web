# Phase 6: 热重载机制

**文档版本**: 1.0
**最后更新**: 2026-03-16

---

## 1. 概述

热重载机制允许用户在 Web 端修改 Provider 和模型配置后，无需重启服务即可生效。

### 核心功能
- **WebSocket 实时推送**: 配置变更时立即通知所有客户端
- **轮询 Fallback**: WebSocket 断开时自动降级到轮询
- **版本号机制**: 客户端缓存版本号，只在变化时拉取
- **缓存失效**: Redis 缓存自动失效

---

## 2. 架构设计

### 2.1 整体架构

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           Web 端用户                                     │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    Provider 配置页面                              │   │
│  │                 POST /api/providers                              │   │
│  └─────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────┘
                                     │
                                     ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         OpenNovel Server                                │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                 │
│  │   REST API  │───▶│   Storage   │───▶│   Redis     │                 │
│  │   Handler   │    │  (PostgreSQL)│   │  (Cache)    │                 │
│  └─────────────┘    └─────────────┘    └─────────────┘                 │
│         │                                      │                        │
│         │                                      │                        │
│         ▼                                      ▼                        │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                 │
│  │  WebSocket  │◀───│   Redis     │◀───│   Config    │                 │
│  │   Server    │    │   Pub/Sub   │    │   Manager   │                 │
│  └─────────────┘    └─────────────┘    └─────────────┘                 │
│         │                                                              │
└─────────┼────────────────────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         WebSocket 连接                                   │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                 │
│  │   Client A  │    │   Client B  │    │   Client C  │                 │
│  │  (实时推送)  │    │  (实时推送)  │    │  (轮询降级)  │                 │
│  └─────────────┘    └─────────────┘    └─────────────┘                 │
└─────────────────────────────────────────────────────────────────────────┘
```

### 2.2 数据流

```
配置变更流程：
1. 用户修改配置 → POST /api/providers
2. API Handler → 验证并保存到 PostgreSQL
3. Storage → 发布事件到 Redis Pub/Sub
4. Redis Pub/Sub → 通知 WebSocket Server
5. WebSocket Server → 推送更新到所有连接的客户端
6. 客户端 → 更新本地缓存

配置读取流程：
1. 应用请求配置 → ModelResolver
2. ModelResolver → 先查 Redis 缓存
3. Redis Miss → 回源 PostgreSQL
4. PostgreSQL → 返回配置并更新 Redis
```

---

## 3. WebSocket 实现

### 3.1 服务端

```typescript
// packages/llm/src/config/websocket-server.ts

import { WebSocketServer, WebSocket } from 'ws'
import Redis from 'ioredis'
import { Logger } from '../utils/logger'

const log = new Logger('WebSocketServer')

interface Client {
  ws: WebSocket
  subscriptions: Set<string>
}

export class HotReloadWebSocketServer {
  private wss: WebSocketServer
  private redis: Redis
  private clients: Map<WebSocket, Client> = new Map()
  private configVersion: number = 0
  
  constructor(port: number, redis: Redis) {
    this.redis = redis
    this.wss = new WebSocketServer({ port })
    
    this.setupWebSocket()
    this.setupRedisSubscription()
    
    log.info(`WebSocket server started on port ${port}`)
  }
  
  private setupWebSocket(): void {
    this.wss.on('connection', (ws) => {
      const client: Client = {
        ws,
        subscriptions: new Set(),
      }
      this.clients.set(ws, client)
      
      log.debug(`Client connected. Total: ${this.clients.size}`)
      
      // 发送当前配置版本
      ws.send(JSON.stringify({
        type: 'version',
        version: this.configVersion,
      }))
      
      ws.on('message', (data) => {
        try {
          const message = JSON.parse(data.toString())
          this.handleMessage(client, message)
        } catch (error) {
          log.warn('Invalid message:', error)
        }
      })
      
      ws.on('close', () => {
        this.clients.delete(ws)
        log.debug(`Client disconnected. Total: ${this.clients.size}`)
      })
      
      ws.on('error', (error) => {
        log.error('WebSocket error:', error)
        this.clients.delete(ws)
      })
    })
  }
  
  private handleMessage(client: Client, message: any): void {
    switch (message.type) {
      case 'subscribe':
        client.subscriptions.add(message.channel)
        break
        
      case 'unsubscribe':
        client.subscriptions.delete(message.channel)
        break
        
      case 'ping':
        client.ws.send(JSON.stringify({ type: 'pong' }))
        break
    }
  }
  
  private setupRedisSubscription(): void {
    const subscriber = this.redis.duplicate()
    
    subscriber.subscribe('config:update', (err) => {
      if (err) {
        log.error('Failed to subscribe to Redis:', err)
        return
      }
      log.info('Subscribed to config:update channel')
    })
    
    subscriber.on('message', (channel, message) => {
      if (channel !== 'config:update') return
      
      try {
        const data = JSON.parse(message)
        this.broadcastUpdate(data)
      } catch (error) {
        log.error('Failed to parse Redis message:', error)
      }
    })
  }
  
  private broadcastUpdate(data: any): void {
    // 更新配置版本
    this.configVersion = Date.now()
    
    const message = JSON.stringify({
      type: 'config_update',
      version: this.configVersion,
      data,
      timestamp: Date.now(),
    })
    
    // 广播给所有客户端
    for (const [ws, client] of this.clients) {
      if (ws.readyState === WebSocket.OPEN) {
        ws.send(message)
      }
    }
    
    log.info(`Broadcast update to ${this.clients.size} clients`)
  }
  
  /**
   * 手动触发配置更新广播
   */
  async triggerUpdate(type: string, id: string): Promise<void> {
    await this.redis.publish('config:update', JSON.stringify({
      type,
      id,
      timestamp: Date.now(),
    }))
  }
}
```

### 3.2 客户端

```typescript
// packages/llm/src/config/websocket-client.ts

import { Logger } from '../utils/logger'

const log = new Logger('WebSocketClient')

interface HotReloadClientOptions {
  url: string
  onConfigUpdate: (data: any) => void
  onConnectionChange?: (connected: boolean) => void
  pollInterval?: number  // 轮询间隔（毫秒）
  pollFallback?: boolean // 是否启用轮询降级
}

export class HotReloadClient {
  private ws: WebSocket | null = null
  private options: HotReloadClientOptions
  private reconnectAttempts: number = 0
  private maxReconnectAttempts: number = 5
  private reconnectDelay: number = 1000
  private pollTimer: NodeJS.Timeout | null = null
  private localVersion: number = 0
  private isPolling: boolean = false
  
  constructor(options: HotReloadClientOptions) {
    this.options = {
      pollInterval: 5000,
      pollFallback: true,
      ...options,
    }
    
    this.connect()
  }
  
  private connect(): void {
    try {
      this.ws = new WebSocket(this.options.url)
      
      this.ws.onopen = () => {
        log.info('WebSocket connected')
        this.reconnectAttempts = 0
        this.stopPolling()
        this.options.onConnectionChange?.(true)
      }
      
      this.ws.onmessage = (event) => {
        try {
          const message = JSON.parse(event.data)
          this.handleMessage(message)
        } catch (error) {
          log.warn('Invalid message:', error)
        }
      }
      
      this.ws.onclose = () => {
        log.warn('WebSocket disconnected')
        this.options.onConnectionChange?.(false)
        this.handleDisconnect()
      }
      
      this.ws.onerror = (error) => {
        log.error('WebSocket error:', error)
      }
      
    } catch (error) {
      log.error('Failed to connect:', error)
      this.handleDisconnect()
    }
  }
  
  private handleMessage(message: any): void {
    switch (message.type) {
      case 'version':
        this.localVersion = message.version
        break
        
      case 'config_update':
        this.localVersion = message.version
        this.options.onConfigUpdate(message.data)
        break
        
      case 'pong':
        // Heartbeat response
        break
    }
  }
  
  private handleDisconnect(): void {
    // 尝试重连
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++
      const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1)
      
      log.info(`Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts})`)
      
      setTimeout(() => {
        this.connect()
      }, delay)
      
    } else {
      // 重连失败，启用轮询降级
      if (this.options.pollFallback && !this.isPolling) {
        log.warn('WebSocket reconnection failed, falling back to polling')
        this.startPolling()
      }
    }
  }
  
  private startPolling(): void {
    if (this.isPolling) return
    
    this.isPolling = true
    log.info('Started polling fallback')
    
    this.pollTimer = setInterval(async () => {
      await this.pollConfig()
    }, this.options.pollInterval)
  }
  
  private stopPolling(): void {
    if (this.pollTimer) {
      clearInterval(this.pollTimer)
      this.pollTimer = null
    }
    this.isPolling = false
  }
  
  private async pollConfig(): Promise<void> {
    try {
      const response = await fetch('/api/config/version')
      const data = await response.json()
      
      if (data.version > this.localVersion) {
        log.info('Config version changed, fetching update')
        
        const configResponse = await fetch('/api/config')
        const config = await configResponse.json()
        
        this.localVersion = data.version
        this.options.onConfigUpdate(config)
      }
    } catch (error) {
      log.error('Polling failed:', error)
    }
  }
  
  /**
   * 关闭连接
   */
  close(): void {
    this.stopPolling()
    if (this.ws) {
      this.ws.close()
      this.ws = null
    }
  }
}
```

---

## 4. 版本号机制

### 4.1 版本号存储

```typescript
// packages/llm/src/config/version-manager.ts

import Redis from 'ioredis'

export class ConfigVersionManager {
  private redis: Redis
  private readonly VERSION_KEY = 'config:version'
  private readonly VERSION_TTL = 3600 // 1小时
  
  constructor(redis: Redis) {
    this.redis = redis
  }
  
  /**
   * 获取当前版本号
   */
  async getVersion(): Promise<number> {
    const version = await this.redis.get(this.VERSION_KEY)
    return version ? parseInt(version, 10) : 0
  }
  
  /**
   * 递增版本号
   */
  async incrementVersion(): Promise<number> {
    const newVersion = await this.redis.incr(this.VERSION_KEY)
    await this.redis.expire(this.VERSION_KEY, this.VERSION_TTL)
    return newVersion
  }
  
  /**
   * 设置版本号
   */
  async setVersion(version: number): Promise<void> {
    await this.redis.set(this.VERSION_KEY, version.toString())
    await this.redis.expire(this.VERSION_KEY, this.VERSION_TTL)
  }
}
```

### 4.2 API 端点

```typescript
// 在 REST API 中添加版本检查端点

/**
 * GET /api/config/version
 * 返回当前配置版本号
 */
app.get('/api/config/version', async (req, res) => {
  const version = await versionManager.getVersion()
  res.json({ version })
})

/**
 * GET /api/config
 * 返回完整配置
 */
app.get('/api/config', async (req, res) => {
  const providers = await providerRegistry.listEnabled()
  const version = await versionManager.getVersion()
  
  res.json({
    version,
    providers,
  })
})
```

---

## 5. 缓存失效

### 5.1 Redis 缓存策略

```typescript
// packages/llm/src/config/cache-manager.ts

import Redis from 'ioredis'
import { Logger } from '../utils/logger'

const log = new Logger('CacheManager')

export class ConfigCacheManager {
  private redis: Redis
  private readonly PROVIDER_PREFIX = 'provider:'
  private readonly MODEL_PREFIX = 'model:'
  private readonly TTL = 300 // 5分钟
  
  constructor(redis: Redis) {
    this.redis = redis
  }
  
  /**
   * 缓存 Provider 配置
   */
  async cacheProvider(providerId: string, config: any): Promise<void> {
    const key = `${this.PROVIDER_PREFIX}${providerId}`
    await this.redis.setex(key, this.TTL, JSON.stringify(config))
    log.debug(`Cached provider: ${providerId}`)
  }
  
  /**
   * 获取缓存的 Provider 配置
   */
  async getProvider(providerId: string): Promise<any | null> {
    const key = `${this.PROVIDER_PREFIX}${providerId}`
    const cached = await this.redis.get(key)
    
    if (cached) {
      log.debug(`Cache hit for provider: ${providerId}`)
      return JSON.parse(cached)
    }
    
    log.debug(`Cache miss for provider: ${providerId}`)
    return null
  }
  
  /**
   * 使 Provider 缓存失效
   */
  async invalidateProvider(providerId: string): Promise<void> {
    const key = `${this.PROVIDER_PREFIX}${providerId}`
    await this.redis.del(key)
    log.info(`Invalidated cache for provider: ${providerId}`)
  }
  
  /**
   * 使所有缓存失效
   */
  async invalidateAll(): Promise<void> {
    const keys = await this.redis.keys(`${this.PROVIDER_PREFIX}*`)
    
    if (keys.length > 0) {
      await this.redis.del(...keys)
      log.info(`Invalidated ${keys.length} cache entries`)
    }
  }
}
```

### 5.2 自动失效触发

```typescript
// 在 Storage 层自动触发缓存失效

export class ConfigStorage {
  // ... 其他代码
  
  async saveProvider(config: ProviderConfig): Promise<void> {
    // ... 保存到 PostgreSQL
    
    // 使缓存失效
    await this.cacheManager.invalidateProvider(config.id)
    
    // 递增版本号
    await this.versionManager.incrementVersion()
    
    // 发布更新事件
    await this.publishUpdate('provider', config.id)
  }
  
  async deleteProvider(id: string): Promise<void> {
    // ... 从 PostgreSQL 删除
    
    // 使缓存失效
    await this.cacheManager.invalidateProvider(id)
    
    // 递增版本号
    await this.versionManager.incrementVersion()
    
    // 发布更新事件
    await this.publishUpdate('provider', id)
  }
}
```

---

## 6. 配置更新流程

### 6.1 完整流程图

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        配置更新完整流程                                   │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  1. Web 端修改配置                                                       │
│     │                                                                   │
│     ▼                                                                   │
│  2. POST /api/providers                                                 │
│     │                                                                   │
│     ▼                                                                   │
│  3. API Handler 验证配置                                                 │
│     │                                                                   │
│     ▼                                                                   │
│  4. 保存到 PostgreSQL                                                   │
│     │                                                                   │
│     ├──────────────────────┬──────────────────────┐                    │
│     ▼                      ▼                      ▼                    │
│  5a. 使 Redis 缓存失效   5b. 递增版本号         5c. 发布 Redis Pub/Sub   │
│     │                      │                      │                    │
│     └──────────────────────┴──────────────────────┘                    │
│                                 │                                      │
│                                 ▼                                      │
│  6. WebSocket Server 接收 Pub/Sub 消息                                 │
│     │                                                                   │
│     ▼                                                                   │
│  7. 广播更新到所有 WebSocket 客户端                                      │
│     │                                                                   │
│     ├──────────────────────┬──────────────────────┐                    │
│     ▼                      ▼                      ▼                    │
│  8a. Client A 接收      8b. Client B 接收      8c. Client C 断线        │
│     │                      │                      │                    │
│     ▼                      ▼                      ▼                    │
│  9a. 更新本地配置       9b. 更新本地配置      9c. 轮询检测到版本变化      │
│     │                      │                      │                    │
│     └──────────────────────┴──────────────────────┘                    │
│                                 │                                      │
│                                 ▼                                      │
│  10. 应用使用新配置                                                      │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 7. 相关文档

- [02-provider-system.md](./02-provider-system.md) - Provider 注册与管理
- [03-model-resolution.md](./03-model-resolution.md) - 模型解析与 Fallback
- [05-api-design.md](./05-api-design.md) - API 设计