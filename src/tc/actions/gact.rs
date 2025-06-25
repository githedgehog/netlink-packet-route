// SPDX-License-Identifier: MIT

use crate::tc::actions::{TcfBuffer, TC_TCF_BUF_LEN};
use crate::tc::{TcActionGeneric, TcActionGenericBuffer, Tcf};
use netlink_packet_utils::nla::{DefaultNla, Nla, NlaBuffer};
use netlink_packet_utils::{DecodeError, Emitable, Parseable};

/// Generic action

pub struct TcGenericAction {}
impl TcGenericAction {
    pub const KIND: &'static str = "gact";
}

const TCA_GACT_TM: u16 = 1;
const TCA_GACT_PARMS: u16 = 2;
// const TCA_GACT_PROB: u16 = 3; // TODO

/// Options for the `TcActionMirror` action.
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum TcActionGenericOption {
    /// Rule installation and usage time
    Tm(Tcf),
    /// Parameters for the mirred action.
    Parms(TcActionGeneric),
    /// Other attributes unknown at the time of writing.
    Other(DefaultNla),
}

impl Nla for TcActionGenericOption {
    fn value_len(&self) -> usize {
        match self {
            Self::Tm(_) => TC_TCF_BUF_LEN,
            Self::Parms(_) => TcActionGeneric::BUF_LEN,
            Self::Other(attr) => attr.value_len(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        match self {
            Self::Tm(p) => p.emit(buffer),
            Self::Parms(p) => p.emit(buffer),
            Self::Other(attr) => attr.emit_value(buffer),
        }
    }
    fn kind(&self) -> u16 {
        match self {
            Self::Tm(_) => TCA_GACT_TM,
            Self::Parms(_) => TCA_GACT_PARMS,
            Self::Other(nla) => nla.kind(),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for TcActionGenericOption {
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            TCA_GACT_TM => Self::Tm(Tcf::parse(&TcfBuffer::new_checked(payload)?)?),
            TCA_GACT_PARMS => Self::Parms(TcActionGeneric::parse(&TcActionGenericBuffer::new(
                payload,
            ))?),
            _ => Self::Other(DefaultNla::parse(buf)?),
        })
    }
}
