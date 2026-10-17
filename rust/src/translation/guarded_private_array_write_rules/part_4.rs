// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlPlanConstructedGuardedArrayWritesLoop21Result;
use super::MlPlanConstructedGuardedArrayWritesLoop22Result;
use super::accept_constructed_guarded_array_uses;
use super::ml_plan_constructed_guarded_array_writes_loop21;
use super::ml_plan_constructed_guarded_array_writes_loop22;
use super::ml_plan_constructed_guarded_array_writes_loop23;
use super::ml_plan_constructed_guarded_array_writes_loop24;
use super::read_guarded_write_count;
use super::read_guarded_write_element;
use super::read_guarded_write_field;
use super::read_guarded_write_kind;
use super::read_guarded_write_text_field;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_constructed_guarded_array_writes_loop20(
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
                    if (statement != "const") && (statement != "let") {
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
                            let value = self::read_guarded_write_field(
                                columns, rows, size, allocation, "value",
                            );
                            if ((self::read_guarded_write_kind(columns, rows, size, value))
                                != "array")
                                || ((self::read_guarded_write_count(
                                    columns,
                                    rows,
                                    size,
                                    self::read_guarded_write_field(
                                        columns, rows, size, value, "items",
                                    ),
                                )) != 0f64)
                            {
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
                                    let owner = self::read_guarded_write_text_field(
                                        columns, rows, size, allocation, "name",
                                    );
                                    if (owner).is_empty() {
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
                                            let writes = Vec::<f64>::new();
                                            {
                                                let conditionals = Vec::<f64>::new();
                                                {
                                                    let appends = Vec::<f64>::new();
                                                    {
                                                        let append_statements = Vec::<f64>::new();
                                                        {
                                                            let later = index + 1f64;
                                                            {
                                                                let ml_s18 = self::ml_plan_constructed_guarded_array_writes_loop21(columns, rows, size, statements, count, &(owner), writes, conditionals, later);
                                                                match ml_s18 {
                                                                    self::MlPlanConstructedGuardedArrayWritesLoop21Result::MlPlanConstructedGuardedArrayWritesLoop21Done(writes_2, conditionals_2) => {
                                                                        if (writes_2.len() as f64) == 0f64 {
                                                                            {
                                                                                let index_5 = index + 1f64;
                                                                                {
                                                                                    index = index_5;
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
{
                                                                            {
                                                                                let candidate = allocation + 1f64;
                                                                                {
                                                                                    let ml_s19 = self::ml_plan_constructed_guarded_array_writes_loop22(columns, rows, size, &(owner), appends, append_statements, candidate);
                                                                                    match ml_s19 {
                                                                                        self::MlPlanConstructedGuardedArrayWritesLoop22Result::MlPlanConstructedGuardedArrayWritesLoop22Done(appends_2, append_statements_2) => {
                                                                                            if ((appends_2.len() as f64) == 0f64) || !(self::accept_constructed_guarded_array_uses(columns, rows, size, &(owner), allocation, &(writes_2), &(appends_2))) {
                                                                                                {
                                                                                                    let index_6 = index + 1f64;
                                                                                                    {
                                                                                                        index = index_6;
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
{
                                                                                                {
                                                                                                    let at = 0f64;
                                                                                                    {
                                                                                                        let result_2 = self::ml_plan_constructed_guarded_array_writes_loop23(result, allocation, &(writes_2), &(conditionals_2), at);
                                                                                                        {
                                                                                                            let at_2 = 0f64;
                                                                                                            {
                                                                                                                let result_3 = self::ml_plan_constructed_guarded_array_writes_loop24(result_2, allocation, &(appends_2), &(append_statements_2), at_2);
                                                                                                                {
                                                                                                                    let index_7 = index + 1f64;
                                                                                                                    {
                                                                                                                        (result, index) = (result_3, index_7);
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
