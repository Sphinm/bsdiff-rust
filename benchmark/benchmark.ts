#!/usr/bin/env node

import * as fs from 'fs'
import * as path from 'path'
import { fileURLToPath } from 'url'
import { dirname } from 'path'
import { Bench } from 'tinybench'
import bsdiff from '../index'

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

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
  const oldFile = path.join(__dirname, `../temp/old_${prefix}.bin`)
  const newFile = path.join(__dirname, `../temp/new_${prefix}.bin`)
  const patchFile = path.join(__dirname, `../temp/patch_${prefix}.bin`)

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

// 单独测试大文件性能（可选）
async function benchmarkLargeFile(): Promise<void> {
  console.log('\n🐘 测试大文件性能 (5MB)')
  console.log('='.repeat(50))
  console.log('⚠️  注意：此测试可能需要较长时间，请耐心等待...')

  const size = 5 * 1024 * 1024 // 5MB
  const oldData = generateTestData(size)
  const newData = generateDiffData(oldData, 0.1)
  const { oldFile, newFile, patchFile, cleanup } = createTempFiles(oldData, newData, 'large')

  try {
    console.log('📝 正在生成补丁文件...')
    const startTime = Date.now()
    await bsdiff.diff(oldFile, newFile, patchFile)
    const diffTime = Date.now() - startTime
    console.log(`✅ 补丁生成完成，耗时: ${formatTime(diffTime)}`)

    const bench = new Bench({
      time: 5000, // 运行5秒
      iterations: 2, // 最少2次迭代
      warmupTime: 500, // 预热500ms
    })

    bench.add('diff 5MB', async () => {
      await bsdiff.diff(oldFile, newFile, patchFile)
    })

    await bench.run()
    console.table(bench.table())

    // 显示额外信息
    const info = bsdiff.getPatchInfoSync(patchFile)
    const ratio = bsdiff.getCompressionRatioSync(oldFile, newFile, patchFile)
    console.log(`   补丁大小: ${formatFileSize(info.size)}`)
    console.log(`   压缩比: ${(ratio.ratio * 100).toFixed(2)}%`)
  } catch (error) {
    console.error('❌ 大文件测试失败:', (error as Error).message)
  } finally {
    cleanup()
  }
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

  const appliedFile = path.join(__dirname, '../temp/applied_patch.bin')

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

// 主函数
async function main(): Promise<void> {
  console.log('🚀 bsdiff-rust 性能基准测试 (使用 tinybench)')
  console.log('='.repeat(60))

  try {
    // 测试不同文件大小
    await benchmarkDifferentSizes()

    // 测试不同变化比例
    await benchmarkChangeRatios()

    // 测试工具方法
    await benchmarkUtils()

    // 测试补丁应用
    await benchmarkPatch()

    // 综合性能测试
    await benchmarkComprehensive()

    console.log('\n✅ 基准测试完成！')
    console.log('\n📊 测试总结:')
    console.log('   - 使用 tinybench 进行现代化基准测试')
    console.log('   - 包含预热时间和多次迭代')
    console.log('   - 提供详细的性能统计信息')
    console.log('   - 自动清理临时文件')
  } catch (error) {
    console.error('\n❌ 基准测试失败:', (error as Error).message)
    process.exit(1)
  }
}

// 运行基准测试
if (import.meta.url === `file://${process.argv[1]}`) {
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
