//! IEEE488.2 Trigger command
//!
use scpi::{cmd_nquery, error::Result, tree::prelude::*};

use super::IEEE4882;

/// Implements trigger logic for the `*TRG` command
pub trait CommonTrg {
    /// Called when `*TRG` is executed.
    fn trig_bus(&mut self) -> Result<()>;
}

///## 10.37 *TRG, Trigger Command
///> The Trigger command is the device-specific analog of the IEEE 488.1 defined Group Execute Trigger (GET) interface
///> message, and has exactly the same effect as a GET when received, parsed, and executed by the device. GET operation
///> is discussed in detail in 6.1.4.2.5.
pub struct TrgCommand;

impl<D> Command<D> for TrgCommand
where
    D: Device + IEEE4882 + CommonTrg,
{
    cmd_nquery!();

    fn event(
        &self,
        device: &mut D,
        _context: &mut Context,
        _params: Parameters,
        _response: ResponseUnit,
    ) -> Result<()> {
        device.trig_bus()
    }
}

/// Create a command node for `*TRG`. See [TrgCommand]
///
/// Optional, not required by IEEE488.2 (unless DT1 capability) / SCPI.
#[macro_export]
macro_rules! ieee488_trg {
    () => {
        $crate::prelude::Leaf {
            name: b"*TRG",
            default: false,
            handler: &TrgCommand,
        }
    };
}
