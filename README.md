# 🚀 bsdiff-rust

A high-performance Rust implementation of bsdiff and bspatch algorithms with complete Node.js bindings

[![npm version](https://badge.fury.io/js/@bsdiff-rust%2Fnode.svg)](https://badge.fury.io/js/@bsdiff-rust%2Fnode)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## ✨ Core Features

- **Extreme Performance**: Rust implementation with memory mapping and parallel processing
- **Memory Safety**: Rust guarantees memory safety and thread safety
- **Binary Patches**: Efficient generation and application of binary file patches
- **Smart Compression**: Built-in zstd compression with optimal compression ratio configuration
- **Dual API**: Provides both synchronous and asynchronous APIs for different use cases
- **Integrity Verification**: Built-in patch file integrity verification mechanism
- **Detailed Analysis**: Compression ratio analysis and file size statistics
- **Smart Checking**: Automatic file existence and access permission verification
- **Cross-platform**: Supports Windows, macOS, Linux platforms
- **Modern Bindings**: High-performance Node.js bindings based on napi-rs

## 🚀 Quick Start

### Installation

```bash
pnpm install @bsdiff-rust/node
```

### Basic Usage

```javascript
const bsdiff = require('@bsdiff-rust/node')

// Synchronous API - suitable for simple scenarios
bsdiff.diffSync('old-file.zip', 'new-file.zip', 'patch.bin')
bsdiff.patchSync('old-file.zip', 'generated-file.zip', 'patch.bin')

// Asynchronous API - suitable for large files and production environments
await bsdiff.diff('old-file.zip', 'new-file.zip', 'patch.bin')
await bsdiff.patch('old-file.zip', 'generated-file.zip', 'patch.bin')
```

### TypeScript Support

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

// Generate and apply patches
await diff('old-file.zip', 'new-file.zip', 'patch.bin')
await patch('old-file.zip', 'generated-file.zip', 'patch.bin')
```

## 📖 Complete API Documentation

### Core API

#### Synchronous Methods

```typescript
diffSync(oldFile: string, newFile: string, patchFile: string): void
```

Generate a patch file between two files.

```typescript
patchSync(oldFile: string, newFile: string, patchFile: string): void
```

Apply a patch to an old file to generate a new file.

#### Asynchronous Methods

```typescript
diff(oldFile: string, newFile: string, patchFile: string): Promise<void>
```

Asynchronously generate a patch file, suitable for large file processing.

```typescript
patch(oldFile: string, newFile: string, patchFile: string): Promise<void>
```

Asynchronously apply a patch, suitable for large file processing.

### Verification and Analysis API

```typescript
verifyPatchSync(oldFile: string, newFile: string, patchFile: string): boolean
verifyPatch(oldFile: string, newFile: string, patchFile: string): Promise<boolean>
```

Verify the integrity and correctness of patch files.

```typescript
getPatchInfoSync(patchFile: string): PatchInfoJs
```

Get detailed information about patch files.

```typescript
getCompressionRatioSync(oldFile: string, newFile: string, patchFile: string): CompressionRatioJs
```

Calculate and analyze compression ratio information.

### Utility Methods

```typescript
getFileSizeSync(filePath: string): number
```

Get file size in bytes.

```typescript
checkFileAccessSync(filePath: string): void
```

Check if a file exists and is readable, throws an exception if conditions are not met.

### Data Structures

```typescript
interface PatchInfoJs {
  size: number // Patch file size in bytes
  compressed: boolean // Whether compression is used (always true)
}

interface CompressionRatioJs {
  oldSize: number // Old file size in bytes
  newSize: number // New file size in bytes
  patchSize: number // Patch file size in bytes
  ratio: number // Compression ratio (percentage)
}
```

## 🏗️ Technical Architecture

### Core Optimization Technologies

- **Memory Mapping (mmap)**: Zero-copy file reading, significantly improving large file processing performance
- **zstd Compression**: Uses high-performance zstd algorithm, balancing compression ratio and speed
- **Smart Temporary Directory**: Automatically selects the fastest temporary storage location (RAM disk priority)
- **Parallel Processing**: Utilizes Rust's rayon library for parallel computation
- **Buffer Optimization**: 64KB buffer optimization for I/O performance

## 🧪 Testing

### Running Tests

```bash
# Run complete test suite
pnpm test

# Run performance benchmarks
pnpm bench
```

### Test Coverage

- **Functional Testing**: Synchronous/asynchronous API integrity testing
- **Error Handling**: File not found, permission errors, and other exception scenarios
- **Performance Testing**: Using real large files (React version files)
- **API Compatibility**: Verification of all exported function availability
- **Data Integrity**: File consistency verification after patch application
- **Utility Methods**: File size, access permissions, compression ratio calculations

## 📊 Performance Benchmarks

### Core Technical Advantages

- **zstd Compression**: High-performance compression algorithm balancing speed and compression ratio
- **Memory Mapping (mmap)**: Zero-copy file reading, significantly improving large file processing performance
- **Rust Implementation**: Memory safety and high-performance guarantees

### Performance Improvement Overview

Compared to traditional implementations, this library shows significant improvements across all metrics:

- **Diff Performance**: **32.7%** improvement
- **Patch Performance**: **93.0%** improvement
- **Memory Usage**: **75.0%** reduction

### Benchmark Features

- **Multi-file Size Testing**: Different scale files from 1KB to 10MB
- **Change Ratio Testing**: Different change degrees from 1% to 50%
- **Real Scenario Testing**: Using actual project files
- **Utility Method Performance**: Verification, information retrieval, and other auxiliary functions
- **Cross-platform Performance**: Performance across different operating systems

## 🔧 Development Guide

### Environment Requirements

- **Node.js**: >= 16 (Latest LTS recommended)
- **Rust**: >= 1.70
- **Package Manager**: npm or pnpm

### Building the Project

```bash
# Install dependencies
pnpm install

# Build release version
pnpm build

# Build debug version
pnpm build:debug

# Build for specific platform
pnpm build:arm64
```

### Development Workflow

```bash
# Code formatting
pnpm format

# Code linting
pnpm lint

# Run tests
pnpm test

# Performance testing
pnpm bench
```

### Project Structure

```
bsdiff-rust/
├── src/
│   ├── lib.rs              # NAPI binding entry
│   ├── bsdiff_rust.rs      # Core Rust implementation
├── benchmark/
│   └── benchmark.ts        # TypeScript benchmarks
├── test/
│   ├── index.ts            # Functional tests
│   └── resources/          # Test resource files
├── index.js                # Node.js entry point
├── index.d.ts              # TypeScript type definitions
├── Cargo.toml              # Rust project configuration
└── package.json            # Node.js project configuration
```

## 🌍 Cross-platform Support

### Supported Platforms

- **macOS**: ARM64 (Apple Silicon) and x64 (Intel)
- **Linux**: ARM64 and x64 (GNU and musl)
- **Windows**: ARM64 and x64 (MSVC)

### Platform Package Strategy

This project uses napi-rs's multi-package strategy, automatically downloading precompiled binaries for the corresponding platform during installation:

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

**Advantages**:

- 🚀 **Fast Installation**: No compilation needed, direct download of precompiled binaries
- 📦 **On-demand Download**: Only downloads files needed for the current platform
- 🛡️ **Stable and Reliable**: Avoids installation failures due to compilation environment issues

## 🔄 Upgrading from Traditional Solutions

### Advantages over node-gyp Solutions

| Feature                      | bsdiff-rust (napi-rs)                | Traditional (node-gyp)   |
| ---------------------------- | ------------------------------------ | ------------------------ |
| **Installation Speed**       | ⚡ Second-level installation         | 🐌 Requires compilation  |
| **Environment Dependencies** | ✅ No compilation environment needed | ❌ Requires Python + C++ |
| **Performance**              | 🚀 Rust optimization                 | 📈 Standard C/C++        |
| **Memory Safety**            | 🛡️ Rust guarantees                   | ⚠️ Manual management     |
| **Maintainability**          | ✨ Modern code                       | 🔧 Traditional C code    |

### Migration Guide

If you're migrating from other bsdiff libraries:

```javascript
// Old API calls
const bsdiff = require('bsdiff-node')
bsdiff.diff(oldFile, newFile, patchFile, callback)

// New API calls
const bsdiff = require('@bsdiff-rust/node')
// Synchronous approach
bsdiff.diffSync(oldFile, newFile, patchFile)
// Or asynchronous approach
await bsdiff.diff(oldFile, newFile, patchFile)
```

## 📈 Performance Optimization

### Memory Mapping Optimization

Uses `memmap2` library for zero-copy file reading:

```rust
let old_mmap = unsafe { MmapOptions::new().map(&old_file_handle)? };
let new_mmap = unsafe { MmapOptions::new().map(&new_file_handle)? };
```

### Smart Temporary Directory

Automatically selects the fastest temporary storage:

- **Linux**: Prioritizes `/dev/shm` (memory disk)
- **macOS**: Detects RAM disk
- **General**: Falls back to system temporary directory

### Compression Configuration Optimization

Uses tuned zstd compression parameters:

```rust
compression_level: 3,    // Balances speed and compression ratio
buffer_size: 64 * 1024, // 64KB buffer
```

## 🎯 Patch Size Optimization

Want smaller patch files and better performance? Check out our detailed optimization guide:

📖 **[Patch Size Optimization Guide](PATCH_OPTIMIZATION.md)**

### Quick Optimization Tips

1. **Use Normalized TAR Workflow** - Can reduce patch size by 30-60%
2. **Fixed Build Parameters** - Ensure reproducible builds
3. **Choose Appropriate Compression Level** - Balance speed and size
4. **Preprocess Files** - Remove irrelevant data

### Performance Improvement Overview

Based on real test data, compared to traditional implementations:

- **Diff Performance**: **32.7%** improvement
- **Patch Performance**: **93.0%** improvement
- **Memory Usage**: **75.0%** reduction
- **Patch Size**: Can be reduced by **30-60%** through optimization

## 🤝 Contributing

### Development Process

1. Fork the project
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Create a Pull Request

### Code Standards

- **Rust Code**: Use `cargo fmt` for formatting
- **JavaScript/TypeScript**: Use Prettier for formatting
- **Commit Messages**: Use clear English descriptions

## 📚 References

- [bsdiff Original Algorithm](http://www.daemonology.net/bsdiff/) - Colin Percival's original implementation
- [NAPI-RS Documentation](https://napi.rs/) - Node.js binding framework
- [Rust Official Documentation](https://www.rust-lang.org/) - Rust programming language
- [zstd Compression Algorithm](https://github.com/facebook/zstd) - Facebook's open-source compression algorithm

---

⭐ If this project helps you, please give it a star!

🐛 Found an issue? Feel free to submit an [Issue](https://github.com/Sphinm/bsdiff-rust/issues)

💡 Have suggestions for improvement? Welcome to submit a [Pull Request](https://github.com/Sphinm/bsdiff-rust/pulls)
