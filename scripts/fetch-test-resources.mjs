#!/usr/bin/env node
// Fetch the test fixtures that are intentionally NOT committed to git.
//
// Usage:  node scripts/fetch-test-resources.mjs   (or: pnpm test:resources)
//
// test/index.ts diffs two consecutive React releases, which are too large to
// keep in the repository. CI runs this script before the test suite; locally it
// makes `pnpm test` runnable after a fresh clone.

import { createWriteStream, existsSync, mkdirSync, statSync, rmSync } from 'node:fs'
import { Readable } from 'node:stream'
import { pipeline } from 'node:stream/promises'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const REPO_ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..')
const TARGET_DIR = path.join(REPO_ROOT, 'test', 'resources')

// A truncated/failed download is the typical failure mode, so require a
// plausible archive size before treating an existing file as usable.
const MIN_VALID_BYTES = 1024 * 1024

const RESOURCES = [
  {
    name: 'react-18.1.0.zip',
    url: 'https://github.com/facebook/react/archive/refs/tags/v18.1.0.zip',
  },
  {
    name: 'react-19.1.0.zip',
    url: 'https://github.com/facebook/react/archive/refs/tags/v19.1.0.zip',
  },
]

const mb = (bytes) => (bytes / 1024 / 1024).toFixed(2) + ' MB'

function isUsable(file) {
  return existsSync(file) && statSync(file).size >= MIN_VALID_BYTES
}

async function download({ name, url }) {
  const dest = path.join(TARGET_DIR, name)
  if (isUsable(dest)) {
    console.log(`✓ ${name} already present (${mb(statSync(dest).size)})`)
    return
  }

  process.stdout.write(`↓ ${name} ... `)
  try {
    const res = await fetch(url, { redirect: 'follow' })
    if (!res.ok) throw new Error(`HTTP ${res.status} ${res.statusText}`)
    await pipeline(Readable.fromWeb(res.body), createWriteStream(dest))
  } catch (err) {
    rmSync(dest, { force: true })
    throw new Error(`failed to download ${name}: ${err.message}`)
  }

  if (!isUsable(dest)) {
    rmSync(dest, { force: true })
    throw new Error(`${name} looks truncated (${mb(statSync(dest).size)})`)
  }
  console.log(mb(statSync(dest).size))
}

mkdirSync(TARGET_DIR, { recursive: true })

try {
  for (const resource of RESOURCES) {
    await download(resource)
  }
  console.log(`\n✅ test resources ready in ${path.relative(REPO_ROOT, TARGET_DIR)}/`)
} catch (err) {
  console.error(`\n❌ ${err.message}`)
  console.error('   Download them manually and re-run, or run without the resource-dependent tests.')
  process.exit(1)
}
