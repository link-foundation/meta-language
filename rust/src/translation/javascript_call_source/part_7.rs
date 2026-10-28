// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSourceCompilation;
use super::ml_array;
use super::test_java_script_call_code_coordinate;

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptCallSourceLoop44Result {
    MlTemporaryCompileJavaScriptCallSourceLoop44Done,
    MlTemporaryCompileJavaScriptCallSourceLoop44Return(Box<self::JavaScriptCallSourceCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_call_source_loop44(parameters: &[f64], mut index: f64) -> self::MlTemporaryCompileJavaScriptCallSourceLoop44Result {
    loop {
        return if (index < ((parameters.len() as f64))) {
            {
                let owner = (self::ml_array::optional_at(parameters, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if (!(self::test_java_script_call_code_coordinate(owner, 65536f64)) || ((self::ml_array::index_of(parameters, &owner, 0f64, false, |a, b| a == b)) != index)) {
                    self::MlTemporaryCompileJavaScriptCallSourceLoop44Result::MlTemporaryCompileJavaScriptCallSourceLoop44Return(Box::new(self::JavaScriptCallSourceCompilation::SourceCompiled(Vec::<self::JavaScriptCallInstruction>::new(), 0f64, String::from("source parameters require distinct actual lexical owners"))))
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCompileJavaScriptCallSourceLoop44Result::MlTemporaryCompileJavaScriptCallSourceLoop44Done
        };
    }
}
