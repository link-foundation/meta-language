// Generated from the complete shared grammar-lexer-rules.js and recorded source closure.
// Do not edit by hand.
use super::*;

/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    #[must_use]
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if x < 0.0 && x >= -0.5 {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 { floor + 1.0 } else { floor }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where f64::signum does not.
    #[must_use]
    pub fn sign(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            x
        }
    }

    /// Math.max of two Numbers: NaN when either is, and 0 above -0.
    #[must_use]
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else if b > a {
            b
        } else if a == b {
            if a.is_sign_negative() { b } else { a }
        } else {
            f64::NAN
        }
    }

    /// Math.min of two Numbers: NaN when either is, and -0 below 0.
    #[must_use]
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else if b < a {
            b
        } else if a == b {
            if a.is_sign_negative() { a } else { b }
        } else {
            f64::NAN
        }
    }

    #[must_use]
    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    #[must_use]
    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    #[must_use]
    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    #[must_use]
    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9_007_199_254_740_991.0
    }
}

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    #[must_use]
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    #[must_use]
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        match index.and_then(|index| values.get(index)) {
            Some(value) => value.clone(),
            None => panic!("array index out of range"),
        }
    }

    /// The index a Number names: a non-negative integer, -0 included.
    #[must_use]
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9_007_199_254_740_992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    #[must_use]
    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 {
            (length as f64 + value).max(0.0) as usize
        } else {
            value.min(length as f64) as usize
        }
    }
    #[must_use]
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len());
        let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    #[must_use]
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> f64 {
        if values.is_empty() {
            return -1.0;
        }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) {
                return -1.0;
            }
            let start = if from < 0.0 {
                (values.len() as f64 + from) as usize
            } else {
                from.min((values.len() - 1) as f64) as usize
            };
            (0..=start)
                .rev()
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 {
                (values.len() as f64 + from).max(0.0) as usize
            } else {
                from.min(values.len() as f64) as usize
            };
            (start..values.len())
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        }
    }
    #[must_use]
    pub fn includes<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    #[must_use]
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values {
            if !out.iter().any(|previous| equal(previous, &value)) {
                out.push(value);
            }
        }
        out
    }
    #[must_use]
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) {
                out[index].1 = value;
            } else {
                out.push((key, value));
            }
        }
        out
    }
}

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
        pub fn from_units(units: Vec<u16>) -> Self {
            Self(units)
        }
        #[must_use]
        pub fn length(&self) -> f64 {
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
            let separator = match separator {
                Some(value) => value,
                None => return vec![self.clone()],
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
            if (0xd800..=0xdbff).contains(&first) {
                if let Some(&second) = self.0.get(index + 1) {
                    if (0xdc00..=0xdfff).contains(&second) {
                        return Some(
                            ((first as u32 - 0xd800) * 1024 + second as u32 - 0xdc00 + 0x10000)
                                as f64,
                        );
                    }
                }
            }
            Some(first as f64)
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
                        .map_or(false, |second| (0xdc00..=0xdfff).contains(second))
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
            if count < 0.0 || !count.is_finite() {
                panic!("Invalid count value: {}", count);
            }
            if self.0.is_empty() {
                return Self(Vec::new());
            }
            let size = count * self.0.len() as f64;
            if size > 536_870_888.0 {
                panic!("Invalid string length");
            }
            Self(self.0.repeat(count as usize))
        }
        #[must_use]
        pub fn pad(&self, length: f64, fill: &Self, start: bool) -> Self {
            let length = integer(length).max(0.0);
            if length <= self.0.len() as f64 || fill.0.is_empty() {
                return self.clone();
            }
            if length > 536_870_888.0 {
                panic!("Invalid string length");
            }
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
                char::from_u32(unit as u32).map_or(false, |c| {
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
    pub fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    #[must_use]
    pub fn absolute(value: f64, length: usize) -> usize {
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
pub fn refuse_required_number_fact() -> f64 {
    panic!("{}", "missing required Number fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> self::ml_string::Text {
    panic!("{}", "missing required text fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    panic!("{}", "missing required Boolean fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required Number fact column position")
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(|| self::refuse_required_number_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(
    values: &[self::ml_string::Text],
    index: f64,
) -> self::ml_string::Text {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required text fact column position")
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required Boolean fact column position")
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(|| self::refuse_required_boolean_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_lexer_entry_form(kind: self::ml_string::Text) -> f64 {
    if kind
        == (self::ml_string::Text::from_units(vec![
            108, 101, 120, 105, 99, 97, 108, 79, 119, 110, 101, 114,
        ]))
    {
        1f64
    } else if kind
        == (self::ml_string::Text::from_units(vec![
            108, 101, 120, 105, 99, 97, 108, 66, 105, 110, 100, 105, 110, 103,
        ]))
    {
        2f64
    } else if kind
        == (self::ml_string::Text::from_units(vec![
            108, 101, 120, 105, 99, 97, 108, 76, 105, 116, 101, 114, 97, 108,
        ]))
    {
        3f64
    } else if kind
        == (self::ml_string::Text::from_units(vec![
            108, 101, 120, 105, 99, 97, 108, 77, 111, 100, 101,
        ]))
    {
        4f64
    } else if kind
        == (self::ml_string::Text::from_units(vec![
            108, 101, 120, 105, 99, 97, 108, 67, 104, 97, 110, 110, 101, 108,
        ]))
    {
        5f64
    } else if kind
        == (self::ml_string::Text::from_units(vec![
            108, 101, 120, 105, 99, 97, 108, 79, 98, 108, 105, 103, 97, 116, 105, 111, 110,
        ]))
    {
        6f64
    } else if kind
        == (self::ml_string::Text::from_units(vec![
            108, 101, 120, 105, 99, 97, 108, 82, 117, 108, 101,
        ]))
    {
        7f64
    } else if kind
        == (self::ml_string::Text::from_units(vec![
            108, 101, 120, 105, 99, 97, 108, 70, 114, 97, 103, 109, 101, 110, 116,
        ]))
    {
        8f64
    } else if kind
        == (self::ml_string::Text::from_units(vec![
            108, 101, 120, 105, 99, 97, 108, 84, 121, 112, 101,
        ]))
    {
        9f64
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_lexer_identifier(text: self::ml_string::Text) -> bool {
    if (text.length()) == 0f64 {
        false
    } else {
        {
            let index = 0f64;
            {
                let ml_s1 = self::ml_accept_grammar_lexer_identifier_loop1(text.clone(), index);
                match ml_s1 {
                    self::MlAcceptGrammarLexerIdentifierLoop1Result::MlAcceptGrammarLexerIdentifierLoop1Done => {
                        true
                    }
                    self::MlAcceptGrammarLexerIdentifierLoop1Result::MlAcceptGrammarLexerIdentifierLoop1Return(ml_result1) => {
                        ml_result1
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_lexer_decimal(text: self::ml_string::Text) -> bool {
    if (text.length()) == 0f64 {
        false
    } else {
        {
            let index = 0f64;
            {
                let ml_s2 = self::ml_accept_grammar_lexer_decimal_loop2(text.clone(), index);
                match ml_s2 {
                    self::MlAcceptGrammarLexerDecimalLoop2Result::MlAcceptGrammarLexerDecimalLoop2Done => {
                        true
                    }
                    self::MlAcceptGrammarLexerDecimalLoop2Result::MlAcceptGrammarLexerDecimalLoop2Return(ml_result2) => {
                        ml_result2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn contains_grammar_lexer_text(
    values: &[self::ml_string::Text],
    value: self::ml_string::Text,
) -> bool {
    {
        let index = 0f64;
        {
            let ml_s3 = self::ml_contains_grammar_lexer_text_loop3(values, value.clone(), index);
            match ml_s3 {
                self::MlContainsGrammarLexerTextLoop3Result::MlContainsGrammarLexerTextLoop3Done => {
                    false
                }
                self::MlContainsGrammarLexerTextLoop3Result::MlContainsGrammarLexerTextLoop3Return(ml_result3) => {
                    ml_result3
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn contains_grammar_lexer_position(values: &[f64], value: f64) -> bool {
    {
        let index = 0f64;
        {
            let ml_s4 = self::ml_contains_grammar_lexer_position_loop4(values, value, index);
            match ml_s4 {
                self::MlContainsGrammarLexerPositionLoop4Result::MlContainsGrammarLexerPositionLoop4Done => {
                    false
                }
                self::MlContainsGrammarLexerPositionLoop4Result::MlContainsGrammarLexerPositionLoop4Return(ml_result4) => {
                    ml_result4
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_lexer_section_rule_positions(
    section: f64,
    section_kinds: &[self::ml_string::Text],
    section_names: &[self::ml_string::Text],
    token_vocabularies: &[self::ml_string::Text],
    rule_sections: &[f64],
    rule_kinds: &[self::ml_string::Text],
) -> Vec<f64> {
    {
        let selected = Vec::<f64>::new();
        {
            let index = 0f64;
            {
                let ml_s5 = self::ml_read_grammar_lexer_section_rule_positions_loop5(
                    section,
                    rule_sections,
                    rule_kinds,
                    selected,
                    index,
                );
                match ml_s5 {
                    self::MlReadGrammarLexerSectionRulePositionsLoop5Result::MlReadGrammarLexerSectionRulePositionsLoop5Done(selected_2, index_2) => {
                        {
                            let selected_3 = { let (ml_append_right, ml_append_left) = (vec![-1f64], selected_2); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let index_3 = 0f64;
                                {
                                    self::ml_read_grammar_lexer_section_rule_positions_loop6(section, section_kinds, section_names, token_vocabularies, rule_sections, rule_kinds, selected_3, index_3)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_lexer_literal_rule_position(
    value: self::ml_string::Text,
    section_kind: self::ml_string::Text,
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    literals: &[bool],
) -> f64 {
    {
        let selected = -1f64;
        {
            let count = 0f64;
            {
                let index = 0f64;
                {
                    let ml_s6 = self::ml_read_grammar_lexer_literal_rule_position_loop7(
                        value.clone(),
                        kinds,
                        values,
                        literals,
                        selected,
                        count,
                        index,
                    );
                    match ml_s6 {
                        self::MlReadGrammarLexerLiteralRulePositionLoop7Result::MlReadGrammarLexerLiteralRulePositionLoop7Done(selected_2, count_2) => {
                            if count_2 == 1f64 {
                                selected_2
                            } else if (count_2 > 1f64) || (section_kind == (self::ml_string::Text::from_units(vec![108, 101, 120, 101, 114]))) {
                                    -2f64
                                } else {
                                    -1f64
                                }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_lexer_text_position(
    values: &[self::ml_string::Text],
    value: self::ml_string::Text,
) -> f64 {
    {
        let index = 0f64;
        {
            let ml_s7 =
                self::ml_read_grammar_lexer_text_position_loop9(values, value.clone(), index);
            match ml_s7 {
                self::MlReadGrammarLexerTextPositionLoop9Result::MlReadGrammarLexerTextPositionLoop9Done => {
                    -1f64
                }
                self::MlReadGrammarLexerTextPositionLoop9Result::MlReadGrammarLexerTextPositionLoop9Return(ml_result9) => {
                    ml_result9
                }
            }
        }
    }
}
