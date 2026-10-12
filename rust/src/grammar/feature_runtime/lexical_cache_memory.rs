//! Optional lexical cache storage shared by the memory budget of one parse.
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use super::executor::Executor;
use super::operations::State;
use super::results::Res;
use crate::translation::frontend_rules::accept_lexical_cache_reservation;

#[derive(Debug, Default)]
pub(super) struct LexicalCache {
    pub(super) entries: RefCell<HashMap<(usize, usize, State), Vec<Res>>>,
    pub(super) cells: Cell<usize>,
}

pub(super) type LexicalCaches = Rc<RefCell<Vec<Rc<LexicalCache>>>>;

impl Executor<'_> {
    pub(super) fn accepts_lexical_cache_reservation(&self, count: usize) -> bool {
        accept_lexical_cache_reservation(
            self.shared.cells.get() as f64,
            count as f64,
            self.shared.memory_limit as f64,
        )
    }

    pub(super) fn release_lexical_caches(&self) {
        for cache in self.shared.lexical_caches.borrow_mut().drain(..) {
            self.shared
                .cells
                .set(self.shared.cells.get() - cache.cells.replace(0));
            cache.entries.borrow_mut().clear();
        }
    }

    pub(super) fn retain_lexical_cache(&self, count: usize) -> bool {
        if !self.accepts_lexical_cache_reservation(count) {
            self.release_lexical_caches();
        }
        if !self.accepts_lexical_cache_reservation(count) {
            return false;
        }
        if self.lexical_cache.cells.get() == 0 {
            self.shared
                .lexical_caches
                .borrow_mut()
                .push(Rc::clone(&self.lexical_cache));
        }
        self.shared.cells.set(self.shared.cells.get() + count);
        self.lexical_cache
            .cells
            .set(self.lexical_cache.cells.get() + count);
        true
    }
}
