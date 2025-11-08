// DCOM protocol support
// SPDX-License-Identifier: Apache-2.0

pub mod objref;
pub mod unmarshal;

pub use objref::{ObjRef, Standard, DualStringArray, StringBinding, SecurityBinding, TowerProtocol};
pub use unmarshal::{unmarshal_object, unmarshal_unknown};
