//! Mechanical declared and solved type transport for the common inference terms.
use super::{FLOAT, INT, Inference, ROOT, Rc, RecordTerm, RefCell, Span, Term, Type};

impl Inference {
    /// A declared surface type as a term; the frontend writes `bigint` as `int`.
    pub(super) fn declared(&mut self, ty: Option<&Type>) -> Term {
        match ty {
            None => self.fresh(false),
            Some(Type::Named { path, .. }) => Term::Known(Type::Data {
                name: path.last().cloned().unwrap_or_default(),
            }),
            Some(Type::Nat) => Term::Known(INT),
            Some(Type::Array { element, .. }) => {
                Term::Array(Box::new(self.declared(Some(element))))
            }
            Some(Type::Option { element, absence }) => Term::Option {
                element: Box::new(self.declared(Some(element))),
                absence: Rc::new(RefCell::new(absence.clone())),
            },
            Some(Type::Tuple { elements }) => Term::Tuple(
                elements
                    .iter()
                    .map(|element| self.declared(Some(element)))
                    .collect(),
            ),
            Some(Type::Union { alternatives }) => Term::Union(
                alternatives
                    .iter()
                    .map(|ty| self.declared(Some(ty)))
                    .collect(),
            ),
            Some(Type::Map {
                key,
                value,
                reference,
            }) => Term::Map {
                key: Box::new(self.declared(Some(key))),
                value: Box::new(self.declared(Some(value))),
                reference: *reference,
            },
            Some(Type::Set { element, reference }) => Term::Set {
                element: Box::new(self.declared(Some(element))),
                reference: *reference,
            },
            Some(Type::Function { params, ret }) => Term::Function {
                params: params
                    .iter()
                    .map(|param| self.declared(Some(param)))
                    .collect(),
                ret: Box::new(self.declared(Some(ret))),
            },
            Some(Type::Record { fields, open, .. }) => {
                Term::Record(Rc::new(RefCell::new(RecordTerm {
                    fields: fields
                        .iter()
                        .map(|field| (field.name.clone(), self.declared(Some(&field.ty))))
                        .collect(),
                    open: *open,
                })))
            }
            Some(Type::Cell { element }) => Term::Cell(Box::new(self.declared(Some(element)))),
            Some(other) => Term::Known(other.clone()),
        }
    }

    /// The inferred type as a surface type; an unconstrained `BigInt` is `bigint`, anything else unconstrained a Number.
    pub(super) fn surface(&self, term: &Term, place: Option<Span>) -> Type {
        match self.resolve(term) {
            Term::Var(id) => {
                if self.bigints.contains(&id) {
                    INT
                } else {
                    FLOAT
                }
            }
            Term::Known(Type::Data { name }) => Type::Named {
                path: vec![ROOT.to_owned(), name],
                span: place,
            },
            Term::Known(ty) => ty,
            Term::Array(element) => Type::Array {
                element: Box::new(self.surface(&element, place)),
                reference: false,
            },
            Term::Option { element, absence } => crate::translation::types::option(
                self.surface(&element, place),
                absence.borrow().clone(),
            ),
            Term::Tuple(elements) => Type::Tuple {
                elements: elements
                    .iter()
                    .map(|element| self.surface(element, place))
                    .collect(),
            },
            Term::Union(alternatives) => Type::Union {
                alternatives: alternatives
                    .iter()
                    .map(|ty| self.surface(ty, place))
                    .collect(),
            },
            Term::Map {
                key,
                value,
                reference,
            } => Type::Map {
                key: Box::new(self.surface(&key, place)),
                value: Box::new(self.surface(&value, place)),
                reference,
            },
            Term::Set { element, reference } => Type::Set {
                element: Box::new(self.surface(&element, place)),
                reference,
            },
            Term::Function { params, ret } => Type::Function {
                params: params
                    .iter()
                    .map(|param| self.surface(param, place))
                    .collect(),
                ret: Box::new(self.surface(&ret, place)),
            },
            Term::Record(record) => {
                let record = record.borrow();
                Type::Record {
                    fields: record
                        .fields
                        .iter()
                        .map(|(name, term)| crate::translation::types::TypeField {
                            name: name.clone(),
                            ty: self.surface(term, place),
                        })
                        .collect(),
                    open: record.open,
                    reference: false,
                }
            }
            Term::Cell(element) => Type::Cell {
                element: Box::new(self.surface(&element, place)),
            },
        }
    }
}
