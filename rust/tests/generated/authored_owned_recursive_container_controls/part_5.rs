fn ml_main() {
    assert!(((self::accept_rust_owned_container_source(true, 90f64)) == (true)), "assertion 1");
    assert!(((self::accept_rust_owned_container_source(true, 99f64)) == (true)), "assertion 2");
    assert!(((self::accept_rust_owned_container_source(true, 100f64)) == (false)), "assertion 3");
    assert!(((self::accept_rust_owned_box_type_node(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box")])), String::from("library"), String::from("std::boxed::Box"), 1f64)) == (true)), "assertion 4");
    assert!(((self::accept_rust_owned_box_type_node(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box")])), String::from("declaration"), String::from("local::Box"), 1f64)) == (false)), "assertion 5");
    assert!(((self::accept_rust_owned_box_type_node(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("box")])), String::from("library"), String::from("std::boxed::Box"), 1f64)) == (false)), "assertion 6");
    assert!(((self::accept_rust_owned_standard_sum_box_payload(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box"), String::from("box")])), 0f64, String::from("library"), String::from("std::boxed::Box"), 1f64, 2f64)) == (true)), "assertion 7");
    assert!(((self::accept_rust_owned_standard_sum_box_payload(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box"), String::from("reference")])), 1f64, String::from("library"), String::from("std::boxed::Box"), 1f64, 1f64)) == (false)), "assertion 8");
    assert!(((self::read_rust_owned_box_type_error(true, false, true, String::from("owned"), true)) == ""), "assertion 9");
    assert!(((self::read_rust_owned_box_type_error(true, true, true, String::from(""), false)) == ""), "assertion 10");
    assert!(((self::read_rust_owned_box_type_error(true, false, true, String::from(""), false)) == "an owned Box source type needs its complete closed payload ownership graph"), "assertion 11");
    assert!(((self::accept_rust_owned_box_call_result(true, true, true, true, true)) == (true)), "assertion 12");
    assert!(((self::accept_rust_owned_box_call_result(true, true, false, true, true)) == (false)), "assertion 13");
    assert!(((self::accept_rust_owned_box_call_result(true, true, true, true, false)) == (false)), "assertion 14");
    assert!(((self::read_rust_owned_box_move_error(true, true, false)) == ""), "assertion 15");
    assert!(((self::read_rust_owned_box_payload_error(false, true, true, false)) == "stored Box payload references or callable captures need their complete lifetime/drop profile"), "assertion 16");
    assert!(((self::read_rust_owned_box_payload_error(true, false, false, false)) == ""), "assertion 17");
    assert!(((self::read_rust_owned_container_depth_error(256f64, 256f64)) == ""), "assertion 18");
    assert!(((self::read_rust_owned_container_depth_error(257f64, 0f64)) == "owned container source syntax and value lineage exceed their finite depth profile"), "assertion 19");
    assert!(((self::read_rust_owned_container_depth_error(f64::NAN, 0f64)) == "owned container source syntax and value lineage exceed their finite depth profile"), "assertion 20");
    assert!(((self::read_rust_owned_container_depth_error(0f64, f64::INFINITY)) == "owned container source syntax and value lineage exceed their finite depth profile"), "assertion 21");
    assert!(((self::read_rust_owned_container_depth_error(1.5f64, 0f64)) == "owned container source syntax and value lineage exceed their finite depth profile"), "assertion 22");
    assert!(((self::read_rust_owned_container_runtime_metadata_cost(String::from("owner"), 4f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 9f64])))), "assertion 23");
    assert!(((self::read_rust_owned_container_runtime_metadata_cost(String::from("origin"), 0f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![16f64, 256f64])))), "assertion 24");
    assert!(((self::plan_rust_owned_nominal_runtime_facts(true, String::from(""), false, String::from("owned"), false, true, 1f64, 0f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("owned"), String::from("")])))), "assertion 25");
    assert!(((self::plan_rust_owned_nominal_runtime_facts(true, String::from(""), false, String::from("owned"), true, true, 2f64, 1f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("owned"), String::from("Clone")])))), "assertion 26");
    assert!(((self::plan_rust_owned_nominal_runtime_facts(false, String::from(""), false, String::from("owned"), true, true, 1f64, 0f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from(""), String::from("")])))), "assertion 27");
    assert!(((self::plan_rust_owned_nominal_runtime_facts(true, String::from(""), false, String::from("owned"), true, false, 1f64, 0f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from(""), String::from("")])))), "assertion 28");
    assert!(((self::plan_rust_owned_nominal_runtime_facts(true, String::from(""), false, String::from("owned"), true, true, (-1f64), 0f64)) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from(""), String::from("")])))), "assertion 29");
    assert!(((self::plan_rust_owned_nominal_runtime_facts(false, String::from("copy"), true, String::from(""), false, false, (-1f64), (-1f64))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("copy"), String::from("Clone")])))), "assertion 30");
    assert!(((self::accept_rust_owned_vector_named_element(String::from("owned"), true, 0f64)) == (true)), "assertion 31");
    assert!(((self::accept_rust_owned_vector_named_element(String::from("copy"), false, 0f64)) == (false)), "assertion 32");
    assert!(((self::accept_rust_owned_vector_named_element(String::from("owned"), true, 1f64)) == (false)), "assertion 33");
    assert!(((self::read_rust_owned_vector_named_admission(String::from("refuse"), true)) == "admit"), "assertion 34");
    assert!(((self::read_rust_owned_vector_named_admission(String::from("defer"), true)) == "defer"), "assertion 35");
    assert!(((self::read_rust_owned_vector_signature_phase(true, true, false, String::from("array"), 0f64, String::from("library"), String::from("std::vec::Vec"), 1f64, String::from("named"), String::from(""), false)) == "defer"), "assertion 36");
    assert!(((self::read_rust_owned_vector_signature_phase(true, false, true, String::from("array"), 0f64, String::from("library"), String::from("std::vec::Vec"), 1f64, String::from("named"), String::from("owned"), true)) == "admit"), "assertion 37");
    assert!(((self::read_rust_owned_vector_signature_phase(true, false, true, String::from("array"), 0f64, String::from("declaration"), String::from("local::Vec"), 1f64, String::from("named"), String::from("owned"), true)) == ""), "assertion 38");
    assert!(((self::read_rust_owned_vector_signature_phase(true, false, true, String::from("array"), 0f64, String::from("library"), String::from("std::vec::Vec"), 1f64, String::from("named"), String::from("owned"), false)) == ""), "assertion 39");
    assert!(((self::read_rust_owned_vector_element_production_error(String::from("owned"), true, false, true, false, false)) == ""), "assertion 40");
    assert!(((self::read_rust_owned_vector_element_production_error(String::from("owned"), true, false, false, false, false)) == "an owned vector element needs its actual available binding or completed producer"), "assertion 41");
    assert!(((self::read_rust_owned_vector_element_production_error(String::from("owned"), true, false, false, false, true)) == ""), "assertion 42");
    assert!(((self::read_rust_owned_vector_element_production_error(String::from("owned"), false, true, true, true, true)) == "owned vector construction needs each actual complete matching element type"), "assertion 43");
    assert!(((self::accept_rust_owned_vector_call_result(true, true, true, true, true)) == (true)), "assertion 44");
    assert!(((self::accept_rust_owned_vector_call_result(true, true, true, true, false)) == (false)), "assertion 45");
    assert!(((self::accept_rust_owned_vector_produced_value(String::from("vector"), String::from("owned"), true, true)) == (true)), "assertion 46");
    assert!(((self::accept_rust_owned_vector_produced_value(String::from("vector"), String::from("owned"), true, false)) == (false)), "assertion 47");
    assert!(((self::accept_rust_owned_vector_function_result(String::from("admit"), true, true, true, true, false, false)) == (true)), "assertion 48");
    assert!(((self::accept_rust_owned_vector_function_result(String::from("admit"), true, true, true, false, false, true)) == (true)), "assertion 49");
    assert!(((self::accept_rust_owned_vector_function_result(String::from("admit"), true, true, true, false, false, false)) == (false)), "assertion 50");
    assert!(((self::accept_rust_owned_vector_function_result(String::from("admit"), true, true, true, true, true, true)) == (false)), "assertion 51");
    assert!(((self::read_rust_owned_vector_empty_constructor_disposition(String::from("admit"), String::from("std::vec::Vec"), String::from("new"), 0f64)) == "construct"), "assertion 52");
    assert!(((self::read_rust_owned_vector_empty_constructor_disposition(String::from("admit"), String::from("local::Vec"), String::from("new"), 0f64)) == ""), "assertion 53");
    assert!(((self::read_rust_owned_vector_empty_constructor_disposition(String::from("admit"), String::from("std::vec::Vec"), String::from("new"), 1f64)) == "arity"), "assertion 54");
    assert!(((self::read_rust_owned_vector_clone_error(false, true, false, true, true, true, true)) == "vector Clone needs its actual closed element Clone implementation"), "assertion 55");
    assert!(((self::read_rust_owned_vector_clone_error(true, true, true, true, false, false, false)) == ""), "assertion 56");
    assert!(((self::read_rust_owned_vector_clone_error(false, true, true, true, false, false, false)) == "vector Clone needs its actual available source value or actual producer"), "assertion 57");
    assert!(((self::read_rust_owned_box_clone_source_error(true, true, true, false, true, false, false)) == ""), "assertion 58");
    assert!(((self::read_rust_owned_box_clone_source_error(true, true, false, false, true, true, true)) == "Box Clone needs its actual admitted payload Clone implementation"), "assertion 59");
    assert!(((self::accept_rust_owned_argument_read_check(1f64, true)) == (false)), "assertion 60");
    assert!(((self::accept_rust_owned_argument_read_check(1f64, false)) == (true)), "assertion 61");
    assert!(((self::read_rust_owned_vector_element_count_error(65536f64)) == ""), "assertion 62");
    assert!(((self::read_rust_owned_vector_element_count_error(65537f64)) == "owned vector construction exceeds its finite actual element profile"), "assertion 63");
    assert!(((self::read_rust_owned_vector_element_count_error(f64::NAN)) == "owned vector construction exceeds its finite actual element profile"), "assertion 64");
    assert!(((self::read_rust_owned_nominal_field_lifetime_error(true, std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("")])))) == "a nongeneric datatype reference field requires its explicit static lifetime"), "assertion 65");
    assert!(((self::read_rust_owned_nominal_field_lifetime_error(true, std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("box"), String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("static")])))) == ""), "assertion 66");
    assert!(((self::read_rust_owned_nominal_field_lifetime_error(false, std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("")])))) == ""), "assertion 67");
    assert!(((self::plan_rust_owned_recursive_nominal_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Tree")])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![true])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("box"), String::from("constructor"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64, 2f64, 4f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64, 3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, (-1f64), (-1f64), (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![4f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![false, false, false, false, false])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles"), String::from("owned"), String::from("Clone"), String::from("closed"), String::from("1"), String::from("1")])))), "assertion 68");
    assert!(((self::plan_rust_owned_recursive_nominal_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Tree")])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("pending vector"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64, 3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, (-1f64), (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![false, false, false, false])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles"), String::from("owned"), String::from(""), String::from("closed"), String::from("1"), String::from("0")])))), "assertion 69");
    assert!(((self::plan_rust_owned_recursive_nominal_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Bad")])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![false, false, false])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("error"), String::from("a recursive source datatype size cycle needs an actual Box or Vec indirection")])))), "assertion 70");
    assert!(((self::plan_rust_owned_recursive_nominal_owners(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Endless")])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![false])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("box"), String::from("constructor"), String::from("nominal")])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64, 3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, (-1f64), (-1f64), (-1f64)])), std::rc::Rc::new(std::cell::RefCell::new(vec![3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![false, false, false, false])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("profiles"), String::from(""), String::from(""), String::from(""), String::from("-1"), String::from("-1")])))), "assertion 71");
    assert!(((self::read_rust_owned_vector_element_role(String::from(""), true, true, false, String::from(""))) == "copy"), "assertion 72");
    assert!(((self::read_rust_owned_vector_element_role(String::from(""), false, true, false, String::from(""))) == ""), "assertion 73");
    assert!(((self::read_rust_owned_vector_element_role(String::from("named"), true, true, false, String::from(""))) == ""), "assertion 74");
    assert!(((self::read_rust_owned_vector_element_role(String::from("named"), false, true, false, String::from("owned"))) == "owned"), "assertion 75");
    assert!(((self::accept_rust_owned_vector_element_type_match(String::from(""), String::from("fixed"), false, true)) == (true)), "assertion 76");
    assert!(((self::accept_rust_owned_vector_element_type_match(String::from(""), String::from("fixed"), false, false)) == (false)), "assertion 77");
    assert!(((self::accept_rust_owned_vector_element_type_match(String::from(""), String::from("named"), false, true)) == (false)), "assertion 78");
    assert!(((self::accept_rust_owned_vector_nominal_type_identity(String::from("foreign.Message"), String::from("foreign.Message"), true)) == (true)), "assertion 79");
    assert!(((self::accept_rust_owned_vector_nominal_type_identity(String::from("foreign.Message"), String::from("local.Message"), true)) == (false)), "assertion 80");
    assert!(((self::accept_rust_owned_vector_nominal_type_identity(String::from(""), String::from(""), true)) == (false)), "assertion 81");
    assert!(((self::accept_rust_owned_vector_nominal_type_identity(String::from("foreign.Message"), String::from("foreign.Message"), false)) == (false)), "assertion 82");
    assert!(((self::accept_rust_owned_box_pointer_owner_child(String::from("library"), String::from("alloc::boxed::Box"), 1f64)) == (true)), "assertion 83");
    assert!(((self::accept_rust_owned_box_pointer_owner_child(String::from("declaration"), String::from("alloc::boxed::Box"), 1f64)) == (false)), "assertion 84");
    assert!(((self::accept_rust_owned_box_pointer_owner_child(String::from("library"), String::from("alloc::boxed::Box"), 2f64)) == (false)), "assertion 85");
    assert!(((self::accept_rust_owned_box_pointer_owner_child(String::from("library"), String::from("std::boxed::Box"), 1f64)) == (false)), "assertion 86");
}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(move || { ml_main() })
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}

#[test]
fn all_literal_owned_recursive_container_controls() { main(); }
