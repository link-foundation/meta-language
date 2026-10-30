// Generated from complete js/src/translation/rust-owned-binding-rules.js. Do not edit by hand.
use super::MlTemporaryReadRustOwnedLoanReadErrorLoop7Result;
use super::ml;
use super::ml_array;
use super::read_rust_owned_binding_read_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_owned_loan_read_error_loop7(
    source_ml_temporary_values6: &[String],
    mut source_ml_temporary_index6: self::ml::Big,
) -> self::MlTemporaryReadRustOwnedLoanReadErrorLoop7Result {
    loop {
        return if source_ml_temporary_index6
            < (self::ml::Big::from_u128(source_ml_temporary_values6.len() as u128))
        {
            {
                let state = self::ml_array::at(
                    source_ml_temporary_values6,
                    source_ml_temporary_index6.to_index(),
                );
                {
                    let ml_library_inequality_1 = state != "available";
                    if ml_library_inequality_1 {
                        self::MlTemporaryReadRustOwnedLoanReadErrorLoop7Result::MlTemporaryReadRustOwnedLoanReadErrorLoop7Return(self::read_rust_owned_binding_read_error(""))
                    } else {
                        {
                            let source_ml_temporary_index6_2 =
                                source_ml_temporary_index6.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index6 = source_ml_temporary_index6_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedLoanReadErrorLoop7Result::MlTemporaryReadRustOwnedLoanReadErrorLoop7Done
        };
    }
}
