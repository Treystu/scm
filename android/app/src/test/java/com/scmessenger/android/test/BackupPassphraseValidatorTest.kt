package com.scmessenger.android.utils

import org.junit.Assert.assertEquals
import org.junit.Test

class BackupPassphraseValidatorTest {

    @Test
    fun `passphrase shorter than 8 characters is too short`() {
        assertEquals(
            BackupPassphraseValidation.TooShort,
            validateBackupPassphrase("short", "short")
        )
    }

    @Test
    fun `mismatched confirmation is rejected even if both are long enough`() {
        assertEquals(
            BackupPassphraseValidation.Mismatch,
            validateBackupPassphrase("correct-horse-battery", "correct-horse-batteryy")
        )
    }

    @Test
    fun `matching passphrase of sufficient length is valid`() {
        assertEquals(
            BackupPassphraseValidation.Valid,
            validateBackupPassphrase("correct-horse-battery", "correct-horse-battery")
        )
    }

    @Test
    fun `length check takes priority over mismatch check`() {
        // Both are short AND don't match; TooShort should win so the user
        // isn't told to "fix the mismatch" into another too-short passphrase.
        assertEquals(
            BackupPassphraseValidation.TooShort,
            validateBackupPassphrase("ab", "cd")
        )
    }
}
