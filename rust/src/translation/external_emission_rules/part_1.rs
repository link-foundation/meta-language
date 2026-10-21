// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// JavaScript strings retain every UTF-16 code unit, including isolated surrogates.
pub mod ml_string {
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Text(pub Vec<u16>);

    impl std::fmt::Display for Text {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.0))
        }
    }

    impl Text {
        #[must_use]
        pub fn from_text(text: &str) -> Self {
            Self(text.encode_utf16().collect())
        }
        #[must_use]
        pub const fn from_units(units: Vec<u16>) -> Self {
            Self(units)
        }
        #[must_use]
        pub const fn length(&self) -> f64 {
            self.0.len() as f64
        }
        #[must_use]
        pub fn concat(&self, other: &Self) -> Self {
            let mut units = self.0.clone();
            units.extend_from_slice(&other.0);
            Self(units)
        }
        #[must_use]
        pub fn starts_with(&self, search: &Self) -> bool {
            self.0.starts_with(&search.0)
        }
        #[must_use]
        pub fn ends_with(&self, search: &Self) -> bool {
            self.0.ends_with(&search.0)
        }
        #[must_use]
        pub fn contains(&self, search: &Self) -> bool {
            self.index_of(search, 0.0) >= 0.0
        }
        #[must_use]
        pub fn slice(&self, start: f64, end: f64) -> Self {
            let first = relative(start, self.0.len());
            let last = relative(end, self.0.len());
            Self(self.0[first..last.max(first)].to_vec())
        }
        #[must_use]
        pub fn index_of(&self, search: &Self, position: f64) -> f64 {
            let start = absolute(position, self.0.len());
            if search.0.is_empty() {
                return start as f64;
            }
            if search.0.len() > self.0.len() {
                return -1.0;
            }
            (start..=self.0.len() - search.0.len())
                .find(|&index| self.0[index..].starts_with(&search.0))
                .map_or(-1.0, |index| index as f64)
        }
        #[must_use]
        pub fn last_index_of(&self, search: &Self, position: f64) -> f64 {
            let position = if position.is_nan() {
                f64::INFINITY
            } else {
                position
            };
            let start = absolute(position, self.0.len());
            if search.0.is_empty() {
                return start as f64;
            }
            if search.0.len() > self.0.len() {
                return -1.0;
            }
            (0..=start.min(self.0.len() - search.0.len()))
                .rev()
                .find(|&index| self.0[index..].starts_with(&search.0))
                .map_or(-1.0, |index| index as f64)
        }
        #[must_use]
        pub fn split(&self, separator: Option<&Self>, limit: f64) -> Vec<Self> {
            let maximum = uint32(limit) as usize;
            if maximum == 0 {
                return Vec::new();
            }
            let Some(separator) = separator else {
                return vec![self.clone()];
            };
            if separator.0.is_empty() {
                return self
                    .0
                    .iter()
                    .take(maximum)
                    .map(|&unit| Self(vec![unit]))
                    .collect();
            }
            let mut out = Vec::new();
            let mut start = 0;
            loop {
                let index = self.index_of(separator, start as f64);
                if index < 0.0 {
                    break;
                }
                out.push(Self(self.0[start..index as usize].to_vec()));
                if out.len() == maximum {
                    return out;
                }
                start = index as usize + separator.0.len();
            }
            out.push(Self(self.0[start..].to_vec()));
            out
        }
        #[must_use]
        pub fn code_point_at(&self, position: f64) -> Option<f64> {
            let index = integer(position);
            if index < 0.0 || index >= self.0.len() as f64 {
                return None;
            }
            let index = index as usize;
            let first = self.0[index];
            if (0xd800..=0xdbff).contains(&first)
                && let Some(&second) = self.0.get(index + 1)
                && (0xdc00..=0xdfff).contains(&second)
            {
                return Some(f64::from(
                    (u32::from(first) - 0xd800) * 1024 + u32::from(second) - 0xdc00 + 0x10000,
                ));
            }
            Some(f64::from(first))
        }
        #[must_use]
        pub fn code_points(&self) -> Vec<Self> {
            let mut out = Vec::new();
            let mut index = 0;
            while index < self.0.len() {
                let first = self.0[index];
                let width = if (0xd800..=0xdbff).contains(&first)
                    && self
                        .0
                        .get(index + 1)
                        .is_some_and(|second| (0xdc00..=0xdfff).contains(second))
                {
                    2
                } else {
                    1
                };
                out.push(Self(self.0[index..index + width].to_vec()));
                index += width;
            }
            out
        }
        #[must_use]
        pub fn repeat(&self, count: f64) -> Self {
            let count = integer(count);
            assert!(
                count >= 0.0 && count.is_finite(),
                "Invalid count value: {count}"
            );
            if self.0.is_empty() {
                return Self(Vec::new());
            }
            let size = count * self.0.len() as f64;
            assert!(size <= 536_870_888.0, "Invalid string length");
            Self(self.0.repeat(count as usize))
        }
        #[must_use]
        pub fn pad(&self, length: f64, fill: &Self, start: bool) -> Self {
            let length = integer(length).max(0.0);
            if length <= self.0.len() as f64 || fill.0.is_empty() {
                return self.clone();
            }
            assert!(length <= 536_870_888.0, "Invalid string length");
            let count = length as usize - self.0.len();
            let padding: Vec<u16> = fill.0.iter().copied().cycle().take(count).collect();
            let pad = Self(padding);
            if start {
                pad.concat(self)
            } else {
                self.concat(&pad)
            }
        }
        #[must_use]
        pub fn replace(&self, search: &Self, replacement: &Self, all: bool) -> Self {
            let mut out = Vec::new();
            let mut start = 0;
            let mut from = 0;
            loop {
                let index = self.index_of(search, from as f64);
                if index < 0.0 {
                    break;
                }
                let index = index as usize;
                out.extend_from_slice(&self.0[start..index]);
                substitute(&mut out, &self.0, &search.0, &replacement.0, index);
                start = index + search.0.len();
                if !all || (search.0.is_empty() && index == self.0.len()) {
                    break;
                }
                from = start + usize::from(search.0.is_empty());
            }
            out.extend_from_slice(&self.0[start..]);
            Self(out)
        }
        #[must_use]
        pub fn trim(&self, start: bool, end: bool, rust: bool) -> Self {
            let whitespace = |unit: u16| {
                char::from_u32(u32::from(unit)).is_some_and(|c| {
                    if rust {
                        c.is_whitespace()
                    } else {
                        (c.is_whitespace() && c != '\u{85}') || c == '\u{feff}'
                    }
                })
            };
            let mut first = 0;
            let mut last = self.0.len();
            if start {
                while first < last && whitespace(self.0[first]) {
                    first += 1;
                }
            }
            if end {
                while last > first && whitespace(self.0[last - 1]) {
                    last -= 1;
                }
            }
            Self(self.0[first..last].to_vec())
        }
        #[must_use]
        pub fn map_case(&self, upper: bool) -> Self {
            let mut out = Vec::new();
            let mut run = String::new();
            let flush = |run: &mut String, out: &mut Vec<u16>| {
                let mapped = if upper {
                    run.to_uppercase()
                } else {
                    run.to_lowercase()
                };
                out.extend(mapped.encode_utf16());
                run.clear();
            };
            for scalar in char::decode_utf16(self.0.iter().copied()) {
                match scalar {
                    Ok(c) => run.push(c),
                    Err(error) => {
                        flush(&mut run, &mut out);
                        out.push(error.unpaired_surrogate());
                    }
                }
            }
            flush(&mut run, &mut out);
            Self(out)
        }
    }
    #[must_use]
    pub fn join(values: &[Text], separator: &Text) -> Text {
        let mut units = Vec::new();
        for (index, value) in values.iter().enumerate() {
            if index != 0 {
                units.extend_from_slice(&separator.0);
            }
            units.extend_from_slice(&value.0);
        }
        Text(units)
    }
    #[must_use]
    pub const fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    #[must_use]
    pub const fn absolute(value: f64, length: usize) -> usize {
        integer(value).max(0.0).min(length as f64) as usize
    }
    #[must_use]
    pub fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 {
            (length as f64 + value).max(0.0) as usize
        } else {
            value.min(length as f64) as usize
        }
    }
    fn uint32(value: f64) -> u32 {
        if !value.is_finite() || value == 0.0 {
            0
        } else {
            value.trunc().rem_euclid(4_294_967_296.0) as u32
        }
    }
    fn substitute(
        out: &mut Vec<u16>,
        text: &[u16],
        search: &[u16],
        replacement: &[u16],
        position: usize,
    ) {
        let mut index = 0;
        while index < replacement.len() {
            if replacement[index] == 36 && index + 1 < replacement.len() {
                match replacement[index + 1] {
                    36 => {
                        out.push(36);
                        index += 2;
                        continue;
                    }
                    38 => {
                        out.extend_from_slice(search);
                        index += 2;
                        continue;
                    }
                    96 => {
                        out.extend_from_slice(&text[..position]);
                        index += 2;
                        continue;
                    }
                    39 => {
                        out.extend_from_slice(&text[position + search.len()..]);
                        index += 2;
                        continue;
                    }
                    _ => {}
                }
            }
            out.push(replacement[index]);
            index += 1;
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn external_identifier_head(unit: f64) -> bool {
    ((((65f64..=90f64).contains(&unit)) || ((97f64..=122f64).contains(&unit))) || (unit == 95f64))
        || (unit == 36f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn external_reserved_statement(name: &self::ml_string::Text) -> bool {
    ((((((((*name) == (self::ml_string::Text::from_units(vec![98, 114, 101, 97, 107])))
        || ((*name) == (self::ml_string::Text::from_units(vec![99, 97, 115, 101]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![99, 97, 116, 99, 104]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![99, 111, 110, 116, 105, 110, 117, 101]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![100, 101, 98, 117, 103, 103, 101, 114]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn external_reserved_control(name: &self::ml_string::Text) -> bool {
    ((((((((*name) == (self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101])))
        || ((*name) == (self::ml_string::Text::from_units(vec![100, 111]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![101, 108, 115, 101]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![101, 110, 117, 109]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![101, 120, 112, 111, 114, 116]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![101, 120, 116, 101, 110, 100, 115]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn external_reserved_declaration(name: &self::ml_string::Text) -> bool {
    ((((((((*name) == (self::ml_string::Text::from_units(vec![102, 111, 114])))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![
                102, 117, 110, 99, 116, 105, 111, 110,
            ]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![105, 102]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![105, 109, 112, 111, 114, 116]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![105, 110]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![
                105, 110, 115, 116, 97, 110, 99, 101, 111, 102,
            ]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![110, 101, 119]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn external_reserved_expression(name: &self::ml_string::Text) -> bool {
    ((((((((*name) == (self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110])))
        || ((*name) == (self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![115, 119, 105, 116, 99, 104]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![116, 114, 117, 101]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![116, 114, 121]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![116, 121, 112, 101, 111, 102])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn external_reserved_binding(name: &self::ml_string::Text) -> bool {
    ((((((((*name) == (self::ml_string::Text::from_units(vec![118, 97, 114])))
        || ((*name) == (self::ml_string::Text::from_units(vec![118, 111, 105, 100]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![119, 104, 105, 108, 101]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![119, 105, 116, 104]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![121, 105, 101, 108, 100]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![97, 119, 97, 105, 116]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![108, 101, 116]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![115, 116, 97, 116, 105, 99])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn external_reserved_strict_binding(name: &self::ml_string::Text) -> bool {
    ((((((((*name)
        == (self::ml_string::Text::from_units(vec![
            105, 109, 112, 108, 101, 109, 101, 110, 116, 115,
        ])))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![
                105, 110, 116, 101, 114, 102, 97, 99, 101,
            ]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![112, 97, 99, 107, 97, 103, 101]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![112, 114, 105, 118, 97, 116, 101]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![
                112, 114, 111, 116, 101, 99, 116, 101, 100,
            ]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![112, 117, 98, 108, 105, 99]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![101, 118, 97, 108]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![
                97, 114, 103, 117, 109, 101, 110, 116, 115,
            ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_external_binding(name: &self::ml_string::Text) -> bool {
    if ((name.length()) == 0f64)
        || !(self::external_identifier_head((std::rc::Rc::new(
            move |text: self::ml_string::Text, position: f64| -> f64 {
                {
                    let index = if position.is_nan() {
                        0f64
                    } else {
                        position.trunc()
                    };
                    if (index < 0f64) || (index >= (text.length())) {
                        f64::NAN
                    } else {
                        ((text.slice(index, (index + 1f64))).code_point_at(0f64))
                            .clone()
                            .unwrap_or_else(|| f64::NAN)
                    }
                }
            },
        ))(name.clone(), 0f64)))
    {
        false
    } else {
        {
            let at = 1f64;
            {
                let ml_s1 = self::ml_accept_java_script_external_binding_loop1(name, at);
                match ml_s1 {
                    self::MlAcceptJavaScriptExternalBindingLoop1Result::MlAcceptJavaScriptExternalBindingLoop1Done => {
                        ((((!(self::external_reserved_statement(name)) && !(self::external_reserved_control(name))) && !(self::external_reserved_declaration(name))) && !(self::external_reserved_expression(name))) && !(self::external_reserved_binding(name))) && !(self::external_reserved_strict_binding(name))
                    }
                    self::MlAcceptJavaScriptExternalBindingLoop1Result::MlAcceptJavaScriptExternalBindingLoop1Return(ml_result1) => {
                        ml_result1
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_external_call(
    kind: &self::ml_string::Text,
    name: &self::ml_string::Text,
    arguments_text: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (self::accept_java_script_external_binding(name)) {
        if (*kind) == (self::ml_string::Text::from_units(vec![102, 110])) {
            ((name.concat(&self::ml_string::Text::from_units(vec![40]))).concat(arguments_text))
                .concat(&self::ml_string::Text::from_units(vec![41]))
        } else if ((*kind) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116])))
            && ((*arguments_text) == (self::ml_string::Text::from_units(vec![])))
        {
            name.clone()
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptJavaScriptExternalBindingLoop1Result {
    MlAcceptJavaScriptExternalBindingLoop1Done,
    MlAcceptJavaScriptExternalBindingLoop1Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_java_script_external_binding_loop1(
    name: &self::ml_string::Text,
    mut at: f64,
) -> self::MlAcceptJavaScriptExternalBindingLoop1Result {
    loop {
        return if at < (name.length()) {
            {
                let unit =
                    (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 {
                        {
                            let index = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if (index < 0f64) || (index >= (text.length())) {
                                f64::NAN
                            } else {
                                ((text.slice(index, (index + 1f64))).code_point_at(0f64))
                                    .clone()
                                    .unwrap_or_else(|| f64::NAN)
                            }
                        }
                    }))(name.clone(), at);
                if ((self::external_identifier_head(unit)) || ((48f64..=57f64).contains(&unit))) {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
                {
                    self::MlAcceptJavaScriptExternalBindingLoop1Result::MlAcceptJavaScriptExternalBindingLoop1Return(false)
                }
            }
        } else {
            self::MlAcceptJavaScriptExternalBindingLoop1Result::MlAcceptJavaScriptExternalBindingLoop1Done
        };
    }
}
