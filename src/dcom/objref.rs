// DCOM OBJREF protocol implementation (MS-DCOM §2.2.18)
// SPDX-License-Identifier: Apache-2.0

use crate::error::{GodPotatoError, Result};
use std::io::{Cursor, Read, Write};

const OBJREF_SIGNATURE: u32 = 0x574F454D; // "MEOW" in little-endian
const OBJREF_TYPE_STANDARD: u32 = 0x1;

/// DCOM OBJREF structure
#[derive(Debug, Clone, PartialEq)]
pub struct ObjRef {
    pub guid: Guid,
    pub standard: Standard,
}

impl ObjRef {
    /// Parse OBJREF from byte slice
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(data);

        // Read signature
        let signature = read_u32(&mut cursor)?;
        if signature != OBJREF_SIGNATURE {
            return Err(GodPotatoError::InvalidObjRefSignature(signature));
        }

        // Read type flags
        let flags = read_u32(&mut cursor)?;
        if flags != OBJREF_TYPE_STANDARD {
            return Err(GodPotatoError::InvalidObjRefType(flags));
        }

        // Read GUID
        let guid = read_guid(&mut cursor)?;

        // Read Standard OBJREF
        let standard = Standard::read(&mut cursor)?;

        Ok(ObjRef { guid, standard })
    }

    /// Serialize OBJREF to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // Write signature
        buf.extend_from_slice(&OBJREF_SIGNATURE.to_le_bytes());

        // Write type (Standard)
        buf.extend_from_slice(&OBJREF_TYPE_STANDARD.to_le_bytes());

        // Write GUID
        write_guid(&mut buf, &self.guid);

        // Write Standard
        self.standard.write(&mut buf);

        buf
    }
}

/// Standard OBJREF
#[derive(Debug, Clone, PartialEq)]
pub struct Standard {
    pub flags: u32,
    pub public_refs: u32,
    pub oxid: u64,
    pub oid: u64,
    pub ipid: Guid,
    pub dual_string_array: DualStringArray,
}

impl Standard {
    fn read(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let flags = read_u32(cursor)?;
        let public_refs = read_u32(cursor)?;
        let oxid = read_u64(cursor)?;
        let oid = read_u64(cursor)?;
        let ipid = read_guid(cursor)?;
        let dual_string_array = DualStringArray::read(cursor)?;

        Ok(Standard {
            flags,
            public_refs,
            oxid,
            oid,
            ipid,
            dual_string_array,
        })
    }

    fn write(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.flags.to_le_bytes());
        buf.extend_from_slice(&self.public_refs.to_le_bytes());
        buf.extend_from_slice(&self.oxid.to_le_bytes());
        buf.extend_from_slice(&self.oid.to_le_bytes());
        write_guid(buf, &self.ipid);
        self.dual_string_array.write(buf);
    }
}

/// Dual string array containing bindings
#[derive(Debug, Clone, PartialEq)]
pub struct DualStringArray {
    pub string_binding: StringBinding,
    pub security_binding: SecurityBinding,
}

impl DualStringArray {
    fn read(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let _num_entries = read_u16(cursor)?;
        let _security_offset = read_u16(cursor)?;

        let string_binding = StringBinding::read(cursor)?;
        let security_binding = SecurityBinding::read(cursor)?;

        Ok(DualStringArray {
            string_binding,
            security_binding,
        })
    }

    fn write(&self, buf: &mut Vec<u8>) {
        let string_bytes = self.string_binding.to_bytes();
        let security_bytes = self.security_binding.to_bytes();

        let num_entries = (string_bytes.len() + security_bytes.len()) / 2;
        let security_offset = string_bytes.len() / 2;

        buf.extend_from_slice(&(num_entries as u16).to_le_bytes());
        buf.extend_from_slice(&(security_offset as u16).to_le_bytes());
        buf.extend_from_slice(&string_bytes);
        buf.extend_from_slice(&security_bytes);
    }
}

/// String binding (protocol and network address)
#[derive(Debug, Clone, PartialEq)]
pub struct StringBinding {
    pub tower_id: TowerProtocol,
    pub network_address: String,
}

impl StringBinding {
    fn read(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let tower_id = TowerProtocol::from_u16(read_u16(cursor)?)?;
        let network_address = read_utf16_null_terminated(cursor)?;

        Ok(StringBinding {
            tower_id,
            network_address,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&(self.tower_id as u16).to_le_bytes());
        write_utf16_null_terminated(&mut buf, &self.network_address);
        buf
    }
}

/// Security binding
#[derive(Debug, Clone, PartialEq)]
pub struct SecurityBinding {
    pub authn_svc: u16,
    pub authz_svc: u16,
    pub principal_name: Option<String>,
}

impl SecurityBinding {
    fn read(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let authn_svc = read_u16(cursor)?;
        let authz_svc = read_u16(cursor)?;
        let principal_name = read_utf16_null_terminated(cursor)?;

        let principal_name = if principal_name.is_empty() {
            None
        } else {
            Some(principal_name)
        };

        Ok(SecurityBinding {
            authn_svc,
            authz_svc,
            principal_name,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.authn_svc.to_le_bytes());
        buf.extend_from_slice(&self.authz_svc.to_le_bytes());

        if let Some(ref name) = self.principal_name {
            write_utf16_null_terminated(&mut buf, name);
        } else {
            // Write double null terminator
            buf.extend_from_slice(&[0u8, 0u8, 0u8, 0u8]);
        }

        buf
    }
}

/// Tower protocol identifiers (MS-DCOM §2.2.20.1)
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TowerProtocol {
    DnetNsp = 0x04,
    OsiTp4 = 0x05,
    OsiClns = 0x06,
    Tcp = 0x07,
    Udp = 0x08,
    Ip = 0x09,
    Ncadg = 0x0a,
    Ncacn = 0x0b,
    Ncalrpc = 0x0c,
    Uuid = 0x0d,
    Ipx = 0x0e,
    Smb = 0x0f,
    NamedPipe = 0x10,
    Netbios = 0x11,
    Netbeui = 0x12,
    Spx = 0x13,
    NbIpx = 0x14,
    Dsp = 0x16,
    Ddp = 0x17,
    Appletalk = 0x18,
    VinesSpp = 0x1a,
    VinesIpc = 0x1b,
    Streettalk = 0x1c,
    Http = 0x1f,
    UnixDs = 0x20,
    Null = 0x21,
}

impl TowerProtocol {
    fn from_u16(value: u16) -> Result<Self> {
        match value {
            0x04 => Ok(TowerProtocol::DnetNsp),
            0x05 => Ok(TowerProtocol::OsiTp4),
            0x06 => Ok(TowerProtocol::OsiClns),
            0x07 => Ok(TowerProtocol::Tcp),
            0x08 => Ok(TowerProtocol::Udp),
            0x09 => Ok(TowerProtocol::Ip),
            0x0a => Ok(TowerProtocol::Ncadg),
            0x0b => Ok(TowerProtocol::Ncacn),
            0x0c => Ok(TowerProtocol::Ncalrpc),
            0x0d => Ok(TowerProtocol::Uuid),
            0x0e => Ok(TowerProtocol::Ipx),
            0x0f => Ok(TowerProtocol::Smb),
            0x10 => Ok(TowerProtocol::NamedPipe),
            0x11 => Ok(TowerProtocol::Netbios),
            0x12 => Ok(TowerProtocol::Netbeui),
            0x13 => Ok(TowerProtocol::Spx),
            0x14 => Ok(TowerProtocol::NbIpx),
            0x16 => Ok(TowerProtocol::Dsp),
            0x17 => Ok(TowerProtocol::Ddp),
            0x18 => Ok(TowerProtocol::Appletalk),
            0x1a => Ok(TowerProtocol::VinesSpp),
            0x1b => Ok(TowerProtocol::VinesIpc),
            0x1c => Ok(TowerProtocol::Streettalk),
            0x1f => Ok(TowerProtocol::Http),
            0x20 => Ok(TowerProtocol::UnixDs),
            0x21 => Ok(TowerProtocol::Null),
            _ => Err(GodPotatoError::InvalidObjRefType(value as u32)),
        }
    }
}

/// GUID (128-bit UUID)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Guid {
    pub data: [u8; 16],
}

impl Guid {
    pub fn new(data: [u8; 16]) -> Self {
        Guid { data }
    }
}

// Helper functions for binary I/O

fn read_u16(cursor: &mut Cursor<&[u8]>) -> Result<u16> {
    let mut buf = [0u8; 2];
    cursor.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

fn read_u32(cursor: &mut Cursor<&[u8]>) -> Result<u32> {
    let mut buf = [0u8; 4];
    cursor.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_u64(cursor: &mut Cursor<&[u8]>) -> Result<u64> {
    let mut buf = [0u8; 8];
    cursor.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}

fn read_guid(cursor: &mut Cursor<&[u8]>) -> Result<Guid> {
    let mut data = [0u8; 16];
    cursor.read_exact(&mut data)?;
    Ok(Guid::new(data))
}

fn write_guid(buf: &mut Vec<u8>, guid: &Guid) {
    buf.extend_from_slice(&guid.data);
}

fn read_utf16_null_terminated(cursor: &mut Cursor<&[u8]>) -> Result<String> {
    let mut chars = Vec::new();

    loop {
        let c = read_u16(cursor)?;
        if c == 0 {
            // Read second null byte (UTF-16 is two bytes per char)
            let _ = read_u16(cursor)?;
            break;
        }
        chars.push(c);
    }

    String::from_utf16(&chars).map_err(|_| GodPotatoError::Utf16Decode)
}

fn write_utf16_null_terminated(buf: &mut Vec<u8>, s: &str) {
    for c in s.encode_utf16() {
        buf.extend_from_slice(&c.to_le_bytes());
    }
    // Double null terminator
    buf.extend_from_slice(&[0u8, 0u8, 0u8, 0u8]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tower_protocol_from_u16() {
        assert_eq!(TowerProtocol::from_u16(0x07).unwrap(), TowerProtocol::Tcp);
        assert_eq!(TowerProtocol::from_u16(0x10).unwrap(), TowerProtocol::NamedPipe);
        assert!(TowerProtocol::from_u16(0xff).is_err());
    }

    #[test]
    fn test_utf16_roundtrip() {
        let original = "127.0.0.1";
        let mut buf = Vec::new();
        write_utf16_null_terminated(&mut buf, original);

        let mut cursor = Cursor::new(buf.as_slice());
        let decoded = read_utf16_null_terminated(&mut cursor).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_objref_roundtrip() {
        let guid = Guid::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        let ipid = Guid::new([16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);

        let objref = ObjRef {
            guid,
            standard: Standard {
                flags: 0,
                public_refs: 1,
                oxid: 0x1234567890abcdef,
                oid: 0xfedcba0987654321,
                ipid,
                dual_string_array: DualStringArray {
                    string_binding: StringBinding {
                        tower_id: TowerProtocol::Tcp,
                        network_address: "127.0.0.1".to_string(),
                    },
                    security_binding: SecurityBinding {
                        authn_svc: 0xa,
                        authz_svc: 0xffff,
                        principal_name: None,
                    },
                },
            },
        };

        let bytes = objref.to_bytes();
        let parsed = ObjRef::from_bytes(&bytes).unwrap();

        assert_eq!(objref, parsed);
    }

    #[test]
    fn test_invalid_signature() {
        let mut bad_data = vec![0xff, 0xff, 0xff, 0xff]; // Invalid signature
        bad_data.extend_from_slice(&[0u8; 100]);

        let result = ObjRef::from_bytes(&bad_data);
        assert!(matches!(result, Err(GodPotatoError::InvalidObjRefSignature(_))));
    }
}
