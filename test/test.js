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

  describe('File access validation', () => {
    it('should validate file access correctly', () => {
      // 测试存在的文件
      try {
        bsdiff.checkFileAccessSync(oldFile)
      } catch (e) {
        throw new Error('Expected file access to succeed for existing file')
      }

      // 测试不存在的文件
      let errorThrown = false
      try {
        bsdiff.checkFileAccessSync('non-existent-file.zip')
      } catch (e) {
        errorThrown = true
      }

      if (!errorThrown) {
        throw new Error('Expected error to be thrown for non-existent file')
      }
    })
  })

  describe('File size calculation', () => {
    it('should calculate file size correctly', () => {
      const oldSize = bsdiff.getFileSizeSync(oldFile)
      const newSize = bsdiff.getFileSizeSync(newFile)

      if (typeof oldSize !== 'number') {
        throw new Error('Expected oldSize to be a number')
      }
      if (typeof newSize !== 'number') {
        throw new Error('Expected newSize to be a number')
      }
      if (oldSize <= 0) {
        throw new Error('Expected oldSize to be greater than 0')
      }
      if (newSize <= 0) {
        throw new Error('Expected newSize to be greater than 0')
      }

      console.log(
        `📊 文件大小: old=${(oldSize / 1024 / 1024).toFixed(2)} MB, new=${(newSize / 1024 / 1024).toFixed(2)} MB`,
      )
    })
  })

  describe('Patch verification', () => {
    it('should verify patch integrity correctly', async () => {
      // 生成补丁
      bsdiff.diffSync(oldFile, newFile, patchFile)

      // 验证补丁
      const isValid = bsdiff.verifyPatchSync(oldFile, newFile, patchFile)
      if (!isValid) {
        throw new Error('Expected patch verification to succeed')
      }

      // 异步验证
      const isValidAsync = await bsdiff.verifyPatch(oldFile, newFile, patchFile)
      if (!isValidAsync) {
        throw new Error('Expected async patch verification to succeed')
      }

      console.log('✅ 补丁验证成功')
    })
  })

  describe('Patch info retrieval', () => {
    it('should get patch information correctly', () => {
      // 生成补丁
      bsdiff.diffSync(oldFile, newFile, patchFile)

      // 获取补丁信息
      const info = bsdiff.getPatchInfoSync(patchFile)

      if (!info.hasOwnProperty('size')) {
        throw new Error('Expected info to have size property')
      }
      if (!info.hasOwnProperty('compressed')) {
        throw new Error('Expected info to have compressed property')
      }
      if (typeof info.size !== 'number') {
        throw new Error('Expected info.size to be a number')
      }
      if (info.compressed !== true) {
        throw new Error('Expected info.compressed to be true')
      }
      if (info.size <= 0) {
        throw new Error('Expected info.size to be greater than 0')
      }

      console.log(`📦 补丁信息: 大小=${(info.size / 1024).toFixed(2)} KB, 压缩=${info.compressed}`)
    })
  })

  describe('Compression ratio calculation', () => {
    it('should calculate compression ratio correctly', () => {
      // 生成补丁
      bsdiff.diffSync(oldFile, newFile, patchFile)

      // 计算压缩比
      const ratio = bsdiff.getCompressionRatioSync(oldFile, newFile, patchFile)

      if (!ratio.hasOwnProperty('oldSize')) {
        throw new Error('Expected ratio to have oldSize property')
      }
      if (!ratio.hasOwnProperty('newSize')) {
        throw new Error('Expected ratio to have newSize property')
      }
      if (!ratio.hasOwnProperty('patchSize')) {
        throw new Error('Expected ratio to have patchSize property')
      }
      if (!ratio.hasOwnProperty('ratio')) {
        throw new Error('Expected ratio to have ratio property')
      }

      if (typeof ratio.oldSize !== 'number') {
        throw new Error('Expected ratio.oldSize to be a number')
      }
      if (typeof ratio.newSize !== 'number') {
        throw new Error('Expected ratio.newSize to be a number')
      }
      if (typeof ratio.patchSize !== 'number') {
        throw new Error('Expected ratio.patchSize to be a number')
      }
      if (typeof ratio.ratio !== 'number') {
        throw new Error('Expected ratio.ratio to be a number')
      }

      if (ratio.ratio <= 0) {
        throw new Error('Expected ratio.ratio to be greater than 0')
      }
      if (ratio.ratio >= 100) {
        throw new Error('Expected ratio.ratio to be less than 100')
      }

      console.log(`📈 压缩比: ${ratio.ratio.toFixed(2)}%`)
      console.log(`   - 旧文件: ${(ratio.oldSize / 1024 / 1024).toFixed(2)} MB`)
      console.log(`   - 新文件: ${(ratio.newSize / 1024 / 1024).toFixed(2)} MB`)
      console.log(`   - 补丁文件: ${(ratio.patchSize / 1024).toFixed(2)} KB`)
    })
  })

  describe('Error handling improvements', () => {
    it('should provide better error messages', () => {
      // 测试文件不存在的情况
      let errorThrown = false
      try {
        bsdiff.diffSync('non-existent-old.zip', newFile, patchFile)
      } catch (e) {
        if (e.message.includes('Old file not found')) {
          errorThrown = true
        }
      }
      if (!errorThrown) {
        throw new Error('Expected error message to contain "Old file not found"')
      }

      errorThrown = false
      try {
        bsdiff.diffSync(oldFile, 'non-existent-new.zip', patchFile)
      } catch (e) {
        if (e.message.includes('New file not found')) {
          errorThrown = true
        }
      }
      if (!errorThrown) {
        throw new Error('Expected error message to contain "New file not found"')
      }

      errorThrown = false
      try {
        bsdiff.patchSync(oldFile, generatedFile, 'non-existent-patch.zip')
      } catch (e) {
        if (e.message.includes('Patch file not found')) {
          errorThrown = true
        }
      }
      if (!errorThrown) {
        throw new Error('Expected error message to contain "Patch file not found"')
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
      if (typeof bsdiff.verifyPatch !== 'function') {
        throw new Error('verifyPatch function not found')
      }
      if (typeof bsdiff.verifyPatchSync !== 'function') {
        throw new Error('verifyPatchSync function not found')
      }
      if (typeof bsdiff.getPatchInfoSync !== 'function') {
        throw new Error('getPatchInfoSync function not found')
      }
      if (typeof bsdiff.getFileSizeSync !== 'function') {
        throw new Error('getFileSizeSync function not found')
      }
      if (typeof bsdiff.checkFileAccessSync !== 'function') {
        throw new Error('checkFileAccessSync function not found')
      }
      if (typeof bsdiff.getCompressionRatioSync !== 'function') {
        throw new Error('getCompressionRatioSync function not found')
      }
    })
  })

  describe('Performance test', () => {
    it('should handle large files with improved performance', function () {
      // 统一使用10秒超时
      this.timeout(10000)

      const startTime = Date.now()

      // 使用改进的API
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

      console.log(
        `⚡ 改进性能测试: 处理 ${(fs.statSync(oldFile).size / 1024 / 1024).toFixed(2)} MB 文件用时 ${duration}ms`,
      )

      // 性能应该保持在合理范围内
      if (duration >= 6000) {
        throw new Error(`Performance test failed: took ${duration}ms, expected less than 6000ms`)
      }
    })
  })
})
