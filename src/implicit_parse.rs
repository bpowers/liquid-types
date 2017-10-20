use common::Id;
use common::Op2::*;
use common::Const::*;
use implicit::Expr;
use implicit::Expr::*;
use tok::{self, Tok};
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use common::Id;
    use common::Op2::*;
    use common::Const::*;
    use implicit::Expr;
    use implicit::Expr::*;
    use tok::{self, Tok};
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_21_22(Tok<'input>),
        Term_22_28_22(Tok<'input>),
        Term_22_29_22(Tok<'input>),
        Term_22_2a_22(Tok<'input>),
        Term_22_2b_22(Tok<'input>),
        Term_22_2c_22(Tok<'input>),
        Term_22_2d_22(Tok<'input>),
        Term_22_2d_3e_22(Tok<'input>),
        Term_22_3a_3a_22(Tok<'input>),
        Term_22_3c_22(Tok<'input>),
        Term_22_3c_2d_22(Tok<'input>),
        Term_22_3c_3d_22(Tok<'input>),
        Term_22_3d_22(Tok<'input>),
        Term_22_3e_22(Tok<'input>),
        Term_22_3e_3d_22(Tok<'input>),
        Term_22Iden_22(&'input str),
        Term_22Num_22(i64),
        Term_22_5b_22(Tok<'input>),
        Term_22_5d_22(Tok<'input>),
        Term_22array_22(Tok<'input>),
        Term_22begin_22(Tok<'input>),
        Term_22else_22(Tok<'input>),
        Term_22empty_22(Tok<'input>),
        Term_22empty_3f_22(Tok<'input>),
        Term_22end_22(Tok<'input>),
        Term_22false_22(Tok<'input>),
        Term_22fix_22(Tok<'input>),
        Term_22fun_22(Tok<'input>),
        Term_22head_22(Tok<'input>),
        Term_22if_22(Tok<'input>),
        Term_22in_22(Tok<'input>),
        Term_22let_22(Tok<'input>),
        Term_22rec_22(Tok<'input>),
        Term_22set_22(Tok<'input>),
        Term_22tail_22(Tok<'input>),
        Term_22then_22(Tok<'input>),
        Term_22true_22(Tok<'input>),
        Term_22_3bd_22(Tok<'input>),
        Term_22_2227_22(Tok<'input>),
        Term_22_2228_22(Tok<'input>),
        Term_22_2605_22(Tok<'input>),
        NtACmp(Box<Expr>),
        NtAdd(Box<Expr>),
        NtApp(Box<Expr>),
        NtAtom(Box<Expr>),
        NtExpr(Box<Expr>),
        NtExprs(Box<Expr>),
        NtGet(Box<Expr>),
        NtIdent(Id),
        NtInt(Box<Expr>),
        NtLCmp(Box<Expr>),
        NtMul(Box<Expr>),
        NtNum(i64),
        NtProgram(Box<Expr>),
        Nt____Program(Box<Expr>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "begin" Expr "end" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fix" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fun" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" "rec" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∧" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∨" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Program = (*) Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     __Program = (*) Program ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        19,  // on "begin", goto 18
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        21,  // on "fix", goto 20
        22,  // on "fun", goto 21
        0,  // on "head", error
        23,  // on "if", goto 22
        0,  // on "in", error
        24,  // on "let", goto 23
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 1
        //     LCmp = ACmp (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = ACmp (*) "∧" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = ACmp (*) "∨" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        -36,  // on ")", reduce `LCmp = ACmp => ActionFn(11);`
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        -36,  // on "else", reduce `LCmp = ACmp => ActionFn(11);`
        0,  // on "empty", error
        0,  // on "empty?", error
        -36,  // on "end", reduce `LCmp = ACmp => ActionFn(11);`
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        -36,  // on "in", reduce `LCmp = ACmp => ActionFn(11);`
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        -36,  // on "then", reduce `LCmp = ACmp => ActionFn(11);`
        0,  // on "true", error
        0,  // on "ν", error
        28,  // on "∧", goto 27
        29,  // on "∨", goto 28
        0,  // on "★", error

        // State 2
        //     ACmp = Add (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = Add (*) "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = Add (*) "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = Add (*) "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = Add (*) ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = Add (*) ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = Add (*) "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = Add (*) "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        -6,  // on ")", reduce `ACmp = Add => ActionFn(17);`
        0,  // on "*", error
        30,  // on "+", goto 29
        0,  // on ",", error
        31,  // on "-", goto 30
        0,  // on "->", error
        0,  // on "::", error
        32,  // on "<", goto 31
        0,  // on "<-", error
        33,  // on "<=", goto 32
        34,  // on "=", goto 33
        35,  // on ">", goto 34
        36,  // on ">=", goto 35
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        -6,  // on "else", reduce `ACmp = Add => ActionFn(17);`
        0,  // on "empty", error
        0,  // on "empty?", error
        -6,  // on "end", reduce `ACmp = Add => ActionFn(17);`
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        -6,  // on "in", reduce `ACmp = Add => ActionFn(17);`
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        -6,  // on "then", reduce `ACmp = Add => ActionFn(17);`
        0,  // on "true", error
        0,  // on "ν", error
        -6,  // on "∧", reduce `ACmp = Add => ActionFn(17);`
        -6,  // on "∨", reduce `ACmp = Add => ActionFn(17);`
        0,  // on "★", error

        // State 3
        //     App = App (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = App (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        15,  // on "(", goto 14
        -31,  // on ")", reduce `Get = App => ActionFn(25);`
        -31,  // on "*", reduce `Get = App => ActionFn(25);`
        -31,  // on "+", reduce `Get = App => ActionFn(25);`
        -31,  // on ",", reduce `Get = App => ActionFn(25);`
        -31,  // on "-", reduce `Get = App => ActionFn(25);`
        0,  // on "->", error
        0,  // on "::", error
        -31,  // on "<", reduce `Get = App => ActionFn(25);`
        0,  // on "<-", error
        -31,  // on "<=", reduce `Get = App => ActionFn(25);`
        -31,  // on "=", reduce `Get = App => ActionFn(25);`
        -31,  // on ">", reduce `Get = App => ActionFn(25);`
        -31,  // on ">=", reduce `Get = App => ActionFn(25);`
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        -31,  // on "]", reduce `Get = App => ActionFn(25);`
        0,  // on "array", error
        0,  // on "begin", error
        -31,  // on "else", reduce `Get = App => ActionFn(25);`
        0,  // on "empty", error
        0,  // on "empty?", error
        -31,  // on "end", reduce `Get = App => ActionFn(25);`
        20,  // on "false", goto 19
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        -31,  // on "in", reduce `Get = App => ActionFn(25);`
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        -31,  // on "then", reduce `Get = App => ActionFn(25);`
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        -31,  // on "∧", reduce `Get = App => ActionFn(25);`
        -31,  // on "∨", reduce `Get = App => ActionFn(25);`
        27,  // on "★", goto 26

        // State 4
        //     App = Atom (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -13,  // on "!", reduce `App = Atom => ActionFn(29);`
        -13,  // on "(", reduce `App = Atom => ActionFn(29);`
        -13,  // on ")", reduce `App = Atom => ActionFn(29);`
        -13,  // on "*", reduce `App = Atom => ActionFn(29);`
        -13,  // on "+", reduce `App = Atom => ActionFn(29);`
        -13,  // on ",", reduce `App = Atom => ActionFn(29);`
        -13,  // on "-", reduce `App = Atom => ActionFn(29);`
        -13,  // on "->", reduce `App = Atom => ActionFn(29);`
        -13,  // on "::", reduce `App = Atom => ActionFn(29);`
        -13,  // on "<", reduce `App = Atom => ActionFn(29);`
        -13,  // on "<-", reduce `App = Atom => ActionFn(29);`
        -13,  // on "<=", reduce `App = Atom => ActionFn(29);`
        -13,  // on "=", reduce `App = Atom => ActionFn(29);`
        -13,  // on ">", reduce `App = Atom => ActionFn(29);`
        -13,  // on ">=", reduce `App = Atom => ActionFn(29);`
        -13,  // on "Iden", reduce `App = Atom => ActionFn(29);`
        -13,  // on "Num", reduce `App = Atom => ActionFn(29);`
        -13,  // on "[", reduce `App = Atom => ActionFn(29);`
        -13,  // on "]", reduce `App = Atom => ActionFn(29);`
        -13,  // on "array", reduce `App = Atom => ActionFn(29);`
        -13,  // on "begin", reduce `App = Atom => ActionFn(29);`
        -13,  // on "else", reduce `App = Atom => ActionFn(29);`
        -13,  // on "empty", reduce `App = Atom => ActionFn(29);`
        -13,  // on "empty?", reduce `App = Atom => ActionFn(29);`
        -13,  // on "end", reduce `App = Atom => ActionFn(29);`
        -13,  // on "false", reduce `App = Atom => ActionFn(29);`
        -13,  // on "fix", reduce `App = Atom => ActionFn(29);`
        -13,  // on "fun", reduce `App = Atom => ActionFn(29);`
        -13,  // on "head", reduce `App = Atom => ActionFn(29);`
        -13,  // on "if", reduce `App = Atom => ActionFn(29);`
        -13,  // on "in", reduce `App = Atom => ActionFn(29);`
        -13,  // on "let", reduce `App = Atom => ActionFn(29);`
        -13,  // on "rec", reduce `App = Atom => ActionFn(29);`
        -13,  // on "set", reduce `App = Atom => ActionFn(29);`
        -13,  // on "tail", reduce `App = Atom => ActionFn(29);`
        -13,  // on "then", reduce `App = Atom => ActionFn(29);`
        -13,  // on "true", reduce `App = Atom => ActionFn(29);`
        -13,  // on "ν", reduce `App = Atom => ActionFn(29);`
        -13,  // on "∧", reduce `App = Atom => ActionFn(29);`
        -13,  // on "∨", reduce `App = Atom => ActionFn(29);`
        -13,  // on "★", reduce `App = Atom => ActionFn(29);`

        // State 5
        //     Program = Expr (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -40,  // on "!", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "(", reduce `Program = Expr => ActionFn(1);`
        -40,  // on ")", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "*", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "+", reduce `Program = Expr => ActionFn(1);`
        -40,  // on ",", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "-", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "->", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "::", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "<", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "<-", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "<=", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "=", reduce `Program = Expr => ActionFn(1);`
        -40,  // on ">", reduce `Program = Expr => ActionFn(1);`
        -40,  // on ">=", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "Iden", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "Num", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "[", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "]", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "array", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "begin", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "else", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "empty", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "empty?", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "end", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "false", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "fix", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "fun", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "head", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "if", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "in", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "let", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "rec", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "set", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "tail", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "then", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "true", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "ν", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "∧", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "∨", reduce `Program = Expr => ActionFn(1);`
        -40,  // on "★", reduce `Program = Expr => ActionFn(1);`

        // State 6
        //     Mul = Get (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -38,  // on "!", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "(", reduce `Mul = Get => ActionFn(22);`
        -38,  // on ")", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "*", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "+", reduce `Mul = Get => ActionFn(22);`
        -38,  // on ",", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "-", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "->", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "::", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "<", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "<-", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "<=", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "=", reduce `Mul = Get => ActionFn(22);`
        -38,  // on ">", reduce `Mul = Get => ActionFn(22);`
        -38,  // on ">=", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "Iden", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "Num", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "[", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "]", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "array", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "begin", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "else", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "empty", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "empty?", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "end", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "false", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "fix", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "fun", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "head", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "if", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "in", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "let", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "rec", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "set", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "tail", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "then", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "true", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "ν", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "∧", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "∨", reduce `Mul = Get => ActionFn(22);`
        -38,  // on "★", reduce `Mul = Get => ActionFn(22);`

        // State 7
        //     Atom = Ident (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = Ident (*) "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = Ident (*) "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        -14,  // on "(", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on ")", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "*", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "+", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on ",", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "-", reduce `Atom = Ident => ActionFn(30);`
        0,  // on "->", error
        0,  // on "::", error
        -14,  // on "<", reduce `Atom = Ident => ActionFn(30);`
        0,  // on "<-", error
        -14,  // on "<=", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "=", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on ">", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on ">=", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "Iden", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "Num", reduce `Atom = Ident => ActionFn(30);`
        39,  // on "[", goto 38
        -14,  // on "]", reduce `Atom = Ident => ActionFn(30);`
        0,  // on "array", error
        0,  // on "begin", error
        -14,  // on "else", reduce `Atom = Ident => ActionFn(30);`
        0,  // on "empty", error
        0,  // on "empty?", error
        -14,  // on "end", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "false", reduce `Atom = Ident => ActionFn(30);`
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        -14,  // on "in", reduce `Atom = Ident => ActionFn(30);`
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        -14,  // on "then", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "true", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "ν", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "∧", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "∨", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "★", reduce `Atom = Ident => ActionFn(30);`

        // State 8
        //     Atom = Int (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -20,  // on "!", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "(", reduce `Atom = Int => ActionFn(36);`
        -20,  // on ")", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "*", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "+", reduce `Atom = Int => ActionFn(36);`
        -20,  // on ",", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "-", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "->", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "::", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "<", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "<-", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "<=", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "=", reduce `Atom = Int => ActionFn(36);`
        -20,  // on ">", reduce `Atom = Int => ActionFn(36);`
        -20,  // on ">=", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "Iden", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "Num", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "[", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "]", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "array", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "begin", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "else", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "empty", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "empty?", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "end", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "false", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "fix", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "fun", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "head", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "if", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "in", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "let", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "rec", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "set", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "tail", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "then", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "true", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "ν", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "∧", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "∨", reduce `Atom = Int => ActionFn(36);`
        -20,  // on "★", reduce `Atom = Int => ActionFn(36);`

        // State 9
        //     Expr = LCmp (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -27,  // on "!", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "(", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on ")", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "*", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "+", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on ",", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "-", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "->", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "::", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "<", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "<-", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "<=", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "=", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on ">", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on ">=", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "Iden", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "Num", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "[", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "]", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "array", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "begin", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "else", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "empty", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "empty?", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "end", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "false", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "fix", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "fun", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "head", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "if", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "in", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "let", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "rec", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "set", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "tail", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "then", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "true", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "ν", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "∧", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "∨", reduce `Expr = LCmp => ActionFn(8);`
        -27,  // on "★", reduce `Expr = LCmp => ActionFn(8);`

        // State 10
        //     Add = Mul (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = Mul (*) "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        -9,  // on ")", reduce `Add = Mul => ActionFn(20);`
        40,  // on "*", goto 39
        -9,  // on "+", reduce `Add = Mul => ActionFn(20);`
        -9,  // on ",", reduce `Add = Mul => ActionFn(20);`
        -9,  // on "-", reduce `Add = Mul => ActionFn(20);`
        0,  // on "->", error
        0,  // on "::", error
        -9,  // on "<", reduce `Add = Mul => ActionFn(20);`
        0,  // on "<-", error
        -9,  // on "<=", reduce `Add = Mul => ActionFn(20);`
        -9,  // on "=", reduce `Add = Mul => ActionFn(20);`
        -9,  // on ">", reduce `Add = Mul => ActionFn(20);`
        -9,  // on ">=", reduce `Add = Mul => ActionFn(20);`
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        -9,  // on "]", reduce `Add = Mul => ActionFn(20);`
        0,  // on "array", error
        0,  // on "begin", error
        -9,  // on "else", reduce `Add = Mul => ActionFn(20);`
        0,  // on "empty", error
        0,  // on "empty?", error
        -9,  // on "end", reduce `Add = Mul => ActionFn(20);`
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        -9,  // on "in", reduce `Add = Mul => ActionFn(20);`
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        -9,  // on "then", reduce `Add = Mul => ActionFn(20);`
        0,  // on "true", error
        0,  // on "ν", error
        -9,  // on "∧", reduce `Add = Mul => ActionFn(20);`
        -9,  // on "∨", reduce `Add = Mul => ActionFn(20);`
        0,  // on "★", error

        // State 11
        //     Int = Num (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -33,  // on "!", reduce `Int = Num => ActionFn(38);`
        -33,  // on "(", reduce `Int = Num => ActionFn(38);`
        -33,  // on ")", reduce `Int = Num => ActionFn(38);`
        -33,  // on "*", reduce `Int = Num => ActionFn(38);`
        -33,  // on "+", reduce `Int = Num => ActionFn(38);`
        -33,  // on ",", reduce `Int = Num => ActionFn(38);`
        -33,  // on "-", reduce `Int = Num => ActionFn(38);`
        -33,  // on "->", reduce `Int = Num => ActionFn(38);`
        -33,  // on "::", reduce `Int = Num => ActionFn(38);`
        -33,  // on "<", reduce `Int = Num => ActionFn(38);`
        -33,  // on "<-", reduce `Int = Num => ActionFn(38);`
        -33,  // on "<=", reduce `Int = Num => ActionFn(38);`
        -33,  // on "=", reduce `Int = Num => ActionFn(38);`
        -33,  // on ">", reduce `Int = Num => ActionFn(38);`
        -33,  // on ">=", reduce `Int = Num => ActionFn(38);`
        -33,  // on "Iden", reduce `Int = Num => ActionFn(38);`
        -33,  // on "Num", reduce `Int = Num => ActionFn(38);`
        -33,  // on "[", reduce `Int = Num => ActionFn(38);`
        -33,  // on "]", reduce `Int = Num => ActionFn(38);`
        -33,  // on "array", reduce `Int = Num => ActionFn(38);`
        -33,  // on "begin", reduce `Int = Num => ActionFn(38);`
        -33,  // on "else", reduce `Int = Num => ActionFn(38);`
        -33,  // on "empty", reduce `Int = Num => ActionFn(38);`
        -33,  // on "empty?", reduce `Int = Num => ActionFn(38);`
        -33,  // on "end", reduce `Int = Num => ActionFn(38);`
        -33,  // on "false", reduce `Int = Num => ActionFn(38);`
        -33,  // on "fix", reduce `Int = Num => ActionFn(38);`
        -33,  // on "fun", reduce `Int = Num => ActionFn(38);`
        -33,  // on "head", reduce `Int = Num => ActionFn(38);`
        -33,  // on "if", reduce `Int = Num => ActionFn(38);`
        -33,  // on "in", reduce `Int = Num => ActionFn(38);`
        -33,  // on "let", reduce `Int = Num => ActionFn(38);`
        -33,  // on "rec", reduce `Int = Num => ActionFn(38);`
        -33,  // on "set", reduce `Int = Num => ActionFn(38);`
        -33,  // on "tail", reduce `Int = Num => ActionFn(38);`
        -33,  // on "then", reduce `Int = Num => ActionFn(38);`
        -33,  // on "true", reduce `Int = Num => ActionFn(38);`
        -33,  // on "ν", reduce `Int = Num => ActionFn(38);`
        -33,  // on "∧", reduce `Int = Num => ActionFn(38);`
        -33,  // on "∨", reduce `Int = Num => ActionFn(38);`
        -33,  // on "★", reduce `Int = Num => ActionFn(38);`

        // State 12
        //     __Program = Program (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -41,  // on "!", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "(", reduce `__Program = Program => ActionFn(0);`
        -41,  // on ")", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "*", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "+", reduce `__Program = Program => ActionFn(0);`
        -41,  // on ",", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "-", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "->", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "::", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "<", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "<-", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "<=", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "=", reduce `__Program = Program => ActionFn(0);`
        -41,  // on ">", reduce `__Program = Program => ActionFn(0);`
        -41,  // on ">=", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "Iden", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "Num", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "[", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "]", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "array", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "begin", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "else", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "empty", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "empty?", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "end", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "false", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "fix", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "fun", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "head", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "if", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "in", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "let", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "rec", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "set", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "tail", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "then", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "true", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "ν", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "∧", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "∨", reduce `__Program = Program => ActionFn(0);`
        -41,  // on "★", reduce `__Program = Program => ActionFn(0);`

        // State 13
        //     App = "!" (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 14
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = "(" (*) Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "begin" Expr "end" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fix" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fun" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" "rec" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Exprs = (*) Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∧" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∨" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        19,  // on "begin", goto 18
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        21,  // on "fix", goto 20
        22,  // on "fun", goto 21
        0,  // on "head", error
        23,  // on "if", goto 22
        0,  // on "in", error
        24,  // on "let", goto 23
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 15
        //     Ident = "Iden" (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -32,  // on "!", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "(", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on ")", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "*", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "+", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on ",", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "-", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "->", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "::", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "<", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "<-", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "<=", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "=", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on ">", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on ">=", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "Iden", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "Num", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "[", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "]", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "array", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "begin", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "else", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "empty", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "empty?", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "end", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "false", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "fix", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "fun", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "head", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "if", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "in", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "let", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "rec", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "set", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "tail", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "then", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "true", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "ν", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "∧", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "∨", reduce `Ident = "Iden" => ActionFn(39);`
        -32,  // on "★", reduce `Ident = "Iden" => ActionFn(39);`

        // State 16
        //     Num = "Num" (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -39,  // on "!", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "(", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on ")", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "*", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "+", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on ",", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "-", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "->", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "::", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "<", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "<-", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "<=", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "=", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on ">", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on ">=", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "Iden", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "Num", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "[", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "]", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "array", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "begin", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "else", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "empty", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "empty?", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "end", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "false", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "fix", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "fun", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "head", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "if", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "in", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "let", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "rec", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "set", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "tail", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "then", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "true", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "ν", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "∧", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "∨", reduce `Num = "Num" => ActionFn(40);`
        -39,  // on "★", reduce `Num = "Num" => ActionFn(40);`

        // State 17
        //     App = "array" (*) "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        44,  // on "(", goto 43
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 18
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "begin" Expr "end" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = "begin" (*) Expr "end" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fix" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fun" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" "rec" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∧" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∨" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        19,  // on "begin", goto 18
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        21,  // on "fix", goto 20
        22,  // on "fun", goto 21
        0,  // on "head", error
        23,  // on "if", goto 22
        0,  // on "in", error
        24,  // on "let", goto 23
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 19
        //     Atom = "false" (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -18,  // on "!", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "(", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on ")", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "*", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "+", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on ",", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "-", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "->", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "::", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "<", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "<-", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "<=", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "=", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on ">", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on ">=", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "Iden", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "Num", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "[", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "]", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "array", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "begin", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "else", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "empty", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "empty?", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "end", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "false", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "fix", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "fun", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "head", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "if", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "in", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "let", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "rec", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "set", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "tail", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "then", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "true", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "ν", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "∧", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "∨", reduce `Atom = "false" => ActionFn(34);`
        -18,  // on "★", reduce `Atom = "false" => ActionFn(34);`

        // State 20
        //     Expr = "fix" (*) Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 21
        //     Expr = "fun" (*) Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 22
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "begin" Expr "end" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fix" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fun" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = "if" (*) Expr "then" Expr "else" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" "rec" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∧" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∨" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        19,  // on "begin", goto 18
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        21,  // on "fix", goto 20
        22,  // on "fun", goto 21
        0,  // on "head", error
        23,  // on "if", goto 22
        0,  // on "in", error
        24,  // on "let", goto 23
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 23
        //     Expr = "let" (*) Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = "let" (*) "rec" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        50,  // on "rec", goto 49
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 24
        //     Atom = "true" (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -17,  // on "!", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "(", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on ")", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "*", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "+", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on ",", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "-", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "->", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "::", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "<", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "<-", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "<=", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "=", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on ">", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on ">=", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "Iden", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "Num", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "[", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "]", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "array", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "begin", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "else", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "empty", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "empty?", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "end", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "false", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "fix", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "fun", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "head", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "if", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "in", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "let", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "rec", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "set", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "tail", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "then", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "true", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "ν", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "∧", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "∨", reduce `Atom = "true" => ActionFn(33);`
        -17,  // on "★", reduce `Atom = "true" => ActionFn(33);`

        // State 25
        //     Atom = "ν" (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -15,  // on "!", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "(", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on ")", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "*", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "+", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on ",", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "-", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "->", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "::", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "<", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "<-", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "<=", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "=", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on ">", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on ">=", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "Iden", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "Num", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "[", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "]", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "array", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "begin", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "else", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "empty", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "empty?", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "end", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "false", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "fix", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "fun", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "head", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "if", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "in", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "let", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "rec", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "set", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "tail", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "then", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "true", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "ν", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "∧", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "∨", reduce `Atom = "ν" => ActionFn(31);`
        -15,  // on "★", reduce `Atom = "ν" => ActionFn(31);`

        // State 26
        //     Atom = "★" (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -16,  // on "!", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "(", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on ")", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "*", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "+", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on ",", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "-", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "->", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "::", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "<", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "<-", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "<=", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "=", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on ">", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on ">=", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "Iden", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "Num", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "[", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "]", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "array", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "begin", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "else", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "empty", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "empty?", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "end", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "false", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "fix", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "fun", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "head", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "if", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "in", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "let", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "rec", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "set", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "tail", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "then", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "true", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "ν", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "∧", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "∨", reduce `Atom = "★" => ActionFn(32);`
        -16,  // on "★", reduce `Atom = "★" => ActionFn(32);`

        // State 27
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∧" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = ACmp "∧" (*) LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∨" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 28
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∧" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∨" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = ACmp "∨" (*) LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 29
        //     Add = Add "+" (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 30
        //     Add = Add "-" (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 31
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = Add "<" (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 32
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = Add "<=" (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 33
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = Add "=" (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 34
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = Add ">" (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 35
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = Add ">=" (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 36
        //     App = App Atom (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -12,  // on "!", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "(", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on ")", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "*", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "+", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on ",", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "-", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "->", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "::", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "<", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "<-", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "<=", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "=", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on ">", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on ">=", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "Iden", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "Num", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "[", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "]", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "array", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "begin", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "else", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "empty", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "empty?", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "end", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "false", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "fix", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "fun", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "head", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "if", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "in", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "let", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "rec", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "set", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "tail", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "then", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "true", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "ν", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "∧", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "∨", reduce `App = App, Atom => ActionFn(28);`
        -12,  // on "★", reduce `App = App, Atom => ActionFn(28);`

        // State 37
        //     Atom = Ident (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -14,  // on "!", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "(", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on ")", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "*", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "+", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on ",", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "-", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "->", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "::", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "<", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "<-", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "<=", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "=", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on ">", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on ">=", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "Iden", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "Num", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "[", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "]", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "array", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "begin", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "else", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "empty", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "empty?", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "end", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "false", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "fix", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "fun", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "head", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "if", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "in", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "let", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "rec", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "set", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "tail", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "then", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "true", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "ν", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "∧", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "∨", reduce `Atom = Ident => ActionFn(30);`
        -14,  // on "★", reduce `Atom = Ident => ActionFn(30);`

        // State 38
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = Ident "[" (*) Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = Ident "[" (*) Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 39
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = Mul "*" (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 40
        //     App = "!" Atom (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -11,  // on "!", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "(", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on ")", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "*", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "+", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on ",", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "-", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "->", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "::", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "<", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "<-", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "<=", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "=", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on ">", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on ">=", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "Iden", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "Num", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "[", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "]", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "array", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "begin", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "else", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "empty", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "empty?", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "end", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "false", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "fix", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "fun", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "head", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "if", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "in", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "let", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "rec", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "set", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "tail", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "then", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "true", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "ν", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "∧", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "∨", reduce `App = "!", Atom => ActionFn(27);`
        -11,  // on "★", reduce `App = "!", Atom => ActionFn(27);`

        // State 41
        //     Exprs = Expr (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -28,  // on "!", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "(", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on ")", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "*", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "+", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on ",", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "-", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "->", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "::", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "<", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "<-", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "<=", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "=", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on ">", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on ">=", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "Iden", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "Num", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "[", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "]", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "array", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "begin", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "else", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "empty", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "empty?", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "end", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "false", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "fix", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "fun", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "head", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "if", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "in", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "let", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "rec", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "set", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "tail", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "then", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "true", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "ν", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "∧", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "∨", reduce `Exprs = Expr => ActionFn(37);`
        -28,  // on "★", reduce `Exprs = Expr => ActionFn(37);`

        // State 42
        //     Atom = "(" Exprs (*) ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        62,  // on ")", goto 61
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 43
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = "array" "(" (*) Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 44
        //     Expr = "begin" Expr (*) "end" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        64,  // on "end", goto 63
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 45
        //     Expr = "fix" Ident (*) "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        65,  // on "->", goto 64
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 46
        //     Expr = "fun" Ident (*) "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        66,  // on "->", goto 65
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 47
        //     Expr = "if" Expr (*) "then" Expr "else" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        67,  // on "then", goto 66
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 48
        //     Expr = "let" Ident (*) "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        68,  // on "=", goto 67
        0,  // on ">", error
        0,  // on ">=", error
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 49
        //     Expr = "let" "rec" (*) Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 50
        //     LCmp = ACmp "∧" LCmp (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -34,  // on "!", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "(", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on ")", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "*", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "+", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on ",", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "-", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "->", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "::", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "<", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "<-", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "<=", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "=", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on ">", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on ">=", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "Iden", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "Num", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "[", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "]", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "array", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "begin", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "else", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "empty", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "empty?", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "end", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "false", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "fix", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "fun", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "head", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "if", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "in", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "let", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "rec", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "set", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "tail", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "then", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "true", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "ν", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "∧", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "∨", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`
        -34,  // on "★", reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`

        // State 51
        //     LCmp = ACmp "∨" LCmp (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -35,  // on "!", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "(", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on ")", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "*", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "+", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on ",", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "-", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "->", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "::", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "<", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "<-", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "<=", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "=", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on ">", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on ">=", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "Iden", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "Num", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "[", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "]", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "array", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "begin", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "else", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "empty", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "empty?", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "end", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "false", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "fix", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "fun", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "head", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "if", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "in", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "let", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "rec", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "set", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "tail", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "then", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "true", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "ν", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "∧", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "∨", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`
        -35,  // on "★", reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`

        // State 52
        //     Add = Add "+" Mul (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = Mul (*) "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        -8,  // on ")", reduce `Add = Add, "+", Mul => ActionFn(19);`
        40,  // on "*", goto 39
        -8,  // on "+", reduce `Add = Add, "+", Mul => ActionFn(19);`
        -8,  // on ",", reduce `Add = Add, "+", Mul => ActionFn(19);`
        -8,  // on "-", reduce `Add = Add, "+", Mul => ActionFn(19);`
        0,  // on "->", error
        0,  // on "::", error
        -8,  // on "<", reduce `Add = Add, "+", Mul => ActionFn(19);`
        0,  // on "<-", error
        -8,  // on "<=", reduce `Add = Add, "+", Mul => ActionFn(19);`
        -8,  // on "=", reduce `Add = Add, "+", Mul => ActionFn(19);`
        -8,  // on ">", reduce `Add = Add, "+", Mul => ActionFn(19);`
        -8,  // on ">=", reduce `Add = Add, "+", Mul => ActionFn(19);`
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        -8,  // on "]", reduce `Add = Add, "+", Mul => ActionFn(19);`
        0,  // on "array", error
        0,  // on "begin", error
        -8,  // on "else", reduce `Add = Add, "+", Mul => ActionFn(19);`
        0,  // on "empty", error
        0,  // on "empty?", error
        -8,  // on "end", reduce `Add = Add, "+", Mul => ActionFn(19);`
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        -8,  // on "in", reduce `Add = Add, "+", Mul => ActionFn(19);`
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        -8,  // on "then", reduce `Add = Add, "+", Mul => ActionFn(19);`
        0,  // on "true", error
        0,  // on "ν", error
        -8,  // on "∧", reduce `Add = Add, "+", Mul => ActionFn(19);`
        -8,  // on "∨", reduce `Add = Add, "+", Mul => ActionFn(19);`
        0,  // on "★", error

        // State 53
        //     Add = Add "-" Mul (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = Mul (*) "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        -7,  // on ")", reduce `Add = Add, "-", Mul => ActionFn(18);`
        40,  // on "*", goto 39
        -7,  // on "+", reduce `Add = Add, "-", Mul => ActionFn(18);`
        -7,  // on ",", reduce `Add = Add, "-", Mul => ActionFn(18);`
        -7,  // on "-", reduce `Add = Add, "-", Mul => ActionFn(18);`
        0,  // on "->", error
        0,  // on "::", error
        -7,  // on "<", reduce `Add = Add, "-", Mul => ActionFn(18);`
        0,  // on "<-", error
        -7,  // on "<=", reduce `Add = Add, "-", Mul => ActionFn(18);`
        -7,  // on "=", reduce `Add = Add, "-", Mul => ActionFn(18);`
        -7,  // on ">", reduce `Add = Add, "-", Mul => ActionFn(18);`
        -7,  // on ">=", reduce `Add = Add, "-", Mul => ActionFn(18);`
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        -7,  // on "]", reduce `Add = Add, "-", Mul => ActionFn(18);`
        0,  // on "array", error
        0,  // on "begin", error
        -7,  // on "else", reduce `Add = Add, "-", Mul => ActionFn(18);`
        0,  // on "empty", error
        0,  // on "empty?", error
        -7,  // on "end", reduce `Add = Add, "-", Mul => ActionFn(18);`
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        -7,  // on "in", reduce `Add = Add, "-", Mul => ActionFn(18);`
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        -7,  // on "then", reduce `Add = Add, "-", Mul => ActionFn(18);`
        0,  // on "true", error
        0,  // on "ν", error
        -7,  // on "∧", reduce `Add = Add, "-", Mul => ActionFn(18);`
        -7,  // on "∨", reduce `Add = Add, "-", Mul => ActionFn(18);`
        0,  // on "★", error

        // State 54
        //     ACmp = Add "<" ACmp (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -2,  // on "!", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "(", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on ")", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "*", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "+", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on ",", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "-", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "->", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "::", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "<", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "<-", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "<=", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "=", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on ">", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on ">=", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "Iden", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "Num", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "[", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "]", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "array", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "begin", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "else", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "empty", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "empty?", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "end", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "false", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "fix", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "fun", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "head", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "if", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "in", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "let", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "rec", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "set", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "tail", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "then", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "true", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "ν", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "∧", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "∨", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`
        -2,  // on "★", reduce `ACmp = Add, "<", ACmp => ActionFn(13);`

        // State 55
        //     ACmp = Add "<=" ACmp (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -3,  // on "!", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "(", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on ")", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "*", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "+", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on ",", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "-", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "->", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "::", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "<", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "<-", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "<=", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "=", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on ">", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on ">=", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "Iden", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "Num", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "[", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "]", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "array", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "begin", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "else", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "empty", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "empty?", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "end", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "false", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "fix", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "fun", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "head", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "if", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "in", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "let", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "rec", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "set", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "tail", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "then", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "true", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "ν", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "∧", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "∨", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`
        -3,  // on "★", reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`

        // State 56
        //     ACmp = Add "=" ACmp (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -1,  // on "!", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "(", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on ")", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "*", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "+", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on ",", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "-", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "->", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "::", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "<", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "<-", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "<=", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "=", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on ">", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on ">=", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "Iden", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "Num", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "[", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "]", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "array", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "begin", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "else", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "empty", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "empty?", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "end", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "false", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "fix", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "fun", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "head", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "if", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "in", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "let", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "rec", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "set", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "tail", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "then", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "true", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "ν", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "∧", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "∨", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`
        -1,  // on "★", reduce `ACmp = Add, "=", ACmp => ActionFn(12);`

        // State 57
        //     ACmp = Add ">" ACmp (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -4,  // on "!", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "(", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on ")", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "*", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "+", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on ",", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "-", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "->", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "::", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "<", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "<-", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "<=", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "=", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on ">", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on ">=", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "Iden", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "Num", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "[", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "]", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "array", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "begin", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "else", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "empty", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "empty?", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "end", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "false", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "fix", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "fun", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "head", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "if", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "in", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "let", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "rec", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "set", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "tail", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "then", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "true", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "ν", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "∧", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "∨", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`
        -4,  // on "★", reduce `ACmp = Add, ">", ACmp => ActionFn(15);`

        // State 58
        //     ACmp = Add ">=" ACmp (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -5,  // on "!", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "(", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on ")", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "*", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "+", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on ",", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "-", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "->", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "::", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "<", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "<-", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "<=", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "=", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on ">", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on ">=", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "Iden", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "Num", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "[", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "]", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "array", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "begin", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "else", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "empty", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "empty?", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "end", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "false", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "fix", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "fun", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "head", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "if", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "in", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "let", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "rec", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "set", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "tail", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "then", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "true", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "ν", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "∧", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "∨", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`
        -5,  // on "★", reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`

        // State 59
        //     Add = Add (*) "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = Add (*) "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = Ident "[" Add (*) "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = Ident "[" Add (*) "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        0,  // on ")", error
        0,  // on "*", error
        30,  // on "+", goto 29
        0,  // on ",", error
        31,  // on "-", goto 30
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        70,  // on "]", goto 69
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 60
        //     Mul = Mul "*" Get (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -37,  // on "!", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "(", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on ")", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "*", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "+", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on ",", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "-", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "->", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "::", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "<", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "<-", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "<=", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "=", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on ">", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on ">=", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "Iden", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "Num", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "[", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "]", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "array", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "begin", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "else", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "empty", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "empty?", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "end", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "false", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "fix", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "fun", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "head", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "if", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "in", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "let", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "rec", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "set", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "tail", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "then", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "true", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "ν", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "∧", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "∨", reduce `Mul = Mul, "*", Get => ActionFn(21);`
        -37,  // on "★", reduce `Mul = Mul, "*", Get => ActionFn(21);`

        // State 61
        //     Atom = "(" Exprs ")" (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -19,  // on "!", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "(", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on ")", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "*", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "+", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on ",", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "-", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "->", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "::", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "<", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "<-", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "<=", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "=", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on ">", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on ">=", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "Iden", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "Num", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "[", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "]", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "array", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "begin", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "else", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "empty", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "empty?", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "end", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "false", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "fix", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "fun", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "head", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "if", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "in", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "let", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "rec", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "set", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "tail", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "then", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "true", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "ν", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "∧", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "∨", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`
        -19,  // on "★", reduce `Atom = "(", Exprs, ")" => ActionFn(35);`

        // State 62
        //     Add = Add (*) "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = Add (*) "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = "array" "(" Add (*) "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        0,  // on ")", error
        0,  // on "*", error
        30,  // on "+", goto 29
        71,  // on ",", goto 70
        31,  // on "-", goto 30
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 63
        //     Expr = "begin" Expr "end" (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -26,  // on "!", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "(", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on ")", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "*", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "+", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on ",", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "-", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "->", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "::", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "<", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "<-", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "<=", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "=", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on ">", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on ">=", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "Iden", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "Num", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "[", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "]", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "array", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "begin", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "else", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "empty", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "empty?", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "end", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "false", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "fix", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "fun", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "head", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "if", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "in", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "let", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "rec", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "set", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "tail", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "then", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "true", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "ν", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "∧", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "∨", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`
        -26,  // on "★", reduce `Expr = "begin", Expr, "end" => ActionFn(7);`

        // State 64
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "begin" Expr "end" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fix" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = "fix" Ident "->" (*) Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fun" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" "rec" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∧" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∨" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        19,  // on "begin", goto 18
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        21,  // on "fix", goto 20
        22,  // on "fun", goto 21
        0,  // on "head", error
        23,  // on "if", goto 22
        0,  // on "in", error
        24,  // on "let", goto 23
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 65
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "begin" Expr "end" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fix" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fun" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = "fun" Ident "->" (*) Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" "rec" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∧" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∨" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        19,  // on "begin", goto 18
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        21,  // on "fix", goto 20
        22,  // on "fun", goto 21
        0,  // on "head", error
        23,  // on "if", goto 22
        0,  // on "in", error
        24,  // on "let", goto 23
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 66
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "begin" Expr "end" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fix" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fun" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = "if" Expr "then" (*) Expr "else" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" "rec" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∧" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∨" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        19,  // on "begin", goto 18
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        21,  // on "fix", goto 20
        22,  // on "fun", goto 21
        0,  // on "head", error
        23,  // on "if", goto 22
        0,  // on "in", error
        24,  // on "let", goto 23
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 67
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "begin" Expr "end" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fix" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fun" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = "let" Ident "=" (*) Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" "rec" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∧" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∨" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        19,  // on "begin", goto 18
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        21,  // on "fix", goto 20
        22,  // on "fun", goto 21
        0,  // on "head", error
        23,  // on "if", goto 22
        0,  // on "in", error
        24,  // on "let", goto 23
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 68
        //     Expr = "let" "rec" Ident (*) "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        76,  // on "=", goto 75
        0,  // on ">", error
        0,  // on ">=", error
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 69
        //     Get = Ident "[" Add "]" (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = Ident "[" Add "]" (*) "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        -30,  // on ")", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        -30,  // on "*", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        -30,  // on "+", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        -30,  // on ",", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        -30,  // on "-", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        0,  // on "->", error
        0,  // on "::", error
        -30,  // on "<", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        77,  // on "<-", goto 76
        -30,  // on "<=", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        -30,  // on "=", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        -30,  // on ">", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        -30,  // on ">=", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        -30,  // on "]", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        0,  // on "array", error
        0,  // on "begin", error
        -30,  // on "else", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        0,  // on "empty", error
        0,  // on "empty?", error
        -30,  // on "end", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        -30,  // on "in", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        -30,  // on "then", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        0,  // on "true", error
        0,  // on "ν", error
        -30,  // on "∧", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        -30,  // on "∨", reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`
        0,  // on "★", error

        // State 70
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = "array" "(" Add "," (*) Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 71
        //     Expr = "fix" Ident "->" Expr (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -25,  // on "!", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "(", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on ")", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "*", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "+", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on ",", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "-", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "->", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "::", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "<", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "<-", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "<=", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "=", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on ">", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on ">=", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "Iden", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "Num", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "[", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "]", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "array", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "begin", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "else", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "empty", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "empty?", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "end", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "false", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "fix", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "fun", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "head", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "if", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "in", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "let", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "rec", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "set", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "tail", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "then", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "true", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "ν", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "∧", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "∨", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`
        -25,  // on "★", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`

        // State 72
        //     Expr = "fun" Ident "->" Expr (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -24,  // on "!", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "(", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on ")", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "*", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "+", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on ",", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "-", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "->", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "::", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "<", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "<-", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "<=", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "=", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on ">", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on ">=", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "Iden", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "Num", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "[", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "]", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "array", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "begin", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "else", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "empty", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "empty?", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "end", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "false", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "fix", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "fun", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "head", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "if", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "in", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "let", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "rec", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "set", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "tail", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "then", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "true", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "ν", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "∧", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "∨", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`
        -24,  // on "★", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`

        // State 73
        //     Expr = "if" Expr "then" Expr (*) "else" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        79,  // on "else", goto 78
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 74
        //     Expr = "let" Ident "=" Expr (*) "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        80,  // on "in", goto 79
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 75
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "begin" Expr "end" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fix" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fun" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" "rec" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = "let" "rec" Ident "=" (*) Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∧" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∨" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        19,  // on "begin", goto 18
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        21,  // on "fix", goto 20
        22,  // on "fun", goto 21
        0,  // on "head", error
        23,  // on "if", goto 22
        0,  // on "in", error
        24,  // on "let", goto 23
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 76
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = Ident "[" Add "]" "<-" (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 77
        //     Add = Add (*) "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = Add (*) "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = "array" "(" Add "," Add (*) ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        83,  // on ")", goto 82
        0,  // on "*", error
        30,  // on "+", goto 29
        0,  // on ",", error
        31,  // on "-", goto 30
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        0,  // on "in", error
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 78
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "begin" Expr "end" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fix" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fun" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = "if" Expr "then" Expr "else" (*) Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" "rec" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∧" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∨" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        19,  // on "begin", goto 18
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        21,  // on "fix", goto 20
        22,  // on "fun", goto 21
        0,  // on "head", error
        23,  // on "if", goto 22
        0,  // on "in", error
        24,  // on "let", goto 23
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 79
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "begin" Expr "end" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fix" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fun" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = "let" Ident "=" Expr "in" (*) Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" "rec" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∧" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∨" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        19,  // on "begin", goto 18
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        21,  // on "fix", goto 20
        22,  // on "fun", goto 21
        0,  // on "head", error
        23,  // on "if", goto 22
        0,  // on "in", error
        24,  // on "let", goto 23
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 80
        //     Expr = "let" "rec" Ident "=" Expr (*) "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        0,  // on "!", error
        0,  // on "(", error
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        0,  // on "Iden", error
        0,  // on "Num", error
        0,  // on "[", error
        0,  // on "]", error
        0,  // on "array", error
        0,  // on "begin", error
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        0,  // on "false", error
        0,  // on "fix", error
        0,  // on "fun", error
        0,  // on "head", error
        0,  // on "if", error
        86,  // on "in", goto 85
        0,  // on "let", error
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        0,  // on "true", error
        0,  // on "ν", error
        0,  // on "∧", error
        0,  // on "∨", error
        0,  // on "★", error

        // State 81
        //     Get = Ident "[" Add "]" "<-" Atom (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -29,  // on "!", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "(", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on ")", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "*", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "+", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on ",", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "-", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "->", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "::", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "<", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "<-", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "<=", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "=", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on ">", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on ">=", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "Iden", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "Num", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "[", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "]", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "array", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "begin", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "else", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "empty", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "empty?", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "end", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "false", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "fix", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "fun", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "head", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "if", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "in", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "let", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "rec", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "set", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "tail", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "then", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "true", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "ν", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "∧", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "∨", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`
        -29,  // on "★", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`

        // State 82
        //     App = "array" "(" Add "," Add ")" (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -10,  // on "!", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "(", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on ")", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "*", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "+", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on ",", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "-", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "->", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "::", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "<", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "<-", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "<=", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "=", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on ">", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on ">=", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "Iden", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "Num", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "[", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "]", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "array", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "begin", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "else", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "empty", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "empty?", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "end", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "false", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "fix", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "fun", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "head", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "if", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "in", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "let", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "rec", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "set", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "tail", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "then", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "true", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "ν", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "∧", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "∨", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`
        -10,  // on "★", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`

        // State 83
        //     Expr = "if" Expr "then" Expr "else" Expr (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -21,  // on "!", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "(", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on ")", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "*", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "+", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on ",", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "-", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "->", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "::", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "<", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "<-", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "<=", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "=", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on ">", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on ">=", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "Iden", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "Num", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "[", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "]", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "array", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "begin", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "else", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "empty", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "empty?", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "end", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "false", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "fix", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "fun", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "head", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "if", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "in", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "let", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "rec", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "set", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "tail", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "then", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "true", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "ν", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "∧", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "∨", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21,  // on "★", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`

        // State 84
        //     Expr = "let" Ident "=" Expr "in" Expr (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -22,  // on "!", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "(", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on ")", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "*", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "+", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on ",", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "-", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "->", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "::", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "<", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "<-", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "<=", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "=", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on ">", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on ">=", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "Iden", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "Num", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "[", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "]", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "array", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "begin", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "else", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "empty", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "empty?", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "end", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "false", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "fix", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "fun", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "head", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "if", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "in", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "let", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "rec", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "set", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "tail", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "then", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "true", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "ν", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "∧", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "∨", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        -22,  // on "★", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`

        // State 85
        //     ACmp = (*) Add ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "<=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add "=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     ACmp = (*) Add ">=" ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "+" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Add "-" Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Add = (*) Mul ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) App Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "!" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Ident ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) Int ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "(" Exprs ")" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "false" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "true" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "ν" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Atom = (*) "★" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "begin" Expr "end" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fix" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "fun" Ident "->" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = (*) "let" "rec" Ident "=" Expr "in" Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Expr = "let" "rec" Ident "=" Expr "in" (*) Expr ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) App ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Ident = (*) "Iden" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Int = (*) Num ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∧" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     LCmp = (*) ACmp "∨" LCmp ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Mul = (*) Mul "*" Get ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        //     Num = (*) "Num" ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        14,  // on "!", goto 13
        15,  // on "(", goto 14
        0,  // on ")", error
        0,  // on "*", error
        0,  // on "+", error
        0,  // on ",", error
        0,  // on "-", error
        0,  // on "->", error
        0,  // on "::", error
        0,  // on "<", error
        0,  // on "<-", error
        0,  // on "<=", error
        0,  // on "=", error
        0,  // on ">", error
        0,  // on ">=", error
        16,  // on "Iden", goto 15
        17,  // on "Num", goto 16
        0,  // on "[", error
        0,  // on "]", error
        18,  // on "array", goto 17
        19,  // on "begin", goto 18
        0,  // on "else", error
        0,  // on "empty", error
        0,  // on "empty?", error
        0,  // on "end", error
        20,  // on "false", goto 19
        21,  // on "fix", goto 20
        22,  // on "fun", goto 21
        0,  // on "head", error
        23,  // on "if", goto 22
        0,  // on "in", error
        24,  // on "let", goto 23
        0,  // on "rec", error
        0,  // on "set", error
        0,  // on "tail", error
        0,  // on "then", error
        25,  // on "true", goto 24
        26,  // on "ν", goto 25
        0,  // on "∧", error
        0,  // on "∨", error
        27,  // on "★", goto 26

        // State 86
        //     Expr = "let" "rec" Ident "=" Expr "in" Expr (*) ["!", "(", ")", "*", "+", ",", "-", "->", "::", "<", "<-", "<=", "=", ">", ">=", "Iden", "Num", "[", "]", "array", "begin", "else", "empty", "empty?", "end", "false", "fix", "fun", "head", "if", "in", "let", "rec", "set", "tail", "then", "true", "ν", "∧", "∨", "★", EOF]
        -23,  // on "!", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "(", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on ")", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "*", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "+", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on ",", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "-", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "->", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "::", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "<", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "<-", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "<=", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "=", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on ">", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on ">=", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "Iden", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "Num", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "[", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "]", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "array", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "begin", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "else", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "empty", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "empty?", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "end", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "false", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "fix", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "fun", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "head", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "if", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "in", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "let", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "rec", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "set", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "tail", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "then", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "true", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "ν", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "∧", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "∨", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`
        -23,  // on "★", reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`

    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,  // on EOF, error

        -36,  // on EOF, reduce `LCmp = ACmp => ActionFn(11);`

        -6,  // on EOF, reduce `ACmp = Add => ActionFn(17);`

        -31,  // on EOF, reduce `Get = App => ActionFn(25);`

        -13,  // on EOF, reduce `App = Atom => ActionFn(29);`

        -40,  // on EOF, reduce `Program = Expr => ActionFn(1);`

        -38,  // on EOF, reduce `Mul = Get => ActionFn(22);`

        -14,  // on EOF, reduce `Atom = Ident => ActionFn(30);`

        -20,  // on EOF, reduce `Atom = Int => ActionFn(36);`

        -27,  // on EOF, reduce `Expr = LCmp => ActionFn(8);`

        -9,  // on EOF, reduce `Add = Mul => ActionFn(20);`

        -33,  // on EOF, reduce `Int = Num => ActionFn(38);`

        -41,  // on EOF, reduce `__Program = Program => ActionFn(0);`

        0,  // on EOF, error

        0,  // on EOF, error

        -32,  // on EOF, reduce `Ident = "Iden" => ActionFn(39);`

        -39,  // on EOF, reduce `Num = "Num" => ActionFn(40);`

        0,  // on EOF, error

        0,  // on EOF, error

        -18,  // on EOF, reduce `Atom = "false" => ActionFn(34);`

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        -17,  // on EOF, reduce `Atom = "true" => ActionFn(33);`

        -15,  // on EOF, reduce `Atom = "ν" => ActionFn(31);`

        -16,  // on EOF, reduce `Atom = "★" => ActionFn(32);`

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        -12,  // on EOF, reduce `App = App, Atom => ActionFn(28);`

        -14,  // on EOF, reduce `Atom = Ident => ActionFn(30);`

        0,  // on EOF, error

        0,  // on EOF, error

        -11,  // on EOF, reduce `App = "!", Atom => ActionFn(27);`

        -28,  // on EOF, reduce `Exprs = Expr => ActionFn(37);`

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        -34,  // on EOF, reduce `LCmp = ACmp, "∧", LCmp => ActionFn(9);`

        -35,  // on EOF, reduce `LCmp = ACmp, "∨", LCmp => ActionFn(10);`

        -8,  // on EOF, reduce `Add = Add, "+", Mul => ActionFn(19);`

        -7,  // on EOF, reduce `Add = Add, "-", Mul => ActionFn(18);`

        -2,  // on EOF, reduce `ACmp = Add, "<", ACmp => ActionFn(13);`

        -3,  // on EOF, reduce `ACmp = Add, "<=", ACmp => ActionFn(14);`

        -1,  // on EOF, reduce `ACmp = Add, "=", ACmp => ActionFn(12);`

        -4,  // on EOF, reduce `ACmp = Add, ">", ACmp => ActionFn(15);`

        -5,  // on EOF, reduce `ACmp = Add, ">=", ACmp => ActionFn(16);`

        0,  // on EOF, error

        -37,  // on EOF, reduce `Mul = Mul, "*", Get => ActionFn(21);`

        -19,  // on EOF, reduce `Atom = "(", Exprs, ")" => ActionFn(35);`

        0,  // on EOF, error

        -26,  // on EOF, reduce `Expr = "begin", Expr, "end" => ActionFn(7);`

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        -30,  // on EOF, reduce `Get = Ident, "[", Add, "]" => ActionFn(24);`

        0,  // on EOF, error

        -25,  // on EOF, reduce `Expr = "fix", Ident, "->", Expr => ActionFn(6);`

        -24,  // on EOF, reduce `Expr = "fun", Ident, "->", Expr => ActionFn(5);`

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        0,  // on EOF, error

        -29,  // on EOF, reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);`

        -10,  // on EOF, reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(26);`

        -21,  // on EOF, reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`

        -22,  // on EOF, reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`

        0,  // on EOF, error

        -23,  // on EOF, reduce `Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);`

    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2,  // on ACmp, goto 1
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        6,  // on Expr, goto 5
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        10,  // on LCmp, goto 9
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        13,  // on Program, goto 12
        0,  // on __Program, error

        // State 1
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 2
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 3
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        37,  // on Atom, goto 36
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        38,  // on Ident, goto 37
        9,  // on Int, goto 8
        0,  // on LCmp, error
        0,  // on Mul, error
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 4
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 5
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 6
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 7
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 8
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 9
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 10
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 11
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 12
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 13
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        41,  // on Atom, goto 40
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        38,  // on Ident, goto 37
        9,  // on Int, goto 8
        0,  // on LCmp, error
        0,  // on Mul, error
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 14
        2,  // on ACmp, goto 1
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        42,  // on Expr, goto 41
        43,  // on Exprs, goto 42
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        10,  // on LCmp, goto 9
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 15
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 16
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 17
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 18
        2,  // on ACmp, goto 1
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        45,  // on Expr, goto 44
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        10,  // on LCmp, goto 9
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 19
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 20
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        46,  // on Ident, goto 45
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 21
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        47,  // on Ident, goto 46
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 22
        2,  // on ACmp, goto 1
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        48,  // on Expr, goto 47
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        10,  // on LCmp, goto 9
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 23
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        49,  // on Ident, goto 48
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 24
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 25
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 26
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 27
        2,  // on ACmp, goto 1
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        0,  // on Expr, error
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        51,  // on LCmp, goto 50
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 28
        2,  // on ACmp, goto 1
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        0,  // on Expr, error
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        52,  // on LCmp, goto 51
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 29
        0,  // on ACmp, error
        0,  // on Add, error
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        0,  // on Expr, error
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        0,  // on LCmp, error
        53,  // on Mul, goto 52
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 30
        0,  // on ACmp, error
        0,  // on Add, error
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        0,  // on Expr, error
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        0,  // on LCmp, error
        54,  // on Mul, goto 53
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 31
        55,  // on ACmp, goto 54
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        0,  // on Expr, error
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        0,  // on LCmp, error
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 32
        56,  // on ACmp, goto 55
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        0,  // on Expr, error
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        0,  // on LCmp, error
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 33
        57,  // on ACmp, goto 56
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        0,  // on Expr, error
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        0,  // on LCmp, error
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 34
        58,  // on ACmp, goto 57
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        0,  // on Expr, error
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        0,  // on LCmp, error
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 35
        59,  // on ACmp, goto 58
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        0,  // on Expr, error
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        0,  // on LCmp, error
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 36
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 37
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 38
        0,  // on ACmp, error
        60,  // on Add, goto 59
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        0,  // on Expr, error
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        0,  // on LCmp, error
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 39
        0,  // on ACmp, error
        0,  // on Add, error
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        0,  // on Expr, error
        0,  // on Exprs, error
        61,  // on Get, goto 60
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        0,  // on LCmp, error
        0,  // on Mul, error
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 40
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 41
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 42
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 43
        0,  // on ACmp, error
        63,  // on Add, goto 62
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        0,  // on Expr, error
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        0,  // on LCmp, error
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 44
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 45
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 46
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 47
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 48
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 49
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        69,  // on Ident, goto 68
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 50
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 51
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 52
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 53
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 54
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 55
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 56
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 57
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 58
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 59
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 60
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 61
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 62
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 63
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 64
        2,  // on ACmp, goto 1
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        72,  // on Expr, goto 71
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        10,  // on LCmp, goto 9
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 65
        2,  // on ACmp, goto 1
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        73,  // on Expr, goto 72
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        10,  // on LCmp, goto 9
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 66
        2,  // on ACmp, goto 1
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        74,  // on Expr, goto 73
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        10,  // on LCmp, goto 9
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 67
        2,  // on ACmp, goto 1
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        75,  // on Expr, goto 74
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        10,  // on LCmp, goto 9
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 68
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 69
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 70
        0,  // on ACmp, error
        78,  // on Add, goto 77
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        0,  // on Expr, error
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        0,  // on LCmp, error
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 71
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 72
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 73
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 74
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 75
        2,  // on ACmp, goto 1
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        81,  // on Expr, goto 80
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        10,  // on LCmp, goto 9
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 76
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        82,  // on Atom, goto 81
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        38,  // on Ident, goto 37
        9,  // on Int, goto 8
        0,  // on LCmp, error
        0,  // on Mul, error
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 77
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 78
        2,  // on ACmp, goto 1
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        84,  // on Expr, goto 83
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        10,  // on LCmp, goto 9
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 79
        2,  // on ACmp, goto 1
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        85,  // on Expr, goto 84
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        10,  // on LCmp, goto 9
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 80
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 81
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 82
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 83
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 84
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

        // State 85
        2,  // on ACmp, goto 1
        3,  // on Add, goto 2
        4,  // on App, goto 3
        5,  // on Atom, goto 4
        87,  // on Expr, goto 86
        0,  // on Exprs, error
        7,  // on Get, goto 6
        8,  // on Ident, goto 7
        9,  // on Int, goto 8
        10,  // on LCmp, goto 9
        11,  // on Mul, goto 10
        12,  // on Num, goto 11
        0,  // on Program, error
        0,  // on __Program, error

        // State 86
        0,  // on ACmp, error
        0,  // on Add, error
        0,  // on App, error
        0,  // on Atom, error
        0,  // on Expr, error
        0,  // on Exprs, error
        0,  // on Get, error
        0,  // on Ident, error
        0,  // on Int, error
        0,  // on LCmp, error
        0,  // on Mul, error
        0,  // on Num, error
        0,  // on Program, error
        0,  // on __Program, error

    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""->""###,
            r###""::""###,
            r###""<""###,
            r###""<-""###,
            r###""<=""###,
            r###""=""###,
            r###"">""###,
            r###"">=""###,
            r###""Iden""###,
            r###""Num""###,
            r###""[""###,
            r###""]""###,
            r###""array""###,
            r###""begin""###,
            r###""else""###,
            r###""empty""###,
            r###""empty?""###,
            r###""end""###,
            r###""false""###,
            r###""fix""###,
            r###""fun""###,
            r###""head""###,
            r###""if""###,
            r###""in""###,
            r###""let""###,
            r###""rec""###,
            r###""set""###,
            r###""tail""###,
            r###""then""###,
            r###""true""###,
            r###""ν""###,
            r###""∧""###,
            r###""∨""###,
            r###""★""###,
        ];
        __ACTION[(__state * 41)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Program<
        'input,
        __TOKEN: __ToTriple<'input, Error=tok::Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        text: &'input str,
        __tokens0: __TOKENS,
    ) -> Result<Box<Expr>, __lalrpop_util::ParseError<usize, Tok<'input>, tok::Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Tok::Exclaim if true => 0,
                Tok::LParen if true => 1,
                Tok::RParen if true => 2,
                Tok::Mul if true => 3,
                Tok::Plus if true => 4,
                Tok::Comma if true => 5,
                Tok::Minus if true => 6,
                Tok::RArrow if true => 7,
                Tok::Cons if true => 8,
                Tok::Lt if true => 9,
                Tok::LArrow if true => 10,
                Tok::Lte if true => 11,
                Tok::Eq if true => 12,
                Tok::Gt if true => 13,
                Tok::Gte if true => 14,
                Tok::Ident(_) if true => 15,
                Tok::Num(_) if true => 16,
                Tok::LBracket if true => 17,
                Tok::RBracket if true => 18,
                Tok::Array if true => 19,
                Tok::Begin if true => 20,
                Tok::Else if true => 21,
                Tok::Empty if true => 22,
                Tok::Emptyq if true => 23,
                Tok::End if true => 24,
                Tok::False if true => 25,
                Tok::Fix if true => 26,
                Tok::Fun if true => 27,
                Tok::Head if true => 28,
                Tok::If if true => 29,
                Tok::In if true => 30,
                Tok::Let if true => 31,
                Tok::Rec if true => 32,
                Tok::Set if true => 33,
                Tok::Tail if true => 34,
                Tok::Then if true => 35,
                Tok::True if true => 36,
                Tok::V if true => 37,
                Tok::And if true => 38,
                Tok::Or if true => 39,
                Tok::Star if true => 40,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 41 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Tok::Exclaim => __Symbol::Term_22_21_22((__tok)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Tok::LParen => __Symbol::Term_22_28_22((__tok)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Tok::RParen => __Symbol::Term_22_29_22((__tok)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Tok::Mul => __Symbol::Term_22_2a_22((__tok)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Tok::Plus => __Symbol::Term_22_2b_22((__tok)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Tok::Comma => __Symbol::Term_22_2c_22((__tok)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Tok::Minus => __Symbol::Term_22_2d_22((__tok)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            __tok @ Tok::RArrow => __Symbol::Term_22_2d_3e_22((__tok)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Tok::Cons => __Symbol::Term_22_3a_3a_22((__tok)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            __tok @ Tok::Lt => __Symbol::Term_22_3c_22((__tok)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            __tok @ Tok::LArrow => __Symbol::Term_22_3c_2d_22((__tok)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Tok::Lte => __Symbol::Term_22_3c_3d_22((__tok)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            __tok @ Tok::Eq => __Symbol::Term_22_3d_22((__tok)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            __tok @ Tok::Gt => __Symbol::Term_22_3e_22((__tok)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            __tok @ Tok::Gte => __Symbol::Term_22_3e_3d_22((__tok)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            Tok::Ident(__tok0) => __Symbol::Term_22Iden_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            Tok::Num(__tok0) => __Symbol::Term_22Num_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            __tok @ Tok::LBracket => __Symbol::Term_22_5b_22((__tok)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            __tok @ Tok::RBracket => __Symbol::Term_22_5d_22((__tok)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            __tok @ Tok::Array => __Symbol::Term_22array_22((__tok)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            __tok @ Tok::Begin => __Symbol::Term_22begin_22((__tok)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            __tok @ Tok::Else => __Symbol::Term_22else_22((__tok)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            __tok @ Tok::Empty => __Symbol::Term_22empty_22((__tok)),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            __tok @ Tok::Emptyq => __Symbol::Term_22empty_3f_22((__tok)),
                            _ => unreachable!(),
                        },
                        24 => match __lookahead.1 {
                            __tok @ Tok::End => __Symbol::Term_22end_22((__tok)),
                            _ => unreachable!(),
                        },
                        25 => match __lookahead.1 {
                            __tok @ Tok::False => __Symbol::Term_22false_22((__tok)),
                            _ => unreachable!(),
                        },
                        26 => match __lookahead.1 {
                            __tok @ Tok::Fix => __Symbol::Term_22fix_22((__tok)),
                            _ => unreachable!(),
                        },
                        27 => match __lookahead.1 {
                            __tok @ Tok::Fun => __Symbol::Term_22fun_22((__tok)),
                            _ => unreachable!(),
                        },
                        28 => match __lookahead.1 {
                            __tok @ Tok::Head => __Symbol::Term_22head_22((__tok)),
                            _ => unreachable!(),
                        },
                        29 => match __lookahead.1 {
                            __tok @ Tok::If => __Symbol::Term_22if_22((__tok)),
                            _ => unreachable!(),
                        },
                        30 => match __lookahead.1 {
                            __tok @ Tok::In => __Symbol::Term_22in_22((__tok)),
                            _ => unreachable!(),
                        },
                        31 => match __lookahead.1 {
                            __tok @ Tok::Let => __Symbol::Term_22let_22((__tok)),
                            _ => unreachable!(),
                        },
                        32 => match __lookahead.1 {
                            __tok @ Tok::Rec => __Symbol::Term_22rec_22((__tok)),
                            _ => unreachable!(),
                        },
                        33 => match __lookahead.1 {
                            __tok @ Tok::Set => __Symbol::Term_22set_22((__tok)),
                            _ => unreachable!(),
                        },
                        34 => match __lookahead.1 {
                            __tok @ Tok::Tail => __Symbol::Term_22tail_22((__tok)),
                            _ => unreachable!(),
                        },
                        35 => match __lookahead.1 {
                            __tok @ Tok::Then => __Symbol::Term_22then_22((__tok)),
                            _ => unreachable!(),
                        },
                        36 => match __lookahead.1 {
                            __tok @ Tok::True => __Symbol::Term_22true_22((__tok)),
                            _ => unreachable!(),
                        },
                        37 => match __lookahead.1 {
                            __tok @ Tok::V => __Symbol::Term_22_3bd_22((__tok)),
                            _ => unreachable!(),
                        },
                        38 => match __lookahead.1 {
                            __tok @ Tok::And => __Symbol::Term_22_2227_22((__tok)),
                            _ => unreachable!(),
                        },
                        39 => match __lookahead.1 {
                            __tok @ Tok::Or => __Symbol::Term_22_2228_22((__tok)),
                            _ => unreachable!(),
                        },
                        40 => match __lookahead.1 {
                            __tok @ Tok::Star => __Symbol::Term_22_2605_22((__tok)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(text, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(text, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        text: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Box<Expr>,__lalrpop_util::ParseError<usize, Tok<'input>, tok::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // ACmp = Add, "=", ACmp => ActionFn(12);
                let __sym2 = __pop_NtACmp(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtACmp(__nt), __end));
                0
            }
            2 => {
                // ACmp = Add, "<", ACmp => ActionFn(13);
                let __sym2 = __pop_NtACmp(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action13::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtACmp(__nt), __end));
                0
            }
            3 => {
                // ACmp = Add, "<=", ACmp => ActionFn(14);
                let __sym2 = __pop_NtACmp(__symbols);
                let __sym1 = __pop_Term_22_3c_3d_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtACmp(__nt), __end));
                0
            }
            4 => {
                // ACmp = Add, ">", ACmp => ActionFn(15);
                let __sym2 = __pop_NtACmp(__symbols);
                let __sym1 = __pop_Term_22_3e_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtACmp(__nt), __end));
                0
            }
            5 => {
                // ACmp = Add, ">=", ACmp => ActionFn(16);
                let __sym2 = __pop_NtACmp(__symbols);
                let __sym1 = __pop_Term_22_3e_3d_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtACmp(__nt), __end));
                0
            }
            6 => {
                // ACmp = Add => ActionFn(17);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtACmp(__nt), __end));
                0
            }
            7 => {
                // Add = Add, "-", Mul => ActionFn(18);
                let __sym2 = __pop_NtMul(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action18::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAdd(__nt), __end));
                1
            }
            8 => {
                // Add = Add, "+", Mul => ActionFn(19);
                let __sym2 = __pop_NtMul(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAdd(__nt), __end));
                1
            }
            9 => {
                // Add = Mul => ActionFn(20);
                let __sym0 = __pop_NtMul(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAdd(__nt), __end));
                1
            }
            10 => {
                // App = "array", "(", Add, ",", Add, ")" => ActionFn(26);
                let __sym5 = __pop_Term_22_29_22(__symbols);
                let __sym4 = __pop_NtAdd(__symbols);
                let __sym3 = __pop_Term_22_2c_22(__symbols);
                let __sym2 = __pop_NtAdd(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22array_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action26::<>(text, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtApp(__nt), __end));
                2
            }
            11 => {
                // App = "!", Atom => ActionFn(27);
                let __sym1 = __pop_NtAtom(__symbols);
                let __sym0 = __pop_Term_22_21_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action27::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtApp(__nt), __end));
                2
            }
            12 => {
                // App = App, Atom => ActionFn(28);
                let __sym1 = __pop_NtAtom(__symbols);
                let __sym0 = __pop_NtApp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtApp(__nt), __end));
                2
            }
            13 => {
                // App = Atom => ActionFn(29);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtApp(__nt), __end));
                2
            }
            14 => {
                // Atom = Ident => ActionFn(30);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                3
            }
            15 => {
                // Atom = "ν" => ActionFn(31);
                let __sym0 = __pop_Term_22_3bd_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                3
            }
            16 => {
                // Atom = "★" => ActionFn(32);
                let __sym0 = __pop_Term_22_2605_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                3
            }
            17 => {
                // Atom = "true" => ActionFn(33);
                let __sym0 = __pop_Term_22true_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                3
            }
            18 => {
                // Atom = "false" => ActionFn(34);
                let __sym0 = __pop_Term_22false_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                3
            }
            19 => {
                // Atom = "(", Exprs, ")" => ActionFn(35);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExprs(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action35::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                3
            }
            20 => {
                // Atom = Int => ActionFn(36);
                let __sym0 = __pop_NtInt(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                3
            }
            21 => {
                // Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);
                let __sym5 = __pop_NtExpr(__symbols);
                let __sym4 = __pop_Term_22else_22(__symbols);
                let __sym3 = __pop_NtExpr(__symbols);
                let __sym2 = __pop_Term_22then_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22if_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action2::<>(text, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                4
            }
            22 => {
                // Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);
                let __sym5 = __pop_NtExpr(__symbols);
                let __sym4 = __pop_Term_22in_22(__symbols);
                let __sym3 = __pop_NtExpr(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22let_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action3::<>(text, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                4
            }
            23 => {
                // Expr = "let", "rec", Ident, "=", Expr, "in", Expr => ActionFn(4);
                let __sym6 = __pop_NtExpr(__symbols);
                let __sym5 = __pop_Term_22in_22(__symbols);
                let __sym4 = __pop_NtExpr(__symbols);
                let __sym3 = __pop_Term_22_3d_22(__symbols);
                let __sym2 = __pop_NtIdent(__symbols);
                let __sym1 = __pop_Term_22rec_22(__symbols);
                let __sym0 = __pop_Term_22let_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action4::<>(text, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                4
            }
            24 => {
                // Expr = "fun", Ident, "->", Expr => ActionFn(5);
                let __sym3 = __pop_NtExpr(__symbols);
                let __sym2 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22fun_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action5::<>(text, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                4
            }
            25 => {
                // Expr = "fix", Ident, "->", Expr => ActionFn(6);
                let __sym3 = __pop_NtExpr(__symbols);
                let __sym2 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22fix_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action6::<>(text, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                4
            }
            26 => {
                // Expr = "begin", Expr, "end" => ActionFn(7);
                let __sym2 = __pop_Term_22end_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22begin_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                4
            }
            27 => {
                // Expr = LCmp => ActionFn(8);
                let __sym0 = __pop_NtLCmp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                4
            }
            28 => {
                // Exprs = Expr => ActionFn(37);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprs(__nt), __end));
                5
            }
            29 => {
                // Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(23);
                let __sym5 = __pop_NtAtom(__symbols);
                let __sym4 = __pop_Term_22_3c_2d_22(__symbols);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtAdd(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action23::<>(text, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtGet(__nt), __end));
                6
            }
            30 => {
                // Get = Ident, "[", Add, "]" => ActionFn(24);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtAdd(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action24::<>(text, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtGet(__nt), __end));
                6
            }
            31 => {
                // Get = App => ActionFn(25);
                let __sym0 = __pop_NtApp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtGet(__nt), __end));
                6
            }
            32 => {
                // Ident = "Iden" => ActionFn(39);
                let __sym0 = __pop_Term_22Iden_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action39::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent(__nt), __end));
                7
            }
            33 => {
                // Int = Num => ActionFn(38);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInt(__nt), __end));
                8
            }
            34 => {
                // LCmp = ACmp, "∧", LCmp => ActionFn(9);
                let __sym2 = __pop_NtLCmp(__symbols);
                let __sym1 = __pop_Term_22_2227_22(__symbols);
                let __sym0 = __pop_NtACmp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLCmp(__nt), __end));
                9
            }
            35 => {
                // LCmp = ACmp, "∨", LCmp => ActionFn(10);
                let __sym2 = __pop_NtLCmp(__symbols);
                let __sym1 = __pop_Term_22_2228_22(__symbols);
                let __sym0 = __pop_NtACmp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLCmp(__nt), __end));
                9
            }
            36 => {
                // LCmp = ACmp => ActionFn(11);
                let __sym0 = __pop_NtACmp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLCmp(__nt), __end));
                9
            }
            37 => {
                // Mul = Mul, "*", Get => ActionFn(21);
                let __sym2 = __pop_NtGet(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtMul(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtMul(__nt), __end));
                10
            }
            38 => {
                // Mul = Get => ActionFn(22);
                let __sym0 = __pop_NtGet(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMul(__nt), __end));
                10
            }
            39 => {
                // Num = "Num" => ActionFn(40);
                let __sym0 = __pop_Term_22Num_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action40::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                11
            }
            40 => {
                // Program = Expr => ActionFn(1);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                12
            }
            41 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(text, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_21_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_21_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22Iden_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22Iden_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22Num_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22Num_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22array_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22array_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22begin_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22begin_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22else_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22else_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22empty_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22empty_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22empty_3f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22empty_3f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22end_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22end_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22false_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22false_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22fix_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22fix_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22fun_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22fun_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22head_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22head_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22if_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22if_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22in_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22in_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22let_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22let_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22rec_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22rec_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22set_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22set_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22tail_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22tail_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22then_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22then_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22true_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22true_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3bd_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3bd_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2227_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2227_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2228_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2228_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2605_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2605_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtACmp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtACmp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAdd<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAdd(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtApp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtApp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAtom<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAtom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtGet<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtGet(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdent<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Id, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdent(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtInt<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInt(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLCmp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLCmp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMul<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMul(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Program::parse_Program;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, c, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, t, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, f, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(If(c, t, f))
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, id, _): (usize, Id, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, v, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, e, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Let(id, v, e))
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, f, _): (usize, Id, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, v, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, e, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Let(f.clone(), Box::new(Fix(f, v)), e))
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, id, _): (usize, Id, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, e, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Fun(id, e))
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, id, _): (usize, Id, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, e, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Fix(id, e))
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, e, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Box<Expr>
{
    e
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Op2(And, l, r))
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Op2(Or, l, r))
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Op2(Eq, l, r))
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Op2(LT, l, r))
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Op2(LTE, l, r))
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Op2(GT, l, r))
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Op2(GTE, l, r))
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Op2(Sub, l, r))
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Op2(Add, l, r))
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Op2(Mul, l, r))
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    text: &'input str,
    (_, id, _): (usize, Id, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, idx, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, e, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(SetArray(Box::new(Var(id)), idx, e))
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    text: &'input str,
    (_, id, _): (usize, Id, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, idx, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Box<Expr>
{
    Box::new(GetArray(Box::new(Var(id)), idx))
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, sz, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, n, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Box<Expr>
{
    Box::new(MkArray(sz, n))
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, a, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(App(Box::new(Var(String::from("not"))), a))
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    text: &'input str,
    (_, a, _): (usize, Box<Expr>, usize),
    (_, b, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(App(a, b))
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    text: &'input str,
    (_, id, _): (usize, Id, usize),
) -> Box<Expr>
{
    Box::new(Var(id))
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Box<Expr>
{
    Box::new(V)
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Box<Expr>
{
    Box::new(Star)
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Box<Expr>
{
    Box::new(Const(Bool(true)))
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Box<Expr>
{
    Box::new(Const(Bool(true)))
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, i64, usize),
) -> Box<Expr>
{
    Box::new(Const(Int(__0)))
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    text: &'input str,
    (_, id, _): (usize, &'input str, usize),
) -> Id
{
    String::from(id)
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, i64, usize),
) -> i64
{
    (__0)
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, Tok<'input>, usize) {
    type Error = tok::Error;
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize),tok::Error> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Tok<'input>, usize),tok::Error> {
    type Error = tok::Error;
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize),tok::Error> {
        value
    }
}
