// Safe Windows HANDLE wrappers and token types
// SPDX-License-Identifier: Apache-2.0

#[cfg(windows)]
use windows::Win32::{
    Foundation::{HANDLE, CloseHandle},
    Security::TOKEN_ACCESS_MASK,
};

/// Safe wrapper for Windows HANDLE with automatic cleanup
#[cfg(windows)]
#[derive(Debug)]
pub struct SafeHandle(HANDLE);

#[cfg(windows)]
impl SafeHandle {
    /// Create a new SafeHandle from a raw HANDLE
    ///
    /// # Safety
    /// The caller must ensure the HANDLE is valid and owned by this wrapper
    pub unsafe fn from_raw(handle: HANDLE) -> Self {
        SafeHandle(handle)
    }

    /// Get the raw HANDLE value
    pub fn as_raw(&self) -> HANDLE {
        self.0
    }

    /// Check if this is a valid handle
    pub fn is_valid(&self) -> bool {
        !self.0.is_invalid()
    }

    /// Take ownership of the handle, consuming self without calling Drop
    pub fn into_raw(self) -> HANDLE {
        let handle = self.0;
        std::mem::forget(self); // Prevent Drop from being called
        handle
    }
}

#[cfg(windows)]
impl Drop for SafeHandle {
    fn drop(&mut self) {
        if self.is_valid() {
            unsafe {
                let _ = CloseHandle(self.0);
            }
        }
    }
}

#[cfg(windows)]
unsafe impl Send for SafeHandle {}

/// Token access rights commonly used
#[cfg(windows)]
pub mod token_access {
    use super::TOKEN_ACCESS_MASK;

    pub const TOKEN_QUERY: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(0x0008);
    pub const TOKEN_DUPLICATE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(0x0002);
    pub const TOKEN_ASSIGN_PRIMARY: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(0x0001);
    pub const TOKEN_IMPERSONATE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(0x0004);
    pub const TOKEN_ADJUST_PRIVILEGES: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(0x0020);
    pub const TOKEN_ADJUST_DEFAULT: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(0x0080);
    pub const TOKEN_ADJUST_SESSIONID: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(0x0100);

    pub const TOKEN_ALL_ACCESS: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(
        0x000F_0000 | // STANDARD_RIGHTS_REQUIRED
        TOKEN_QUERY.0 |
        TOKEN_DUPLICATE.0 |
        TOKEN_ASSIGN_PRIMARY.0 |
        TOKEN_IMPERSONATE.0 |
        TOKEN_ADJUST_PRIVILEGES.0 |
        TOKEN_ADJUST_DEFAULT.0 |
        TOKEN_ADJUST_SESSIONID.0
    );
}

/// Token integrity levels
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntegrityLevel {
    Untrusted = 0,
    Low = 0x1000,
    Medium = 0x2000,
    MediumPlus = 0x2100,
    High = 0x3000,
    ProtectedProcess = 0x4000,
}

impl IntegrityLevel {
    /// Parse integrity level from RID value
    /// Note: SYSTEM tokens typically have High integrity (0x3000 range)
    pub fn from_rid(rid: u32) -> Self {
        match rid {
            0..=0x0FFF => IntegrityLevel::Untrusted,
            0x1000..=0x1FFF => IntegrityLevel::Low,
            0x2000..=0x20FF => IntegrityLevel::Medium,
            0x2100..=0x2FFF => IntegrityLevel::MediumPlus,
            0x3000..=0x3FFF => IntegrityLevel::High, // SYSTEM tokens are High
            _ => IntegrityLevel::ProtectedProcess,
        }
    }
}

/// Token elevation type
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenElevationType {
    Default = 1,
    Full = 2,
    Limited = 3,
}

/// Stub types for non-Windows platforms
#[cfg(not(windows))]
#[derive(Debug)]
pub struct SafeHandle(());

#[cfg(not(windows))]
impl SafeHandle {
    pub fn as_raw(&self) -> () {
        ()
    }
}

/// Stub token_access module for non-Windows
#[cfg(not(windows))]
pub mod token_access {
    // Empty module for non-Windows platforms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integrity_level_ordering() {
        assert!(IntegrityLevel::Low < IntegrityLevel::Medium);
        assert!(IntegrityLevel::Medium < IntegrityLevel::High);
        assert!(IntegrityLevel::High < IntegrityLevel::ProtectedProcess);
    }

    #[test]
    fn test_integrity_level_from_rid() {
        assert_eq!(IntegrityLevel::from_rid(0x1000), IntegrityLevel::Low);
        assert_eq!(IntegrityLevel::from_rid(0x2000), IntegrityLevel::Medium);
        assert_eq!(IntegrityLevel::from_rid(0x3000), IntegrityLevel::High);
    }
}
