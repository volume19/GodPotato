// DCOM unmarshaling wrapper
// SPDX-License-Identifier: Apache-2.0

#[cfg(windows)]
use windows::{
    core::{GUID, Interface},
    Win32::System::Com::{CoUnmarshalInterface, IStream},
};

use crate::error::Result;

/// Unmarshal a COM object from a stream
///
/// This wraps the Windows API CoUnmarshalInterface to unmarshal a serialized
/// OBJREF into a live COM interface pointer.
#[cfg(windows)]
pub fn unmarshal_object(stream: &IStream, iid: &GUID) -> Result<windows::core::IUnknown> {
    unsafe {
        let mut ppv = std::ptr::null_mut();
        CoUnmarshalInterface(stream, iid, &mut ppv)?;
        Ok(windows::core::IUnknown::from_raw(ppv))
    }
}

/// Unmarshal using IID_IUnknown (default)
#[cfg(windows)]
pub fn unmarshal_unknown(stream: &IStream) -> Result<windows::core::IUnknown> {
    const IID_IUNKNOWN: GUID = GUID::from_u128(0x00000000_0000_0000_C000_000000000046);
    unmarshal_object(stream, &IID_IUNKNOWN)
}

/// Stub for non-Windows platforms
#[cfg(not(windows))]
pub fn unmarshal_object(_stream: &(), _iid: &()) -> Result<()> {
    Err(crate::error::GodPotatoError::PlatformNotSupported)
}

/// Stub for non-Windows platforms
#[cfg(not(windows))]
pub fn unmarshal_unknown(_stream: &()) -> Result<()> {
    Err(crate::error::GodPotatoError::PlatformNotSupported)
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    #[test]
    fn test_iunknown_guid() {
        const IID_IUNKNOWN: GUID = GUID::from_u128(0x00000000_0000_0000_C000_000000000046);
        // Verify the GUID is correctly formatted
        assert_eq!(format!("{:?}", IID_IUNKNOWN), "{00000000-0000-0000-C000-000000000046}");
    }
}
