'use strict'
const path = require('path')
const fs = require('fs')
const bsdiff = require('../index')

describe('bsdiff (rust)', function () {
  const resDir = path.join(__dirname, 'resources')
  const oldFile = path.join(resDir, 'react-0.3-stable.zip')
  const newFile = path.join(resDir, 'react-0.4-stable.zip')
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
    } catch (_) {}
    try {
      fs.unlinkSync(generatedFile)
    } catch (_) {}
  })

  describe('#diff() + #patch() async', () => {
    it('should generate a file identical to original file after applying a patch to an old file', async function () {
      // 统一使用10秒超时
      this.timeout(10000)

      // 使用新的异步 API（简化版本，不包含进度回调）
      await bsdiff.diff(oldFile, newFile, patchFile)
      await bsdiff.patch(oldFile, generatedFile, patchFile)

      // 验证生成的文件与原始文件相同
      const originalContent = fs.readFileSync(newFile)
      const generatedContent = fs.readFileSync(generatedFile)

      if (!originalContent.equals(generatedContent)) {
        throw new Error('Generated file content does not match original file')
      }

      console.log(`✅ 异步测试成功: 补丁大小 ${(fs.statSync(patchFile).size / 1024).toFixed(2)} KB`)
    })
  })

  describe('#diffSync() + #patchSync() sync', () => {
    it('should generate a file identical to original file after applying a patch to an old file', function () {
      // 统一使用10秒超时
      this.timeout(10000)

      // 使用同步 API
      bsdiff.diffSync(oldFile, newFile, patchFile)
      bsdiff.patchSync(oldFile, generatedFile, patchFile)

      // 验证生成的文件与原始文件相同
      const originalContent = fs.readFileSync(newFile)
      const generatedContent = fs.readFileSync(generatedFile)

      if (!originalContent.equals(generatedContent)) {
        throw new Error('Generated file content does not match original file')
      }

      console.log(`✅ 同步测试成功: 补丁大小 ${(fs.statSync(patchFile).size / 1024).toFixed(2)} KB`)
    })
  })

  describe('Error handling', () => {
    it('should handle non-existent files gracefully', () => {
      const nonExistentFile = path.join(resDir, 'non-existent-file.zip')

      let errorThrown = false
      try {
        bsdiff.diffSync(nonExistentFile, newFile, patchFile)
      } catch (e) {
        errorThrown = true
      }

      if (!errorThrown) {
        throw new Error('Expected error to be thrown for non-existent file')
      }
    })
  })

  describe('API compatibility', () => {
    it('should export all expected functions', () => {
      if (typeof bsdiff.diff !== 'function') {
        throw new Error('diff function not found')
      }
      if (typeof bsdiff.diffSync !== 'function') {
        throw new Error('diffSync function not found')
      }
      if (typeof bsdiff.patch !== 'function') {
        throw new Error('patch function not found')
      }
      if (typeof bsdiff.patchSync !== 'function') {
        throw new Error('patchSync function not found')
      }
    })
  })

  describe('Performance test', () => {
    it('should handle large files efficiently', function () {
      // 统一使用10秒超时
      this.timeout(10000)

      const startTime = Date.now()

      bsdiff.diffSync(oldFile, newFile, patchFile)
      bsdiff.patchSync(oldFile, generatedFile, patchFile)

      const endTime = Date.now()
      const duration = endTime - startTime

      // 验证生成的文件与原始文件相同
      const originalContent = fs.readFileSync(newFile)
      const generatedContent = fs.readFileSync(generatedFile)

      if (!originalContent.equals(generatedContent)) {
        throw new Error('Generated file content does not match original file')
      }

      console.log(`⚡ 性能测试: 处理 ${(fs.statSync(oldFile).size / 1024 / 1024).toFixed(2)} MB 文件用时 ${duration}ms`)

      // 在CI环境中允许更长的处理时间，但保持在10秒超时内
      const maxDuration = process.env.CI ? 9000 : 6000 // CI: 9秒, 本地: 6秒
      if (duration > maxDuration) {
        console.warn(`⚠️ 性能警告: 处理时间 ${duration}ms 超过预期的 ${maxDuration}ms`)
      }
    })
  })
})
