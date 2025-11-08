// IStream COM interface implementation
// SPDX-License-Identifier: Apache-2.0

#[cfg(windows)]
use windows::{
    core::*,
    Win32::System::Com::*,
    Win32::Foundation::*,
};

#[cfg(windows)]
use std::io::{Cursor, Read, Write, Seek, SeekFrom};

/// Memory-backed implementation of IStream COM interface
///
/// This wraps a Cursor<Vec<u8>> and implements the IStream interface
/// for use with COM marshaling operations.
#[cfg(windows)]
#[implement(IStream)]
pub struct MemoryStream {
    cursor: std::cell::RefCell<Cursor<Vec<u8>>>,
}

#[cfg(windows)]
impl MemoryStream {
    /// Create a new MemoryStream from a byte vector
    pub fn new(data: Vec<u8>) -> Self {
        MemoryStream {
            cursor: std::cell::RefCell::new(Cursor::new(data)),
        }
    }

    /// Create a new empty MemoryStream
    pub fn empty() -> Self {
        MemoryStream {
            cursor: std::cell::RefCell::new(Cursor::new(Vec::new())),
        }
    }

    /// Get the current data as a slice
    pub fn get_data(&self) -> Vec<u8> {
        self.cursor.borrow().get_ref().clone()
    }

    /// Convert to IStream interface
    pub fn as_istream(&self) -> IStream {
        self.into()
    }
}

#[cfg(windows)]
impl IStream_Impl for MemoryStream_Impl {
    fn Read(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> Result<()> {
        if pv.is_null() {
            return Err(Error::from(E_POINTER));
        }

        let buffer = unsafe { std::slice::from_raw_parts_mut(pv as *mut u8, cb as usize) };
        let mut cursor = self.cursor.borrow_mut();

        match cursor.read(buffer) {
            Ok(bytes_read) => {
                if !pcbread.is_null() {
                    unsafe { *pcbread = bytes_read as u32 };
                }
                Ok(())
            }
            Err(e) => Err(Error::new(E_FAIL, HSTRING::from(format!("Read error: {}", e)))),
        }
    }

    fn Write(&self, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> Result<()> {
        if pv.is_null() {
            return Err(Error::from(E_POINTER));
        }

        let buffer = unsafe { std::slice::from_raw_parts(pv as *const u8, cb as usize) };
        let mut cursor = self.cursor.borrow_mut();

        match cursor.write_all(buffer) {
            Ok(()) => {
                if !pcbwritten.is_null() {
                    unsafe { *pcbwritten = cb };
                }
                Ok(())
            }
            Err(e) => Err(Error::new(E_FAIL, HSTRING::from(format!("Write error: {}", e)))),
        }
    }

    fn Seek(&self, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: *mut u64) -> Result<()> {
        let mut cursor = self.cursor.borrow_mut();

        let seek_from = match dworigin {
            STREAM_SEEK_SET => SeekFrom::Start(dlibmove as u64),
            STREAM_SEEK_CUR => SeekFrom::Current(dlibmove),
            STREAM_SEEK_END => SeekFrom::End(dlibmove),
            _ => return Err(Error::from(STG_E_INVALIDFUNCTION)),
        };

        match cursor.seek(seek_from) {
            Ok(new_pos) => {
                if !plibnewposition.is_null() {
                    unsafe { *plibnewposition = new_pos };
                }
                Ok(())
            }
            Err(e) => Err(Error::new(E_FAIL, HSTRING::from(format!("Seek error: {}", e)))),
        }
    }

    fn SetSize(&self, _libnewsize: u64) -> Result<()> {
        // Not implemented - return E_NOTIMPL
        Err(Error::from(E_NOTIMPL))
    }

    fn CopyTo(&self, _pstm: Option<&IStream>, _cb: u64, _pcbread: *mut u64, _pcbwritten: *mut u64) -> Result<()> {
        // Not implemented
        Err(Error::from(E_NOTIMPL))
    }

    fn Commit(&self, _grfcommitflags: STGC) -> Result<()> {
        // Not implemented for memory stream
        Err(Error::from(E_NOTIMPL))
    }

    fn Revert(&self) -> Result<()> {
        // Not implemented
        Err(Error::from(E_NOTIMPL))
    }

    fn LockRegion(&self, _liboffset: u64, _cb: u64, _dwlocktype: u32) -> Result<()> {
        // Not implemented
        Err(Error::from(E_NOTIMPL))
    }

    fn UnlockRegion(&self, _liboffset: u64, _cb: u64, _dwlocktype: u32) -> Result<()> {
        // Not implemented
        Err(Error::from(E_NOTIMPL))
    }

    fn Stat(&self, pstatstg: *mut STATSTG, _grfstatflag: STATFLAG) -> Result<()> {
        if pstatstg.is_null() {
            return Err(Error::from(E_POINTER));
        }

        let cursor = self.cursor.borrow();
        let size = cursor.get_ref().len() as u64;

        unsafe {
            (*pstatstg).pwcsName = PWSTR::null();
            (*pstatstg).r#type = STGTY_STREAM.0 as u32;
            (*pstatstg).cbSize = size;
            (*pstatstg).mtime = Default::default();
            (*pstatstg).ctime = Default::default();
            (*pstatstg).atime = Default::default();
            (*pstatstg).grfMode = STGM_READ;
            (*pstatstg).grfLocksSupported = 0;
            (*pstatstg).clsid = Default::default();
            (*pstatstg).grfStateBits = 0;
            (*pstatstg).reserved = 0;
        }

        Ok(())
    }

    fn Clone(&self, _ppstm: *mut Option<IStream>) -> Result<()> {
        // Not implemented
        Err(Error::from(E_NOTIMPL))
    }
}

/// Stub for non-Windows platforms
#[cfg(not(windows))]
pub struct MemoryStream {
    data: Vec<u8>,
}

#[cfg(not(windows))]
impl MemoryStream {
    pub fn new(data: Vec<u8>) -> Self {
        MemoryStream { data }
    }

    pub fn empty() -> Self {
        MemoryStream { data: Vec::new() }
    }

    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    #[test]
    fn test_memory_stream_create() {
        let data = vec![1, 2, 3, 4, 5];
        let stream = MemoryStream::new(data.clone());
        assert_eq!(stream.get_data(), data);
    }

    #[test]
    fn test_memory_stream_empty() {
        let stream = MemoryStream::empty();
        assert_eq!(stream.get_data().len(), 0);
    }
}
