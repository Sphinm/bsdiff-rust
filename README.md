# 🚀 bsdiff-rust

A high-performance binary differential patching library based on Rust and NAPI-RS, providing an optimized bsdiff/bspatch algorithm implementation for Node.js. Built on qbsdiff library with standard BSDIFF40 format support, featuring suffix array algorithms and parallel processing.

[![npm version](https://badge.fury.io/js/@bsdiff-rust%2Fnode.svg)](https://badge.fury.io/js/@bsdiff-rust%2Fnode)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[Chinese README](./README-ZH.md)

## ✨ Core Features

- **Standard Compatible**: Generates standard BSDIFF40 format, fully compatible with bsdiff-node
- **Memory Safe**: Rust guarantees memory safety and thread safety, high-performance Node.js bindings via napi-rs
- **Optimized Compression**: Uses bzip2 compression with memory pre-allocation optimizations
- **Cross-platform**: Supports Windows, macOS, Linux

## 🚀 Quick Start

### Installation

```bash
pnpm install @bsdiff-rust/node
```

### Basic Usage

```javascript
const bsdiff = require('@bsdiff-rust/node')

// Synchronous API
bsdiff.diffSync('old-file.zip', 'new-file.zip', 'patch.bin')
bsdiff.patchSync('old-file.zip', 'generated-file.zip', 'patch.bin')

// Asynchronous API
await bsdiff.diff('old-file.zip', 'new-file.zip', 'patch.bin')
await bsdiff.patch('old-file.zip', 'generated-file.zip', 'patch.bin')
```

### TypeScript Support

```typescript
import { diff, diffSync, patch, patchSync } from '@bsdiff-rust/node'

// Generate and apply patches
await diff('old-file.zip', 'new-file.zip', 'patch.bin')
await patch('old-file.zip', 'generated-file.zip', 'patch.bin')
```

Need performance monitoring or custom configuration? See the [Complete Guide](./docs/GUIDE.md)

## 📖 API Documentation

### Core API

```typescript
// Synchronous methods
diffSync(oldFile: string, newFile: string, patchFile: string): void
patchSync(oldFile: string, newFile: string, patchFile: string): void

// Asynchronous methods
diff(oldFile: string, newFile: string, patchFile: string): Promise<void>
patch(oldFile: string, newFile: string, patchFile: string): Promise<void>
```

**Need advanced features?** See the [Complete Guide](./docs/GUIDE.md) for performance stats, configuration options, testing, and benchmarks.

## 🧪 Testing

```bash
# Run functional tests
pnpm test

# Run performance benchmarks
pnpm run bench
```

## 🔧 Development Guide

### Environment Requirements

- **Node.js**: >= 16 (Latest LTS recommended)
- **Rust**: >= 1.88 (required by napi v3 / edition 2024 dependencies)
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

### Project Structure

```
bsdiff-rust/
├── core/                   # bsdiff-core: platform-agnostic engine + C FFI
│   └── src/
│       ├── bsdiff.rs       # diff/patch implementation
│       ├── utils.rs        # file info, verification, compression ratio
│       └── ffi.rs          # extern "C" exports (iOS / desktop)
├── bindings/
│   ├── node/               # napi-rs binding -> node.<platform>.node
│   ├── android/            # JNI binding + prebuilt jniLibs
│   └── ios/                # XCFramework + Swift wrapper
├── benchmark/
│   └── benchmark.ts        # TypeScript benchmarks
├── test/
│   ├── index.ts            # Functional tests
│   └── resources/          # Test resource files (not in git)
├── index.js                # Node.js entry point (generated)
├── index.d.ts              # TypeScript type definitions (generated)
├── Cargo.toml              # Cargo workspace manifest
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

- [Complete Guide](./docs/GUIDE.md) - API reference, testing, and performance benchmarks
- [bsdiff Original Algorithm](http://www.daemonology.net/bsdiff/) - Colin Percival's original implementation
- [NAPI-RS Documentation](https://napi.rs/) - Node.js binding framework
- [qbsdiff Library](https://crates.io/crates/qbsdiff) - Underlying Rust implementation

---

⭐ If this project helps you, please give it a star!

🐛 Found an issue? Feel free to submit an [Issue](https://github.com/Sphinm/bsdiff-rust/issues)

💡 Have suggestions for improvement? Welcome to submit a [Pull Request](https://github.com/Sphinm/bsdiff-rust/pulls)
