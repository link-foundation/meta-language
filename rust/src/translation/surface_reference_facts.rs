//! Mechanical JSON transport for the generated surface reference rules.

use serde_json::Value;

/// Packed primitive columns; the JavaScript transport defines their order.
pub(crate) struct SurfaceReferenceFacts<'a> {
    pub columns: Vec<String>,
    pub rows: Vec<f64>,
    pub size: f64,
    pub root: f64,
    pub forms: Vec<String>,
    pub texts: Vec<String>,
    pub starts: Vec<f64>,
    pub counts: Vec<f64>,
    pub labels: Vec<String>,
    pub targets: Vec<f64>,
    pub values: Vec<&'a Value>,
}

impl SurfaceReferenceFacts<'_> {
    pub(crate) fn value(&self, identity: f64) -> Option<&Value> {
        self.values
            .get(identity.to_string().parse::<usize>().ok()?)
            .copied()
    }
}

fn number(value: usize) -> f64 {
    serde_json::Number::from(value)
        .as_f64()
        .expect("a host collection position has a finite Number representation")
}

/// Serialize only field labels, primitive spellings and ordered graph edges.
pub(crate) fn create_surface_reference_facts(value: &Value) -> SurfaceReferenceFacts<'_> {
    #[derive(Default)]
    struct Columns<'a> {
        forms: Vec<String>,
        texts: Vec<String>,
        starts: Vec<f64>,
        counts: Vec<f64>,
        labels: Vec<String>,
        targets: Vec<f64>,
        values: Vec<&'a Value>,
    }
    fn insert<'a>(value: &'a Value, into: &mut Columns<'a>) -> f64 {
        let identity = into.forms.len();
        let (form, text) = match value {
            Value::Null => ("null", String::new()),
            Value::Bool(value) => ("boolean", value.to_string()),
            Value::Number(value) => (
                "number",
                super::js_number::js_number(
                    value
                        .as_f64()
                        .expect("surface numbers have a binary64 view"),
                ),
            ),
            Value::String(value) => ("string", value.clone()),
            Value::Array(_) => ("array", String::new()),
            Value::Object(_) => ("object", String::new()),
        };
        into.forms.push(form.to_owned());
        into.texts.push(text);
        into.starts.push(0.0);
        into.counts.push(0.0);
        into.values.push(value);
        let edges: Vec<_> = match value {
            Value::Array(values) => values
                .iter()
                .enumerate()
                .map(|(position, value)| (position.to_string(), insert(value, into)))
                .collect(),
            Value::Object(fields) => fields
                .iter()
                .map(|(label, value)| (label.clone(), insert(value, into)))
                .collect(),
            _ => Vec::new(),
        };
        if matches!(value, Value::Array(_) | Value::Object(_)) {
            into.starts[identity] = number(into.labels.len());
            into.counts[identity] = number(edges.len());
        }
        for (label, target) in edges {
            into.labels.push(label);
            into.targets.push(target);
        }
        number(identity)
    }
    let mut data = Columns::default();
    let root = insert(value, &mut data);
    let size = number(data.forms.len());
    let columns = [data.forms.clone(), data.texts.clone(), data.labels.clone()].concat();
    let rows = [
        data.starts.clone(),
        data.counts.clone(),
        data.targets.clone(),
    ]
    .concat();
    SurfaceReferenceFacts {
        columns,
        rows,
        size,
        root,
        forms: data.forms,
        texts: data.texts,
        starts: data.starts,
        counts: data.counts,
        labels: data.labels,
        targets: data.targets,
        values: data.values,
    }
}
