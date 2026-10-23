// Generated from complete js/src/project-translation-rules.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_project_translation_frontier_loop2(
    importers: &[self::ml_string::Text],
    providers: &[self::ml_string::Text],
    active: &[self::ml_string::Text],
    mut selected: Vec<self::ml_string::Text>,
    mut edge: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        return if edge < (importers.len() as f64) {
            {
                let provider =
                    self::ml_array::optional_at(providers, self::ml_array::number_index(edge));
                {
                    let importer =
                        self::ml_array::optional_at(importers, self::ml_array::number_index(edge));
                    if (!(match (provider.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && !(match (importer.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    })) && {
                        let ml_collection = active.to_vec();
                        let ml_search = provider
                            .clone()
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        ml_collection.iter().any(|ml_key| ml_key == &ml_search)
                    } {
                        {
                            let selected_2 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![importer.clone().unwrap_or_else(|| {
                                        self::ml_string::Text::from_units(vec![])
                                    })],
                                    selected,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let edge_2 = edge + 1f64;
                                {
                                    (selected, edge) = (selected_2, edge_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let edge_3 = edge + 1f64;
                            {
                                edge = edge_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            selected
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_project_translation_frontier_loop3(
    affected: &[self::ml_string::Text],
    mut next: Vec<self::ml_string::Text>,
    ml_values1: &[self::ml_string::Text],
    mut ml_index1: self::ml::Big,
) -> Vec<self::ml_string::Text> {
    loop {
        return if ml_index1 < (self::ml::Big::from_u128(ml_values1.len() as u128)) {
            {
                let identifier = self::ml_array::at(ml_values1, ml_index1.to_index());
                if {
                    let ml_collection = affected.to_vec();
                    let ml_search = identifier.clone();
                    ml_collection.iter().any(|ml_key| ml_key == &ml_search)
                } {
                    {
                        let next_2 = {
                            let (ml_append_right, ml_append_left) =
                                (vec![identifier.clone()], next);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let ml_index1_2 = ml_index1.add(&self::ml::Big::from_i128(1));
                            {
                                (next, ml_index1) = (next_2, ml_index1_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let ml_index1_3 = ml_index1.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index1 = ml_index1_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            next
        };
    }
}
