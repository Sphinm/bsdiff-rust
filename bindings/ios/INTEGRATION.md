# iOS Integration Guide

## 1. Build the static library

```bash
cd bsdiff-rust
bash scripts/build-ios.sh
# → bindings/ios/libbsdiff_core.a
```

## 2. Add to Xcode project

- Drag `bindings/ios/bsdiff.h` and `bindings/ios/BsdiffCore.swift` into your project
- Drag `bindings/ios/libbsdiff_core.a` into **Link Binary With Libraries**
- Add `bindings/ios/` to **Header Search Paths**

## 3. Replace the existing BSDiff code

### Before (current code in OfflineDownloadPatchHandler.swift)

```swift
import BSDiff  // ← Remove this pod

// In mergePatch():
BSDiff.patch(withOldFilePath: oldFilePath,
             newFilePath: newFilePath,
             patchFilePath: realPatchPath) { [weak self] path, err in
    guard let self = self else { ... }
    if !self.handleMergePatchResult(patchPath: realPatchPath, err: err, ...) {
        res.reject(...)
        return
    }
    // path/err handling nightmare (see TODO comment about ObjC nullable)
    ...
}
```

### After

```swift
// No import BSDiff needed

// In mergePatch():
let result = BsdiffCore.patch(old: oldFilePath,
                              new: newFilePath,
                              patch: realPatchPath)
switch result {
case .success:
    let url = URL(fileURLWithPath: newFilePath)
    if FileManager.default.fileExists(atPath: newFilePath) {
        res.fulfill(url)
    } else {
        res.reject(OfflineError(...))
    }
case .failure(let error):
    OfflineLogUtil.debugLog("bspatch failed: \(error)", resourceType: resourceType)
    res.reject(OfflineError(code: .patchMergeFailed, message: error.localizedDescription))
}
```

## 4. Remove the BSDiff pod

In your Podfile:
```ruby
# pod 'BSDiff'  ← Remove this line
```

Then run `pod install`.
