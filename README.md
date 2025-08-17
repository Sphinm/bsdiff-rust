# 🚀 bsdiff-rust

A high-performance Rust implementation of the bsdiff and bspatch algorithms with Node.js bindings

[![npm version](https://badge.fury.io/js/bsdiff-rust.svg)](https://badge.fury.io/js/bsdiff-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/your-username/bsdiff-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/your-username/bsdiff-rust/actions/workflows/rust.yml)

## ✨ 特性

- 🔥 **高性能**: 使用 Rust 实现，性能优异
- 🛡️ **内存安全**: Rust 保证内存安全和线程安全
- 📦 **二进制补丁**: 生成和应用二进制文件补丁
- 🗜️ **压缩支持**: 使用 bzip2 压缩补丁文件
- 🔄 **异步支持**: 提供同步和异步 API
- 🎯 **跨平台**: 支持 Windows、macOS、Linux
- 📱 **Node.js 绑定**: 完整的 JavaScript/TypeScript 支持

## 🚀 快速开始

### 安装

```bash
npm install bsdiff-rust
```

### 基本用法

```javascript
const bsdiff = require('bsdiff-rust')

// 同步 API
bsdiff.diffSync('old-file.zip', 'new-file.zip', 'patch.bin')
bsdiff.patchSync('old-file.zip', 'generated-file.zip', 'patch.bin')

// 异步 API
await bsdiff.diff('old-file.zip', 'new-file.zip', 'patch.bin')
await bsdiff.patch('old-file.zip', 'generated-file.zip', 'patch.bin')
```

### TypeScript 支持

```typescript
import { diff, diffSync, patch, patchSync } from 'bsdiff-rust'

// 使用 TypeScript 类型
await diff('old-file.zip', 'new-file.zip', 'patch.bin')
diffSync('old-file.zip', 'new-file.zip', 'patch.bin')
```

## 📖 API 文档

### 同步 API

#### `diffSync(oldFile: string, newFile: string, patchFile: string): void`

生成两个文件之间的补丁。

- `oldFile`: 旧文件路径
- `newFile`: 新文件路径
- `patchFile`: 补丁文件输出路径

#### `patchSync(oldFile: string, newFile: string, patchFile: string): void`

应用补丁到旧文件，生成新文件。

- `oldFile`: 旧文件路径
- `newFile`: 新文件输出路径
- `patchFile`: 补丁文件路径

### 异步 API

#### `diff(oldFile: string, newFile: string, patchFile: string): Promise<void>`

异步生成补丁。

#### `patch(oldFile: string, newFile: string, patchFile: string): Promise<void>`

异步应用补丁。

## 🧪 测试

运行测试：

```bash
npm test
```

测试包括：

- ✅ 同步 API 测试
- ✅ 异步 API 测试
- ✅ 错误处理测试
- ✅ API 兼容性测试
- ✅ 性能测试（使用真实 React 版本文件）

## 🔧 开发

### 环境要求

- Node.js >= 16
- Rust >= 1.70
- pnpm (推荐) 或 npm

### 构建

```bash
# 安装依赖
pnpm install

# 构建发布版本
pnpm build

# 构建调试版本
pnpm build:debug
```

### 本地开发

```bash
# 运行测试
pnpm test

# 代码格式化
pnpm format

# 代码检查
pnpm lint
```

## 📊 性能对比

| 场景           | 文件大小          | 处理时间 | 补丁大小 |
| -------------- | ----------------- | -------- | -------- |
| React 版本更新 | 1.31 MB → 1.86 MB | ~560ms   | 785 KB   |
| 小文件测试     | 100KB             | ~5ms     | - |

## 🏗️ 技术架构

### 后端 (Rust)

- **bsdiff = "0.2.1"**: 成熟的 bsdiff 算法库
- **bzip2 = "0.4"**: 压缩库
- **NAPI-RS**: Node.js 绑定框架

### 前端 (Node.js)

- **TypeScript**: 类型支持
- **Mocha**: 测试框架
- **跨平台支持**: Windows, macOS, Linux

## 🔄 从 C 到 Rust 的迁移

本项目成功将原始的 C 实现迁移到 Rust：

### 迁移成果

- ✅ **代码简化**: 从 2000+ 行 C 代码减少到 200 行 Rust 代码
- ✅ **性能提升**: 更小的二进制文件和更好的性能
- ✅ **安全性**: 内存安全和类型安全保证
- ✅ **维护性**: 更简洁的代码，更少的错误

### 技术优势

- 🛡️ **内存安全**: Rust 保证无内存泄漏
- 🔒 **类型安全**: 编译时错误检查
- ⚡ **零成本抽象**: 高性能保证
- 🎯 **现代化**: 使用最新的技术栈

## 📁 项目结构

```
bsdiff-rust/
├── src/
│   ├── lib.rs              # NAPI 绑定
│   └── bsdiff_rust.rs      # 纯 Rust 实现
├── test/
│   ├── test.js             # 测试文件
│   └── resources/          # 测试资源
│       ├── react-0.3-stable.zip
│       └── react-0.4-stable.zip
├── index.js                # Node.js 入口
├── Cargo.toml              # Rust 配置
├── package.json            # Node.js 配置
└── *.node                  # 原生模块
```

## 🤝 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详情。

### 开发流程

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [bsdiff](http://www.daemonology.net/bsdiff/) - 原始算法
- [NAPI-RS](https://napi.rs/) - Node.js 绑定框架
- [Rust](https://www.rust-lang.org/) - 系统编程语言

## 📞 支持

如果您遇到问题或有建议，请：

- 📧 发送邮件到: sumin1500160640@gmail.com

---

⭐ 如果这个项目对您有帮助，请给它一个星标！
