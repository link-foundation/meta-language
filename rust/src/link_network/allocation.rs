//! Actual native identifier transport into the shared allocation definition.
use super::{LinkId, LinkNetwork};

impl LinkNetwork {
    fn cursor_has_capacity(cursor: u128, count: u32) -> bool {
        let maximum = u128::from(u64::MAX) + 1;
        let word = |shift| {
            f64::from(
                u16::try_from((cursor >> shift) & u128::from(u16::MAX))
                    .expect("a masked allocator word fits u16"),
            )
        };
        cursor <= maximum
            && crate::link_allocation_rules::accept_link_allocation_capacity(
                word(0),
                word(16),
                word(32),
                word(48),
                cursor == maximum,
                f64::from(count),
            )
    }

    pub(crate) fn has_allocation_capacity(&self, count: u32) -> bool {
        Self::cursor_has_capacity(self.next_id, count)
    }

    pub(crate) fn require_allocation_capacity(&self, count: u32) {
        assert!(
            self.has_allocation_capacity(count),
            "meta-language link identifier space exhausted"
        );
    }

    pub(crate) fn allocate_id(&mut self) -> LinkId {
        Self::allocate_cursor(&mut self.next_id)
    }

    pub(crate) fn allocate_cursor(cursor: &mut u128) -> LinkId {
        assert!(
            Self::cursor_has_capacity(*cursor, 1),
            "meta-language link identifier space exhausted"
        );
        let id = LinkId::from_u64(
            u64::try_from(*cursor).expect("a capacity-checked link identity fits u64"),
        );
        *cursor += 1;
        id
    }

    pub(crate) fn observe_identifier(&mut self, id: LinkId) {
        self.next_id = self.next_id.max(u128::from(id.as_u64()) + 1);
    }
}
