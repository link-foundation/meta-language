// Generated from complete js/src/translation/javascript-primitive-conversion.js. Do not edit by hand.
use super::ml;
use super::ml_array;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_register_java_script_ordinary_symbol_loop28(
    mut identity: self::ml::Big,
    mut at: f64,
    mut source_ml_temporary_values27: Vec<self::ml::Big>,
    mut source_ml_temporary_index27: self::ml::Big,
) -> f64 {
    loop {
        if source_ml_temporary_index27
            < (self::ml::Big::from_u128(source_ml_temporary_values27.len() as u128))
        {
            {
                let symbol = self::ml_array::at(
                    &source_ml_temporary_values27,
                    source_ml_temporary_index27.to_index(),
                );
                if symbol < identity {
                    {
                        let at_2 = at + 1f64;
                        {
                            let source_ml_temporary_index27_2 =
                                source_ml_temporary_index27.add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_temporary_index27) =
                                    (at_2, source_ml_temporary_index27_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_temporary_index27_3 =
                            source_ml_temporary_index27.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index27 = source_ml_temporary_index27_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { at };
    }
}
