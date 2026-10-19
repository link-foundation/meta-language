//! The existing scanner expectation request operation, kept separate from
//! rule evaluation so bounded source modules retain their original behavior.

use super::executor::Executor;
use super::forking::target_name;
use super::program::Expr;

impl Executor<'_> {
    /// Records that the parse requests `item`, a literal or a rule a
    /// scanner's `expected` may ask about, at `position` (see
    /// `Expectations`).
    pub(super) fn request_item(&self, item: &Expr, position: usize) {
        let program = self.program;
        let id = match item {
            Expr::Ref(target) => program
                .expected_references
                .get(&**target_name(target, &program.rules))
                .copied(),
            Expr::Terminal { expected, .. } => *expected,
            _ => None,
        };
        if let Some(id) = id {
            self.shared
                .expectations
                .borrow_mut()
                .request(position, (self.program_index, id));
        }
    }
}
