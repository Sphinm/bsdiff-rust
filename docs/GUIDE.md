# 📖 bsdiff-rust Complete Guide

## Table of Contents

- [API Reference](#api-reference)
  - [Core API](#core-api)
  - [Performance Statistics API](#performance-statistics-api)
  - [Advanced Configuration API](#advanced-configuration-api)
  - [Verification Tools API](#verification-tools-api)
- [Testing](#testing)
- [Performance Benchmarks](#performance-benchmarks)

---

## API Reference

### Core API

**Synchronous Methods**

```typescript
diffSync(oldFile: string, newFile: string, patchFile: string): void
patchSync(oldFile: string, newFile: string, patchFile: string): void
```

**Asynchronous Methods**

```typescript
diff(oldFile: string, newFile: string, patchFile: string): Promise<void>
patch(oldFile: string, newFile: string, patchFile: string): Promise<void>
```

**Example**

```javascript
const bsdiff = require('@bsdiff-rust/node')

// Generate and apply patches
bsdiff.diffSync('old.zip', 'new.zip', 'patch.bin')
bsdiff.patchSync('old.zip', 'result.zip', 'patch.bin')
```

### Performance Statistics API

Returns `PerformanceStatsJs` object:

```typescript
interface PerformanceStatsJs {
  elapsedMs: number           // Operation time in milliseconds
  oldSize: number             // Old file size in bytes
  newSize: number             // New file size in bytes
  patchSize: number           // Patch file size in bytes
  compressionRatio: number    // Compression ratio (percentage)
}
```

**Methods**

```typescript
diffWithStatsSync(oldFile: string, newFile: string, patchFile: string): PerformanceStatsJs
diffWithStats(oldFile: string, newFile: string, patchFile: string): Promise<PerformanceStatsJs>
patchWithStatsSync(oldFile: string, newFile: string, patchFile: string): PerformanceStatsJs
patchWithStats(oldFile: string, newFile: string, patchFile: string): Promise<PerformanceStatsJs>
```

**Example**

```javascript
const stats = bsdiff.diffWithStatsSync('old.zip', 'new.zip', 'patch.bin')
console.log(`Time: ${stats.elapsedMs}ms`)
console.log(`Patch: ${(stats.patchSize / 1024).toFixed(2)} KB`)
console.log(`Compression: ${stats.compressionRatio.toFixed(2)}%`)
console.log(`Throughput: ${((stats.oldSize + stats.newSize) / 1024 / 1024 / (stats.elapsedMs / 1000)).toFixed(2)} MB/s`)
```

### Advanced Configuration API

Configuration options:

```typescript
interface DiffOptionsJs {
  compressionLevel?: number  // Compression level (0-9, default: 6)
  enableParallel?: boolean   // Enable parallel processing (default: true)
}
```

**Methods**

```typescript
diffWithOptionsSync(oldFile: string, newFile: string, patchFile: string, options: DiffOptionsJs): void
diffWithOptions(oldFile: string, newFile: string, patchFile: string, options: DiffOptionsJs): Promise<void>
diffWithOptionsAndStatsSync(oldFile: string, newFile: string, patchFile: string, options: DiffOptionsJs): PerformanceStatsJs
```

**Example**

```javascript
// Fast compression (development)
await bsdiff.diffWithOptions('old.zip', 'new.zip', 'patch.bin', {
  compressionLevel: 1,
  enableParallel: true
})

// Best compression (production)
await bsdiff.diffWithOptions('old.zip', 'new.zip', 'patch.bin', {
  compressionLevel: 9,
  enableParallel: true
})

// With performance stats
const stats = bsdiff.diffWithOptionsAndStatsSync('old.zip', 'new.zip', 'patch.bin', {
  compressionLevel: 6,
  enableParallel: true
})
```

### Verification Tools API

**Patch Verification**

```typescript
verifyPatchSync(oldFile: string, newFile: string, patchFile: string): boolean
verifyPatch(oldFile: string, newFile: string, patchFile: string): Promise<boolean>
```

**Patch Information**

```typescript
getPatchInfoSync(patchFile: string): PatchInfoJs

interface PatchInfoJs {
  size: number          // Patch file size in bytes
  isBsdiff40: boolean   // Whether the file starts with a valid BSDIFF40 header
}
```

**Compression Ratio Analysis**

```typescript
getCompressionRatioSync(oldFile: string, newFile: string, patchFile: string): CompressionRatioJs

interface CompressionRatioJs {
  oldSize: number    // Old file size in bytes
  newSize: number    // New file size in bytes
  patchSize: number  // Patch file size in bytes
  ratio: number      // Compression ratio (percentage)
}
```

**Utility Methods**

```typescript
getFileSizeSync(filePath: string): number
checkFileAccessSync(filePath: string): void
```

### Use Cases

**Use Case 1: Performance Monitoring**

```javascript
const stats = bsdiff.diffWithStatsSync('old.zip', 'new.zip', 'patch.bin')
console.log(`Patch generation time: ${stats.elapsedMs}ms`)
console.log(`Patch size: ${(stats.patchSize / 1024 / 1024).toFixed(2)} MB`)
console.log(`Compression rate: ${stats.compressionRatio.toFixed(2)}%`)

// Calculate bandwidth savings
const savingsPercent = (1 - stats.patchSize / stats.newSize) * 100
console.log(`Bandwidth savings vs full update: ${savingsPercent.toFixed(1)}%`)
```

**Use Case 2: Fast Development Iteration**

```javascript
await bsdiff.diffWithOptions('old.app', 'new.app', 'dev.patch', {
  compressionLevel: 1,  // Fastest compression
  enableParallel: true
})
```

**Use Case 3: Production Optimization**

```javascript
const stats = await bsdiff.diffWithOptionsAndStats('v1.zip', 'v2.zip', 'update.patch', {
  compressionLevel: 9,  // Best compression
  enableParallel: true
})

logger.info('Patch generated', {
  time: stats.elapsedMs,
  size: stats.patchSize,
  ratio: stats.compressionRatio
})
```

**Use Case 4: Integrity Verification**

```javascript
bsdiff.diffSync('old.zip', 'new.zip', 'patch.bin')

const isValid = bsdiff.verifyPatchSync('old.zip', 'new.zip', 'patch.bin')
if (isValid) {
  console.log('✅ Patch verified')
} else {
  console.error('❌ Patch verification failed')
}
```

### Error Handling

```javascript
try {
  bsdiff.diffSync('old.zip', 'new.zip', 'patch.bin')
  console.log('✅ Patch generated successfully')
} catch (error) {
  console.error('❌ Generation failed:', error.message)
}
```

---

## Testing

### Running Tests

```bash
pnpm test              # Functional tests
pnpm run bench         # Performance benchmarks
```

### Test Coverage

- **Functional Testing**: Sync/async API, file I/O, patch generation and application
- **Error Handling**: Missing files, permission errors, invalid parameters, corrupted patches
- **Performance Testing**: Different file sizes, compression levels, parallel processing
- **Data Integrity**: Patched output byte-compared against the target file
- **Utility Methods**: File operations, compression ratio calculations, patch info

### Writing Tests Example

```typescript
import { diffSync, patchSync, diffWithStatsSync } from '@bsdiff-rust/node'
import * as assert from 'assert'
import * as fs from 'fs'

describe('bsdiff test', () => {
  it('should generate and apply patch', () => {
    diffSync('old.zip', 'new.zip', 'test.patch')
    assert.ok(fs.existsSync('test.patch'))
    
    patchSync('old.zip', 'result.zip', 'test.patch')
    
    const expected = fs.readFileSync('new.zip')
    const actual = fs.readFileSync('result.zip')
    assert.deepStrictEqual(actual, expected)
  })
  
  it('should track performance', () => {
    const stats = diffWithStatsSync('old.zip', 'new.zip', 'test.patch')
    assert.ok(stats.elapsedMs < 5000)
    assert.ok(stats.compressionRatio < 100)
  })
  
  it('should throw on missing file', () => {
    assert.throws(() => {
      diffSync('nonexistent.zip', 'new.zip', 'test.patch')
    }, /file not found/i)
  })
})
```

---

## Performance Benchmarks

### Test Environment

- **Test File**: React library (1.31 MB → 1.86 MB)
- **Platform**: Linux x64
- **Node.js**: 20.x

### Test Results

**Diff Performance (Patch Generation)**

| Configuration | Time | Patch Size | Throughput | Notes |
|--------------|------|------------|------------|-------|
| Default (level 6, parallel) | 201 ms | 781.56 KB | 15.78 MB/s | Best balance |
| Fast (level 1, parallel) | 221 ms | 783.99 KB | 14.36 MB/s | Faster, only 0.3% larger |
| Best (level 9, parallel) | 217 ms | 780.96 KB | 14.62 MB/s | Smallest, only 0.1% smaller |
| Sequential (level 6, serial) | 287 ms | 781.56 KB | 11.06 MB/s | 30% slower |

**Patch Performance (Patch Application)**

- Time: 50-59 ms
- Throughput: 35-41 MB/s
- Applying patches is ~3-4x faster than generating them

### Key Findings

1. **Compression Level**: Level 1-9 patch size difference is less than 0.5%, recommend default level 6
2. **Parallel Processing**: Significant improvement for large files (>1MB), overhead exceeds benefit for small files (<500KB)
3. **vs bsdiff-node**: Diff speed +32.7%, Patch speed +93.0%, Memory usage -75%

### Configuration Recommendations

**By File Size**

| File Size | Compression | Parallel | Reason |
|-----------|------------|----------|---------|
| < 100KB | 1-3 | false | Small file parallel overhead is high |
| 100KB - 1MB | 6 | true | Balanced performance and size |
| 1MB - 10MB | 6 | true | Parallel advantage significant |
| > 10MB | 6-9 | true | Large files benefit from higher compression |

**By Use Case**

| Scenario | Recommended Config | Description |
|----------|-------------------|-------------|
| Development/Testing | Level 1, parallel on | Fast iteration |
| Production | Level 6, parallel on | Best balance |
| Software Distribution | Level 9, parallel on | Smallest patch |

### Running Benchmarks

```bash
pnpm run bench
```

**Custom Benchmarks**

```typescript
import { diffWithOptionsAndStatsSync } from '@bsdiff-rust/node'

const configs = [
  { compressionLevel: 1, enableParallel: true },
  { compressionLevel: 6, enableParallel: true },
  { compressionLevel: 9, enableParallel: true },
]

configs.forEach(config => {
  const stats = diffWithOptionsAndStatsSync('old.zip', 'new.zip', 'test.patch', config)
  console.log(`Config: ${JSON.stringify(config)}`)
  console.log(`Time: ${stats.elapsedMs}ms`)
  console.log(`Size: ${(stats.patchSize / 1024).toFixed(2)} KB`)
})
```

### Performance Monitoring

```typescript
// Set performance baseline
const BASELINE = {
  maxDiffTime: 250,   // ms
  maxPatchTime: 70,   // ms
}

it('should not regress performance', () => {
  const stats = diffWithStatsSync('old.zip', 'new.zip', 'test.patch')
  assert.ok(stats.elapsedMs < BASELINE.maxDiffTime)
  
  const patchStats = patchWithStatsSync('old.zip', 'result.zip', 'test.patch')
  assert.ok(patchStats.elapsedMs < BASELINE.maxPatchTime)
})
```

---

## Related Resources

- [Main README](../README.md)
- [GitHub Repository](https://github.com/Sphinm/bsdiff-rust)
- [npm Package](https://www.npmjs.com/package/@bsdiff-rust/node)

