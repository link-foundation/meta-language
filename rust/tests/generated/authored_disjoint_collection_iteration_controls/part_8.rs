pub fn render_array_state_method_expression(target: self::ml_string::Text, operation: self::ml_string::Text, receiver: self::ml_string::Text, arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, profiles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, types: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, suffix: self::ml_string::Text) -> self::ml_string::Text {
    if ((self::read_array_state_method_result(operation.clone())) == (self::ml_string::Text::from_units(vec![]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
            ((((((self::ml_string::Text::from_units(vec![40])).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![41, 46]))).concat(&operation)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&self::ml_string::join(&{ let ml_reference_value12 = arguments_.borrow().clone(); ml_reference_value12 }, &self::ml_string::Text::from_units(vec![44, 32])))).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            if (target != (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                {
                    let owner = (self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 111, 119, 110, 101, 114])).concat(&suffix);
                    {
                        let output = ((((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32])).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![59, 32]));
                        {
                            let index = 0f64;
                            {
                                let output_2 = self::source_ml_source1_temporary_render_array_state_method_expression_loop10(arguments_.clone(), types.clone(), suffix.clone(), output.clone(), index);
                                {
                                    let names = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                    {
                                        let index_2 = 0f64;
                                        {
                                            let names_2 = self::source_ml_source1_temporary_render_array_state_method_expression_loop11(arguments_.clone(), suffix.clone(), names.clone(), index_2);
                                            {
                                                let output_3 = output_2.concat(&((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 61, 32])).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 40, 41, 46, 108, 101, 110, 40, 41, 59, 32])));
                                                if (operation == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101]))) {
                                                    {
                                                        let begin = if (({ let ml_reference_array13 = arguments_.clone(); let ml_reference_projection13 = ml_reference_array13.borrow().len() as f64; ml_reference_projection13 }) == (0f64)) {
                                                            self::ml_string::Text::from_units(vec![48, 102, 54, 52])
                                                        } else {
                                                            self::render_array_state_numeric_value(({
                                                                let source_ml_source2_array_read_value20 = profiles.clone();
                                                                {
                                                                    let source_ml_source2_array_read_index21 = 0f64;
                                                                    { let ml_reference_array14 = source_ml_source2_array_read_value20.clone(); let ml_reference_position14 = source_ml_source2_array_read_index21; let ml_reference_projection14 = self::ml_array::optional_at(&*ml_reference_array14.borrow(), self::ml_array::number_index(ml_reference_position14)); ml_reference_projection14 }
                                                                }
                                                            }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), ({
                                                                let source_ml_source2_array_read_value22 = names_2.clone();
                                                                {
                                                                    let source_ml_source2_array_read_index23 = 0f64;
                                                                    { let ml_reference_array15 = source_ml_source2_array_read_value22.clone(); let ml_reference_position15 = source_ml_source2_array_read_index23; let ml_reference_projection15 = self::ml_array::optional_at(&*ml_reference_array15.borrow(), self::ml_array::number_index(ml_reference_position15)); ml_reference_projection15 }
                                                                }
                                                            }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78]))
                                                        };
                                                        {
                                                            let output_4 = output_3.concat(&((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 61, 32])).concat(&self::render_array_state_relative_index(begin.clone(), self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104])))).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                                                            {
                                                                let count = if (({ let ml_reference_array16 = arguments_.clone(); let ml_reference_projection16 = ml_reference_array16.borrow().len() as f64; ml_reference_projection16 }) == (0f64)) {
                                                                    self::ml_string::Text::from_units(vec![48, 117, 115, 105, 122, 101])
                                                                } else {
                                                                    if (({ let ml_reference_array17 = arguments_.clone(); let ml_reference_projection17 = ml_reference_array17.borrow().len() as f64; ml_reference_projection17 }) == (1f64)) {
                                                                        self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 45, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116])
                                                                    } else {
                                                                        ((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 108, 95, 100, 101, 108, 101, 116, 101, 95, 118, 97, 108, 117, 101, 58, 32, 102, 54, 52, 32, 61, 32])).concat(&self::render_array_state_numeric_value(({
                                                                            let source_ml_source2_array_read_value24 = profiles.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index25 = 1f64;
                                                                                { let ml_reference_array18 = source_ml_source2_array_read_value24.clone(); let ml_reference_position18 = source_ml_source2_array_read_index25; let ml_reference_projection18 = self::ml_array::optional_at(&*ml_reference_array18.borrow(), self::ml_array::number_index(ml_reference_position18)); ml_reference_projection18 }
                                                                            }
                                                                        }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), ({
                                                                            let source_ml_source2_array_read_value26 = names_2.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index27 = 1f64;
                                                                                { let ml_reference_array19 = source_ml_source2_array_read_value26.clone(); let ml_reference_position19 = source_ml_source2_array_read_index27; let ml_reference_projection19 = self::ml_array::optional_at(&*ml_reference_array19.borrow(), self::ml_array::number_index(ml_reference_position19)); ml_reference_projection19 }
                                                                            }
                                                                        }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78])))).concat(&self::ml_string::Text::from_units(vec![59, 32, 109, 108, 95, 100, 101, 108, 101, 116, 101, 95, 118, 97, 108, 117, 101, 46, 116, 114, 117, 110, 99, 40, 41, 46, 109, 97, 120, 40, 48, 102, 54, 52, 41, 46, 109, 105, 110, 40, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 45, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 41, 32, 97, 115, 32, 102, 54, 52, 41, 32, 97, 115, 32, 117, 115, 105, 122, 101, 32, 125]))
                                                                    }
                                                                };
                                                                {
                                                                    let inserted = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                                    {
                                                                        let index_3 = 2f64;
                                                                        {
                                                                            let inserted_2 = self::source_ml_source1_temporary_render_array_state_method_expression_loop12(arguments_.clone(), names_2.clone(), inserted.clone(), index_3);
                                                                            (((((((((output_4.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 32, 61, 32]))).concat(&count)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 115, 101, 114, 116, 101, 100, 32, 61, 32, 118, 101, 99, 33, 91]))).concat(&self::ml_string::join(&{ let ml_reference_value20 = inserted_2.borrow().clone(); ml_reference_value20 }, &self::ml_string::Text::from_units(vec![44, 32])))).concat(&self::ml_string::Text::from_units(vec![93, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 95, 32, 61, 32, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 45, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 41, 46, 99, 104, 101, 99, 107, 101, 100, 95, 97, 100, 100, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 115, 101, 114, 116, 101, 100, 46, 108, 101, 110, 40, 41, 41, 46, 102, 105, 108, 116, 101, 114, 40, 124, 108, 101, 110, 103, 116, 104, 124, 32, 42, 108, 101, 110, 103, 116, 104, 32, 60, 61, 32, 52, 50, 57, 52, 57, 54, 55, 50, 57, 53, 117, 115, 105, 122, 101, 41, 46, 101, 120, 112, 101, 99, 116, 40, 34, 73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 34, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 114, 101, 109, 111, 118, 101, 100, 58, 32, 86, 101, 99, 60, 95, 62, 32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 46, 115, 112, 108, 105, 99, 101, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 46, 46, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 44, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 115, 101, 114, 116, 101, 100, 41, 46, 99, 111, 108, 108, 101, 99, 116, 40, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![115, 116, 100, 58, 58, 114, 99, 58, 58, 82, 99, 58, 58, 110, 101, 119, 40, 115, 116, 100, 58, 58, 99, 101, 108, 108, 58, 58, 82, 101, 102, 67, 101, 108, 108, 58, 58, 110, 101, 119, 40, 109, 108, 95, 114, 101, 109, 111, 118, 101, 100, 41, 41, 32, 125]))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let first = if (operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 108]))) {
                                                            1f64
                                                        } else {
                                                            0f64
                                                        };
                                                        {
                                                            let start_value = if (({ let ml_reference_array21 = arguments_.clone(); let ml_reference_projection21 = ml_reference_array21.borrow().len() as f64; ml_reference_projection21 }) <= first) {
                                                                self::ml_string::Text::from_units(vec![48, 102, 54, 52])
                                                            } else {
                                                                self::render_array_state_numeric_value(({
                                                                    let source_ml_source2_array_read_value28 = profiles.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index29 = first;
                                                                        { let ml_reference_array22 = source_ml_source2_array_read_value28.clone(); let ml_reference_position22 = source_ml_source2_array_read_index29; let ml_reference_projection22 = self::ml_array::optional_at(&*ml_reference_array22.borrow(), self::ml_array::number_index(ml_reference_position22)); ml_reference_projection22 }
                                                                    }
                                                                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), ({
                                                                    let source_ml_source2_array_read_value30 = names_2.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index31 = first;
                                                                        { let ml_reference_array23 = source_ml_source2_array_read_value30.clone(); let ml_reference_position23 = source_ml_source2_array_read_index31; let ml_reference_projection23 = self::ml_array::optional_at(&*ml_reference_array23.borrow(), self::ml_array::number_index(ml_reference_position23)); ml_reference_projection23 }
                                                                    }
                                                                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78]))
                                                            };
                                                            {
                                                                let output_5 = output_3.concat(&((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 61, 32])).concat(&self::render_array_state_relative_index(start_value.clone(), self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104])))).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                                                                {
                                                                    let end_value = if (({ let ml_reference_array24 = arguments_.clone(); let ml_reference_projection24 = ml_reference_array24.borrow().len() as f64; ml_reference_projection24 }) <= (2f64)) {
                                                                        self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 97, 115, 32, 102, 54, 52])
                                                                    } else {
                                                                        self::render_array_state_numeric_value(({
                                                                            let source_ml_source2_array_read_value32 = profiles.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index33 = 2f64;
                                                                                { let ml_reference_array25 = source_ml_source2_array_read_value32.clone(); let ml_reference_position25 = source_ml_source2_array_read_index33; let ml_reference_projection25 = self::ml_array::optional_at(&*ml_reference_array25.borrow(), self::ml_array::number_index(ml_reference_position25)); ml_reference_projection25 }
                                                                            }
                                                                        }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), ({
                                                                            let source_ml_source2_array_read_value34 = names_2.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index35 = 2f64;
                                                                                { let ml_reference_array26 = source_ml_source2_array_read_value34.clone(); let ml_reference_position26 = source_ml_source2_array_read_index35; let ml_reference_projection26 = self::ml_array::optional_at(&*ml_reference_array26.borrow(), self::ml_array::number_index(ml_reference_position26)); ml_reference_projection26 }
                                                                            }
                                                                        }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 97, 115, 32, 102, 54, 52]))
                                                                    };
                                                                    {
                                                                        let end = ((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 101, 110, 100, 32, 61, 32])).concat(&self::render_array_state_relative_index(end_value.clone(), self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104])))).concat(&self::ml_string::Text::from_units(vec![59, 32]));
                                                                        if (operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 108]))) {
                                                                            ((((((((output_5.concat(&end)).concat(&self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 117, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![105, 102, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 60, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 101, 110, 100, 32, 123, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 46, 46, 109, 108, 95, 97, 114, 114, 97, 121, 95, 101, 110, 100, 93, 46, 102, 105, 108, 108, 40]))).concat(&({
                                                                                let source_ml_source2_array_read_value36 = names_2.clone();
                                                                                {
                                                                                    let source_ml_source2_array_read_index37 = 0f64;
                                                                                    { let ml_reference_array27 = source_ml_source2_array_read_value36.clone(); let ml_reference_position27 = source_ml_source2_array_read_index37; let ml_reference_projection27 = self::ml_array::optional_at(&*ml_reference_array27.borrow(), self::ml_array::number_index(ml_reference_position27)); ml_reference_projection27 }
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))).concat(&self::ml_string::Text::from_units(vec![41, 59, 32, 125, 32, 125, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                                                                        } else {
                                                                            {
                                                                                let from_value = if (({ let ml_reference_array28 = arguments_.clone(); let ml_reference_projection28 = ml_reference_array28.borrow().len() as f64; ml_reference_projection28 }) <= (1f64)) {
                                                                                    self::ml_string::Text::from_units(vec![48, 102, 54, 52])
                                                                                } else {
                                                                                    self::render_array_state_numeric_value(({
                                                                                        let source_ml_source2_array_read_value38 = profiles.clone();
                                                                                        {
                                                                                            let source_ml_source2_array_read_index39 = 1f64;
                                                                                            { let ml_reference_array29 = source_ml_source2_array_read_value38.clone(); let ml_reference_position29 = source_ml_source2_array_read_index39; let ml_reference_projection29 = self::ml_array::optional_at(&*ml_reference_array29.borrow(), self::ml_array::number_index(ml_reference_position29)); ml_reference_projection29 }
                                                                                        }
                                                                                    }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), ({
                                                                                        let source_ml_source2_array_read_value40 = names_2.clone();
                                                                                        {
                                                                                            let source_ml_source2_array_read_index41 = 1f64;
                                                                                            { let ml_reference_array30 = source_ml_source2_array_read_value40.clone(); let ml_reference_position30 = source_ml_source2_array_read_index41; let ml_reference_projection30 = self::ml_array::optional_at(&*ml_reference_array30.borrow(), self::ml_array::number_index(ml_reference_position30)); ml_reference_projection30 }
                                                                                        }
                                                                                    }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78]))
                                                                                };
                                                                                (((((((((((output_5.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 61, 32]))).concat(&self::render_array_state_relative_index(from_value.clone(), self::ml_string::Text::from_units(vec![109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104])))).concat(&self::ml_string::Text::from_units(vec![59, 32]))).concat(&end)).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 32, 61, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 101, 110, 100, 46, 115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 115, 117, 98, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 41, 46, 109, 105, 110, 40, 109, 108, 95, 97, 114, 114, 97, 121, 95, 108, 101, 110, 103, 116, 104, 32, 45, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 117, 116, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32, 105, 102, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 60, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 38, 38, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 60, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 32, 123, 32]))).concat(&self::ml_string::Text::from_units(vec![102, 111, 114, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 32, 105, 110, 32, 40, 48, 46, 46, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 41, 46, 114, 101, 118, 40, 41, 32, 123, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 93, 32, 61, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 93, 46, 99, 108, 111, 110, 101, 40, 41, 59, 32, 125, 32]))).concat(&self::ml_string::Text::from_units(vec![125, 32, 101, 108, 115, 101, 32, 123, 32, 102, 111, 114, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 32, 105, 110, 32, 48, 46, 46, 109, 108, 95, 97, 114, 114, 97, 121, 95, 99, 111, 117, 110, 116, 32, 123, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 115, 116, 97, 114, 116, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 93, 32, 61, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 118, 97, 108, 117, 101, 115, 91, 109, 108, 95, 97, 114, 114, 97, 121, 95, 102, 114, 111, 109, 32, 43, 32, 109, 108, 95, 97, 114, 114, 97, 121, 95, 105, 110, 100, 101, 120, 93, 46, 99, 108, 111, 110, 101, 40, 41, 59, 32, 125, 32, 125, 32, 125, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
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

pub fn read_collection_access_result(kind: self::ml_string::Text, operation: self::ml_string::Text) -> self::ml_string::Text {
    if ((kind != (self::ml_string::Text::from_units(vec![109, 97, 112]))) && (kind != (self::ml_string::Text::from_units(vec![115, 101, 116])))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![104, 97, 115]))) {
            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
        } else {
            if ((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && (operation == (self::ml_string::Text::from_units(vec![103, 101, 116])))) {
                self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 32, 118, 97, 108, 117, 101])
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) {
                    self::ml_string::Text::from_units(vec![107, 101, 121, 115])
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))) {
                        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]))) {
                            self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])
                        } else {
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
            }
        }
    }
}

pub fn read_collection_access_argument_count(operation: self::ml_string::Text) -> f64 {
    if ((operation == (self::ml_string::Text::from_units(vec![103, 101, 116]))) || (operation == (self::ml_string::Text::from_units(vec![104, 97, 115])))) {
        1f64
    } else {
        0f64
    }
}

pub fn render_collection_size_expression(target: self::ml_string::Text, object: self::ml_string::Text) -> self::ml_string::Text {
    if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        ((self::ml_string::Text::from_units(vec![40])).concat(&object)).concat(&self::ml_string::Text::from_units(vec![41, 46, 115, 105, 122, 101]))
    } else {
        if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
            ((self::ml_string::Text::from_units(vec![40])).concat(&object)).concat(&self::ml_string::Text::from_units(vec![46, 108, 101, 110, 40, 41, 32, 97, 115, 32, 102, 54, 52, 41]))
        } else {
            if (target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                ((self::ml_string::Text::from_units(vec![40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32])).concat(&object)).concat(&self::ml_string::Text::from_units(vec![46, 115, 105, 122, 101, 41]))
            } else {
                if (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113]))) {
                    ((self::ml_string::Text::from_units(vec![40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 111, 102, 95, 117, 105, 110, 116, 54, 51, 32, 40, 85, 105, 110, 116, 54, 51, 46, 111, 102, 95, 90, 32, 40, 90, 46, 111, 102, 95, 110, 97, 116, 32, 40, 76, 105, 115, 116, 46, 108, 101, 110, 103, 116, 104, 32])).concat(&object)).concat(&self::ml_string::Text::from_units(vec![41, 41, 41, 41]))
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn render_collection_read_expression(target: self::ml_string::Text, kind: self::ml_string::Text, operation: self::ml_string::Text, object: self::ml_string::Text, search: self::ml_string::Text, float_key: bool, comparison: self::ml_string::Text, key_type: self::ml_string::Text, value_type: self::ml_string::Text) -> self::ml_string::Text {
    if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        ((((((self::ml_string::Text::from_units(vec![40])).concat(&object)).concat(&self::ml_string::Text::from_units(vec![41, 46]))).concat(&operation)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&search)).concat(&self::ml_string::Text::from_units(vec![41]))
    } else {
        if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
            {
                let equal = if float_key {
                    self::ml_string::Text::from_units(vec![40, 109, 108, 95, 107, 101, 121, 32, 61, 61, 32, 38, 109, 108, 95, 115, 101, 97, 114, 99, 104, 32, 124, 124, 32, 40, 109, 108, 95, 107, 101, 121, 46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 38, 38, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 46, 105, 115, 95, 110, 97, 110, 40, 41, 41, 41])
                } else {
                    self::ml_string::Text::from_units(vec![109, 108, 95, 107, 101, 121, 32, 61, 61, 32, 38, 109, 108, 95, 115, 101, 97, 114, 99, 104])
                };
                {
                    let bindings = if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                        self::ml_string::Text::from_units(vec![40, 109, 108, 95, 107, 101, 121, 44, 32, 109, 108, 95, 118, 97, 108, 117, 101, 41])
                    } else {
                        self::ml_string::Text::from_units(vec![109, 108, 95, 107, 101, 121])
                    };
                    {
                        let action = if (operation == (self::ml_string::Text::from_units(vec![103, 101, 116]))) {
                            ((((self::ml_string::Text::from_units(vec![102, 105, 110, 100, 95, 109, 97, 112, 40, 124])).concat(&bindings)).concat(&self::ml_string::Text::from_units(vec![124, 32, 105, 102, 32]))).concat(&equal)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 83, 111, 109, 101, 40, 109, 108, 95, 118, 97, 108, 117, 101, 46, 99, 108, 111, 110, 101, 40, 41, 41, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 78, 111, 110, 101, 32, 125, 41]))
                        } else {
                            ((((self::ml_string::Text::from_units(vec![97, 110, 121, 40, 124])).concat(&bindings)).concat(&self::ml_string::Text::from_units(vec![124, 32]))).concat(&equal)).concat(&self::ml_string::Text::from_units(vec![41]))
                        };
                        ((((((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 108, 95, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 61, 32])).concat(&object)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 32, 61, 32]))).concat(&search)).concat(&self::ml_string::Text::from_units(vec![59, 32, 109, 108, 95, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 46, 105, 116, 101, 114, 40, 41, 46]))).concat(&action)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                    }
                }
            }
        } else {
            if (target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                {
                    let key = if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                        self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101, 46, 49])
                    } else {
                        self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])
                    };
                    {
                        let equal_2 = if float_key {
                            ((((self::ml_string::Text::from_units(vec![40])).concat(&key)).concat(&self::ml_string::Text::from_units(vec![32, 61, 61, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 32, 124, 124, 32, 40]))).concat(&key)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 78, 97, 78, 32, 38, 38, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 46, 105, 115, 78, 97, 78, 41, 41]))
                        } else {
                            ((self::ml_string::Text::from_units(vec![40])).concat(&key)).concat(&self::ml_string::Text::from_units(vec![32, 61, 61, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 41]))
                        };
                        {
                            let result = if (operation == (self::ml_string::Text::from_units(vec![103, 101, 116]))) {
                                ((self::ml_string::Text::from_units(vec![40, 40, 109, 108, 95, 118, 97, 108, 117, 101, 115, 46, 116, 111, 76, 105, 115, 116, 46, 102, 105, 110, 100, 63, 32, 40, 102, 117, 110, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 61, 62, 32])).concat(&equal_2)).concat(&self::ml_string::Text::from_units(vec![41, 41, 46, 109, 97, 112, 32, 40, 102, 117, 110, 32, 109, 108, 95, 112, 97, 105, 114, 32, 61, 62, 32, 109, 108, 95, 112, 97, 105, 114, 46, 50, 41, 41]))
                            } else {
                                ((self::ml_string::Text::from_units(vec![40, 109, 108, 95, 118, 97, 108, 117, 101, 115, 46, 97, 110, 121, 32, 40, 102, 117, 110, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 61, 62, 32])).concat(&equal_2)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                            };
                            ((((((self::ml_string::Text::from_units(vec![40, 108, 101, 116, 32, 109, 108, 95, 118, 97, 108, 117, 101, 115, 32, 58, 61, 32])).concat(&object)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 32, 58, 61, 32]))).concat(&search)).concat(&self::ml_string::Text::from_units(vec![59, 32]))).concat(&result)).concat(&self::ml_string::Text::from_units(vec![41]))
                        }
                    }
                }
            } else {
                if (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113]))) {
                    {
                        let key_2 = if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                            self::ml_string::Text::from_units(vec![40, 102, 115, 116, 32, 109, 108, 95, 118, 97, 108, 117, 101, 41])
                        } else {
                            self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])
                        };
                        {
                            let equal_3 = if float_key {
                                ((((self::ml_string::Text::from_units(vec![40, 111, 114, 98, 32])).concat(&comparison)).concat(&self::ml_string::Text::from_units(vec![32, 40, 97, 110, 100, 98, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 105, 115, 95, 110, 97, 110, 32]))).concat(&key_2)).concat(&self::ml_string::Text::from_units(vec![41, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 105, 115, 95, 110, 97, 110, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 41, 41, 41]))
                            } else {
                                comparison.clone()
                            };
                            {
                                let result_2 = if (operation == (self::ml_string::Text::from_units(vec![103, 101, 116]))) {
                                    ((((((self::ml_string::Text::from_units(vec![40, 111, 112, 116, 105, 111, 110, 95, 109, 97, 112, 32, 40, 64, 115, 110, 100, 32])).concat(&key_type)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&value_type)).concat(&self::ml_string::Text::from_units(vec![41, 32, 40, 76, 105, 115, 116, 46, 102, 105, 110, 100, 32, 40, 102, 117, 110, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 61, 62, 32]))).concat(&equal_3)).concat(&self::ml_string::Text::from_units(vec![41, 32, 109, 108, 95, 118, 97, 108, 117, 101, 115, 41, 41]))
                                } else {
                                    ((self::ml_string::Text::from_units(vec![40, 76, 105, 115, 116, 46, 101, 120, 105, 115, 116, 115, 98, 32, 40, 102, 117, 110, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 61, 62, 32])).concat(&equal_3)).concat(&self::ml_string::Text::from_units(vec![41, 32, 109, 108, 95, 118, 97, 108, 117, 101, 115, 41]))
                                };
                                ((((((self::ml_string::Text::from_units(vec![40, 108, 101, 116, 32, 109, 108, 95, 118, 97, 108, 117, 101, 115, 32, 58, 61, 32])).concat(&object)).concat(&self::ml_string::Text::from_units(vec![32, 105, 110, 32, 108, 101, 116, 32, 109, 108, 95, 115, 101, 97, 114, 99, 104, 32, 58, 61, 32]))).concat(&search)).concat(&self::ml_string::Text::from_units(vec![32, 105, 110, 32]))).concat(&result_2)).concat(&self::ml_string::Text::from_units(vec![41]))
                            }
                        }
                    }
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn read_disjoint_collection_iteration_source_operation() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![114, 101, 97, 100, 67, 108, 111, 115, 101, 100, 83, 116, 97, 110, 100, 97, 114, 100, 67, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 105, 111, 110])
}

pub fn read_disjoint_collection_spread_source_operation() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![114, 101, 97, 100, 67, 108, 111, 115, 101, 100, 83, 116, 97, 110, 100, 97, 114, 100, 67, 111, 108, 108, 101, 99, 116, 105, 111, 110, 83, 112, 114, 101, 97, 100])
}

pub fn read_disjoint_collection_iteration_marker(ordinal: f64, operation: self::ml_string::Text) -> self::ml_string::Text {
    (((self::ml_string::Text::from_units(vec![35, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32])).concat(&{
        let source_ml_source2_string_argument42 = ordinal;
        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument42))
    })).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&operation)
}

pub fn read_disjoint_collection_spread_marker(ordinal: f64, operation: self::ml_string::Text) -> self::ml_string::Text {
    (((self::ml_string::Text::from_units(vec![35, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 112, 114, 101, 97, 100, 32])).concat(&{
        let source_ml_source2_string_argument43 = ordinal;
        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument43))
    })).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&operation)
}

pub fn read_disjoint_collection_iteration_marker_role(operation: self::ml_string::Text) -> self::ml_string::Text {
    if operation.starts_with(&self::ml_string::Text::from_units(vec![35, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32])) {
        self::ml_string::Text::from_units(vec![108, 111, 111, 112])
    } else {
        if operation.starts_with(&self::ml_string::Text::from_units(vec![35, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 112, 114, 101, 97, 100, 32])) {
            self::ml_string::Text::from_units(vec![101, 97, 103, 101, 114])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn accept_disjoint_collection_spread_receiver(shape: self::ml_string::Text) -> bool {
    ((shape == (self::ml_string::Text::from_units(vec![110, 97, 109, 101]))) || (shape == (self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100]))))
}

pub fn accept_disjoint_collection_iteration_scope(known: bool, externals: f64, literals: f64, bindings: f64, directory: bool, publication: bool) -> bool {
    (((((known && (externals == (0f64))) && (literals == (0f64))) && (bindings == (0f64))) && !directory) && !publication)
}

pub fn accept_disjoint_collection_iteration_operation(kind: self::ml_string::Text, operation: self::ml_string::Text) -> bool {
    (((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) || (kind == (self::ml_string::Text::from_units(vec![115, 101, 116])))) && (operation == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))))
}

pub fn read_disjoint_collection_iteration_item_form(kind: self::ml_string::Text, operation: self::ml_string::Text) -> self::ml_string::Text {
    if self::accept_disjoint_collection_iteration_operation(kind.clone(), operation.clone()) {
        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])
    } else {
        {
            let form = self::read_collection_access_result(kind.clone(), operation.clone());
            if ((form == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) || (form == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])))) {
                form.clone()
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_disjoint_collection_iteration_identity_kind(kind: self::ml_string::Text) -> bool {
    self::ml_array::includes(&{ let ml_reference_value31 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![115, 101, 116]), self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]), self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]), self::ml_string::Text::from_units(vec![100, 97, 116, 97]), self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![99, 101, 108, 108]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])]))).borrow().clone(); ml_reference_value31 }, &kind.clone(), 0f64, false, |a, b| a == b)
}

pub fn read_disjoint_collection_iteration_method_effect(kind: self::ml_string::Text, operation: self::ml_string::Text) -> self::ml_string::Text {
    if ((((self::read_reference_method_result(kind.clone(), operation.clone())) != (self::ml_string::Text::from_units(vec![]))) && (operation != (self::ml_string::Text::from_units(vec![103, 101, 116])))) && (operation != (self::ml_string::Text::from_units(vec![104, 97, 115])))) {
        self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])
    } else {
        if ((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) || (kind == (self::ml_string::Text::from_units(vec![115, 101, 116])))) {
            if (self::ml_array::includes(&{ let ml_reference_value32 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![104, 97, 115]), self::ml_string::Text::from_units(vec![107, 101, 121, 115]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])]))).borrow().clone(); ml_reference_value32 }, &operation.clone(), 0f64, false, |a, b| a == b) || ((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && (operation == (self::ml_string::Text::from_units(vec![103, 101, 116]))))) {
                self::ml_string::Text::from_units(vec![114, 101, 97, 100])
            } else {
                self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110])
            }
        } else {
            if (kind == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) {
                if self::ml_array::includes(&{ let ml_reference_value33 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 111, 114, 116]), self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101]), self::ml_string::Text::from_units(vec![102, 105, 108, 108]), self::ml_string::Text::from_units(vec![99, 111, 112, 121, 87, 105, 116, 104, 105, 110])]))).borrow().clone(); ml_reference_value33 }, &operation.clone(), 0f64, false, |a, b| a == b) {
                    self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])
                } else {
                    if (self::read_disjoint_collection_iteration_callback(operation.clone()) || self::ml_array::includes(&{ let ml_reference_value34 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101]), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116]), self::ml_string::Text::from_units(vec![106, 111, 105, 110]), self::ml_string::Text::from_units(vec![105, 110, 99, 108, 117, 100, 101, 115]), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 79, 102]), self::ml_string::Text::from_units(vec![108, 97, 115, 116, 73, 110, 100, 101, 120, 79, 102]), self::ml_string::Text::from_units(vec![97, 116]), self::ml_string::Text::from_units(vec![102, 108, 97, 116]), self::ml_string::Text::from_units(vec![116, 111, 82, 101, 118, 101, 114, 115, 101, 100]), self::ml_string::Text::from_units(vec![116, 111, 83, 111, 114, 116, 101, 100])]))).borrow().clone(); ml_reference_value34 }, &operation.clone(), 0f64, false, |a, b| a == b)) {
                        self::ml_string::Text::from_units(vec![114, 101, 97, 100])
                    } else {
                        self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110])
                    }
                }
            } else {
                if (kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) {
                    if ((self::read_disjoint_collection_iteration_write_field(operation.clone())) >= (0f64)) {
                        self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 99, 111, 114, 100, 80, 114, 111, 112, 101, 114, 116, 121, 79, 114, 100, 101, 114]))) {
                            self::ml_string::Text::from_units(vec![114, 101, 97, 100])
                        } else {
                            self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110])
                        }
                    }
                } else {
                    self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110])
                }
            }
        }
    }
}

pub fn accept_disjoint_collection_iteration_read_expression(shape: self::ml_string::Text) -> bool {
    self::ml_array::includes(&{ let ml_reference_value35 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 116]), self::ml_string::Text::from_units(vec![105, 102]), self::ml_string::Text::from_units(vec![109, 97, 116, 99, 104]), self::ml_string::Text::from_units(vec![112, 114, 105, 110, 116]), self::ml_string::Text::from_units(vec![99, 111, 97, 108, 101, 115, 99, 101]), self::ml_string::Text::from_units(vec![115, 111, 109, 101]), self::ml_string::Text::from_units(vec![102, 108, 97, 116, 116, 101, 110, 79, 112, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112, 79, 112, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![108, 105, 116]), self::ml_string::Text::from_units(vec![117, 110, 105, 116]), self::ml_string::Text::from_units(vec![110, 111, 110, 101]), self::ml_string::Text::from_units(vec![97, 98, 111, 114, 116]), self::ml_string::Text::from_units(vec![105, 109, 112, 111, 115, 115, 105, 98, 108, 101]), self::ml_string::Text::from_units(vec![108, 97, 109, 98, 100, 97]), self::ml_string::Text::from_units(vec![99, 101, 108, 108, 82, 101, 97, 100]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 83, 105, 122, 101]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 82, 101, 97, 100]), self::ml_string::Text::from_units(vec![117, 110, 97, 114, 121]), self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]), self::ml_string::Text::from_units(vec![110, 117, 108, 108, 105, 115, 104, 78, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 73, 110, 100, 101, 120, 78, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![116, 114, 117, 116, 104, 121]), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 69, 113, 117, 97, 108]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 84, 121, 112, 101, 79, 102]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 84, 101, 120, 116]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 80, 114, 101, 100, 105, 99, 97, 116, 101]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 70, 105, 101, 108, 100]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 80, 114, 111, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 73, 110, 100, 101, 120]), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 80, 111, 105, 110, 116, 115]), self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 77, 97, 112]), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 84, 101, 115, 116]), self::ml_string::Text::from_units(vec![99, 97, 115, 116]), self::ml_string::Text::from_units(vec![97, 112, 112, 101, 110, 100]), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])]))).borrow().clone(); ml_reference_value35 }, &shape.clone(), 0f64, false, |a, b| a == b)
}

pub fn read_disjoint_collection_iteration_write_field(operation: self::ml_string::Text) -> f64 {
    {
        let prefix = self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100]);
        if !(operation.starts_with(&prefix)) {
            (-1f64)
        } else {
            {
                let text = {
                    let source_ml_source2_string_range_operand44 = operation.clone();
                    {
                        let source_ml_source2_string_range_operand45 = prefix.length();
                        {
                            let source_ml_source2_string_range_position46 = source_ml_source2_string_range_operand45;
                            {
                                let source_ml_source2_string_range_position47 = f64::INFINITY;
                                source_ml_source2_string_range_operand44.slice(source_ml_source2_string_range_position46, source_ml_source2_string_range_position47)
                            }
                        }
                    }
                };
                {
                    let index = self::ml_string_number::convert_java_script_string_number_units(&(text.clone()).0.iter().copied().map(f64::from).collect::<Vec<f64>>());
                    if ((self::ml_math::is_integer(index) && (index >= (0f64))) && (({
                        let source_ml_source2_string_argument49 = index;
                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument49))
                    }) == text)) {
                        index
                    } else {
                        (-1f64)
                    }
                }
            }
        }
    }
}

pub fn read_disjoint_collection_iteration_callback(operation: self::ml_string::Text) -> bool {
    ((((self::read_array_callback_result_form(operation.clone())) != (self::ml_string::Text::from_units(vec![]))) || (operation == (self::ml_string::Text::from_units(vec![115, 111, 114, 116])))) || (operation == (self::ml_string::Text::from_units(vec![116, 111, 83, 111, 114, 116, 101, 100]))))
}

pub fn accept_disjoint_collection_iteration_callback_head(role: self::ml_string::Text) -> bool {
    (((role == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) || (role == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101])))) || (role == (self::ml_string::Text::from_units(vec![118, 97, 114, 105, 97, 98, 108, 101]))))
}

pub fn add_fact(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, fact: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if (((self::read_required_number_fact(facts.clone(), 0f64)) == ((-1f64))) || self::ml_array::includes(&{ let ml_reference_value36 = facts.borrow().clone(); ml_reference_value36 }, &fact, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
        facts.clone()
    } else {
        std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source2_concat_value50 = facts.clone();
            {
                let source_ml_source2_concat_value51 = std::rc::Rc::new(std::cell::RefCell::new(vec![fact]));
                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value37 = source_ml_source2_concat_value50.borrow().clone(); ml_reference_value37 }), { let ml_reference_value38 = source_ml_source2_concat_value51.borrow().clone(); ml_reference_value38 })
            }
        }))
    }
}

pub fn add_value(values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, value: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if self::ml_array::includes(&{ let ml_reference_value39 = values.borrow().clone(); ml_reference_value39 }, &value, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) {
        values.clone()
    } else {
        std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source2_concat_value52 = values.clone();
            {
                let source_ml_source2_concat_value53 = std::rc::Rc::new(std::cell::RefCell::new(vec![value]));
                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value40 = source_ml_source2_concat_value52.borrow().clone(); ml_reference_value40 }), { let ml_reference_value41 = source_ml_source2_concat_value53.borrow().clone(); ml_reference_value41 })
            }
        }))
    }
}

pub fn read_roots(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, tag: f64, node: f64, width: f64, span: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let source_ml_source1_temporary_values13 = facts.clone();
            {
                let source_ml_source1_temporary_index13 = self::ml::Big::from_i128(0);
                {
                    let result_2 = self::source_ml_source1_temporary_read_roots_loop14(tag, node, width, span, result.clone(), source_ml_source1_temporary_values13.clone(), source_ml_source1_temporary_index13.clone());
                    result_2.clone()
                }
            }
        }
    }
}

pub fn add_roots(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, tag: f64, node: f64, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, width: f64, span: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let present = self::read_roots(facts.clone(), tag, node, width, span);
        {
            let source_ml_source1_temporary_values15 = roots.clone();
            {
                let source_ml_source1_temporary_index15 = self::ml::Big::from_i128(0);
                {
                    let present_2 = self::source_ml_source1_temporary_add_roots_loop17(present.clone(), source_ml_source1_temporary_values15.clone(), source_ml_source1_temporary_index15.clone());
                    if (({ let ml_reference_array42 = present_2.clone(); let ml_reference_projection42 = ml_reference_array42.borrow().len() as f64; ml_reference_projection42 }) > (32f64)) {
                        std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)]))
                    } else {
                        {
                            let result = facts.clone();
                            {
                                let source_ml_source1_temporary_values16 = roots.clone();
                                {
                                    let source_ml_source1_temporary_index16 = self::ml::Big::from_i128(0);
                                    {
                                        let result_2 = self::source_ml_source1_temporary_add_roots_loop18(tag, node, width, span, result.clone(), source_ml_source1_temporary_values16.clone(), source_ml_source1_temporary_index16.clone());
                                        result_2.clone()
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

pub fn edges(starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, node: f64, label: self::ml_string::Text) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let at = self::read_required_number_fact(starts.clone(), node);
            {
                let result_2 = self::source_ml_source1_temporary_edges_loop19(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, label.clone(), result.clone(), at);
                result_2.clone()
            }
        }
    }
}

pub fn read_field(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, field: f64, field_width: f64, width: f64, span: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let source_ml_source1_temporary_values20 = roots.clone();
            {
                let source_ml_source1_temporary_index20 = self::ml::Big::from_i128(0);
                {
                    let result_2 = self::source_ml_source1_temporary_read_field_loop22(facts.clone(), field, field_width, width, span, result.clone(), source_ml_source1_temporary_values20.clone(), source_ml_source1_temporary_index20.clone());
                    result_2.clone()
                }
            }
        }
    }
}

pub fn add_field(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, field: f64, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, field_width: f64, width: f64, span: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = facts.clone();
        {
            let source_ml_source1_temporary_values24 = owners.clone();
            {
                let source_ml_source1_temporary_index24 = self::ml::Big::from_i128(0);
                {
                    let result_2 = self::source_ml_source1_temporary_add_field_loop26(field, roots.clone(), field_width, width, span, result.clone(), source_ml_source1_temporary_values24.clone(), source_ml_source1_temporary_index24.clone());
                    result_2.clone()
                }
            }
        }
    }
}

pub fn copy_fields(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, roots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, destination: f64, field_width: f64, width: f64, span: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = facts.clone();
        {
            let source_ml_source1_temporary_values28 = roots.clone();
            {
                let source_ml_source1_temporary_index28 = self::ml::Big::from_i128(0);
                {
                    let result_2 = self::source_ml_source1_temporary_copy_fields_loop30(facts.clone(), destination, field_width, width, span, result.clone(), source_ml_source1_temporary_values28.clone(), source_ml_source1_temporary_index28.clone());
                    result_2.clone()
                }
            }
        }
    }
}

pub fn called(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, source: f64, width: f64, span: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    self::read_roots(facts.clone(), 3f64, source, width, span)
}

pub fn reachable(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, initial: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, width: f64, span: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = initial.clone();
        {
            let progress = true;
            {
                let result_2 = self::source_ml_source1_temporary_reachable_loop34(facts.clone(), width, span, result.clone(), progress);
                result_2.clone()
            }
        }
    }
}

pub fn separate_roots(left: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, right: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> bool {
    (((((({ let ml_reference_array43 = left.clone(); let ml_reference_projection43 = ml_reference_array43.borrow().len() as f64; ml_reference_projection43 }) > (0f64)) && (({ let ml_reference_array44 = right.clone(); let ml_reference_projection44 = ml_reference_array44.borrow().len() as f64; ml_reference_projection44 }) > (0f64))) && !(self::ml_array::includes(&{ let ml_reference_value45 = left.borrow().clone(); ml_reference_value45 }, &(-1f64), 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) && !(self::ml_array::includes(&{ let ml_reference_value46 = right.borrow().clone(); ml_reference_value46 }, &(-1f64), 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) && !({ let ml_values47 = { let ml_reference_value48 = left.borrow().clone(); ml_reference_value48 }; let ml_callback47 = { let right = right.clone(); std::rc::Rc::new(move |root: f64| -> bool { self::ml_array::includes(&{ let ml_reference_value49 = right.borrow().clone(); ml_reference_value49 }, &root, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) }) }; ml_values47.iter().cloned().enumerate().any(|(ml_index47, ml_value47)| ml_callback47(ml_value47)) }))
}

pub fn accepts_effects(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, operations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, scopes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, source: f64, allocation: bool, main_: f64, width: f64, span: f64) -> bool {
    {
        let roots = self::read_roots(facts.clone(), 1f64, source, width, span);
        if (allocation && ((({ let ml_reference_array50 = roots.clone(); let ml_reference_projection50 = ml_reference_array50.borrow().len() as f64; ml_reference_projection50 }) == (0f64)) || { let ml_values51 = { let ml_reference_value52 = roots.borrow().clone(); ml_reference_value52 }; let ml_callback51 = { let roles = roles.clone(); let kinds = kinds.clone(); let source = source; std::rc::Rc::new(move |root: f64| -> bool { (((root < (0f64)) || ((self::text_fact(roles.clone(), root)) != (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110])))) || ((self::text_fact(kinds.clone(), root)) != (self::text_fact(kinds.clone(), source)))) }) }; ml_values51.iter().cloned().enumerate().any(|(ml_index51, ml_value51)| ml_callback51(ml_value51)) })) {
            false
        } else {
            {
                let reached = self::reachable(facts.clone(), scopes.clone(), width, span);
                {
                    let source_ml_source1_temporary_values37 = reached.clone();
                    {
                        let source_ml_source1_temporary_index37 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s54 = self::source_ml_source1_temporary_accepts_effects_loop38(functions.clone(), bodies.clone(), main_, source_ml_source1_temporary_values37.clone(), source_ml_source1_temporary_index37.clone());
                            match source_ml_source2_s54.clone() {
                                self::MlSource1TemporaryAcceptsEffectsLoop38Result::MlSource1TemporaryAcceptsEffectsLoop38Done => {
                                    {
                                        let node = 0f64;
                                        {
                                            let source_ml_source2_s55 = self::source_ml_source1_temporary_accepts_effects_loop39(facts.clone(), roles.clone(), operations.clone(), kinds.clone(), owners.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), functions.clone(), source, width, span, roots.clone(), reached.clone(), node);
                                            match source_ml_source2_s55.clone() {
                                                self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Done => {
                                                    true
                                                }
                                                self::MlSource1TemporaryAcceptsEffectsLoop39Result::MlSource1TemporaryAcceptsEffectsLoop39Return(source_ml_source1_temporary_result39) => {
                                                    source_ml_source1_temporary_result39
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryAcceptsEffectsLoop38Result::MlSource1TemporaryAcceptsEffectsLoop38Return(source_ml_source1_temporary_result40) => {
                                    source_ml_source1_temporary_result40
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn bind_calls(facts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, owner: f64, callees: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, callback: self::ml_string::Text, receiver: f64, kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, main_: f64, width: f64, span: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = facts.clone();
        {
            let source_ml_source1_temporary_values41 = callees.clone();
            {
                let source_ml_source1_temporary_index41 = self::ml::Big::from_i128(0);
                {
                    let result_2 = self::source_ml_source1_temporary_bind_calls_loop43(owner, arguments_.clone(), callback.clone(), receiver, kinds.clone(), functions.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), bodies.clone(), main_, width, span, result.clone(), source_ml_source1_temporary_values41.clone(), source_ml_source1_temporary_index41.clone());
                    result_2.clone()
                }
            }
        }
    }
}

pub fn prove_disjoint_collection_iterations(roles: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, operations: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, owners: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, labels: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, targets: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, functions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, parameter_starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, parameter_counts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, bodies: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, main_: f64, snapshots: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, callbacks: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let size = { let ml_reference_array53 = roles.clone(); let ml_reference_projection53 = ml_reference_array53.borrow().len() as f64; ml_reference_projection53 };
        {
            let width = (size + 1f64);
            {
                let field_width = ({ let ml_reference_array54 = labels.clone(); let ml_reference_projection54 = ml_reference_array54.borrow().len() as f64; ml_reference_projection54 } + 1f64);
                {
                    let span = (((size * field_width) * width) + 1f64);
                    if (((((((((((((((size < (1f64)) || (size > (4096f64))) || (({ let ml_reference_array55 = labels.clone(); let ml_reference_projection55 = ml_reference_array55.borrow().len() as f64; ml_reference_projection55 }) > (32768f64))) || (({ let ml_reference_array56 = operations.clone(); let ml_reference_projection56 = ml_reference_array56.borrow().len() as f64; ml_reference_projection56 }) != size)) || (({ let ml_reference_array57 = kinds.clone(); let ml_reference_projection57 = ml_reference_array57.borrow().len() as f64; ml_reference_projection57 }) != size)) || (({ let ml_reference_array58 = owners.clone(); let ml_reference_projection58 = ml_reference_array58.borrow().len() as f64; ml_reference_projection58 }) != size)) || (({ let ml_reference_array59 = starts.clone(); let ml_reference_projection59 = ml_reference_array59.borrow().len() as f64; ml_reference_projection59 }) != size)) || (({ let ml_reference_array60 = counts.clone(); let ml_reference_projection60 = ml_reference_array60.borrow().len() as f64; ml_reference_projection60 }) != size)) || (({ let ml_reference_array61 = labels.clone(); let ml_reference_projection61 = ml_reference_array61.borrow().len() as f64; ml_reference_projection61 }) != ({ let ml_reference_array62 = targets.clone(); let ml_reference_projection62 = ml_reference_array62.borrow().len() as f64; ml_reference_projection62 }))) || (({ let ml_reference_array63 = functions.clone(); let ml_reference_projection63 = ml_reference_array63.borrow().len() as f64; ml_reference_projection63 }) != ({ let ml_reference_array64 = parameter_starts.clone(); let ml_reference_projection64 = ml_reference_array64.borrow().len() as f64; ml_reference_projection64 }))) || (({ let ml_reference_array65 = functions.clone(); let ml_reference_projection65 = ml_reference_array65.borrow().len() as f64; ml_reference_projection65 }) != ({ let ml_reference_array66 = parameter_counts.clone(); let ml_reference_projection66 = ml_reference_array66.borrow().len() as f64; ml_reference_projection66 }))) || (({ let ml_reference_array67 = functions.clone(); let ml_reference_projection67 = ml_reference_array67.borrow().len() as f64; ml_reference_projection67 }) != ({ let ml_reference_array68 = bodies.clone(); let ml_reference_projection68 = ml_reference_array68.borrow().len() as f64; ml_reference_projection68 }))) || !(self::ml_array::includes(&{ let ml_reference_value69 = functions.borrow().clone(); ml_reference_value69 }, &main_, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) || ((({ let ml_reference_array70 = snapshots.clone(); let ml_reference_projection70 = ml_reference_array70.borrow().len() as f64; ml_reference_projection70 } % 2f64)) != (0f64))) || ((({ let ml_reference_array71 = callbacks.clone(); let ml_reference_projection71 = ml_reference_array71.borrow().len() as f64; ml_reference_projection71 } % 2f64)) != (0f64))) {
                        std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))
                    } else {
                        {
                            let node = 0f64;
                            {
                                let source_ml_source2_s56 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop54(owners.clone(), starts.clone(), counts.clone(), targets.clone(), size, node);
                                match source_ml_source2_s56.clone() {
                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Done => {
                                        if ({ let ml_values72 = { let ml_reference_value73 = targets.borrow().clone(); ml_reference_value73 }; let ml_callback72 = { let size = size; std::rc::Rc::new(move |target: f64| -> bool { ((!(self::ml_math::is_integer(target)) || (target < (0f64))) || (target >= size)) }) }; ml_values72.iter().cloned().enumerate().any(|(ml_index72, ml_value72)| ml_callback72(ml_value72)) } || { let ml_values74 = { let ml_reference_value75 = parameters.borrow().clone(); ml_reference_value75 }; let ml_callback74 = { let size = size; std::rc::Rc::new(move |target: f64| -> bool { ((!(self::ml_math::is_integer(target)) || (target < (0f64))) || (target >= size)) }) }; ml_values74.iter().cloned().enumerate().any(|(ml_index74, ml_value74)| ml_callback74(ml_value74)) }) {
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))
                                        } else {
                                            {
                                                let at = 0f64;
                                                {
                                                    let source_ml_source2_s57 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop55(functions.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), bodies.clone(), size, at);
                                                    match source_ml_source2_s57.clone() {
                                                        self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Done => {
                                                            if ({ let ml_values76 = { let ml_reference_value77 = snapshots.borrow().clone(); ml_reference_value77 }; let ml_callback76 = { let size = size; std::rc::Rc::new(move |node_2: f64| -> bool { ((!(self::ml_math::is_integer(node_2)) || (node_2 < (0f64))) || (node_2 >= size)) }) }; ml_values76.iter().cloned().enumerate().any(|(ml_index76, ml_value76)| ml_callback76(ml_value76)) } || { let ml_values78 = { let ml_reference_value79 = callbacks.borrow().clone(); ml_reference_value79 }; let ml_callback78 = { let size = size; std::rc::Rc::new(move |node_2: f64| -> bool { ((!(self::ml_math::is_integer(node_2)) || (node_2 < (0f64))) || (node_2 >= size)) }) }; ml_values78.iter().cloned().enumerate().any(|(ml_index78, ml_value78)| ml_callback78(ml_value78)) }) {
                                                                std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))
                                                            } else {
                                                                {
                                                                    let facts = std::rc::Rc::new(std::cell::RefCell::new(vec![((4f64 * span) + main_)]));
                                                                    {
                                                                        let work = 0f64;
                                                                        {
                                                                            let settled = false;
                                                                            {
                                                                                let seeded = false;
                                                                                {
                                                                                    let pass = 0f64;
                                                                                    {
                                                                                        let source_ml_source2_s58 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop56(roles.clone(), operations.clone(), kinds.clone(), owners.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), functions.clone(), parameter_starts.clone(), parameter_counts.clone(), parameters.clone(), bodies.clone(), main_, snapshots.clone(), callbacks.clone(), size, width, field_width, span, facts.clone(), work, settled, seeded, pass);
                                                                                        match source_ml_source2_s58.clone() {
                                                                                            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Done(facts_2, settled_2) => {
                                                                                                if !settled_2 {
                                                                                                    std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))
                                                                                                } else {
                                                                                                    {
                                                                                                        let accepted = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                                                                        {
                                                                                                            let at_2 = 0f64;
                                                                                                            {
                                                                                                                let source_ml_source2_s59 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop57(roles.clone(), operations.clone(), kinds.clone(), owners.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), functions.clone(), bodies.clone(), main_, snapshots.clone(), size, width, span, facts_2.clone(), accepted.clone(), at_2);
                                                                                                                match source_ml_source2_s59.clone() {
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Done(accepted_2) => {
                                                                                                                        {
                                                                                                                            let safe = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                                                                                            {
                                                                                                                                let at_3 = 0f64;
                                                                                                                                {
                                                                                                                                    let safe_2 = self::source_ml_source1_temporary_prove_disjoint_collection_iterations_loop58(roles.clone(), operations.clone(), kinds.clone(), owners.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), functions.clone(), bodies.clone(), main_, callbacks.clone(), width, span, facts_2.clone(), safe.clone(), at_3);
                                                                                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                        let source_ml_source2_concat_value60 = std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, { let ml_reference_array80 = accepted_2.clone(); let ml_reference_projection80 = ml_reference_array80.borrow().len() as f64; ml_reference_projection80 }]));
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_concat_value61 = accepted_2.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_concat_value62 = std::rc::Rc::new(std::cell::RefCell::new(vec![{ let ml_reference_array81 = safe_2.clone(); let ml_reference_projection81 = ml_reference_array81.borrow().len() as f64; ml_reference_projection81 }]));
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_concat_value63 = safe_2.clone();
                                                                                                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value82 = source_ml_source2_concat_value60.borrow().clone(); ml_reference_value82 }), { let ml_reference_value83 = source_ml_source2_concat_value61.borrow().clone(); ml_reference_value83 }), { let ml_reference_value84 = source_ml_source2_concat_value62.borrow().clone(); ml_reference_value84 }), { let ml_reference_value85 = source_ml_source2_concat_value63.borrow().clone(); ml_reference_value85 })
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }))
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop57Return(source_ml_source1_temporary_result58) => {
                                                                                                                        source_ml_source1_temporary_result58.clone()
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop56Return(source_ml_source1_temporary_result63) => {
                                                                                                source_ml_source1_temporary_result63.clone()
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
                                                        self::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop55Return(source_ml_source1_temporary_result78) => {
                                                            source_ml_source1_temporary_result78.clone()
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Result::MlSource1TemporaryProveDisjointCollectionIterationsLoop54Return(source_ml_source1_temporary_result78_2) => {
                                        source_ml_source1_temporary_result78_2.clone()
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

pub fn first_number(values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> f64 {
    if (({ let ml_reference_array86 = values.clone(); let ml_reference_projection86 = ml_reference_array86.borrow().len() as f64; ml_reference_projection86 }) == (0f64)) {
        (-1f64)
    } else {
        self::read_required_number_fact(values.clone(), 0f64)
    }
}

pub fn text_fact(values: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, index: f64) -> self::ml_string::Text {
    if ((index < (0f64)) || (index >= ({ let ml_reference_array87 = values.clone(); let ml_reference_projection87 = ml_reference_array87.borrow().len() as f64; ml_reference_projection87 }))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::read_required_text_fact(values.clone(), index)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodResultJoin2 {
    MlSource1TemporaryReadReferenceMethodResultJoin2Next,
    MlSource1TemporaryReadReferenceMethodResultJoin2Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodResultJoin1 {
    MlSource1TemporaryReadReferenceMethodResultJoin1Next,
    MlSource1TemporaryReadReferenceMethodResultJoin1Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodIdentityJoin5 {
    MlSource1TemporaryReadReferenceMethodIdentityJoin5Next,
    MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodIdentityJoin4 {
    MlSource1TemporaryReadReferenceMethodIdentityJoin4Next,
    MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadReferenceMethodIdentityJoin3 {
    MlSource1TemporaryReadReferenceMethodIdentityJoin3Next,
    MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_render_reference_method_expression_loop6(mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut suffix: self::ml_string::Text, mut prefix: self::ml_string::Text, mut index: f64) -> self::ml_string::Text {
    loop {
        return if (index < ({ let ml_reference_array88 = arguments_.clone(); let ml_reference_projection88 = ml_reference_array88.borrow().len() as f64; ml_reference_projection88 })) {
            {
                let prefix_2 = prefix.concat(&((((((self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(index)))).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&({
                    let source_ml_source2_array_read_value64 = arguments_.clone();
                    {
                        let source_ml_source2_array_read_index65 = index;
                        { let ml_reference_array89 = source_ml_source2_array_read_value64.clone(); let ml_reference_position89 = source_ml_source2_array_read_index65; let ml_reference_projection89 = self::ml_array::optional_at(&*ml_reference_array89.borrow(), self::ml_array::number_index(ml_reference_position89)); ml_reference_projection89 }
                    }
                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                {
                    let index_2 = (index + 1f64);
                    {
                        (prefix, index) = (prefix_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            prefix.clone()
        };
    }
}

pub fn source_ml_source1_temporary_render_reference_method_expression_loop7(mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut suffix: self::ml_string::Text, mut names: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    loop {
        return if (index < ({ let ml_reference_array90 = arguments_.clone(); let ml_reference_projection90 = ml_reference_array90.borrow().len() as f64; ml_reference_projection90 })) {
            {
                let names_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value91 = names.borrow().clone(); ml_reference_value91 }, vec![(((self::ml_string::Text::from_units(vec![109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(index)))])));
                {
                    let index_2 = (index + 1f64);
                    {
                        (names, index) = (names_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            names.clone()
        };
    }
}
