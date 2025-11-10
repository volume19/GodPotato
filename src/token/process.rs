// Process token operations
// SPDX-License-Identifier: Apache-2.0

#[cfg(windows)]
use windows::Win32::{
    Foundation::{HANDLE, CloseHandle},
    Security::{
        ImpersonateLoggedOnUser, RevertToSelf,
        DuplicateTokenEx, SecurityImpersonation,
        TOKEN_TYPE, TokenImpersonation, TokenPrimary,
    },
    System::Threading::{
        OpenProcess, OpenProcessToken, GetCurrentProcess,
        PROCESS_ACCESS_RIGHTS, PROCESS_QUERY_INFORMATION,
    },
};

use crate::error::Result;
use super::types::{SafeHandle, token_access, IntegrityLevel, TokenElevationType};
use super::info::{get_elevation_type, get_integrity_level};

/// Process token information
#[derive(Debug)]
pub struct ProcessToken {
    pub handle: SafeHandle,
    pub process_id: u32,
    pub integrity_level: IntegrityLevel,
    pub elevation_type: TokenElevationType,
    pub user_sid: Option<String>,
}

impl ProcessToken {
    /// Open the current process token
    #[cfg(windows)]
    pub fn open_current() -> Result<Self> {
        unsafe {
            let mut token = HANDLE::default();
            OpenProcessToken(
                GetCurrentProcess(),
                token_access::TOKEN_ALL_ACCESS,
                &mut token,
            )?;

            let safe_handle = SafeHandle::from_raw(token);
            let integrity_level = get_integrity_level(token).unwrap_or(IntegrityLevel::Medium);
            let elevation_type = get_elevation_type(token).unwrap_or(TokenElevationType::Default);

            Ok(ProcessToken {
                handle: safe_handle,
                process_id: std::process::id(),
                integrity_level,
                elevation_type,
                user_sid: None,
            })
        }
    }

    /// Open token for a specific process
    #[cfg(windows)]
    pub fn open_process(process_id: u32) -> Result<Self> {
        unsafe {
            let process_handle = OpenProcess(
                PROCESS_QUERY_INFORMATION,
                false,
                process_id,
            )?;

            let mut token = HANDLE::default();
            let result = OpenProcessToken(
                process_handle,
                token_access::TOKEN_ALL_ACCESS,
                &mut token,
            );

            CloseHandle(process_handle)?;
            result?;

            let safe_handle = SafeHandle::from_raw(token);
            let integrity_level = get_integrity_level(token).unwrap_or(IntegrityLevel::Medium);
            let elevation_type = get_elevation_type(token).unwrap_or(TokenElevationType::Default);

            Ok(ProcessToken {
                handle: safe_handle,
                process_id,
                integrity_level,
                elevation_type,
                user_sid: None,
            })
        }
    }

    /// Duplicate this token
    #[cfg(windows)]
    pub fn duplicate(&self, token_type: TOKEN_TYPE) -> Result<ProcessToken> {
        unsafe {
            let mut new_token = HANDLE::default();

            DuplicateTokenEx(
                self.handle.as_raw(),
                token_access::TOKEN_ALL_ACCESS,
                None,
                SecurityImpersonation,
                token_type,
                &mut new_token,
            )?;

            let safe_handle = SafeHandle::from_raw(new_token);
            let integrity_level = get_integrity_level(new_token).unwrap_or(IntegrityLevel::Medium);
            let elevation_type = get_elevation_type(new_token).unwrap_or(TokenElevationType::Default);

            Ok(ProcessToken {
                handle: safe_handle,
                process_id: self.process_id,
                integrity_level,
                elevation_type,
                user_sid: self.user_sid.clone(),
            })
        }
    }

    /// Duplicate as impersonation token
    #[cfg(windows)]
    pub fn duplicate_impersonation(&self) -> Result<ProcessToken> {
        self.duplicate(TokenImpersonation)
    }

    /// Duplicate as primary token
    #[cfg(windows)]
    pub fn duplicate_primary(&self) -> Result<ProcessToken> {
        self.duplicate(TokenPrimary)
    }

    /// Impersonate using this token
    #[cfg(windows)]
    pub fn impersonate(&self) -> Result<()> {
        unsafe {
            ImpersonateLoggedOnUser(self.handle.as_raw())?;
            Ok(())
        }
    }

    /// Revert impersonation
    #[cfg(windows)]
    pub fn revert() -> Result<()> {
        unsafe {
            RevertToSelf()?;
            Ok(())
        }
    }

    /// Check if this is a SYSTEM token
    pub fn is_system(&self) -> bool {
        self.user_sid.as_ref().map_or(false, |sid| sid == "S-1-5-18")
    }

    /// Check if this token has sufficient privileges
    pub fn has_high_integrity(&self) -> bool {
        self.integrity_level >= IntegrityLevel::High
    }
}

/// Stubs for non-Windows platforms
#[cfg(not(windows))]
impl ProcessToken {
    pub fn open_current() -> Result<Self> {
        Err(crate::error::GodPotatoError::PlatformNotSupported)
    }

    pub fn open_process(_process_id: u32) -> Result<Self> {
        Err(crate::error::GodPotatoError::PlatformNotSupported)
    }
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    #[test]
    fn test_open_current_token() {
        let token = ProcessToken::open_current();
        assert!(token.is_ok(), "Should be able to open current process token");

        let token = token.unwrap();
        assert!(token.handle.is_valid());
        assert_eq!(token.process_id, std::process::id());
    }

    #[test]
    fn test_duplicate_token() {
        let token = ProcessToken::open_current().unwrap();
        let dup = token.duplicate_impersonation();

        assert!(dup.is_ok(), "Should be able to duplicate token");
        let dup = dup.unwrap();
        assert!(dup.handle.is_valid());
    }
}
