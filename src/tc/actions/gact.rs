// SPDX-License-Identifier: MIT

/// Generic action
///
/// The `gact` action is the simplest of the tc actions: it does nothing to
/// the packet beyond deciding its fate, which is carried in the generic
/// action parameters every action shares.
use std::mem::size_of;

use netlink_packet_core::{
    DecodeError, DefaultNla, Emitable, Nla, NlaBuffer, Parseable,
};

use super::{TcActionGeneric, TcActionGenericBuffer, Tcf, TcfBuffer};

/// Traffic control action which only decides the fate of a packet.
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub struct TcGenericAction {}

impl TcGenericAction {
    /// The `TcActionAttribute::Kind` of this action.
    pub const KIND: &'static str = "gact";
}

const TCA_GACT_TM: u16 = 1;
const TCA_GACT_PARMS: u16 = 2;
// const TCA_GACT_PROB: u16 = 3; // TODO

/// Options for the [`TcGenericAction`] action.
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum TcActionGenericOption {
    /// Rule installation and usage time
    Tm(Tcf),
    /// Parameters for the gact action.
    Parms(TcActionGeneric),
    /// Other attributes unknown at the time of writing.
    Other(DefaultNla),
}

impl Nla for TcActionGenericOption {
    fn value_len(&self) -> usize {
        match self {
            Self::Tm(_) => size_of::<TcfBuffer>(),
            Self::Parms(_) => size_of::<TcActionGenericBuffer>(),
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

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>>
    for TcActionGenericOption
{
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            TCA_GACT_TM => Self::Tm(Tcf::parse(payload)?),
            TCA_GACT_PARMS => Self::Parms(TcActionGeneric::parse(payload)?),
            _ => Self::Other(DefaultNla::parse(buf)?),
        })
    }
}
