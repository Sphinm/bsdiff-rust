# 🚀 bsdiff-rust

一个基于 Rust + NAPI-RS 的高性能二进制差分补丁库，为 Node.js 提供优化的 bsdiff/bspatch 算法实现。支持标准 BSDIFF40 格式，采用后缀数组算法和并行处理实现

[![npm version](https://badge.fury.io/js/@bsdiff-rust%2Fnode.svg)](https://badge.fury.io/js/@bsdiff-rust%2Fnode)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## ✨ 核心特性

- **标准兼容**: 生成标准 BSDIFF40 标准格式补丁，与 bsdiff-node 完全兼容
- **内存安全**: Rust 保证内存安全和线程安全，基于 napi-rs 的高性能 Node.js 绑定
- **优化压缩**: 使用 bzip2 压缩，配合内存预分配优化
- **跨平台**: 支持 Windows、macOS、Linux 多平台


## 🚀 快速开始

### 安装

```bash
pnpm install @bsdiff-rust/node
```

### 基本用法

```javascript
const bsdiff = require('@bsdiff-rust/node')

// 同步 API
bsdiff.diffSync('old-file.zip', 'new-file.zip', 'patch.bin')
bsdiff.patchSync('old-file.zip', 'generated-file.zip', 'patch.bin')

// 异步 API
await bsdiff.diff('old-file.zip', 'new-file.zip', 'patch.bin')
await bsdiff.patch('old-file.zip', 'generated-file.zip', 'patch.bin')
```

### TypeScript 支持

```typescript
import { diff, diffSync, patch, patchSync } from '@bsdiff-rust/node'

// 生成和应用补丁
await diff('old-file.zip', 'new-file.zip', 'patch.bin')
await patch('old-file.zip', 'generated-file.zip', 'patch.bin')
```

## 📖 API 文档

### 核心 API

```typescript
// 同步方法
diffSync(oldFile: string, newFile: string, patchFile: string): void
patchSync(oldFile: string, newFile: string, patchFile: string): void

// 异步方法
diff(oldFile: string, newFile: string, patchFile: string): Promise<void>
patch(oldFile: string, newFile: string, patchFile: string): Promise<void>
```

## 🔧 开发指南

### 环境要求

- **Node.js**: >= 20
- **Rust**: >= 1.88（napi v3 依赖要求）
- **包管理器**: npm 或 pnpm

### 构建项目

```bash
# 安装依赖
pnpm install

# 构建发布版本
pnpm build

# 构建调试版本
pnpm build:debug

```

### 项目结构

```
bsdiff-rust/
├── core/                   # bsdiff-core：平台无关引擎 + C FFI
│   └── src/
│       ├── bsdiff.rs       # diff/patch 实现
│       ├── utils.rs        # 文件信息、校验、压缩比
│       └── ffi.rs          # extern "C" 导出（C / 静态链接）
├── bindings/
│   └── node/               # napi-rs 绑定 -> node.<platform>.node
├── benchmark/
│   └── benchmark.ts        # TypeScript 基准测试
├── test/
│   ├── index.ts            # 功能测试
│   └── resources/          # 测试资源文件（未入库）
├── index.js                # Node.js 入口（生成物）
├── index.d.ts              # TypeScript 类型定义（生成物）
├── Cargo.toml              # Cargo workspace 清单
└── package.json            # Node.js 项目配置
```

## 🌍 跨平台支持

### 支持的平台

- **macOS**: ARM64 (Apple Silicon) 和 x64 (Intel)
- **Linux**: ARM64 和 x64 (GNU 和 musl)
- **Windows**: ARM64 和 x64 (MSVC)

### 平台包策略

本项目采用 napi-rs 的多包策略，用户安装时会自动下载对应平台的预编译二进制文件：

```
npm/
├── @bsdiff-rust/darwin-arm64/     # macOS ARM64
├── @bsdiff-rust/darwin-x64/       # macOS x64
├── @bsdiff-rust/linux-arm64-gnu/  # Linux ARM64 glibc
├── @bsdiff-rust/linux-x64-gnu/    # Linux x64 glibc
├── @bsdiff-rust/linux-arm64-musl/ # Linux ARM64 musl
├── @bsdiff-rust/linux-x64-musl/   # Linux x64 musl
└── ...
```

**优势**:

- **快速安装**: 无需编译，直接下载预编译二进制
- **按需下载**: 只下载当前平台所需的文件
- **稳定可靠**: 避免编译环境问题导致的安装失败

## 🤝 贡献指南

### 开发流程

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

### 代码规范

- **Rust 代码**: 使用 `cargo fmt` 格式化
- **JavaScript/TypeScript**: 使用 Prettier 格式化
- **提交信息**: 使用清晰的英文描述

## 📚 参考资料

- [完整指南](./docs/GUIDE-ZH.md) - API 参考、测试、性能基准
- [bsdiff 原始算法](http://www.daemonology.net/bsdiff/) - Colin Percival 的原始实现
- [NAPI-RS 文档](https://napi.rs/) - Node.js 绑定框架
- [qbsdiff 库](https://crates.io/crates/qbsdiff) - 底层 Rust 实现

---

⭐ 如果这个项目对您有帮助，请给它一个星标！

🐛 发现问题？欢迎提交 [Issue](https://github.com/Sphinm/bsdiff-rust/issues)

💡 有改进建议？欢迎提交 [Pull Request](https://github.com/Sphinm/bsdiff-rust/pulls)
