#!/usr/bin/env node

import * as fs from 'fs'
import * as path from 'path'
import { Bench } from 'tinybench'
import bsdiff from '../index'

// Test resources directory (relative to project root)
const RESOURCES_DIR = path.resolve(process.cwd(), 'test/resources')
const TEMP_DIR = path.resolve(process.cwd(), 'temp')

// 定义类型
interface TestResult {
  size?: number
  ratio?: number
  oldSize?: number
  newSize?: number
  patchSize?: number
  isValid?: boolean
  success?: boolean
  changeRatio?: number
}

interface FileSize {
  name: string
  size: number
}

interface ChangeRatio {
  name: string
  ratio: number
}

// 生成测试数据
function generateTestData(size: number): Buffer {
  const data = Buffer.alloc(size)
  for (let i = 0; i < size; i++) {
    data[i] = i % 256
  }
  return data
}

// 生成差异化的测试数据
function generateDiffData(baseData: Buffer, changeRatio: number): Buffer {
  const newData = Buffer.from(baseData)
  const changeCount = Math.floor(baseData.length * changeRatio)

  for (let i = 0; i < changeCount; i++) {
    const index = i % baseData.length
    newData[index] = (newData[index] + 1) % 256
  }

  return newData
}

// 格式化文件大小
function formatFileSize(bytes: number): string {
  const units = ['B', 'KB', 'MB', 'GB']
  let size = bytes
  let unitIndex = 0

  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024
    unitIndex++
  }

  return `${size.toFixed(2)} ${units[unitIndex]}`
}

// 格式化时间
function formatTime(ms: number): string {
  if (ms < 1000) {
    return `${ms.toFixed(2)}ms`
  } else {
    return `${(ms / 1000).toFixed(2)}s`
  }
}

// 创建临时文件并返回清理函数
function createTempFiles(
  oldData: Buffer,
  newData: Buffer,
  prefix: string,
): {
  oldFile: string
  newFile: string
  patchFile: string
  cleanup: () => void
} {
  const oldFile = path.join(TEMP_DIR, `old_${prefix}.bin`)
  const newFile = path.join(TEMP_DIR, `new_${prefix}.bin`)
  const patchFile = path.join(TEMP_DIR, `patch_${prefix}.bin`)

  // 确保临时目录存在
  const tempDir = path.dirname(oldFile)
  if (!fs.existsSync(tempDir)) {
    fs.mkdirSync(tempDir, { recursive: true })
  }

  fs.writeFileSync(oldFile, oldData)
  fs.writeFileSync(newFile, newData)

  const cleanup = () => {
    try {
      if (fs.existsSync(oldFile)) fs.unlinkSync(oldFile)
      if (fs.existsSync(newFile)) fs.unlinkSync(newFile)
      if (fs.existsSync(patchFile)) fs.unlinkSync(patchFile)
    } catch (error) {
      // 忽略清理错误
    }
  }

  return { oldFile, newFile, patchFile, cleanup }
}

// 测试不同文件大小
async function benchmarkDifferentSizes(): Promise<void> {
  console.log('\n📏 测试不同文件大小')
  console.log('='.repeat(50))

  const sizes: FileSize[] = [
    { name: '1KB', size: 1024 },
    { name: '10KB', size: 10 * 1024 },
    { name: '100KB', size: 100 * 1024 },
    { name: '1MB', size: 1024 * 1024 },
    // 暂时注释掉 5MB 测试，因为处理时间过长
    // { name: '5MB', size: 5 * 1024 * 1024 },
  ]

  for (const { name, size } of sizes) {
    console.log(`\n🧪 测试文件大小: ${name} (${formatFileSize(size)})`)

    // 根据文件大小调整 benchmark 参数
    const bench = new Bench({
      time: size >= 1024 * 1024 ? 2000 : 1000, // 大文件运行2秒，小文件1秒
      iterations: size >= 1024 * 1024 ? 3 : 5, // 大文件最少3次迭代，小文件5次
      warmupTime: size >= 1024 * 1024 ? 200 : 100, // 大文件预热200ms，小文件100ms
    })

    const oldData = generateTestData(size)
    const newData = generateDiffData(oldData, 0.1)
    const { oldFile, newFile, patchFile, cleanup } = createTempFiles(oldData, newData, name)

    bench
      .add(`diff ${name}`, async () => {
        await bsdiff.diff(oldFile, newFile, patchFile)
      })
      .add(`getPatchInfo ${name}`, () => {
        bsdiff.getPatchInfoSync(patchFile)
      })
      .add(`getCompressionRatio ${name}`, () => {
        bsdiff.getCompressionRatioSync(oldFile, newFile, patchFile)
      })

    await bench.run()
    console.table(bench.table())

    // 显示额外信息
    const info = bsdiff.getPatchInfoSync(patchFile)
    const ratio = bsdiff.getCompressionRatioSync(oldFile, newFile, patchFile)
    console.log(`   补丁大小: ${formatFileSize(info.size)}`)
    console.log(`   压缩比: ${(ratio.ratio * 100).toFixed(2)}%`)

    cleanup()
  }
}

// 测试不同的变化比例
async function benchmarkChangeRatios(): Promise<void> {
  console.log('\n📊 测试不同变化比例')
  console.log('='.repeat(50))

  const ratios: ChangeRatio[] = [
    { name: '1%', ratio: 0.01 },
    { name: '5%', ratio: 0.05 },
    { name: '10%', ratio: 0.1 },
    { name: '20%', ratio: 0.2 },
    { name: '50%', ratio: 0.5 },
  ]

  const size = 1024 * 1024 // 1MB

  for (const { name, ratio } of ratios) {
    console.log(`\n🧪 测试变化比例: ${name}`)

    const bench = new Bench({
      time: 1000,
      iterations: 5,
      warmupTime: 100,
    })

    const oldData = generateTestData(size)
    const newData = generateDiffData(oldData, ratio)
    const { oldFile, newFile, patchFile, cleanup } = createTempFiles(oldData, newData, `ratio_${name}`)

    bench
      .add(`diff ${name}`, async () => {
        await bsdiff.diff(oldFile, newFile, patchFile)
      })
      .add(`getPatchInfo ${name}`, () => {
        bsdiff.getPatchInfoSync(patchFile)
      })
      .add(`getCompressionRatio ${name}`, () => {
        bsdiff.getCompressionRatioSync(oldFile, newFile, patchFile)
      })

    await bench.run()
    console.table(bench.table())

    // 显示额外信息
    const info = bsdiff.getPatchInfoSync(patchFile)
    const ratioInfo = bsdiff.getCompressionRatioSync(oldFile, newFile, patchFile)
    console.log(`   补丁大小: ${formatFileSize(info.size)}`)
    console.log(`   压缩比: ${(ratioInfo.ratio * 100).toFixed(2)}%`)

    cleanup()
  }
}

// 测试工具方法性能
async function benchmarkUtils(): Promise<void> {
  console.log('\n🔧 测试工具方法性能')
  console.log('='.repeat(50))

  const size = 1024 * 1024 // 1MB
  const oldData = generateTestData(size)
  const newData = generateDiffData(oldData, 0.1)
  const { oldFile, newFile, patchFile, cleanup } = createTempFiles(oldData, newData, 'utils')

  // 先生成补丁
  await bsdiff.diff(oldFile, newFile, patchFile)

  const bench = new Bench({
    time: 1000,
    iterations: 10,
    warmupTime: 100,
  })

  bench
    .add('verifyPatch', async () => {
      await bsdiff.verifyPatch(oldFile, newFile, patchFile)
    })
    .add('getPatchInfo', () => {
      bsdiff.getPatchInfoSync(patchFile)
    })
    .add('getCompressionRatio', () => {
      bsdiff.getCompressionRatioSync(oldFile, newFile, patchFile)
    })
    .add('getFileSize', () => {
      bsdiff.getFileSizeSync(oldFile)
    })
    .add('checkFileAccess', () => {
      bsdiff.checkFileAccessSync(oldFile)
    })

  await bench.run()
  console.table(bench.table())

  cleanup()
}

// 测试补丁应用性能
async function benchmarkPatch(): Promise<void> {
  console.log('\n🔧 测试补丁应用性能')
  console.log('='.repeat(50))

  const size = 1024 * 1024 // 1MB
  const oldData = generateTestData(size)
  const newData = generateDiffData(oldData, 0.1)
  const { oldFile, newFile, patchFile, cleanup } = createTempFiles(oldData, newData, 'patch')

  // 先生成补丁
  await bsdiff.diff(oldFile, newFile, patchFile)

  const appliedFile = path.join(TEMP_DIR, 'applied_patch.bin')

  const bench = new Bench({
    time: 1000,
    iterations: 5,
    warmupTime: 100,
  })

  bench.add('applyPatch', async () => {
    await bsdiff.patch(oldFile, patchFile, appliedFile)
  })

  await bench.run()
  console.table(bench.table())

  // 验证补丁应用是否正确
  const appliedData = fs.readFileSync(appliedFile)
  const originalNewData = fs.readFileSync(newFile)
  const isValid = appliedData.equals(originalNewData)
  console.log(`   补丁应用验证: ${isValid ? '✅ 成功' : '❌ 失败'}`)

  // 清理
  cleanup()
  try {
    if (fs.existsSync(appliedFile)) fs.unlinkSync(appliedFile)
  } catch (error) {
    // 忽略清理错误
  }
}

// 综合性能测试
async function benchmarkComprehensive(): Promise<void> {
  console.log('\n🚀 综合性能测试')
  console.log('='.repeat(50))

  const sizes = [1024, 10 * 1024, 100 * 1024, 1024 * 1024] // 1KB, 10KB, 100KB, 1MB
  const changeRatios = [0.01, 0.05, 0.1, 0.2] // 1%, 5%, 10%, 20%

  const bench = new Bench({
    time: 2000,
    iterations: 3,
    warmupTime: 200,
  })

  for (const size of sizes) {
    for (const ratio of changeRatios) {
      const sizeName = formatFileSize(size)
      const ratioName = `${(ratio * 100).toFixed(0)}%`

      bench.add(`diff ${sizeName} ${ratioName}`, async () => {
        const oldData = generateTestData(size)
        const newData = generateDiffData(oldData, ratio)
        const { oldFile, newFile, patchFile, cleanup } = createTempFiles(oldData, newData, `comp_${size}_${ratio}`)

        await bsdiff.diff(oldFile, newFile, patchFile)
        cleanup()
      })
    }
  }

  await bench.run()
  console.table(bench.table())
}

// 测试真实文件性能
async function benchmarkRealFiles(): Promise<void> {
  console.log('\n📁 测试真实文件性能')
  console.log('='.repeat(50))

  const realFiles = [
    {
      name: 'React 18.1.0 → 19.1.0',
      oldFile: path.join(RESOURCES_DIR, 'react-18.1.0.zip'),
      newFile: path.join(RESOURCES_DIR, 'react-19.1.0.zip'),
    },
  ]

  for (const { name, oldFile, newFile } of realFiles) {
    // 检查文件是否存在cls
    if (!fs.existsSync(oldFile) || !fs.existsSync(newFile)) {
      console.log(`⚠️  跳过 ${name}: 测试文件不存在`)
      continue
    }

    console.log(`\n🧪 测试真实文件: ${name}`)

    const oldSize = fs.statSync(oldFile).size
    const newSize = fs.statSync(newFile).size
    console.log(`   文件大小: ${formatFileSize(oldSize)} → ${formatFileSize(newSize)}`)

    const patchFile = path.join(TEMP_DIR, `real_patch_${Date.now()}.bin`)
    const appliedFile = path.join(TEMP_DIR, `real_applied_${Date.now()}.bin`)

    // 确保临时目录存在
    const tempDir = path.dirname(patchFile)
    if (!fs.existsSync(tempDir)) {
      fs.mkdirSync(tempDir, { recursive: true })
    }

    const bench = new Bench({
      time: 5000, // 真实文件测试运行5秒
      iterations: 2, // 最少2次迭代
      warmupTime: 1000, // 预热1秒
    })

    bench
      .add(`diff ${name}`, async () => {
        await bsdiff.diff(oldFile, newFile, patchFile)
      })
      .add(`patch ${name}`, async () => {
        await bsdiff.patch(oldFile, appliedFile, patchFile)
      })
      .add(`verify ${name}`, async () => {
        await bsdiff.verifyPatch(oldFile, newFile, patchFile)
      })

    await bench.run()
    console.table(bench.table())

    // 显示补丁信息
    try {
      const info = bsdiff.getPatchInfoSync(patchFile)
      const ratio = bsdiff.getCompressionRatioSync(oldFile, newFile, patchFile)
      console.log(`   补丁大小: ${formatFileSize(info.size)}`)
      console.log(`   压缩比: ${ratio.ratio.toFixed(2)}%`)
      console.log(`   压缩效率: ${(((oldSize + newSize - info.size) / (oldSize + newSize)) * 100).toFixed(2)}%`)
    } catch (error) {
      console.log(`   ⚠️  无法获取补丁信息: ${(error as Error).message}`)
    }

    // 清理临时文件
    try {
      if (fs.existsSync(patchFile)) fs.unlinkSync(patchFile)
      if (fs.existsSync(appliedFile)) fs.unlinkSync(appliedFile)
    } catch (error) {
      // 忽略清理错误
    }
  }
}

// 对比测试 - 同步 vs 异步性能
async function benchmarkSyncVsAsync(): Promise<void> {
  console.log('\n⚡ 同步 vs 异步性能对比')
  console.log('='.repeat(50))

  const sizes = [
    { name: '10KB', size: 10 * 1024 },
    { name: '100KB', size: 100 * 1024 },
    { name: '1MB', size: 1024 * 1024 },
  ]

  for (const { name, size } of sizes) {
    console.log(`\n🧪 测试文件大小: ${name}`)

    const oldData = generateTestData(size)
    const newData = generateDiffData(oldData, 0.1)
    const { oldFile, newFile, patchFile, cleanup } = createTempFiles(oldData, newData, `sync_${name}`)

    const bench = new Bench({
      time: 2000,
      iterations: 5,
      warmupTime: 200,
    })

    bench
      .add(`diffSync ${name}`, () => {
        bsdiff.diffSync(oldFile, newFile, patchFile)
      })
      .add(`diff async ${name}`, async () => {
        await bsdiff.diff(oldFile, newFile, patchFile)
      })
      .add(`patchSync ${name}`, () => {
        bsdiff.patchSync(oldFile, patchFile + '.applied', patchFile)
      })
      .add(`patch async ${name}`, async () => {
        await bsdiff.patch(oldFile, patchFile + '.applied2', patchFile)
      })

    await bench.run()
    console.table(bench.table())

    cleanup()
  }
}

// 内存使用基准测试
async function benchmarkMemoryUsage(): Promise<void> {
  console.log('\n🧠 内存使用基准测试')
  console.log('='.repeat(50))

  const sizes = [
    { name: '1MB', size: 1024 * 1024 },
    { name: '5MB', size: 5 * 1024 * 1024 },
    { name: '10MB', size: 10 * 1024 * 1024 },
  ]

  for (const { name, size } of sizes) {
    console.log(`\n🧪 测试内存使用: ${name}`)

    const oldData = generateTestData(size)
    const newData = generateDiffData(oldData, 0.1)
    const { oldFile, newFile, patchFile, cleanup } = createTempFiles(oldData, newData, `mem_${name}`)

    // 记录初始内存
    const initialMemory = process.memoryUsage()

    // 执行操作
    const startTime = Date.now()
    await bsdiff.diff(oldFile, newFile, patchFile)
    const diffTime = Date.now() - startTime

    // 记录峰值内存
    const peakMemory = process.memoryUsage()

    console.log(`   处理时间: ${formatTime(diffTime)}`)
    console.log(`   内存增长: ${formatFileSize(peakMemory.heapUsed - initialMemory.heapUsed)}`)
    console.log(`   RSS 增长: ${formatFileSize(peakMemory.rss - initialMemory.rss)}`)
    console.log(`   内存效率: ${(size / (peakMemory.heapUsed - initialMemory.heapUsed)).toFixed(2)}x`)

    cleanup()

    // 强制垃圾回收（如果可用）
    if (global.gc) {
      global.gc()
    }
  }
}

// 主函数
async function main(): Promise<void> {
  console.log('🚀 bsdiff-rust Performance Benchmarks')
  console.log('='.repeat(60))

  try {
    // 测试真实文件
    await benchmarkRealFiles()

    // 测试不同文件大小
    await benchmarkDifferentSizes()

    // 同步 vs 异步对比
    await benchmarkSyncVsAsync()

    // 测试不同变化比例
    await benchmarkChangeRatios()

    // 内存使用测试
    await benchmarkMemoryUsage()

    // 测试工具方法
    await benchmarkUtils()

    // 测试补丁应用
    await benchmarkPatch()

    // 综合性能测试
    await benchmarkComprehensive()

    console.log('\n✅ 基准测试完成！')
    console.log('   - 使用 tinybench 进行现代化基准测试')
    console.log('   - 包含真实文件和合成数据测试')
    console.log('   - 测试同步和异步API性能')
    console.log('   - 监控内存使用情况')
    console.log('   - 提供详细的性能统计信息')
    console.log('   - 自动清理临时文件')
  } catch (error) {
    console.error('\n❌ 基准测试失败:', (error as Error).message)
    process.exit(1)
  }
}

// 运行基准测试
if (process.argv[1] && process.argv[1].endsWith('benchmark.ts')) {
  main()
}

export {
  generateTestData,
  generateDiffData,
  formatFileSize,
  formatTime,
  type TestResult,
  type FileSize,
  type ChangeRatio,
}
