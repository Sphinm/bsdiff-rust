# 🚀 bsdiff-rust

一个高性能的 Rust 实现的 bsdiff 和 bspatch 算法库，提供完整的 Node.js 绑定

[![npm version](https://badge.fury.io/js/@bsdiff-rust%2Fnode.svg)](https://badge.fury.io/js/@bsdiff-rust%2Fnode)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## ✨ 核心特性

- **极致性能**: 使用 Rust 实现，配合内存映射和并行处理
- **内存安全**: Rust 保证内存安全和线程安全
- **二进制补丁**: 高效生成和应用二进制文件补丁
- **智能压缩**: 内置 zstd 压缩，最优压缩比配置
- **双重API**: 提供同步和异步 API，满足不同使用场景
- **完整性验证**: 内置补丁文件完整性验证机制
- **详细分析**: 压缩比分析和文件大小统计
- **智能检查**: 文件存在性和访问权限自动验证
- **跨平台**: 支持 Windows、macOS、Linux 多平台
- **现代化绑定**: 基于 napi-rs 的高性能 Node.js 绑定

## 🚀 快速开始

### 安装

```bash
pnpm install @bsdiff-rust/node
```

### 基本用法

```javascript
const bsdiff = require('@bsdiff-rust/node')

// 同步 API - 适合简单场景
bsdiff.diffSync('old-file.zip', 'new-file.zip', 'patch.bin')
bsdiff.patchSync('old-file.zip', 'generated-file.zip', 'patch.bin')

// 异步 API - 适合大文件和生产环境
await bsdiff.diff('old-file.zip', 'new-file.zip', 'patch.bin')
await bsdiff.patch('old-file.zip', 'generated-file.zip', 'patch.bin')
```

### TypeScript 支持

```typescript
import {
  diff,
  diffSync,
  patch,
  patchSync,
  verifyPatch,
  verifyPatchSync,
  getPatchInfoSync,
  getFileSizeSync,
  checkFileAccessSync,
  getCompressionRatioSync,
  type PatchInfoJs,
  type CompressionRatioJs,
} from '@bsdiff-rust/node'

// 生成和应用补丁
await diff('old-file.zip', 'new-file.zip', 'patch.bin')
await patch('old-file.zip', 'generated-file.zip', 'patch.bin')
```

## 📖 完整 API 文档

### 核心 API

#### 同步方法

```typescript
diffSync(oldFile: string, newFile: string, patchFile: string): void
```

生成两个文件之间的补丁文件。

```typescript
patchSync(oldFile: string, newFile: string, patchFile: string): void
```

应用补丁到旧文件，生成新文件。

#### 异步方法

```typescript
diff(oldFile: string, newFile: string, patchFile: string): Promise<void>
```

异步生成补丁文件，适合大文件处理。

```typescript
patch(oldFile: string, newFile: string, patchFile: string): Promise<void>
```

异步应用补丁，适合大文件处理。

### 验证和分析 API

```typescript
verifyPatchSync(oldFile: string, newFile: string, patchFile: string): boolean
verifyPatch(oldFile: string, newFile: string, patchFile: string): Promise<boolean>
```

验证补丁文件的完整性和正确性。

```typescript
getPatchInfoSync(patchFile: string): PatchInfoJs
```

获取补丁文件的详细信息。

```typescript
getCompressionRatioSync(oldFile: string, newFile: string, patchFile: string): CompressionRatioJs
```

计算和分析压缩比信息。

### 工具方法

```typescript
getFileSizeSync(filePath: string): number
```

获取文件大小（字节）。

```typescript
checkFileAccessSync(filePath: string): void
```

检查文件是否存在且可读，如果不满足条件会抛出异常。

### 数据结构

```typescript
interface PatchInfoJs {
  size: number // 补丁文件大小（字节）
  compressed: boolean // 是否使用压缩（总是 true）
}

interface CompressionRatioJs {
  oldSize: number // 旧文件大小（字节）
  newSize: number // 新文件大小（字节）
  patchSize: number // 补丁文件大小（字节）
  ratio: number // 压缩比（百分比）
}
```

## 🏗️ 技术架构

### 核心优化技术

- **内存映射 (mmap)**: 零拷贝文件读取，显著提升大文件处理性能
- **zstd 压缩**: 使用高性能 zstd 算法，平衡压缩比和速度
- **智能临时目录**: 自动选择最快的临时存储位置（RAM 盘优先）
- **并行处理**: 利用 Rust 的 rayon 库进行并行计算
- **缓冲优化**: 64KB 缓冲区优化 I/O 性能

## 🧪 测试

### 运行测试

```bash
# 运行完整测试套件
pnpm test
```

### 测试覆盖

- **功能测试**: 同步/异步 API 完整性测试
- **错误处理**: 文件不存在、权限错误等异常情况
- **性能测试**: 使用真实大文件（React 版本文件）
- **API 兼容性**: 所有导出函数的可用性验证
- **数据完整性**: 补丁应用后的文件一致性验证
- **工具方法**: 文件大小、访问权限、压缩比计算

## 📊 性能基准测试

### 核心技术优势

- **zstd 压缩**: 高性能压缩算法，平衡速度和压缩比
- **内存映射 (mmap)**: 零拷贝文件读取，显著提升大文件处理性能
- **Rust 实现**: 内存安全和高性能保证

### 性能提升概览

相比传统实现，本库在各项指标上都有显著提升：

- **Diff 性能**: 提升 **32.7%**
- **Patch 性能**: 提升 **93.0%**
- **内存使用**: 降低 **75.0%**

### 基准测试功能

- **多文件大小测试**: 1KB 到 10MB 的不同规模文件
- **变化比例测试**: 1% 到 50% 的不同变化程度
- **真实场景测试**: 使用实际项目文件
- **工具方法性能**: 验证、信息获取等辅助功能
- **跨平台性能**: 不同操作系统的性能表现

## 🔧 开发指南

### 环境要求

- **Node.js**: >= 16 (推荐 Latest LTS)
- **Rust**: >= 1.70
- **包管理器**: npm 或 pnpm

### 构建项目

```bash
# 安装依赖
pnpm install

# 构建发布版本
pnpm build

# 构建调试版本
pnpm build:debug

# 构建特定平台
pnpm build:arm64
```

### 开发工作流

```bash
# 代码格式化
pnpm format

# 代码检查
pnpm lint

# 运行测试
pnpm test
```

### 项目结构

```
bsdiff-rust/
├── src/
│   ├── lib.rs              # NAPI 绑定入口
│   ├── bsdiff_rust.rs      # 核心 Rust 实现
│   └── utils.rs            # 工具方法实现
├── benchmark/
│   └── benchmark.ts        # TypeScript 基准测试
├── test/
│   ├── index.ts             # 功能测试
│   └── resources/          # 测试资源文件
├── index.js                # Node.js 入口
├── index.d.ts              # TypeScript 类型定义
├── Cargo.toml              # Rust 项目配置
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

- 🚀 **快速安装**: 无需编译，直接下载预编译二进制
- 📦 **按需下载**: 只下载当前平台所需的文件
- 🛡️ **稳定可靠**: 避免编译环境问题导致的安装失败

## 🔄 从传统方案的升级

### 相比 node-gyp 方案的优势

| 特性         | bsdiff-rust (napi-rs) | 传统方案 (node-gyp)  |
| ------------ | --------------------- | -------------------- |
| **安装速度** | ⚡ 秒级安装           | 🐌 需要编译          |
| **环境依赖** | ✅ 无需编译环境       | ❌ 需要 Python + C++ |
| **性能**     | 🚀 Rust 优化          | 📈 C/C++ 标准        |
| **内存安全** | 🛡️ Rust 保证          | ⚠️ 手动管理          |
| **维护性**   | ✨ 现代化代码         | 🔧 传统 C 代码       |

### 迁移指南

如果你正在从其他 bsdiff 库迁移：

```javascript
// 旧的 API 调用
const bsdiff = require('bsdiff-node')
bsdiff.diff(oldFile, newFile, patchFile, callback)

// 新的 API 调用
const bsdiff = require('@bsdiff-rust/node')
// 同步方式
bsdiff.diffSync(oldFile, newFile, patchFile)
// 或异步方式
await bsdiff.diff(oldFile, newFile, patchFile)
```

## 📈 性能优化说明

### 内存映射优化

使用 `memmap2` 库实现零拷贝文件读取：

```rust
let old_mmap = unsafe { MmapOptions::new().map(&old_file_handle)? };
let new_mmap = unsafe { MmapOptions::new().map(&new_file_handle)? };
```

### 智能临时目录

自动选择最快的临时存储：

- **Linux**: 优先使用 `/dev/shm` (内存盘)
- **macOS**: 检测 RAM 盘
- **通用**: 回退到系统临时目录

### 压缩配置优化

使用经过调优的 zstd 压缩参数：

```rust
compression_level: 3,    // 平衡速度和压缩比
buffer_size: 64 * 1024, // 64KB 缓冲区
```

## 🎯 补丁大小优化

想要获得更小的补丁文件和更好的性能？查看我们的详细优化指南：

📖 **[补丁大小优化指南](PATCH_OPTIMIZATION.md)**

### 快速优化建议

1. **使用规范化 TAR 流程** - 可减少 30-60% 补丁大小
2. **固定构建参数** - 确保可重复构建
3. **选择合适的压缩级别** - 平衡速度和大小
4. **预处理文件** - 移除无关数据

### 性能提升概览

基于真实测试数据，相比传统实现：

- **Diff 性能**: 提升 **32.7%**
- **Patch 性能**: 提升 **93.0%**
- **内存使用**: 降低 **75.0%**
- **补丁大小**: 通过优化可减少 **30-60%**

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

- [bsdiff 原始算法](http://www.daemonology.net/bsdiff/) - Colin Percival 的原始实现
- [NAPI-RS 文档](https://napi.rs/) - Node.js 绑定框架
- [Rust 官方文档](https://www.rust-lang.org/) - Rust 编程语言
- [zstd 压缩算法](https://github.com/facebook/zstd) - Facebook 开源的压缩算法

---

⭐ 如果这个项目对您有帮助，请给它一个星标！

🐛 发现问题？欢迎提交 [Issue](https://github.com/Sphinm/bsdiff-rust/issues)

💡 有改进建议？欢迎提交 [Pull Request](https://github.com/Sphinm/bsdiff-rust/pulls)
