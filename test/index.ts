import path from 'path'
import fs from 'fs'
import { strict as assert } from 'assert'
import {
  diff,
  diffSync,
  patch,
  patchSync,
  verifyPatch,
  verifyPatchSync,
  diffWithStatsSync,
  patchWithStatsSync,
  diffWithOptionsSync,
  getPatchInfoSync,
  getFileSizeSync,
  checkFileAccessSync,
  getCompressionRatioSync,
  type PatchInfoJs,
  type CompressionRatioJs,
  type PerformanceStatsJs,
  type DiffOptionsJs,
} from '../index'

describe('bsdiff (rust)', function () {
  const resDir = path.join(__dirname, 'resources')
  const oldFile = path.join(resDir, 'react-18.1.0.zip')
  const newFile = path.join(resDir, 'react-19.1.0.zip')
  const patchFile = path.join(resDir, 'react.patch')
  const generatedFile = path.join(resDir, 'react-generated.zip')

  // 设置全局超时时间为10秒，适应CI环境和大文件处理
  this.timeout(10000)

  // 检查测试文件是否存在
  before(() => {
    if (!fs.existsSync(oldFile)) {
      throw new Error(`测试文件不存在: ${oldFile}`)
    }
    if (!fs.existsSync(newFile)) {
      throw new Error(`测试文件不存在: ${newFile}`)
    }
    console.log(`📁 使用测试文件:`)
    console.log(`   old: ${oldFile} (${(fs.statSync(oldFile).size / 1024 / 1024).toFixed(2)} MB)`)
    console.log(`   new: ${newFile} (${(fs.statSync(newFile).size / 1024 / 1024).toFixed(2)} MB)`)
  })

  afterEach(() => {
    try {
      fs.unlinkSync(patchFile)
    } catch (_) {
      // 忽略删除错误
    }
    try {
      fs.unlinkSync(generatedFile)
    } catch (_) {
      // 忽略删除错误
    }
  })

  describe('#diff() + #patch() async', () => {
    it('should generate a file identical to original file after applying a patch to an old file', async function () {
      // 统一使用10秒超时
      this.timeout(10000)

      // 使用新的异步 API（简化版本，不包含进度回调）
      await diff(oldFile, newFile, patchFile)
      await patch(oldFile, generatedFile, patchFile)

      // 验证生成的文件与原始文件相同
      const originalContent = fs.readFileSync(newFile)
      const generatedContent = fs.readFileSync(generatedFile)

      assert.ok(originalContent.equals(generatedContent), 'Generated file content does not match original file')

      console.log(`✅ 异步测试成功: 补丁大小 ${(fs.statSync(patchFile).size / 1024).toFixed(2)} KB`)
    })
  })

  describe('#diffSync() + #patchSync() sync', () => {
    it('should generate a file identical to original file after applying a patch to an old file', function () {
      // 统一使用10秒超时
      this.timeout(10000)

      // 使用同步 API
      diffSync(oldFile, newFile, patchFile)
      patchSync(oldFile, generatedFile, patchFile)

      // 验证生成的文件与原始文件相同
      const originalContent = fs.readFileSync(newFile)
      const generatedContent = fs.readFileSync(generatedFile)

      assert.ok(originalContent.equals(generatedContent), 'Generated file content does not match original file')

      console.log(`✅ 同步测试成功: 补丁大小 ${(fs.statSync(patchFile).size / 1024).toFixed(2)} KB`)
    })
  })

  describe('File access validation', () => {
    it('should validate file access correctly', () => {
      // 测试存在的文件
      assert.doesNotThrow(() => {
        checkFileAccessSync(oldFile)
      }, 'Expected file access to succeed for existing file')

      // 测试不存在的文件
      assert.throws(() => {
        checkFileAccessSync('non-existent-file.zip')
      }, 'Expected error to be thrown for non-existent file')
    })
  })

  describe('File size calculation', () => {
    it('should calculate file size correctly', () => {
      const oldSize = getFileSizeSync(oldFile)
      const newSize = getFileSizeSync(newFile)

      assert.strictEqual(typeof oldSize, 'number', 'Expected oldSize to be a number')
      assert.strictEqual(typeof newSize, 'number', 'Expected newSize to be a number')
      assert.ok(oldSize > 0, 'Expected oldSize to be greater than 0')
      assert.ok(newSize > 0, 'Expected newSize to be greater than 0')

      console.log(
        `📊 文件大小: old=${(oldSize / 1024 / 1024).toFixed(2)} MB, new=${(newSize / 1024 / 1024).toFixed(2)} MB`,
      )
    })
  })

  describe('Patch verification', () => {
    it('should verify patch integrity correctly', async () => {
      // 生成补丁
      diffSync(oldFile, newFile, patchFile)

      // 验证补丁
      const isValid = verifyPatchSync(oldFile, newFile, patchFile)
      assert.strictEqual(isValid, true, 'Expected patch verification to succeed')

      // 异步验证
      const isValidAsync = await verifyPatch(oldFile, newFile, patchFile)
      assert.strictEqual(isValidAsync, true, 'Expected async patch verification to succeed')

      console.log('✅ 补丁验证成功')
    })
  })

  describe('Patch info retrieval', () => {
    it('should get patch information correctly', () => {
      // 生成补丁
      diffSync(oldFile, newFile, patchFile)

      // 获取补丁信息
      const info: PatchInfoJs = getPatchInfoSync(patchFile)

      assert.ok('size' in info, 'Expected info to have size property')
      assert.ok('isBsdiff40' in info, 'Expected info to have isBsdiff40 property')
      assert.strictEqual(typeof info.size, 'number', 'Expected info.size to be a number')
      assert.strictEqual(typeof info.isBsdiff40, 'boolean', 'Expected info.isBsdiff40 to be a boolean')
      assert.strictEqual(info.isBsdiff40, true, 'Expected info.isBsdiff40 to be true for a generated patch')
      assert.ok(info.size > 0, 'Expected info.size to be greater than 0')
      assert.strictEqual(info.size, fs.statSync(patchFile).size, 'info.size should match actual patch file size')

      console.log(`📦 补丁信息: 大小=${(info.size / 1024).toFixed(2)} KB, BSDIFF40=${info.isBsdiff40}`)
    })
  })

  describe('Compression ratio calculation', () => {
    it('should calculate compression ratio correctly', () => {
      // 生成补丁
      diffSync(oldFile, newFile, patchFile)

      // 计算压缩比
      const ratio: CompressionRatioJs = getCompressionRatioSync(oldFile, newFile, patchFile)

      // 验证属性存在
      assert.ok('oldSize' in ratio, 'Expected ratio to have oldSize property')
      assert.ok('newSize' in ratio, 'Expected ratio to have newSize property')
      assert.ok('patchSize' in ratio, 'Expected ratio to have patchSize property')
      assert.ok('ratio' in ratio, 'Expected ratio to have ratio property')

      // 验证类型
      assert.strictEqual(typeof ratio.oldSize, 'number', 'Expected ratio.oldSize to be a number')
      assert.strictEqual(typeof ratio.newSize, 'number', 'Expected ratio.newSize to be a number')
      assert.strictEqual(typeof ratio.patchSize, 'number', 'Expected ratio.patchSize to be a number')
      assert.strictEqual(typeof ratio.ratio, 'number', 'Expected ratio.ratio to be a number')

      // 验证值的合理性
      assert.ok(ratio.ratio > 0, 'Expected ratio.ratio to be greater than 0')
      assert.ok(ratio.ratio < 100, 'Expected ratio.ratio to be less than 100')

      console.log(`📈 压缩比: ${ratio.ratio.toFixed(2)}%`)
      console.log(`   - 旧文件: ${(ratio.oldSize / 1024 / 1024).toFixed(2)} MB`)
      console.log(`   - 新文件: ${(ratio.newSize / 1024 / 1024).toFixed(2)} MB`)
      console.log(`   - 补丁文件: ${(ratio.patchSize / 1024).toFixed(2)} KB`)
    })
  })

  describe('Diff/Patch with stats', () => {
    it('should return valid performance statistics from diffWithStatsSync', function () {
      this.timeout(10000)

      const stats: PerformanceStatsJs = diffWithStatsSync(oldFile, newFile, patchFile)

      // Verify all fields exist and have correct types
      assert.strictEqual(typeof stats.elapsedMs, 'number', 'Expected elapsedMs to be a number')
      assert.strictEqual(typeof stats.oldSize, 'number', 'Expected oldSize to be a number')
      assert.strictEqual(typeof stats.newSize, 'number', 'Expected newSize to be a number')
      assert.strictEqual(typeof stats.patchSize, 'number', 'Expected patchSize to be a number')
      assert.strictEqual(typeof stats.compressionRatio, 'number', 'Expected compressionRatio to be a number')

      // Verify values are reasonable
      assert.ok(stats.elapsedMs >= 0, 'Expected elapsedMs >= 0')
      assert.ok(stats.oldSize > 0, 'Expected oldSize > 0')
      assert.ok(stats.newSize > 0, 'Expected newSize > 0')
      assert.ok(stats.patchSize > 0, 'Expected patchSize > 0')
      assert.ok(stats.compressionRatio > 0, 'Expected compressionRatio > 0')
      assert.ok(stats.compressionRatio < 100, 'Expected compressionRatio < 100')

      // Verify sizes match actual file sizes
      assert.strictEqual(stats.oldSize, fs.statSync(oldFile).size, 'oldSize should match actual file size')
      assert.strictEqual(stats.newSize, fs.statSync(newFile).size, 'newSize should match actual file size')

      console.log(`diffWithStatsSync: elapsed=${stats.elapsedMs}ms, ratio=${stats.compressionRatio.toFixed(2)}%`)
    })

    it('should return valid performance statistics from patchWithStatsSync', function () {
      this.timeout(10000)

      // Generate patch first
      diffSync(oldFile, newFile, patchFile)

      const stats: PerformanceStatsJs = patchWithStatsSync(oldFile, generatedFile, patchFile)

      assert.strictEqual(typeof stats.elapsedMs, 'number', 'Expected elapsedMs to be a number')
      assert.ok(stats.elapsedMs >= 0, 'Expected elapsedMs >= 0')
      assert.ok(stats.oldSize > 0, 'Expected oldSize > 0')
      assert.ok(stats.newSize > 0, 'Expected newSize > 0')
      assert.ok(stats.patchSize > 0, 'Expected patchSize > 0')

      // Verify patched file is correct
      const originalContent = fs.readFileSync(newFile)
      const generatedContent = fs.readFileSync(generatedFile)
      assert.ok(originalContent.equals(generatedContent), 'Patched file should match original')

      console.log(`patchWithStatsSync: elapsed=${stats.elapsedMs}ms`)
    })
  })

  describe('Diff with options', () => {
    it('should generate valid patch with custom options via diffWithOptionsSync', function () {
      this.timeout(10000)

      const options: DiffOptionsJs = {
        compressionLevel: 9,
        enableParallel: false,
      }

      // Should not throw
      diffWithOptionsSync(oldFile, newFile, patchFile, options)

      // Verify patch file was created and is valid
      assert.ok(fs.existsSync(patchFile), 'Patch file should exist')
      assert.ok(fs.statSync(patchFile).size > 0, 'Patch file should not be empty')

      // Apply patch and verify correctness
      patchSync(oldFile, generatedFile, patchFile)
      const originalContent = fs.readFileSync(newFile)
      const generatedContent = fs.readFileSync(generatedFile)
      assert.ok(originalContent.equals(generatedContent), 'Patched file should match original')

      console.log(`diffWithOptionsSync (level=9, parallel=false): patch=${(fs.statSync(patchFile).size / 1024).toFixed(2)} KB`)
    })
  })

  describe('Patch verification - failure path', () => {
    it('should return false when patch does not match the target file', function () {
      this.timeout(10000)

      // Generate a valid patch from oldFile → newFile
      diffSync(oldFile, newFile, patchFile)

      // Verify against a WRONG target: oldFile instead of newFile
      // This should return false since old ≠ new
      const isValid = verifyPatchSync(oldFile, oldFile, patchFile)
      assert.strictEqual(isValid, false, 'verifyPatch should return false when target file does not match')

      console.log('✅ Patch verification correctly returns false for mismatched files')
    })
  })

  describe('Error handling improvements', () => {
    it('should provide better error messages', () => {
      // 测试文件不存在的情况
      assert.throws(
        () => diffSync('non-existent-old.zip', newFile, patchFile),
        (error: Error) => error.message.includes('Old file not found'),
        'Expected error message to contain "Old file not found"',
      )

      assert.throws(
        () => diffSync(oldFile, 'non-existent-new.zip', patchFile),
        (error: Error) => error.message.includes('New file not found'),
        'Expected error message to contain "New file not found"',
      )

      assert.throws(
        () => patchSync(oldFile, generatedFile, 'non-existent-patch.zip'),
        (error: Error) => error.message.includes('Patch file not found'),
        'Expected error message to contain "Patch file not found"',
      )
    })
  })

  describe('API compatibility', () => {
    it('should export all expected functions', () => {
      assert.strictEqual(typeof diff, 'function', 'diff function not found')
      assert.strictEqual(typeof diffSync, 'function', 'diffSync function not found')
      assert.strictEqual(typeof patch, 'function', 'patch function not found')
      assert.strictEqual(typeof patchSync, 'function', 'patchSync function not found')
      assert.strictEqual(typeof verifyPatch, 'function', 'verifyPatch function not found')
      assert.strictEqual(typeof verifyPatchSync, 'function', 'verifyPatchSync function not found')
      assert.strictEqual(typeof getPatchInfoSync, 'function', 'getPatchInfoSync function not found')
      assert.strictEqual(typeof getFileSizeSync, 'function', 'getFileSizeSync function not found')
      assert.strictEqual(typeof checkFileAccessSync, 'function', 'checkFileAccessSync function not found')
      assert.strictEqual(typeof getCompressionRatioSync, 'function', 'getCompressionRatioSync function not found')
    })
  })

  describe('Performance test', () => {
    it('should handle large files with improved performance', function () {
      // 统一使用10秒超时
      this.timeout(10000)

      const startTime = Date.now()

      // 使用改进的API
      diffSync(oldFile, newFile, patchFile)
      patchSync(oldFile, generatedFile, patchFile)

      const endTime = Date.now()
      const duration = endTime - startTime

      // 验证生成的文件与原始文件相同
      const originalContent = fs.readFileSync(newFile)
      const generatedContent = fs.readFileSync(generatedFile)

      assert.ok(originalContent.equals(generatedContent), 'Generated file content does not match original file')

      console.log(
        `⚡ 改进性能测试: 处理 ${(fs.statSync(oldFile).size / 1024 / 1024).toFixed(2)} MB 文件用时 ${duration}ms`,
      )

      // 性能应该保持在合理范围内
      assert.ok(duration < 10000, `Performance test failed: took ${duration}ms, expected less than 10000ms`)
    })
  })
})
