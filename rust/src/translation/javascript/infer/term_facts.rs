//! Mechanical primitive-column view of actual inference terms.
//!
//! Returned plan endpoints resolve to retained term clones. Rc policy owners
//! remain shared; no term is reconstructed from a JSON type description.

use super::{HashMap, Rc, Term};
use serde_json::{Value, json};

pub(super) struct TermFacts {
    pub forms: Vec<String>,
    pub texts: Vec<String>,
    pub starts: Vec<f64>,
    pub counts: Vec<f64>,
    pub labels: Vec<String>,
    pub targets: Vec<f64>,
    positions: Vec<f64>,
    values: Vec<Value>,
    terms: Vec<Option<Term>>,
    variables: HashMap<usize, usize>,
    owners: HashMap<(u8, *const ()), usize>,
}

fn number(value: usize) -> f64 {
    serde_json::Number::from(value)
        .as_f64()
        .expect("a bounded host position has a Number representation")
}

fn position(value: f64) -> Option<usize> {
    value.to_string().parse().ok()
}

impl TermFacts {
    pub(super) fn new(inputs: &[Option<Term>]) -> Self {
        let mut facts = Self {
            forms: Vec::new(),
            texts: Vec::new(),
            starts: Vec::new(),
            counts: Vec::new(),
            labels: Vec::new(),
            targets: Vec::new(),
            positions: Vec::new(),
            values: Vec::new(),
            terms: Vec::new(),
            variables: HashMap::new(),
            owners: HashMap::new(),
        };
        for input in inputs {
            let identity = match input {
                Some(term) => facts.insert(term),
                None => facts.node("undefined", String::new(), Value::Null, None),
            };
            facts.positions.push(number(identity));
        }
        facts
    }

    /// Resolve each actual child through the existing common solver while
    /// projecting columns. Unresolved variable IDs and Rc owners are retained.
    pub(super) fn resolved(inputs: &[Option<Term>], resolve: &dyn Fn(&Term) -> Term) -> Self {
        let mut facts = Self::new(&[]);
        for input in inputs {
            let identity = match input {
                Some(term) => facts.insert_resolved(term, Some(resolve)),
                None => facts.node("undefined", String::new(), Value::Null, None),
            };
            facts.positions.push(number(identity));
        }
        facts
    }

    pub(super) fn position(&self, index: usize) -> f64 {
        self.positions.get(index).copied().unwrap_or(-1.0)
    }

    pub(super) fn term(&self, identity: f64) -> Option<Term> {
        self.terms.get(position(identity)?)?.clone()
    }

    pub(super) fn value(&self, identity: f64) -> Option<&Value> {
        self.values.get(position(identity)?)
    }

    /// Actual primitive numeric payloads; unselected nonnumeric cells use NaN.
    pub(super) fn numbers(&self) -> Vec<f64> {
        self.values
            .iter()
            .map(|value| value.as_f64().unwrap_or(f64::NAN))
            .collect()
    }

    pub(super) fn json(&mut self, value: &Value) -> f64 {
        number(self.value_tree(value))
    }

    pub(super) fn array(&mut self, values: &[Term]) -> f64 {
        number(self.sequence(values))
    }

    /// Parameter descriptors expose the retained term under its actual type field.
    pub(super) fn parameter_array(&mut self, values: &[Term]) -> f64 {
        let identity = self.node("array", String::new(), Value::Null, None);
        let mut edges = Vec::new();
        for (index, value) in values.iter().enumerate() {
            let parameter = self.node("object", String::new(), Value::Null, None);
            let ty = self.insert(value);
            self.edges(parameter, vec![("type".to_owned(), ty)]);
            edges.push((index.to_string(), parameter));
        }
        self.edges(identity, edges);
        number(identity)
    }

    /// Rebuild only a selected source-data subtree; retained terms use term().
    pub(super) fn materialize(&self, identity: f64) -> Option<Value> {
        let identity = position(identity)?;
        let start = position(*self.starts.get(identity)?)?;
        let count = position(*self.counts.get(identity)?)?;
        match self.forms.get(identity)?.as_str() {
            "array" => (start..start + count)
                .map(|edge| self.materialize(*self.targets.get(edge)?))
                .collect::<Option<Vec<_>>>()
                .map(Value::Array),
            "object" => (start..start + count)
                .map(|edge| {
                    Some((
                        self.labels.get(edge)?.clone(),
                        self.materialize(*self.targets.get(edge)?)?,
                    ))
                })
                .collect::<Option<serde_json::Map<_, _>>>()
                .map(Value::Object),
            _ => self.values.get(identity).cloned(),
        }
    }

    fn node(&mut self, form: &str, text: String, value: Value, term: Option<Term>) -> usize {
        let identity = self.forms.len();
        self.forms.push(form.to_owned());
        self.texts.push(text);
        self.starts.push(0.0);
        self.counts.push(0.0);
        self.values.push(value);
        self.terms.push(term);
        identity
    }

    fn edges(&mut self, identity: usize, edges: Vec<(String, usize)>) {
        self.starts[identity] = number(self.labels.len());
        self.counts[identity] = number(edges.len());
        for (label, target) in edges {
            self.labels.push(label);
            self.targets.push(number(target));
        }
    }

    fn scalar(&mut self, value: Value) -> usize {
        let (form, text) = match &value {
            Value::String(text) => ("string", text.clone()),
            Value::Bool(value) => ("boolean", value.to_string()),
            Value::Number(value) => ("number", value.to_string()),
            Value::Null => ("null", String::new()),
            _ => unreachable!("a primitive term fact is scalar"),
        };
        self.node(form, text, value, None)
    }

    fn value_tree(&mut self, value: &Value) -> usize {
        match value {
            Value::Array(values) => {
                let identity = self.node("array", String::new(), Value::Null, None);
                let edges = values
                    .iter()
                    .enumerate()
                    .map(|(index, value)| (index.to_string(), self.value_tree(value)))
                    .collect();
                self.edges(identity, edges);
                identity
            }
            Value::Object(fields) => {
                let identity = self.node("object", String::new(), Value::Null, None);
                let edges = fields
                    .iter()
                    .map(|(name, value)| (name.clone(), self.value_tree(value)))
                    .collect();
                self.edges(identity, edges);
                identity
            }
            _ => self.scalar(value.clone()),
        }
    }

    fn kind(&mut self, value: &str) -> (String, usize) {
        ("kind".to_owned(), self.scalar(json!(value)))
    }

    fn sequence(&mut self, values: &[Term]) -> usize {
        self.sequence_resolved(values, None)
    }

    fn sequence_resolved(
        &mut self,
        values: &[Term],
        resolve: Option<&dyn Fn(&Term) -> Term>,
    ) -> usize {
        let identity = self.node("array", String::new(), Value::Null, None);
        let edges = values
            .iter()
            .enumerate()
            .map(|(index, value)| (index.to_string(), self.insert_resolved(value, resolve)))
            .collect();
        self.edges(identity, edges);
        identity
    }

    fn insert(&mut self, term: &Term) -> usize {
        self.insert_resolved(term, None)
    }

    fn insert_resolved(
        &mut self,
        original: &Term,
        resolve: Option<&dyn Fn(&Term) -> Term>,
    ) -> usize {
        let resolved = resolve.map(|resolve| resolve(original));
        let term = resolved.as_ref().unwrap_or(original);
        if let Term::Var(variable) = term
            && let Some(identity) = self.variables.get(variable)
        {
            return *identity;
        }
        let owner = match term {
            Term::Option { absence, .. } => Some((0, Rc::as_ptr(absence).cast())),
            Term::Record(record) => Some((1, Rc::as_ptr(record).cast())),
            _ => None,
        };
        if let Some(owner) = owner
            && let Some(identity) = self.owners.get(&owner)
        {
            return *identity;
        }
        let identity = self.node(
            "object",
            String::new(),
            json!({"hostIdentity":self.terms.len()}),
            Some(term.clone()),
        );
        if let Term::Var(variable) = term {
            self.variables.insert(*variable, identity);
        }
        if let Some(owner) = owner {
            self.owners.insert(owner, identity);
        }
        let edges = match term {
            Term::Var(variable) => vec![
                self.kind("var"),
                ("id".to_owned(), self.scalar(json!(variable))),
            ],
            Term::Known(ty) => {
                let Value::Object(fields) =
                    serde_json::to_value(ty).expect("a known type is serializable")
                else {
                    unreachable!("known type facts are records");
                };
                fields
                    .iter()
                    .map(|(name, value)| (name.clone(), self.value_tree(value)))
                    .collect()
            }
            Term::Array(element) => vec![
                self.kind("array"),
                ("element".to_owned(), self.insert_resolved(element, resolve)),
            ],
            Term::Tuple(elements) => vec![
                self.kind("tuple"),
                (
                    "elements".to_owned(),
                    self.sequence_resolved(elements, resolve),
                ),
            ],
            Term::Union(elements) => vec![
                self.kind("union"),
                (
                    "alternatives".to_owned(),
                    self.sequence_resolved(elements, resolve),
                ),
            ],
            Term::Map {
                key,
                value,
                reference,
            } => vec![
                self.kind("map"),
                ("key".to_owned(), self.insert_resolved(key, resolve)),
                ("value".to_owned(), self.insert_resolved(value, resolve)),
                ("reference".to_owned(), self.scalar(json!(reference))),
            ],
            Term::Set { element, reference } => vec![
                self.kind("set"),
                ("element".to_owned(), self.insert_resolved(element, resolve)),
                ("reference".to_owned(), self.scalar(json!(reference))),
            ],
            Term::Record(record) => {
                let record = record.borrow();
                let fields = self.node("array", String::new(), Value::Null, None);
                let mut entries = Vec::new();
                for (index, (name, ty)) in record.fields.iter().enumerate() {
                    let field = self.node("object", String::new(), Value::Null, None);
                    let name = self.scalar(json!(name));
                    let ty = self.insert_resolved(ty, resolve);
                    self.edges(
                        field,
                        vec![("name".to_owned(), name), ("type".to_owned(), ty)],
                    );
                    entries.push((index.to_string(), field));
                }
                self.edges(fields, entries);
                vec![
                    self.kind("record"),
                    ("fields".to_owned(), fields),
                    ("open".to_owned(), self.scalar(json!(record.open))),
                ]
            }
            Term::Function { params, ret } => vec![
                self.kind("function"),
                ("params".to_owned(), self.sequence_resolved(params, resolve)),
                ("ret".to_owned(), self.insert_resolved(ret, resolve)),
            ],
            Term::Cell(element) => vec![
                self.kind("cell"),
                ("element".to_owned(), self.insert_resolved(element, resolve)),
            ],
            Term::Option { element, absence } => vec![
                self.kind("option"),
                ("element".to_owned(), self.insert_resolved(element, resolve)),
                ("absence".to_owned(), self.scalar(json!(&*absence.borrow()))),
            ],
        };
        self.edges(identity, edges);
        identity
    }
}
