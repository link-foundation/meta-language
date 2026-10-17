// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlPlanGuardedPrivateArrayWritesLoop13Result;
use super::accept_guarded_write_literal;
use super::accept_guarded_write_uses;
use super::ml_plan_guarded_private_array_writes_loop13;
use super::ml_plan_guarded_private_array_writes_loop14;
use super::read_guarded_write_element;
use super::read_guarded_write_field;
use super::read_guarded_write_text_field;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_guarded_private_array_writes_loop12(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut result: Vec<f64>,
    mut statements: f64,
    mut count: f64,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if index < count {
            {
                let allocation =
                    self::read_guarded_write_element(columns, rows, size, statements, index);
                {
                    let statement =
                        self::read_guarded_write_text_field(columns, rows, size, allocation, "s");
                    if ((statement != "const") && (statement != "let"))
                        || !(self::accept_guarded_write_literal(
                            columns,
                            rows,
                            size,
                            self::read_guarded_write_field(
                                columns, rows, size, allocation, "value",
                            ),
                        ))
                    {
                        {
                            let index_2 = index + 1f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let owner = self::read_guarded_write_text_field(
                                columns, rows, size, allocation, "name",
                            );
                            if (owner).is_empty() {
                                {
                                    let index_3 = index + 1f64;
                                    {
                                        index = index_3;
                                        continue;
                                    }
                                }
                            }
                            {
                                {
                                    let candidates = Vec::<f64>::new();
                                    {
                                        let owners = Vec::<f64>::new();
                                        {
                                            let later = index + 1f64;
                                            {
                                                let ml_s12 = self::ml_plan_guarded_private_array_writes_loop13(columns, rows, size, statements, count, &(owner), candidates, owners, later);
                                                match ml_s12 {
                                                    self::MlPlanGuardedPrivateArrayWritesLoop13Result::MlPlanGuardedPrivateArrayWritesLoop13Done(candidates_2, owners_2) => {
                                                        if ((candidates_2.len() as f64) == 0f64) || !(self::accept_guarded_write_uses(columns, rows, size, &(owner), allocation, &(candidates_2))) {
                                                            {
                                                                let index_4 = index + 1f64;
                                                                {
                                                                    index = index_4;
                                                                    continue;
                                                                }
                                                            }
                                                        }
{
                                                            {
                                                                let at = 0f64;
                                                                {
                                                                    let result_2 = self::ml_plan_guarded_private_array_writes_loop14(result, allocation, &(candidates_2), &(owners_2), at);
                                                                    {
                                                                        let index_5 = index + 1f64;
                                                                        {
                                                                            (result, index) = (result_2, index_5);
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
                    }
                }
            }
        } else {
            result
        };
    }
}
