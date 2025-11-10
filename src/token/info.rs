// Token information queries
// SPDX-License-Identifier: Apache-2.0

#[cfg(windows)]
use windows::Win32::{
    Foundation::HANDLE,
    Security::{
        GetTokenInformation, TOKEN_INFORMATION_CLASS,
        TOKEN_ELEVATION_TYPE, TOKEN_MANDATORY_LABEL,
        TokenElevationType as WinTokenElevationType,
        TokenIntegrityLevel,
    },
    System::SystemServices::SID_AND_ATTRIBUTES,
};

use crate::error::Result;
use super::types::{IntegrityLevel, TokenElevationType};

/// Get token elevation type
#[cfg(windows)]
pub fn get_elevation_type(token: HANDLE) -> Result<TokenElevationType> {
    unsafe {
        let mut elevation_type: TOKEN_ELEVATION_TYPE = std::mem::zeroed();
        let mut return_length = 0u32;

        GetTokenInformation(
            token,
            WinTokenElevationType,
            Some(&mut elevation_type as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION_TYPE>() as u32,
            &mut return_length,
        )?;

        Ok(match elevation_type.0 {
            1 => TokenElevationType::Default,
            2 => TokenElevationType::Full,
            3 => TokenElevationType::Limited,
            _ => TokenElevationType::Default,
        })
    }
}

/// Get token integrity level
#[cfg(windows)]
pub fn get_integrity_level(token: HANDLE) -> Result<IntegrityLevel> {
    unsafe {
        let mut return_length = 0u32;

        // First call to get required size
        let _ = GetTokenInformation(
            token,
            TokenIntegrityLevel,
            None,
            0,
            &mut return_length,
        );

        if return_length == 0 {
            return Ok(IntegrityLevel::Medium);
        }

        // Allocate buffer and get actual data
        let mut buffer = vec![0u8; return_length as usize];
        GetTokenInformation(
            token,
            TokenIntegrityLevel,
            Some(buffer.as_mut_ptr() as *mut _),
            return_length,
            &mut return_length,
        )?;

        let label = &*(buffer.as_ptr() as *const TOKEN_MANDATORY_LABEL);

        // Get the RID (last sub-authority) from the SID
        let sid = label.Label.Sid;
        let sub_auth_count = *windows::Win32::Security::GetSidSubAuthorityCount(sid)?;

        if sub_auth_count > 0 {
            let rid = *windows::Win32::Security::GetSidSubAuthority(sid, (sub_auth_count - 1) as u32)?;
            Ok(IntegrityLevel::from_rid(rid))
        } else {
            Ok(IntegrityLevel::Medium)
        }
    }
}

/// Stub for non-Windows
#[cfg(not(windows))]
pub fn get_elevation_type(_token: ()) -> Result<TokenElevationType> {
    Err(crate::error::GodPotatoError::PlatformNotSupported)
}

/// Stub for non-Windows
#[cfg(not(windows))]
pub fn get_integrity_level(_token: ()) -> Result<IntegrityLevel> {
    Err(crate::error::GodPotatoError::PlatformNotSupported)
}
