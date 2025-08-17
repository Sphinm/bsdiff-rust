# GitHub CI 工作流说明

本项目包含三个 GitHub Actions 工作流，用于自动化构建、测试和发布。

## 📋 工作流概览

### 1. 测试工作流 (`test.yml`)

- **触发条件**: Push 到 main/master 分支，或创建 Pull Request
- **功能**:
  - 在多个平台 (Ubuntu, Windows, macOS) 上测试
  - 支持多个 Node.js 版本 (16, 18, 20)
  - 运行代码检查和格式化检查

### 2. 构建工作流 (`build.yml`)

- **触发条件**: Push 到 main/master 分支，或创建 Pull Request，或发布 Release
- **功能**:
  - 构建所有目标平台的 `.node` 文件
  - 上传构建产物作为 artifacts
  - 发布时创建压缩包

### 3. 发布工作流 (`publish.yml`)

- **触发条件**: 发布 Release
- **功能**:
  - 构建所有平台的 `.node` 文件
  - 自动发布到 npm

## 🎯 支持的平台

| 平台         | 架构   | 目标                         |
| ------------ | ------ | ---------------------------- |
| Linux        | x86_64 | `x86_64-unknown-linux-gnu`   |
| Linux        | ARM64  | `aarch64-unknown-linux-gnu`  |
| Linux (musl) | ARM64  | `aarch64-unknown-linux-musl` |
| Windows      | x86_64 | `x86_64-pc-windows-msvc`     |
| Windows      | ARM64  | `aarch64-pc-windows-msvc`    |
| macOS        | x86_64 | `x86_64-apple-darwin`        |
| macOS        | ARM64  | `aarch64-apple-darwin`       |
| Android      | ARM64  | `aarch64-linux-android`      |

## 🚀 使用方法

### 本地开发

```bash
# 安装依赖
npm install

# 构建当前平台
npm run build:current

# 运行测试
npm test

# 代码检查
npm run lint
```

### 发布新版本

1. 更新 `package.json` 中的版本号
2. 创建 Git tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
3. 在 GitHub 上创建 Release
4. CI 会自动构建并发布到 npm

### 手动触发构建

可以在 GitHub Actions 页面手动触发工作流：

1. 进入 Actions 标签页
2. 选择对应的工作流
3. 点击 "Run workflow"

## 🔧 配置说明

### 环境变量

- `NPM_TOKEN`: npm 发布令牌 (需要在 GitHub Secrets 中设置)

### 缓存

- Node.js 依赖缓存
- Rust 工具链缓存

### 产物

构建产物包括：

- `*.node` - 原生模块文件
- `index.js` - 主入口文件
- `index.d.ts` - TypeScript 类型定义
- `package.json` - 包配置

## 📦 发布包结构

发布时会生成以下文件：

```
bsdiff-rust-1.0.0/
├── index.js
├── index.d.ts
├── package.json
├── README.md
├── LICENSE
├── bsdiff-rust.darwin-arm64.node
├── bsdiff-rust.darwin-x64.node
├── bsdiff-rust.linux-arm64-gnu.node
├── bsdiff-rust.linux-arm64-musl.node
├── bsdiff-rust.linux-x64-gnu.node
├── bsdiff-rust.win32-arm64-msvc.node
└── bsdiff-rust.win32-x64-msvc.node
```

## 🐛 故障排除

### 常见问题

1. **构建失败**: 检查 Rust 工具链是否正确安装
2. **测试失败**: 确保所有依赖都已安装
3. **发布失败**: 检查 NPM_TOKEN 是否正确设置

### 调试步骤

1. 查看 GitHub Actions 日志
2. 检查构建产物是否正确生成
3. 验证平台兼容性

## 📝 注意事项

- 确保 `package.json` 中的 `targets` 配置与 CI 中的平台列表一致
- 发布前请确保所有测试通过
- 建议在本地先测试构建过程
