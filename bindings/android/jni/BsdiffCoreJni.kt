package com.atome.bsdiff

/**
 * JNI bridge to the Rust bsdiff-core library.
 *
 * Replaces `com.atome.bsdiff.PatchUtils` (the existing Android BSDiff wrapper).
 *
 * Usage (identical to current PatchUtils API):
 *   BsdiffCore.patch(oldFile, newFile, patchFile)
 */
object BsdiffCore {

    init {
        System.loadLibrary("bsdiff_core")
    }

    // -----------------------------------------------------------------------
    // External JNI declarations
    // -----------------------------------------------------------------------

    /** Apply a BSDIFF40 patch. Returns 0 on success, -1 on error. */
    @JvmStatic
    private external fun nativePatch(
        oldPath: String,
        newPath: String,
        patchPath: String,
    ): Int

    /** Check if a file is a valid BSDIFF40 patch. Returns 1 = yes, 0 = no, -1 = error. */
    @JvmStatic
    private external fun nativeIsValidPatch(patchPath: String): Int

    /** Get the last error message from native code. Returns null if no error. */
    @JvmStatic
    private external fun nativeLastError(): String?

    // -----------------------------------------------------------------------
    // Public API (compatible with current PatchUtils usage)
    // -----------------------------------------------------------------------

    /**
     * Apply a BSDIFF40 patch.
     *
     * @param oldPath   Path to the original file (e.g. the old .zip/.tar)
     * @param newPath   Path where the reconstructed file will be written
     * @param patchPath Path to the .patch file
     * @throws BsdiffException if the patch fails
     */
    @JvmStatic
    @Throws(BsdiffException::class)
    fun patch(oldPath: String, newPath: String, patchPath: String) {
        // Ensure parent directory exists
        val parentDir = java.io.File(newPath).parentFile
        if (parentDir != null && !parentDir.exists()) {
            parentDir.mkdirs()
        }

        val rc = nativePatch(oldPath, newPath, patchPath)
        if (rc != 0) {
            val error = nativeLastError() ?: "Unknown error"
            throw BsdiffException("bspatch failed: $error")
        }
    }

    /**
     * Check if a file is a valid BSDIFF40 patch.
     */
    @JvmStatic
    fun isValidPatch(patchPath: String): Boolean {
        return nativeIsValidPatch(patchPath) == 1
    }

    /**
     * Get the last error from native code.
     */
    @JvmStatic
    fun lastError(): String? {
        return nativeLastError()
    }
}

/**
 * Exception thrown when a BSDiff patch operation fails.
 */
class BsdiffException(message: String) : Exception(message)
