/// The machine a semantic action runs on: the result's text and children,
/// and the fresh node it may annotate or rename.
struct ActionMachine<'e, 'c, 'n> {
    executor: &'e mut Executor<'c>,
    node: &'n mut Tree,
    children: Children,
    working: Working,
    start: usize,
    end: usize,
}

impl ActionMachine<'_, '_, '_> {
    // A field name selects the direct children captured under it, or else
    // the direct children of that kind.
    fn named(&self, field: &str) -> Vec<Rc<Tree>> {
        let meaningful = self.children.iter().filter(|child| !child.trivia);
        let captured: Vec<Rc<Tree>> = meaningful
            .clone()
            .filter(|child| child.field.as_deref() == Some(field))
            .cloned()
            .collect();
        if captured.is_empty() {
            meaningful
                .filter(|child| child.kind.as_deref() == Some(field))
                .cloned()
                .collect()
        } else {
            captured
        }
    }
}

fn attribute_of(child: &Tree, name: &str) -> OpResult<OperationValue> {
    child
        .attributes
        .as_ref()
        .and_then(|attributes| attributes.get(name))
        .cloned()
        .ok_or(OpError::Failed)
}

impl Machine for ActionMachine<'_, '_, '_> {
    fn state(&mut self) -> &mut Working {
        &mut self.working
    }

    fn step(&mut self) -> Result<(), Abort> {
        self.executor.step()
    }

    fn column(&mut self) -> usize {
        column_of(self.executor.bytes, self.start, self.executor.begin)
    }

    fn at_end(&mut self) -> bool {
        self.end == self.executor.end
    }

    fn at_start(&mut self) -> bool {
        self.end == 0
    }

    fn matched(&mut self) -> OpResult<String> {
        Ok(self.executor.text(self.start, self.end))
    }

    fn attribute(&mut self, field: &str, name: &str) -> OpResult<OperationValue> {
        let named = self.named(field);
        attribute_of(named.first().ok_or(OpError::Failed)?, name)
    }

    fn attributes(&mut self, field: &str, name: &str) -> OpResult<Vec<OperationValue>> {
        self.named(field)
            .iter()
            .map(|child| attribute_of(child, name))
            .collect()
    }

    fn field_text(&mut self, field: &str) -> OpResult<String> {
        let named = self.named(field);
        let child = named.first().ok_or(OpError::Failed)?;
        let start = if child.ty == TreeType::Node {
            content_start(&child.children, child.start)
        } else {
            child.start
        };
        Ok(self.executor.text(start, child.end))
    }

    fn set_attribute(&mut self, name: &str, value: OperationValue) {
        super::attributes::set_attribute(self.node, name, value);
    }

    fn build_node(&mut self, kind: &Arc<str>) {
        self.node.kind = Some(kind.clone());
    }
}

/// The machine an external scanner runs on.
struct ScannerMachine<'e, 'c> {
    executor: &'e mut Executor<'c>,
    requested: Name,
    /// The offset the scanner answers `expected` for.
    context: usize,
    working: Working,
    cursor: usize,
    token_start: usize,
    mark: Option<usize>,
    skipped: Vec<Rc<Tree>>,
}

impl ScannerMachine<'_, '_> {
    // The end of the longest match of `item` at the cursor, in the state the
    // preceding operations leave it in.
    fn match_at(&mut self, item: &Expr) -> OpResult<Option<usize>> {
        let cursor = self.cursor;
        let state = self.working.clone().settle();
        let results = self
            .executor
            .quietly(|this| this.evaluate(item, cursor, &state, true))?;
        Ok(longest_result(results).map(|result| result.end))
    }
}

impl Machine for ScannerMachine<'_, '_> {
    fn state(&mut self) -> &mut Working {
        &mut self.working
    }

    fn step(&mut self) -> Result<(), Abort> {
        self.executor.step()
    }

    fn requested(&self) -> Option<&str> {
        Some(&self.requested)
    }

    fn expected(&mut self, id: usize) -> bool {
        let item = (self.executor.program_index, id);
        self.executor
            .shared
            .expectations
            .borrow_mut()
            .holds(self.context, item)
    }

    fn column(&mut self) -> usize {
        column_of(self.executor.bytes, self.cursor, self.executor.begin)
    }

    fn matched(&mut self) -> OpResult<String> {
        Ok(self.executor.text(self.token_start, self.cursor))
    }

    fn at_end(&mut self) -> bool {
        self.cursor >= self.executor.end
    }

    fn at_start(&mut self) -> bool {
        self.cursor == 0
    }

    fn lookahead(&mut self, item: &Expr) -> OpResult<bool> {
        Ok(self.match_at(item)?.is_some())
    }

    fn advance(&mut self) -> OpResult<()> {
        if self.cursor >= self.executor.end {
            return Err(OpError::Failed);
        }
        self.cursor += decode_at(self.executor.bytes, self.cursor, self.executor.end).1;
        Ok(())
    }

    fn consume(&mut self, item: &Expr) -> OpResult<()> {
        self.cursor = self.match_at(item)?.ok_or(OpError::Failed)?;
        Ok(())
    }

    fn skip(&mut self, item: &Expr) -> OpResult<()> {
        let end = self.match_at(item)?;
        let Some(end) = end.filter(|_| self.cursor == self.token_start) else {
            return Err(OpError::Failed);
        };
        if end > self.cursor {
            let mut leaf = Tree::new(TreeType::Token, None, self.cursor, end);
            leaf.trivia = true;
            self.skipped.push(Rc::new(leaf));
        }
        self.cursor = end;
        self.token_start = end;
        Ok(())
    }

    fn mark(&mut self) {
        self.mark = Some(self.cursor);
    }
}
