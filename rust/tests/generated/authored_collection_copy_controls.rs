// Translated from JavaScript by meta-language: portable core, Rust target.
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        match index.and_then(|index| values.get(index)) {
            Some(value) => value.clone(),
            None => panic!("array index out of range"),
        }
    }

    /// The index a Number names: a non-negative integer, -0 included.
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9007199254740992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    fn integer(value: f64) -> f64 { if value.is_nan() { 0.0 } else { value.trunc() } }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 { (length as f64 + value).max(0.0) as usize } else { value.min(length as f64) as usize }
    }
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len()); let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> f64 {
        if values.is_empty() { return -1.0; }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) { return -1.0; }
            let start = if from < 0.0 { (values.len() as f64 + from) as usize } else { from.min((values.len() - 1) as f64) as usize };
            (0..=start).rev().find(|&index| equal(&values[index], search)).map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 { (values.len() as f64 + from).max(0.0) as usize } else { from.min(values.len() as f64) as usize };
            (start..values.len()).find(|&index| equal(&values[index], search)).map_or(-1.0, |index| index as f64)
        }
    }
    pub fn includes<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values { if !out.iter().any(|previous| equal(previous, &value)) { out.push(value); } }
        out
    }
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) { out[index].1 = value; }
            else { out.push((key, value)); }
        }
        out
    }
}

pub mod ml_source_exception {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SourceException {
        pub class_name: String,
        pub message_units: Vec<u16>,
        pub message_present: bool,
    }
    impl SourceException {
        pub fn raise(self) -> ! {
            std::panic::panic_any((self.class_name, self.message_units, self.message_present))
        }
        pub fn from_text(class_name: &str, message: &str) -> Self {
            Self { class_name: class_name.to_owned(), message_units: message.encode_utf16().collect(), message_present: true }
        }
    }
    impl std::fmt::Display for SourceException {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.message_units))
        }
    }
    impl std::error::Error for SourceException {}
}

pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required Number fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

pub fn refuse_required_text_fact() -> String {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required text fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required Boolean fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

pub fn read_required_number_fact(values: Vec<f64>, index: f64) -> f64 {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required Number fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_number_fact())
    }
}

pub fn read_required_text_fact(values: Vec<String>, index: f64) -> String {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required text fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

pub fn read_required_boolean_fact(values: Vec<bool>, index: f64) -> bool {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required Boolean fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_boolean_fact())
    }
}

pub fn read_collection_construction_field(starts: Vec<f64>, counts: Vec<f64>, labels: Vec<String>, targets: Vec<f64>, node: f64, field: String) -> f64 {
    if (node < (0f64)) {
        (-1f64)
    } else {
        {
            let start = self::read_required_number_fact(starts, node);
            {
                let count = self::read_required_number_fact(counts, node);
                {
                    let at = start;
                    {
                        let source_ml_source2_s1 = self::source_ml_source1_temporary_read_collection_construction_field_loop1(labels, targets, field.clone(), start, count, at);
                        match source_ml_source2_s1.clone() {
                            self::MlSource1TemporaryReadCollectionConstructionFieldLoop1Result::MlSource1TemporaryReadCollectionConstructionFieldLoop1Done => {
                                (-1f64)
                            }
                            self::MlSource1TemporaryReadCollectionConstructionFieldLoop1Result::MlSource1TemporaryReadCollectionConstructionFieldLoop1Return(source_ml_source1_temporary_result1) => {
                                source_ml_source1_temporary_result1
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_collection_construction_element(starts: Vec<f64>, counts: Vec<f64>, targets: Vec<f64>, node: f64, index: f64) -> f64 {
    if (((node < (0f64)) || (index < (0f64))) || (index >= (self::read_required_number_fact(counts, node)))) {
        (-1f64)
    } else {
        self::read_required_number_fact(targets, (self::read_required_number_fact(starts, node) + index))
    }
}

pub fn read_collection_construction_kind(texts: Vec<String>, starts: Vec<f64>, counts: Vec<f64>, labels: Vec<String>, targets: Vec<f64>, node: f64) -> String {
    {
        let kind = self::read_collection_construction_field(starts, counts, labels, targets, node, String::from("kind"));
        if (kind < (0f64)) {
            String::from("")
        } else {
            self::read_required_text_fact(texts, kind)
        }
    }
}

pub fn plan_collection_construction_inference(texts: Vec<String>, starts: Vec<f64>, counts: Vec<f64>, labels: Vec<String>, targets: Vec<f64>, kind: String, expected: f64) -> Vec<f64> {
    {
        let context = ((self::read_collection_construction_kind(texts, starts.clone(), counts.clone(), labels.clone(), targets.clone(), expected)) == kind);
        if (kind == "set") {
            vec![1f64, if context {
                1f64
            } else {
                0f64
            }, if context {
                self::read_collection_construction_field(starts, counts, labels, targets, expected, String::from("element"))
            } else {
                (-1f64)
            }, (-1f64)]
        } else {
            if (kind == "map") {
                vec![2f64, if context {
                    1f64
                } else {
                    0f64
                }, if context {
                    self::read_collection_construction_field(starts.clone(), counts.clone(), labels.clone(), targets.clone(), expected, String::from("key"))
                } else {
                    (-1f64)
                }, if context {
                    self::read_collection_construction_field(starts, counts, labels, targets, expected, String::from("value"))
                } else {
                    (-1f64)
                }]
            } else {
                vec![0f64, 0f64, (-1f64), (-1f64)]
            }
        }
    }
}

pub fn plan_collection_construction_checked(texts: Vec<String>, starts: Vec<f64>, counts: Vec<f64>, labels: Vec<String>, targets: Vec<f64>, kind: String, items: f64) -> Vec<f64> {
    {
        let array = ((self::read_collection_construction_kind(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), items)) == "array");
        {
            let element = self::read_collection_construction_field(starts.clone(), counts.clone(), labels.clone(), targets.clone(), items, String::from("element"));
            if (kind == "set") {
                if array {
                    vec![0f64, element, (-1f64)]
                } else {
                    vec![1f64, (-1f64), (-1f64)]
                }
            } else {
                {
                    let tuple = ((self::read_collection_construction_kind(texts, starts.clone(), counts.clone(), labels.clone(), targets.clone(), element)) == "tuple");
                    {
                        let elements = self::read_collection_construction_field(starts.clone(), counts.clone(), labels, targets.clone(), element, String::from("elements"));
                        if (((!array || !tuple) || (elements < (0f64))) || ((self::read_required_number_fact(counts.clone(), elements)) != (2f64))) {
                            vec![2f64, (-1f64), (-1f64)]
                        } else {
                            vec![0f64, self::read_collection_construction_element(starts.clone(), counts.clone(), targets.clone(), elements, 0f64), self::read_collection_construction_element(starts, counts, targets, elements, 1f64)]
                        }
                    }
                }
            }
        }
    }
}

pub fn read_collection_construction_key_admission(kind: String, primitive: bool) -> f64 {
    if primitive {
        0f64
    } else {
        if (kind == "set") {
            3f64
        } else {
            4f64
        }
    }
}

pub fn read_collection_construction_diagnostic(code: f64, part: String) -> String {
    if ((part == "message") && (code == (1f64))) {
        String::from("Set constructor needs an iterable array")
    } else {
        if ((part == "message") && (code == (2f64))) {
            String::from("Map constructor needs an iterable of key/value pairs")
        } else {
            if ((part == "message") && (code == (3f64))) {
                String::from("set keys need explicit reference identities for structured values")
            } else {
                if ((part == "message") && (code == (4f64))) {
                    String::from("map keys need explicit reference identities for structured values")
                } else {
                    if ((part == "construct") && (code == (3f64))) {
                        String::from("Set object identity")
                    } else {
                        if ((part == "construct") && (code == (4f64))) {
                            String::from("Map object identity")
                        } else {
                            String::from("")
                        }
                    }
                }
            }
        }
    }
}

pub fn read_collection_construction_binding(target: String, kind: String, role: String) -> String {
    {
        let set = (kind == "set");
        if (target == "Rocq") {
            if (role == "previous") {
                if set {
                    String::from("ml_previous")
                } else {
                    String::from("(fst ml_previous)")
                }
            } else {
                if set {
                    String::from("ml_value")
                } else {
                    String::from("(fst ml_value)")
                }
            }
        } else {
            if (role == "previous") {
                if set {
                    String::from("ml_previous")
                } else {
                    String::from("ml_previous.1")
                }
            } else {
                if set {
                    String::from("ml_value")
                } else {
                    String::from("ml_value.1")
                }
            }
        }
    }
}

pub fn render_collection_construction_type(target: String, kind: String, first: String, second: String) -> String {
    if (target == "Lean") {
        if (kind == "set") {
            format!("{}{}", (format!("{}{}", (String::from("(Array ")), first)), (String::from(")")))
        } else {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(Array (")), first)), (String::from(" × ")))), second)), (String::from("))")))
        }
    } else {
        if (target == "Rocq") {
            if (kind == "set") {
                format!("{}{}", (format!("{}{}", (String::from("(list ")), first)), (String::from(")")))
            } else {
                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(list (")), first)), (String::from(" * ")))), second)), (String::from("))%type")))
            }
        } else {
            String::from("")
        }
    }
}

pub fn render_collection_construction(target: String, kind: String, key_kind: String, values: String, collection_type: String, element_type: String, comparison: String) -> String {
    {
        let set = (kind == "set");
        if (target == "JavaScript") {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("new ")), (if set {
                String::from("Set")
            } else {
                String::from("Map")
            }))), (String::from("(")))), values)), (String::from(")")))
        } else {
            if (target == "Rust") {
                {
                    let comparator = if (key_kind == "float") {
                        String::from("|a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())")
                    } else {
                        String::from("|a, b| a == b")
                    };
                    {
                        let values_3 = if (key_kind == "float") {
                            {
                                let values_2 = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), values)), (String::from(").into_iter().map(|ml_value| ")))), (if set {
                                    String::from("if ml_value == 0f64 { 0f64 } else { ml_value }")
                                } else {
                                    String::from("(if ml_value.0 == 0f64 { 0f64 } else { ml_value.0 }, ml_value.1)")
                                }))), (String::from(").collect::<Vec<_>>()")));
                                values_2.clone()
                            }
                        } else {
                            values.clone()
                        };
                        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_array::")), kind)), (String::from("(")))), values_3)), (String::from(", ")))), comparator)), (String::from(")")))
                    }
                }
            } else {
                {
                    let value = self::read_collection_construction_binding(target.clone(), kind.clone(), String::from("value"));
                    {
                        let previous = self::read_collection_construction_binding(target.clone(), kind.clone(), String::from("previous"));
                        if (target == "Lean") {
                            {
                                let equal = if (key_kind == "float") {
                                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), previous)), (String::from(" == ")))), value)), (String::from(" || (")))), previous)), (String::from(".isNaN && ")))), value)), (String::from(".isNaN))")))
                                } else {
                                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), previous)), (String::from(" == ")))), value)), (String::from(")")))
                                };
                                {
                                    let normalized = if (key_kind == "float") {
                                        if set {
                                            String::from("(if ml_original == (0 : Float) then (0 : Float) else ml_original)")
                                        } else {
                                            String::from("((if ml_original.1 == (0 : Float) then (0 : Float) else ml_original.1), ml_original.2)")
                                        }
                                    } else {
                                        String::from("ml_original")
                                    };
                                    {
                                        let existing = if set {
                                            String::from("ml_accumulator")
                                        } else {
                                            format!("{}{}", (format!("{}{}", (String::from("(ml_accumulator.map (fun ml_previous => if ")), equal)), (String::from(" then (ml_previous.1, ml_value.2) else ml_previous))")))
                                        };
                                        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), values)), (String::from(".foldl (fun ml_accumulator ml_original => let ml_value := ")))), normalized)), (String::from("; if ml_accumulator.any (fun ml_previous => ")))), equal)), (String::from(") then ")))), existing)), (String::from(" else ml_accumulator.push ml_value) (#[] : ")))), collection_type)), (String::from("))")))
                                    }
                                }
                            }
                        } else {
                            {
                                let equal_2 = comparison.clone();
                                {
                                    let equal_4 = if (key_kind == "float") {
                                        {
                                            let equal_3 = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(orb ")), equal_2)), (String::from(" (andb (PrimFloat.is_nan ")))), previous)), (String::from(") (PrimFloat.is_nan ")))), value)), (String::from(")))")));
                                            equal_3.clone()
                                        }
                                    } else {
                                        equal_2.clone()
                                    };
                                    {
                                        let normalized_2 = if (key_kind == "float") {
                                            if set {
                                                String::from("(if PrimFloat.eqb ml_original 0%float then 0%float else ml_original)")
                                            } else {
                                                String::from("((if PrimFloat.eqb (fst ml_original) 0%float then 0%float else fst ml_original), snd ml_original)")
                                            }
                                        } else {
                                            String::from("ml_original")
                                        };
                                        {
                                            let existing_2 = if set {
                                                String::from("ml_accumulator")
                                            } else {
                                                format!("{}{}", (format!("{}{}", (String::from("(List.map (fun ml_previous => if ")), equal_4)), (String::from(" then (fst ml_previous, snd ml_value) else ml_previous) ml_accumulator)")))
                                            };
                                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(List.fold_left (fun ml_accumulator ml_original => let ml_value := ")), normalized_2)), (String::from(" in if List.existsb (fun ml_previous => ")))), equal_4)), (String::from(") ml_accumulator then ")))), existing_2)), (String::from(" else List.app ml_accumulator (ml_value :: nil)) ")))), values)), (String::from(" (@nil ")))), element_type)), (String::from("))")))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_collection_copy_inference_role(target: String, source: String, settled: bool) -> String {
    if (((target == "map") || (target == "set")) && (source == target)) {
        String::from("copy")
    } else {
        if ((source == "var") && !settled) {
            String::from("wait")
        } else {
            String::from("legacy array")
        }
    }
}

pub fn read_collection_copy_source_normalization() -> String {
    String::from("arrayFrom")
}

pub fn read_collection_copy_snapshot_operation(kind: String) -> String {
    if (kind == "map") {
        String::from("entries")
    } else {
        if (kind == "set") {
            String::from("values")
        } else {
            String::from("")
        }
    }
}

pub fn accept_collection_copy_array_from_snapshot(kind: String, reference: bool) -> bool {
    (reference && ((kind == "map") || (kind == "set")))
}

pub fn accept_collection_copy_mapper_snapshot(kind: String, operation: String) -> bool {
    {
        let actual = self::read_collection_copy_snapshot_operation(kind.clone());
        ((actual != "") && (actual == operation))
    }
}

pub fn read_collection_copy_mapper_error(snapshot: bool, readonly: bool) -> String {
    if (snapshot && !readonly) {
        String::from("a mutable collection snapshot mapper needs its complete readonly callback graph; factories and live iterator mutations need their original ordered iterator model")
    } else {
        String::from("")
    }
}

pub fn plan_collection_copy_input(texts: Vec<String>, starts: Vec<f64>, counts: Vec<f64>, labels: Vec<String>, targets: Vec<f64>, kind: String, input: f64) -> Vec<f64> {
    if (((kind != "map") && (kind != "set")) || ((self::read_collection_construction_kind(texts, starts.clone(), counts.clone(), labels.clone(), targets.clone(), input)) != kind)) {
        vec![0f64, (-1f64), (-1f64)]
    } else {
        {
            let first = self::read_collection_construction_field(starts.clone(), counts.clone(), labels.clone(), targets.clone(), input, if (kind == "set") {
                String::from("element")
            } else {
                String::from("key")
            });
            {
                let second = if (kind == "map") {
                    self::read_collection_construction_field(starts, counts, labels, targets, input, String::from("value"))
                } else {
                    (-1f64)
                };
                if ((first < (0f64)) || ((kind == "map") && (second < (0f64)))) {
                    vec![0f64, (-1f64), (-1f64)]
                } else {
                    vec![if (kind == "set") {
                        1f64
                    } else {
                        2f64
                    }, first, second]
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadCollectionConstructionFieldLoop1Result {
    MlSource1TemporaryReadCollectionConstructionFieldLoop1Done,
    MlSource1TemporaryReadCollectionConstructionFieldLoop1Return(f64),
}

pub fn source_ml_source1_temporary_read_collection_construction_field_loop1(mut labels: Vec<String>, mut targets: Vec<f64>, mut field: String, mut start: f64, mut count: f64, mut at: f64) -> self::MlSource1TemporaryReadCollectionConstructionFieldLoop1Result {
    loop {
        return if (at < ((start + count))) {
            if ((self::read_required_text_fact(labels.clone(), at)) == field) {
                self::MlSource1TemporaryReadCollectionConstructionFieldLoop1Result::MlSource1TemporaryReadCollectionConstructionFieldLoop1Return(self::read_required_number_fact(targets, at))
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadCollectionConstructionFieldLoop1Result::MlSource1TemporaryReadCollectionConstructionFieldLoop1Done
        };
    }
}

fn ml_main() {
    assert!(((self::read_collection_copy_inference_role(String::from("map"), String::from("map"), false)) == "copy"), "assertion 1");
    assert!(((self::read_collection_copy_inference_role(String::from("set"), String::from("set"), false)) == "copy"), "assertion 2");
    assert!(((self::read_collection_copy_inference_role(String::from("map"), String::from("array"), false)) == "legacy array"), "assertion 3");
    assert!(((self::read_collection_copy_inference_role(String::from("map"), String::from("set"), false)) == "legacy array"), "assertion 4");
    assert!(((self::read_collection_copy_inference_role(String::from("set"), String::from("record"), false)) == "legacy array"), "assertion 5");
    assert!(((self::read_collection_copy_inference_role(String::from("map"), String::from("var"), false)) == "wait"), "assertion 6");
    assert!(((self::read_collection_copy_inference_role(String::from("map"), String::from("var"), true)) == "legacy array"), "assertion 7");
    assert!(((self::read_collection_copy_source_normalization()) == "arrayFrom"), "assertion 8");
    assert!(((self::read_collection_copy_snapshot_operation(String::from("map"))) == "entries"), "assertion 9");
    assert!(((self::read_collection_copy_snapshot_operation(String::from("set"))) == "values"), "assertion 10");
    assert!(((self::read_collection_copy_snapshot_operation(String::from("record"))) == ""), "assertion 11");
    assert!(((self::accept_collection_copy_array_from_snapshot(String::from("map"), true)) == (true)), "assertion 12");
    assert!(((self::accept_collection_copy_array_from_snapshot(String::from("set"), true)) == (true)), "assertion 13");
    assert!(((self::accept_collection_copy_array_from_snapshot(String::from("map"), false)) == (false)), "assertion 14");
    assert!(((self::accept_collection_copy_array_from_snapshot(String::from("array"), true)) == (false)), "assertion 15");
    assert!(((self::read_collection_copy_mapper_error(true, true)) == ""), "assertion 16");
    assert!(((self::read_collection_copy_mapper_error(false, false)) == ""), "assertion 17");
    assert!(((self::read_collection_copy_mapper_error(true, false)) == "a mutable collection snapshot mapper needs its complete readonly callback graph; factories and live iterator mutations need their original ordered iterator model"), "assertion 18");
    assert!(((self::accept_collection_copy_mapper_snapshot(String::from("map"), String::from("entries"))) == (true)), "assertion 19");
    assert!(((self::accept_collection_copy_mapper_snapshot(String::from("set"), String::from("values"))) == (true)), "assertion 20");
    assert!(((self::accept_collection_copy_mapper_snapshot(String::from("array"), String::from("values"))) == (false)), "assertion 21");
}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(move || { ml_main() })
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}

#[test]
fn all_literal_collection_copy_controls() { main(); }
