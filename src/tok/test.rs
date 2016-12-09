use super::{Tok, Tokenizer};
use super::Tok::*;

// straight from LALRPOP
fn test(input: &str, expected: Vec<(&str, Tok)>) {
    // use $ to signal EOL because it can be replaced with a single space
    // for spans, and because it applies also to r#XXX# style strings:
    let input = input.replace("$", "\n");

    let tokenizer = Tokenizer::new(&input);
    let len = expected.len();
    for (token, (expected_span, expected_tok)) in tokenizer.zip(expected.into_iter()) {
        println!("token: {:?}", token);
        let expected_start = expected_span.find("~").unwrap();
        let expected_end = expected_span.rfind("~").unwrap() + 1;
        assert_eq!(Ok((expected_start, expected_tok, expected_end)), token);
    }

    let tokenizer = Tokenizer::new(&input);
    assert_eq!(None, tokenizer.skip(len).next());
}

#[test]
fn basic() {
    test("let rec x in empty", vec![
        ("~~~               ", Let),
        ("    ~~~           ", Rec),
        ("        ~         ", Ident("x")),
        ("          ~~      ", In),
        ("             ~~~~~", Empty),
    ]);
}

#[test]
fn ifstmt() {
    test("if true then 1 else 0", vec![
        ("~~                   ", If),
        ("   ~~~~              ", True),
        ("        ~~~~         ", Then),
        ("             ~       ", Num(1)),
        ("               ~~~~  ", Else),
        ("                    ~", Num(0)),
    ]);
}

#[test]
fn lte() {
    test("<=", vec![
        ("~~", Lte),
    ]);
}

#[test]
fn gte() {
    test(">=", vec![
        ("~~", Gte),
    ]);
}

#[test]
fn negative_num() {
    test("-3", vec![
        ("~~", Num(-3)),
    ]);
}

#[test]
fn lists() {
    test("fun n -> if (empty? (n :: empty)) then true else false", vec![
        ("~~~                                                   ", Fun),
        ("    ~                                                 ", Ident("n")),
        ("      ~~                                              ", RArrow),
        ("         ~~                                           ", If),
        ("            ~                                         ", LParen),
        ("             ~~~~~~                                   ", Emptyq),
        ("                    ~                                 ", LParen),
        ("                     ~                                ", Ident("n")),
        ("                       ~~                             ", Cons),
        ("                          ~~~~~                       ", Empty),
        ("                               ~                      ", RParen),
        ("                                ~                     ", RParen),
        ("                                  ~~~~                ", Then),
        ("                                       ~~~~           ", True),
        ("                                            ~~~~      ", Else),
        ("                                                 ~~~~~", False),
    ]);
}

#[test]
fn pairs() {
    test("((b) 1)", vec![
        ("~      ", LParen),
        (" ~     ", LParen),
        ("  ~    ", Ident("b")),
        ("   ~   ", RParen),
        ("     ~ ", Num(1)),
        ("      ~", RParen),
    ]);
}

#[test]
fn comment() {
    test("a(* xx *)1", vec![
        ("~         ", Ident("a")),
        ("         ~", Num(1)),
    ]);
}

#[test]
fn idents() {
    test("_3 n3_", vec![
        ("~~    ", Ident("_3")),
        ("   ~~~", Ident("n3_")),
    ]);
}

#[test]
fn block() {
    test("begin 3 ∧ 1 end", vec![
        ("~~~~~          ", Begin),
        ("      ~        ", Num(3)),
        ("        ~      ", And),
        ("            ~    ", Num(1)),
        ("              ~~~", End),
    ]);
}

#[test]
fn arrays() {
    test("array(1, 1)[2] = 3", vec![
        ("~~~~~             ", Array),
        ("     ~            ", LParen),
        ("      ~           ", Num(1)),
        ("       ~          ", Comma),
        ("         ~        ", Num(1)),
        ("          ~       ", RParen),
        ("           ~      ", LBracket),
        ("            ~     ", Num(2)),
        ("             ~    ", RBracket),
        ("               ~  ", Eq),
        ("                 ~", Num(3)),
    ]);
}
