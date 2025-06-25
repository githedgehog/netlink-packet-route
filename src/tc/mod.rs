// SPDX-License-Identifier: MIT

mod actions;
mod attribute;
mod filters;
mod header;
mod message;
mod options;
mod qdiscs;
mod stats;

pub use self::actions::{
    TcAction, TcActionAttribute, TcActionGeneric, TcActionGenericBuffer,
    TcActionMessage, TcActionMessageAttribute, TcActionMessageBuffer,
    TcActionMessageFlags, TcActionMessageFlagsWithSelector, TcActionMirror,
    TcActionMirrorOption, TcActionNat, TcActionNatOption, TcActionOption,
    TcActionTunnelKey, TcActionTunnelKeyOption, TcActionType, TcMirror,
    TcMirrorActionType, TcMirrorBuffer, TcNat, TcNatBuffer, TcNatFlags,
    TcTunnelKey, Tcf,
};
pub use self::attribute::TcAttribute;
pub use self::filters::{
    TcFilterFlower, TcFilterFlowerMplsLseOption, TcFilterFlowerMplsOption,
    TcFilterFlowerOption, TcFilterMatchAll, TcFilterMatchAllOption,
    TcFilterU32, TcFilterU32Option, TcFlowerOptionFlags, TcU32Key,
    TcU32OptionFlags, TcU32Selector, TcU32SelectorBuffer, TcU32SelectorFlags,
};
pub use self::header::{TcHandle, TcHeader, TcMessageBuffer};
pub use self::message::TcMessage;
pub use self::options::TcOption;
pub use self::qdiscs::{
    TcFqCodelClStats, TcFqCodelClStatsBuffer, TcFqCodelQdStats,
    TcFqCodelQdStatsBuffer, TcFqCodelXstats, TcQdiscFqCodel,
    TcQdiscFqCodelOption, TcQdiscIngress, TcQdiscIngressOption,
};
pub use self::stats::{
    TcStats, TcStats2, TcStatsBasic, TcStatsBasicBuffer, TcStatsBuffer,
    TcStatsQueue, TcStatsQueueBuffer, TcXstats,
};

pub(crate) use self::options::VecTcOption;
pub use self::{
    actions::{
        TcAction, TcActionAttribute, TcActionGeneric, TcActionGenericBuffer,
        TcActionMessage, TcActionMessageAttribute, TcActionMessageBuffer,
        TcActionMessageFlags, TcActionMessageFlagsWithSelector, TcActionMirror,
        TcActionMirrorOption, TcActionNat, TcActionNatOption, TcActionOption,
        TcActionTunnelKey, TcActionTunnelKeyOption, TcActionType, TcMirror,
        TcMirrorActionType, TcMirrorBuffer, TcNat, TcNatBuffer, TcNatFlags,
        TcTunnelKey, Tcf,
    },
    attribute::TcAttribute,
    filters::{
        TcFilterFlower, TcFilterFlowerMplsLseOption, TcFilterFlowerMplsOption,
        TcFilterFlowerOption, TcFilterMatchAll, TcFilterMatchAllOption,
        TcFilterU32, TcFilterU32Option, TcU32Key, TcU32OptionFlags,
        TcU32Selector, TcU32SelectorBuffer, TcU32SelectorFlags,
    },
    header::{TcHandle, TcHeader, TcMessageBuffer},
    message::TcMessage,
    options::TcOption,
    qdiscs::{
        TcFqCodelClStats, TcFqCodelClStatsBuffer, TcFqCodelQdStats,
        TcFqCodelQdStatsBuffer, TcFqCodelXstats, TcQdiscFqCodel,
        TcQdiscFqCodelOption, TcQdiscIngress, TcQdiscIngressOption,
    },
    stats::{
        TcStats, TcStats2, TcStatsBasic, TcStatsBasicBuffer, TcStatsBuffer,
        TcStatsQueue, TcStatsQueueBuffer, TcXstats,
    },
};

#[cfg(test)]
mod tests;
