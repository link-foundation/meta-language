//! The Rocq frontend: the surface AST it reads and the errors it reports,
//! matching `js/src/translation/rocq.js`.

use meta_language::translation::diagnostics::ErrorKind;
use meta_language::translation::rocq::parse_rocq;
use serde_json::{json, Value};

fn surface(source: &str) -> Value {
    let program = parse_rocq(source).unwrap_or_else(|error| panic!("{source}: {error}"));
    serde_json::to_value(program).expect("the surface AST serialises")
}

fn failure(source: &str) -> (ErrorKind, String) {
    let error = parse_rocq(source).expect_err(source);
    (error.kind, error.message())
}

#[test]
fn reads_a_definition_with_scoped_numerals() {
    let program = surface("Definition f (n : N) : Z := (Z.of_N n + 2)%Z.");
    assert_eq!(program["language"], json!("Rocq"));
    assert_eq!(program["main"], Value::Null);
    let item = &program["items"][0];
    assert_eq!(item["k"], json!("fn"));
    assert_eq!(item["name"], json!("f"));
    assert_eq!(item["params"][0]["rocqType"], json!("N"));
    let body = &item["body"];
    assert_eq!(body["k"], json!("binary"));
    assert_eq!(body["op"], json!("add"));
    assert_eq!(body["left"]["k"], json!("cast"));
    assert_eq!(body["left"]["flavor"], json!("exact"));
    assert_eq!(body["right"]["value"], json!("2"));
    assert_eq!(body["right"]["type"]["kind"], json!("int"));
}

#[test]
fn reads_the_program_output() {
    let program = surface(
        "Definition main : list string :=\n  let x := 1%N in\n  \"a\" :: [NilEmpty.string_of_uint (N.to_uint x)].",
    );
    let effects = &program["main"]["effects"];
    assert_eq!(effects[0]["k"], json!("let"));
    assert_eq!(effects[1]["k"], json!("print"));
    assert_eq!(effects[1]["style"], json!("rocq"));
    assert_eq!(effects[2]["expr"]["k"], json!("toString"));
    assert_eq!(effects.as_array().map(Vec::len), Some(3));
}

#[test]
fn keeps_numeral_patterns_as_decimal_text() {
    let program =
        surface("Definition f (n : nat) : nat := match n with 1_000 as m => m | _ => 0 end.");
    let row = &program["items"][0]["body"]["rows"][0];
    assert_eq!(row["patterns"][0]["k"], json!("numLit"));
    assert_eq!(row["patterns"][0]["value"], json!("1000"));
    assert_eq!(row["body"]["k"], json!("let"));
    assert_eq!(row["body"]["value"]["value"], json!("1000"));
}

#[test]
fn reads_constructor_fields_declared_as_binders() {
    let program = surface("Inductive T : Type := | leaf | node (l r : T) (v : N) : T.");
    let fields = &program["items"][0]["ctors"][1]["fields"];
    assert_eq!(fields[0]["name"], json!("l"));
    assert_eq!(fields[1]["rocqType"], json!("T"));
    assert_eq!(fields[1]["span"], json!({ "start": 36, "end": 46 }));
    assert_eq!(fields[2]["rocqType"], json!("N"));
    assert_eq!(fields[2]["type"]["kind"], json!("nat"));
    assert_eq!(fields[2]["span"], json!({ "start": 46, "end": 54 }));
}

#[test]
fn reads_a_structured_proof() {
    let program = surface(
        "Theorem t (n : nat) : n + 0 = n.\nProof.\n  induction n as [| k ih].\n  - reflexivity.\n  - simpl. rewrite ih. reflexivity.\n Qed.",
    );
    let theorem = &program["items"][0];
    assert_eq!(theorem["prop"]["p"], json!("eq"));
    let proof = &theorem["proof"];
    assert_eq!(proof["sourceLanguage"], json!("Rocq"));
    assert_eq!(
        proof["source"],
        json!("induction n as [| k ih].\n  - reflexivity.\n  - simpl. rewrite ih. reflexivity.")
    );
    let split = &proof["steps"][0];
    assert_eq!(split["t"], json!("induction"));
    assert_eq!(split["blocks"], json!(2));
    assert_eq!(split["cases"][1]["binds"], json!(["k", "ih"]));
    assert_eq!(split["cases"][1]["steps"][1]["t"], json!("rewrite"));
}

#[test]
fn reports_errors_as_the_javascript_frontend() {
    let cases = [
        ("End A.", ErrorKind::Syntax, "unmatched End but found \"End\" at 0..3"),
        ("Module A.\nEnd B.", ErrorKind::Syntax, "End B closes module A at 10..13"),
        (
            "Definition f (n : N) : N := N.add n.",
            ErrorKind::Unsupported,
            "partial application: N.add expects 2 arguments at 28..35",
        ),
        (
            "Definition f (n : N) : N := N.add n n n.",
            ErrorKind::Type,
            "N.add expects 2 arguments but got 3 at 28..39",
        ),
        (
            "Definition f (n : nat) : nat := Nat.add (1 :: nil) [n].",
            ErrorKind::Unsupported,
            "list value: lists are outside the portable core except as the program output at 41..49",
        ),
        (
            "Definition main : list string := [].\nDefinition main : list string := [].",
            ErrorKind::Type,
            "duplicate main at 37..47",
        ),
        (
            "Theorem t (n : N) : n = n.\nProof.\n  induction n.\nQed.",
            ErrorKind::Unsupported,
            "binary induction on N: N is split as zero and successor only with `using N.peano_ind` at 36..47",
        ),
        (
            "Definition f (p : nat * nat) : nat := 1.",
            ErrorKind::Unsupported,
            "product type: tuples are outside the portable core at 18..22",
        ),
        (
            "Definition f (n : nat) : nat := match n with _ as m => m end.",
            ErrorKind::Unsupported,
            "as-pattern: an aliased pattern must bind every field at 45..52",
        ),
        (
            "Definition f (n : nat) : nat := (n)%R.",
            ErrorKind::Unsupported,
            "%R scope: outside the portable core at 36..37",
        ),
        (
            "Definition f (n : nat) : nat := n",
            ErrorKind::Syntax,
            "expected . in Definition but found end of input at 33..33",
        ),
    ];
    for (source, kind, message) in cases {
        assert_eq!(failure(source), (kind, message.to_owned()), "{source}");
    }
}

#[test]
fn never_reads_a_reversed_rewrite() {
    // The lexer reads `<-` as `<` then `-`, as in the JavaScript runtime.
    let source = "Theorem t (n : nat) : n = n.\nProof.\n  rewrite <- H. reflexivity.\nQed.";
    assert_eq!(
        failure(source),
        (
            ErrorKind::Syntax,
            "expected identifier in rewrite rule but found \"<\" at 46..47".to_owned()
        )
    );
}
