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
pub fn read_owned_outcome_integer_decimal_limit(
    negative: bool,
    signed: bool,
    width: f64,
) -> self::ml_string::Text {
    if (width != 32f64) && (width != 64f64) {
        self::ml_string::Text::from_units(vec![])
    } else if signed {
        if negative {
            self::ml_string::Text::from_units(vec![
                57, 50, 50, 51, 51, 55, 50, 48, 51, 54, 56, 53, 52, 55, 55, 53, 56, 48, 56,
            ])
        } else {
            self::ml_string::Text::from_units(vec![
                57, 50, 50, 51, 51, 55, 50, 48, 51, 54, 56, 53, 52, 55, 55, 53, 56, 48, 55,
            ])
        }
    } else if negative {
        self::ml_string::Text::from_units(vec![])
    } else if width == 32f64 {
        self::ml_string::Text::from_units(vec![52, 50, 57, 52, 57, 54, 55, 50, 57, 53])
    } else {
        self::ml_string::Text::from_units(vec![
            49, 56, 52, 52, 54, 55, 52, 52, 48, 55, 51, 55, 48, 57, 53, 53, 49, 54, 49, 53,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_owned_outcome_integer_decimal(
    text: &self::ml_string::Text,
    signed: bool,
    width: f64,
) -> bool {
    {
        let count = text.length();
        if (count == 0f64) || (count > 20f64) {
            false
        } else {
            {
                let negative = ((std::rc::Rc::new(
                    move |text_2: self::ml_string::Text, position: f64| -> self::ml_string::Text {
                        {
                            let index = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if (index < 0f64) || (index >= (text_2.length())) {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                text_2.slice(index, (index + 1f64))
                            }
                        }
                    },
                ))(text.clone(), 0f64))
                    == (self::ml_string::Text::from_units(vec![45]));
                {
                    let offset = if negative { 1f64 } else { 0f64 };
                    {
                        let limit =
                            self::read_owned_outcome_integer_decimal_limit(negative, signed, width);
                        {
                            let digits = count - offset;
                            if (((limit.length()) == 0f64) || (digits == 0f64))
                                || (digits > (limit.length()))
                            {
                                false
                            } else if (((std::rc::Rc::new(
                                move |text_2: self::ml_string::Text,
                                      position: f64|
                                      -> self::ml_string::Text {
                                    {
                                        let index = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if (index < 0f64) || (index >= (text_2.length())) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            text_2.slice(index, (index + 1f64))
                                        }
                                    }
                                },
                            ))(text.clone(), offset))
                                == (self::ml_string::Text::from_units(vec![48])))
                                && (negative || (digits > 1f64))
                            {
                                false
                            } else {
                                {
                                    let smaller = digits < (limit.length());
                                    {
                                        let larger = false;
                                        {
                                            let index = offset;
                                            {
                                                let ml_s1 = self::ml_accept_owned_outcome_integer_decimal_loop1(text, count, offset, &(limit), smaller, larger, index);
                                                match ml_s1 {
                                                        self::MlAcceptOwnedOutcomeIntegerDecimalLoop1Result::MlAcceptOwnedOutcomeIntegerDecimalLoop1Done(larger_2) => {
                                                            !larger_2
                                                        }
                                                        self::MlAcceptOwnedOutcomeIntegerDecimalLoop1Result::MlAcceptOwnedOutcomeIntegerDecimalLoop1Return(ml_result1) => {
                                                            ml_result1
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
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_owned_outcome_integer_projection(
    text: &self::ml_string::Text,
    signed: bool,
    width: f64,
    projected: f64,
) -> bool {
    if (!(self::accept_owned_outcome_integer_decimal(text, signed, width))
        || !(projected.is_finite()))
        || ((projected == 0f64) && ((1f64 / projected) < 0f64))
    {
        false
    } else {
        {
            let negative = ((std::rc::Rc::new(
                move |text_2: self::ml_string::Text, position: f64| -> self::ml_string::Text {
                    {
                        let index = if position.is_nan() {
                            0f64
                        } else {
                            position.trunc()
                        };
                        if (index < 0f64) || (index >= (text_2.length())) {
                            self::ml_string::Text::from_units(vec![])
                        } else {
                            text_2.slice(index, (index + 1f64))
                        }
                    }
                },
            ))(text.clone(), 0f64))
                == (self::ml_string::Text::from_units(vec![45]));
            {
                let digits = text.slice(if negative { 1f64 } else { 0f64 }, f64::INFINITY);
                {
                    let power = 1f64;
                    {
                        let value = 0f64;
                        {
                            let position = 0f64;
                            {
                                let first = -1f64;
                                {
                                    let last = -1f64;
                                    {
                                        let ml_s2 =
                                            self::ml_accept_owned_outcome_integer_projection_loop3(
                                                digits, power, value, position, first, last,
                                            );
                                        match ml_s2 {
                                                self::MlAcceptOwnedOutcomeIntegerProjectionLoop3Result::MlAcceptOwnedOutcomeIntegerProjectionLoop3Done(value_2, first_2, last_2) => {
                                                    if (last_2 - first_2) >= 53f64 {
                                                        false
                                                    } else {
                                                        projected == (if negative {
                                                            -value_2
                                                        } else {
                                                            value_2
                                                        })
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
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptOwnedOutcomeIntegerDecimalLoop1Result {
    MlAcceptOwnedOutcomeIntegerDecimalLoop1Done(bool),
    MlAcceptOwnedOutcomeIntegerDecimalLoop1Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptOwnedOutcomeIntegerDecimalJoin2 {
    MlAcceptOwnedOutcomeIntegerDecimalJoin2Next(bool, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_owned_outcome_integer_decimal_loop1(
    text: &self::ml_string::Text,
    mut count: f64,
    mut offset: f64,
    limit: &self::ml_string::Text,
    mut smaller: bool,
    mut larger: bool,
    mut index: f64,
) -> self::MlAcceptOwnedOutcomeIntegerDecimalLoop1Result {
    loop {
        return if index < count {
            {
                let digit = (self::ml_string::Text::from_units(vec![
                    48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                ]))
                .index_of(
                    &(std::rc::Rc::new(
                        move |text_2: self::ml_string::Text,
                              position: f64|
                              -> self::ml_string::Text {
                            {
                                let index_2 = if position.is_nan() {
                                    0f64
                                } else {
                                    position.trunc()
                                };
                                if (index_2 < 0f64) || (index_2 >= (text_2.length())) {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    text_2.slice(index_2, (index_2 + 1f64))
                                }
                            }
                        },
                    ))(text.clone(), index),
                    0f64,
                );
                if digit < 0f64 {
                    self::MlAcceptOwnedOutcomeIntegerDecimalLoop1Result::MlAcceptOwnedOutcomeIntegerDecimalLoop1Return(false)
                } else {
                    {
                        let ml_s3 = if ((smaller) || (larger)) {
                            self::MlAcceptOwnedOutcomeIntegerDecimalJoin2::MlAcceptOwnedOutcomeIntegerDecimalJoin2Next(smaller, larger)
                        } else {
                            {
                                let boundary = (self::ml_string::Text::from_units(vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57])).index_of(&(std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                    let index_2 = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index_2 < 0f64) || (index_2 >= (text_2.length())) {
                                        self::ml_string::Text::from_units(vec![])
                                    } else {
                                        text_2.slice(index_2, (index_2 + 1f64))
                                    }
                                } }))(limit.clone(), (index - offset)), 0f64);
                                {
                                    let smaller_3 =
                                        if digit < boundary { { true } } else { smaller };
                                    if digit > boundary {
                                        {
                                            let larger_2 = true;
                                            self::MlAcceptOwnedOutcomeIntegerDecimalJoin2::MlAcceptOwnedOutcomeIntegerDecimalJoin2Next(smaller_3, larger_2)
                                        }
                                    } else {
                                        self::MlAcceptOwnedOutcomeIntegerDecimalJoin2::MlAcceptOwnedOutcomeIntegerDecimalJoin2Next(smaller_3, larger)
                                    }
                                }
                            }
                        };
                        match ml_s3 {
                            self::MlAcceptOwnedOutcomeIntegerDecimalJoin2::MlAcceptOwnedOutcomeIntegerDecimalJoin2Next(smaller_4, larger_3) => {
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        (smaller, larger, index) = (smaller_4, larger_3, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptOwnedOutcomeIntegerDecimalLoop1Result::MlAcceptOwnedOutcomeIntegerDecimalLoop1Done(larger)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptOwnedOutcomeIntegerProjectionLoop3Result {
    MlAcceptOwnedOutcomeIntegerProjectionLoop3Done(f64, f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptOwnedOutcomeIntegerProjectionJoin5 {
    MlAcceptOwnedOutcomeIntegerProjectionJoin5Next(f64, f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptOwnedOutcomeIntegerProjectionLoop4Result {
    MlAcceptOwnedOutcomeIntegerProjectionLoop4Done(self::ml_string::Text, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_owned_outcome_integer_projection_loop4(
    digits: &self::ml_string::Text,
    mut quotient: self::ml_string::Text,
    mut carry: f64,
    mut index: f64,
) -> self::MlAcceptOwnedOutcomeIntegerProjectionLoop4Result {
    loop {
        if index < (digits.length()) {
            {
                let digit = (self::ml_string::Text::from_units(vec![
                    48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                ]))
                .index_of(
                    &(std::rc::Rc::new(
                        move |text: self::ml_string::Text,
                              position: f64|
                              -> self::ml_string::Text {
                            {
                                let index_2 = if position.is_nan() {
                                    0f64
                                } else {
                                    position.trunc()
                                };
                                if (index_2 < 0f64) || (index_2 >= (text.length())) {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    text.slice(index_2, (index_2 + 1f64))
                                }
                            }
                        },
                    ))(digits.clone(), index),
                    0f64,
                );
                {
                    let dividend = (carry * 10f64) + digit;
                    {
                        let next = (dividend / 2f64).trunc();
                        {
                            let carry_2 = dividend - (next * 2f64);
                            {
                                let quotient_3 = if ((quotient.length()) > 0f64) || (next > 0f64) {
                                    {
                                        quotient.concat(&(std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                            let index_2 = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if (index_2 < 0f64) || (index_2 >= (text.length())) {
                                                self::ml_string::Text::from_units(vec![])
                                            } else {
                                                text.slice(index_2, (index_2 + 1f64))
                                            }
                                        } }))(self::ml_string::Text::from_units(vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57]), next))
                                    }
                                } else {
                                    quotient.clone()
                                };
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        (quotient, carry, index) = (quotient_3, carry_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlAcceptOwnedOutcomeIntegerProjectionLoop4Result::MlAcceptOwnedOutcomeIntegerProjectionLoop4Done(quotient.clone(), carry)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_owned_outcome_integer_projection_loop3(
    mut digits: self::ml_string::Text,
    mut power: f64,
    mut value: f64,
    mut position: f64,
    mut first: f64,
    mut last: f64,
) -> self::MlAcceptOwnedOutcomeIntegerProjectionLoop3Result {
    loop {
        return {
            let ml_library_inequality_1 = digits != (self::ml_string::Text::from_units(vec![48]));
            if ml_library_inequality_1 {
                {
                    let quotient = self::ml_string::Text::from_units(vec![]);
                    {
                        let carry = 0f64;
                        {
                            let index = 0f64;
                            {
                                let ml_s4 = self::ml_accept_owned_outcome_integer_projection_loop4(
                                    &(digits),
                                    quotient,
                                    carry,
                                    index,
                                );
                                match ml_s4 {
                                self::MlAcceptOwnedOutcomeIntegerProjectionLoop4Result::MlAcceptOwnedOutcomeIntegerProjectionLoop4Done(quotient_2, carry_2) => {
                                    {
                                        let ml_s5 = if carry_2 == 1f64 {
                                            {
                                                let first_3 = if first < 0f64 {
                                                    {
                                                        position
                                                    }
                                                } else {
                                                    first
                                                };
                                                {
                                                    let last_2 = position;
                                                    {
                                                        let value_2 = value + power;
                                                        self::MlAcceptOwnedOutcomeIntegerProjectionJoin5::MlAcceptOwnedOutcomeIntegerProjectionJoin5Next(value_2, first_3, last_2)
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlAcceptOwnedOutcomeIntegerProjectionJoin5::MlAcceptOwnedOutcomeIntegerProjectionJoin5Next(value, first, last)
                                        };
                                        match ml_s5 {
                                            self::MlAcceptOwnedOutcomeIntegerProjectionJoin5::MlAcceptOwnedOutcomeIntegerProjectionJoin5Next(value_3, first_4, last_3) => {
                                                {
                                                    let digits_2 = if (quotient_2.length()) == 0f64 {
                                                        self::ml_string::Text::from_units(vec![48])
                                                    } else {
                                                        quotient_2
                                                    };
                                                    {
                                                        let position_2 = position + 1f64;
                                                        {
                                                            let power_2 = power * 2f64;
                                                            {
                                                                (digits, power, value, position, first, last) = (digits_2, power_2, value_3, position_2, first_4, last_3);
                                                                continue;
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
                    }
                }
            } else {
                self::MlAcceptOwnedOutcomeIntegerProjectionLoop3Result::MlAcceptOwnedOutcomeIntegerProjectionLoop3Done(value, first, last)
            }
        };
    }
}
