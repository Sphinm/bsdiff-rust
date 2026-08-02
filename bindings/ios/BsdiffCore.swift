import Foundation

// MARK: - BsdiffCore

/// Swift wrapper around the Rust bsdiff-core C FFI.
/// Replaces `import BSDiff` (ObjC pod) with a single Rust static library.
///
/// Usage:
///   let result = BsdiffCore.patch(old: oldPath, new: newPath, patch: patchPath)
///   if case .failure(let error) = result { print(error) }
public enum BsdiffCore {

    // MARK: Errors

    public enum Error: Swift.Error, LocalizedError {
        case patchFailed(String)
        case invalidInput(String)

        public var errorDescription: String? {
            switch self {
            case .patchFailed(let msg): return "BsdiffCore.patch failed: \(msg)"
            case .invalidInput(let msg): return "BsdiffCore.invalidInput: \(msg)"
            }
        }
    }

    // MARK: Result type

    public typealias PatchResult = Swift.Result<Void, Error>

    // MARK: Public API

    /// Apply a BSDIFF40 patch.
    ///
    /// - Parameters:
    ///   - oldPath: Path to the original file (e.g. `.../v1.0.0/v1.0.0.tar`)
    ///   - newPath: Path where the reconstructed file will be written
    ///   - patchPath: Path to the `.patch` file
    /// - Returns: `.success(())` or `.failure(Error)`
    public static func patch(old oldPath: String,
                             new newPath: String,
                             patch patchPath: String) -> PatchResult {
        // Ensure the parent directory for newPath exists
        let newURL = URL(fileURLWithPath: newPath)
        let parentDir = newURL.deletingLastPathComponent()
        do {
            try FileManager.default.createDirectory(
                at: parentDir,
                withIntermediateDirectories: true,
                attributes: nil
            )
        } catch {
            return .failure(.invalidInput("Cannot create directory \(parentDir.path): \(error)"))
        }

        let rc = bspatch(oldPath, newPath, patchPath)
        if rc == 0 {
            return .success(())
        }

        let msg = lastErrorString()
        return .failure(.patchFailed(msg))
    }

    /// Check if a file is a valid BSDIFF40 patch.
    public static func isValidPatch(_ path: String) -> Bool {
        return bsdiff_is_valid_patch(path) == 1
    }

    /// Verify that applying a patch to `oldPath` produces a file identical to `expectedNewPath`.
    /// - Returns: 1 = identical, 0 = different, -1 = error
    public static func verify(old oldPath: String,
                              expected expectedNewPath: String,
                              patch patchPath: String) -> Int32 {
        return bsdiff_verify(oldPath, expectedNewPath, patchPath)
    }

    // MARK: Private helpers

    private static func lastErrorString() -> String {
        guard let cStr = bsdiff_last_error() else {
            return "Unknown error"
        }
        defer { bsdiff_free_string(cStr) }
        return String(cString: cStr)
    }
}
