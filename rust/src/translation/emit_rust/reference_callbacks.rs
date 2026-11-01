//! Preserve the established emitter entry point with shared graph transport.
use super::{Expr, RustEmitter};

impl RustEmitter<'_> {
    pub(super) fn read_only_reference_callback(&self, callback: &Expr) -> bool {
        crate::translation::reference_callbacks::read_only_reference_callback(callback,&self.program.declarations)
    }
}
