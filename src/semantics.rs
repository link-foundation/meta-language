/// Many-valued semantic truth value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruthValue {
    True,
    False,
    Unknown,
    Both,
}

impl TruthValue {
    /// Logical conjunction.
    #[must_use]
    pub const fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::False, _) | (_, Self::False) => Self::False,
            (Self::True, value) | (value, Self::True) => value,
            (Self::Both, _) | (_, Self::Both) => Self::Both,
            (Self::Unknown, Self::Unknown) => Self::Unknown,
        }
    }

    /// Logical disjunction.
    #[must_use]
    pub const fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::True, _) | (_, Self::True) => Self::True,
            (Self::False, value) | (value, Self::False) => value,
            (Self::Both, _) | (_, Self::Both) => Self::Both,
            (Self::Unknown, Self::Unknown) => Self::Unknown,
        }
    }

    /// Logical negation.
    #[must_use]
    pub const fn negate(self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Unknown => Self::Unknown,
            Self::Both => Self::Both,
        }
    }
}

const PROBABILITY_SCALE_BASIS_POINTS: u16 = 10_000;
const PROBABILITY_SCALE_U32: u32 = 10_000;
const PROBABILITY_SCALE_U128: u128 = 10_000;

/// Fixed-point probability stored as basis points from `0` to `10_000`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Probability {
    basis_points: u16,
}

impl Probability {
    /// Probability `0.0`.
    pub const ZERO: Self = Self { basis_points: 0 };

    /// Probability `1.0`.
    pub const ONE: Self = Self {
        basis_points: PROBABILITY_SCALE_BASIS_POINTS,
    };

    /// Creates a probability from basis points, where `10_000` means certainty.
    #[must_use]
    pub const fn from_basis_points(basis_points: u16) -> Option<Self> {
        if basis_points <= PROBABILITY_SCALE_BASIS_POINTS {
            Some(Self { basis_points })
        } else {
            None
        }
    }

    /// Creates a probability from a ratio, rounded to the nearest basis point.
    #[must_use]
    pub fn from_ratio(numerator: u64, denominator: u64) -> Option<Self> {
        if denominator == 0 || numerator > denominator {
            return None;
        }

        let scaled = (u128::from(numerator) * PROBABILITY_SCALE_U128 + u128::from(denominator) / 2)
            / u128::from(denominator);
        let basis_points =
            u16::try_from(scaled).expect("scaled probability must fit into basis points");

        Self::from_basis_points(basis_points)
    }

    /// Returns the fixed-point probability in basis points.
    #[must_use]
    pub const fn basis_points(self) -> u16 {
        self.basis_points
    }

    /// Returns `1 - self`.
    #[must_use]
    pub const fn complement(self) -> Self {
        Self {
            basis_points: PROBABILITY_SCALE_BASIS_POINTS - self.basis_points,
        }
    }
}

/// Probabilistic truth value for relative-meta-logic-style confidence links.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProbabilisticTruthValue {
    true_probability: Probability,
}

impl ProbabilisticTruthValue {
    /// Creates a probabilistic truth value from the probability of truth.
    #[must_use]
    pub const fn new(true_probability: Probability) -> Self {
        Self { true_probability }
    }

    /// Creates a probabilistic truth value from a ratio.
    #[must_use]
    pub fn from_ratio(numerator: u64, denominator: u64) -> Option<Self> {
        Some(Self::new(Probability::from_ratio(numerator, denominator)?))
    }

    /// Probability that the proposition is true.
    #[must_use]
    pub const fn true_probability(self) -> Probability {
        self.true_probability
    }

    /// Probability that the proposition is false.
    #[must_use]
    pub const fn false_probability(self) -> Probability {
        self.true_probability.complement()
    }

    /// Logical negation, represented as probability complement.
    #[must_use]
    pub const fn negate(self) -> Self {
        Self::new(self.false_probability())
    }

    /// Independent probabilistic conjunction.
    #[must_use]
    pub fn and(self, other: Self) -> Self {
        Self::new(multiply_probabilities(
            self.true_probability(),
            other.true_probability(),
        ))
    }

    /// Independent probabilistic disjunction.
    #[must_use]
    pub fn or(self, other: Self) -> Self {
        self.negate().and(other.negate()).negate()
    }
}

fn multiply_probabilities(left: Probability, right: Probability) -> Probability {
    let product = u32::from(left.basis_points()) * u32::from(right.basis_points());
    let rounded = (product + PROBABILITY_SCALE_U32 / 2) / PROBABILITY_SCALE_U32;
    let basis_points =
        u16::try_from(rounded).expect("probability product must fit into basis points");

    Probability::from_basis_points(basis_points).expect("probability product stays in range")
}
