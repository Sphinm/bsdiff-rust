# 🚀 bsdiff-rust

A high-performance Rust implementation of the bsdiff and bspatch algorithms with Node.js bindings

[![npm version](https://badge.fury.io/js/bsdiff-rust.svg)](https://badge.fury.io/js/bsdiff-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## ✨ 特性

- 🔥 **高性能**: 使用 Rust 实现，性能优异
- 🛡️ **内存安全**: Rust 保证内存安全和线程安全
- 📦 **二进制补丁**: 生成和应用二进制文件补丁
- 🗜️ **压缩支持**: 使用 zstd 压缩补丁文件
- 🔄 **异步支持**: 提供同步和异步 API
- ✅ **完整性验证**: 补丁文件完整性验证
- 📊 **压缩比分析**: 详细的压缩比和文件大小信息
- 🔍 **文件检查**: 文件存在性和访问权限验证
- 🎯 **跨平台**: 支持 Windows、macOS、Linux
- 📱 **Node.js 绑定**: 完整的 JavaScript/TypeScript 支持

TODO: Windows 构建时间过长，先忽略

## 🚀 快速开始

### 安装

```js
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

// 高级功能
const isValid = bsdiff.verifyPatchSync('old-file.zip', 'new-file.zip', 'patch.bin')
const info = bsdiff.getPatchInfoSync('patch.bin')
const ratio = bsdiff.getCompressionRatioSync('old-file.zip', 'new-file.zip', 'patch.bin')

console.log(`补丁大小: ${(info.size / 1024).toFixed(2)} KB`)
console.log(`压缩比: ${ratio.ratio.toFixed(2)}%`)
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

```js
diffSync(oldFile: string, newFile: string, patchFile: string): void
```

生成两个文件之间的补丁。

- `oldFile`: 旧文件路径
- `newFile`: 新文件路径
- `patchFile`: 补丁文件输出路径

```js
patchSync(oldFile: string, newFile: string, patchFile: string): void
```

应用补丁到旧文件，生成新文件。

- `oldFile`: 旧文件路径
- `newFile`: 新文件输出路径
- `patchFile`: 补丁文件路径

### 异步 API

```js
diff(oldFile: string, newFile: string, patchFile: string): Promise<void>
```

异步生成补丁。

```js
patch(oldFile: string, newFile: string, patchFile: string): Promise<void>
```

异步应用补丁。

```js
verifyPatch(oldFile: string, newFile: string, patchFile: string): Promise<boolean>
```

异步验证补丁完整性。

### 高级功能 API

```js
verifyPatchSync(oldFile: string, newFile: string, patchFile: string): boolean
```

同步验证补丁完整性。

```js
getPatchInfoSync(patchFile: string): PatchInfo
```

获取补丁文件信息。

```js
getFileSizeSync(filePath: string): number
```

获取文件大小（字节）。

```js
checkFileAccessSync(filePath: string): void
```

检查文件是否存在且可读。

```js
getCompressionRatioSync(oldFile: string, newFile: string, patchFile: string): CompressionRatio
```

计算压缩比信息。

### 数据结构

```typescript
interface PatchInfo {
  size: number // 补丁文件大小（字节）
  compressed: boolean // 是否压缩
}

interface CompressionRatio {
  oldSize: number // 旧文件大小（字节）
  newSize: number // 新文件大小（字节）
  patchSize: number // 补丁文件大小（字节）
  ratio: number // 压缩比（百分比）
}
```

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
| 小文件测试     | 100KB             | ~5ms     | -        |

## 🏗️ 技术架构

### 后端 (Rust)

- **bsdiff = "0.2.1"**: 成熟的 bsdiff 算法库
- **zstd = "0.13"**: 压缩库
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

## 🌍 跨平台编译方案对比

本项目采用现代化的 Rust + napi-rs 方案，相比传统的 node-gyp 方案有显著优势。

### 技术栈对比

| 项目            | 技术栈           | 跨平台方案   | 优势     | 劣势           |
| --------------- | ---------------- | ------------ | -------- | -------------- |
| **bsdiff-node** | C/C++ + node-gyp | 源码编译     | 成熟稳定 | 需要编译环境   |
| **bsdiff-rust** | Rust + napi-rs   | 预编译二进制 | 性能更好 | 需要管理多个包 |

### bsdiff-node (传统方案)

**技术特点：**

- 使用 C/C++ 实现 (95.3% C + 3.7% C++)
- 依赖 node-gyp 进行跨平台编译
- 需要 Python 3.x 和 C++ 编译器
- 单一 npm 包管理

**优点：**

- ✅ **单一包**：用户只需要安装一个包
- ✅ **自动适配**：node-gyp 自动处理平台差异
- ✅ **成熟稳定**：node-gyp 是 Node.js 生态的标准方案

**缺点：**

- ❌ **编译时间长**：每次安装都需要编译
- ❌ **环境依赖**：需要 Python、C++ 编译器
- ❌ **安装失败率高**：编译环境问题导致安装失败
- ❌ **性能一般**：C/C++ 实现，不如 Rust 优化

### bsdiff-rust (现代化方案)

**技术特点：**

- 使用 Rust 实现，性能优异
- 采用 napi-rs 进行 Node.js 绑定
- 预编译二进制，无需编译环境
- 多包策略，按需下载

**优点：**

- ✅ **性能优异**：Rust 实现，性能更好
- ✅ **安装快速**：预编译二进制，无需编译
- ✅ **内存安全**：Rust 保证内存安全
- ✅ **现代化**：使用最新的技术栈

**缺点：**

- ❌ **包管理复杂**：需要管理多个平台包
- ❌ **包体积**：虽然按需下载，但需要维护多个包

### 为什么选择 Rust + napi-rs

1. **性能优势**：Rust 实现比 C/C++ 更安全、性能更好
2. **用户体验**：预编译二进制安装更快、更稳定
3. **维护性**：Rust 代码更易维护，bug 更少
4. **现代化**：符合当前 Node.js 生态的发展趋势

### 多包策略的优势

本项目采用 napi-rs 的多包策略：

```
npm/
├── darwin-arm64/          # macOS ARM64 平台包
├── darwin-x64/           # macOS x64 平台包
├── linux-arm64-gnu/      # Linux ARM64 glibc 平台包
├── linux-x64-gnu/        # Linux x64 glibc 平台包
├── win32-arm64-msvc/     # Windows ARM64 平台包
├── win32-x64-msvc/       # Windows x64 平台包
└── ...                   # 其他平台
```

**用户安装体验：**

```bash
# 用户在 macOS ARM64 上运行
npm install bsdiff-rust

# npm 自动安装：
# 1. bsdiff-rust@1.0.3 (主包)
# 2. bsdiff-rust-darwin-arm64@1.0.3 (平台包)
```

**优势：**

- 🚀 **按需下载**：用户只下载对应平台的二进制文件
- ⚡ **安装快速**：无需编译，直接下载预编译二进制
- 🛡️ **稳定可靠**：避免编译环境问题导致的安装失败
- 📦 **体积优化**：减小了总体包体积

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

### 开发流程

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开 Pull Request

## Reference

- [bsdiff](http://www.daemonology.net/bsdiff/) - 原始算法
- [NAPI-RS](https://napi.rs/) - Node.js 绑定框架
- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [bsdiff-node](https://github.com/gaetandezeiraud/bsdiff-node) - 传统 C/C++ 实现（已归档）

---

⭐ 如果这个项目对您有帮助，请给它一个星标！
