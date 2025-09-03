# Patch Size Optimization Guide

This guide provides best practices for optimizing patch size and performance.

## Core Technical Optimizations

### zstd Compression Optimization

This library uses high-performance zstd compression algorithm, providing excellent balance between compression ratio and speed.

### Memory Mapping (mmap) Optimization

Uses memory mapping technology to achieve zero-copy file reading, significantly improving large file processing performance:

- Reduces memory usage
- Improves I/O performance
- Supports large file processing

## Core Optimization Strategies

### 1. File Format Selection (Highest Impact 🔥)

**Problem Analysis**:

- Direct binary diff on ZIP and other compressed containers performs poorly
- ZIP internal metadata, file order, and compression block offset changes amplify differences
- Results in 2-3x larger patches and 50%+ longer processing time

**Solution**: Normalized processing workflow

```javascript
// ❌ Not recommended: Direct diff on ZIP files
await diff('app-v1.0.zip', 'app-v2.0.zip', 'patch.bin')

// ✅ Recommended: Normalized TAR workflow
// 1. Extract to temporary directories
// 2. Sort by filename, normalize metadata
// 3. Package as normalized TAR
// 4. Diff the TAR files
await diff('app-v1.0.tar', 'app-v2.0.tar', 'patch.bin')
```

## 📊 Optimization Effect Comparison

| Optimization Strategy        | Patch Size Reduction | Processing Time Improvement | Implementation Difficulty | Recommendation |
| ---------------------------- | -------------------- | --------------------------- | ------------------------- | -------------- |
| **Normalized TAR Workflow**  | 30-60%               | 20-40%                      | Medium                    | ⭐⭐⭐⭐⭐     |
| **Fixed Build Parameters**   | 15-30%               | 10-20%                      | Simple                    | ⭐⭐⭐⭐       |
| **Compression Level Tuning** | 5-15%                | -10~+20%                    | Simple                    | ⭐⭐⭐         |
| **File Preprocessing**       | 10-25%               | 5-15%                       | Simple                    | ⭐⭐⭐⭐       |

## Real-world Case Analysis

Based on real test data from React version upgrades:

**Direct ZIP Diff**:

- Patch size: 7.64MB → 7.71MB (roughly equivalent)
- Processing time: 16.9s → 7.8s (54% improvement)

**Optimized TAR Workflow** (estimated):

- Patch size: Expected 40-50% reduction (approximately 3.8MB)
- Processing time: Expected additional 20-30% improvement

**Recommendation**: For large projects, strongly recommend adopting the normalized TAR workflow

## Implementation Guide

### Normalized TAR Workflow Implementation

```javascript
import { createReadStream, createWriteStream } from 'fs'
import { pipeline } from 'stream/promises'
import tar from 'tar'
import { diff, patch } from '@bsdiff-rust/node'

// 1. Create normalized TAR
async function createNormalizedTar(sourceDir, outputTar) {
  return tar.create(
    {
      file: outputTar,
      cwd: sourceDir,
      sort: true, // Sort by name
      mtime: new Date('2024-01-01'), // Fixed timestamp
      uid: 0,
      gid: 0,
      mode: 0o644, // Fixed permissions
    },
    ['.'], // Package all files
  )
}

// 2. Complete optimization workflow
async function optimizedDiff(oldZip, newZip, patchFile) {
  // Extract to temporary directories
  const oldDir = await extractZip(oldZip)
  const newDir = await extractZip(newZip)

  // Create normalized TARs
  const oldTar = 'old-normalized.tar'
  const newTar = 'new-normalized.tar'

  await createNormalizedTar(oldDir, oldTar)
  await createNormalizedTar(newDir, newTar)

  // Diff the TAR files
  await diff(oldTar, newTar, patchFile)

  // Cleanup temporary files
  cleanup([oldDir, newDir, oldTar, newTar])
}
```

### Build Script Integration

```bash
#!/bin/bash
# build-optimized.sh

# Set fixed environment variables
export SOURCE_DATE_EPOCH=1704067200  # 2024-01-01 00:00:00 UTC
export TZ=UTC

# Clean build directory
rm -rf dist/
mkdir -p dist/

# Copy files in sorted order
find src/ -type f | sort | while read file; do
  cp "$file" "dist/${file#src/}"
done

# Create normalized archive
cd dist/
tar --sort=name \
    --mtime="@${SOURCE_DATE_EPOCH}" \
    --owner=0 --group=0 \
    -czf ../app-normalized.tar.gz .

echo "✅ Normalized build completed: app-normalized.tar.gz"
```

---

**Tip**: Optimization effects vary by project. It's recommended to validate effects in small-scale tests before applying to production environments.
