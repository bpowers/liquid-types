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
        Term_22_3d_22(Tok<'input>),
        Term_22_3e_22(Tok<'input>),
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
        Term_22set_22(Tok<'input>),
        Term_22tail_22(Tok<'input>),
        Term_22then_22(Tok<'input>),
        Term_22true_22(Tok<'input>),
        NtAdd(Box<Expr>),
        NtApp(Box<Expr>),
        NtAtom(Box<Expr>),
        NtCmp(Box<Expr>),
        NtExpr(Box<Expr>),
        NtExprs(Box<Expr>),
        NtGet(Box<Expr>),
        NtIdent(Id),
        NtInt(Box<Expr>),
        NtList(Box<Expr>),
        NtMul(Box<Expr>),
        NtNum(i64),
        NtProgram(Box<Expr>),
        Nt____Program(Box<Expr>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        //     Add = (*) Add "+" Mul ["+", "-", "<", "=", ">", EOF]
        //     Add = (*) Add "-" Mul ["+", "-", "<", "=", ">", EOF]
        //     Add = (*) Mul ["+", "-", "<", "=", ">", EOF]
        //     App = (*) App Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     App = (*) Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     App = (*) "empty?" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     App = (*) "head" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     App = (*) "tail" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Atom = (*) Ident ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Atom = (*) Int ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Atom = (*) "(" Exprs ")" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Atom = (*) "empty" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Atom = (*) "false" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Atom = (*) "true" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Cmp = (*) Add [EOF]
        //     Cmp = (*) Add "<" Cmp [EOF]
        //     Cmp = (*) Add "=" Cmp [EOF]
        //     Cmp = (*) Add ">" Cmp [EOF]
        //     Expr = (*) Cmp [EOF]
        //     Expr = (*) "begin" Expr "end" [EOF]
        //     Expr = (*) "fix" Ident "->" Expr [EOF]
        //     Expr = (*) "fun" Ident "->" Expr [EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr [EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr [EOF]
        //     Get = (*) App ["*", "+", "-", "::", "<", "=", ">", EOF]
        //     Get = (*) Ident "[" Add "]" ["*", "+", "-", "::", "<", "=", ">", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["*", "+", "-", "::", "<", "=", ">", EOF]
        //     Ident = (*) "Iden" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "[", "empty", "false", "true", EOF]
        //     Int = (*) Num ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     List = (*) Get ["*", "+", "-", "<", "=", ">", EOF]
        //     List = (*) Get "::" List ["*", "+", "-", "<", "=", ">", EOF]
        //     Mul = (*) List ["*", "+", "-", "<", "=", ">", EOF]
        //     Mul = (*) Mul "*" List ["*", "+", "-", "<", "=", ">", EOF]
        //     Num = (*) "Num" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Program = (*) Expr [EOF]
        //     __Program = (*) Program [EOF]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        18, // on "begin", goto 17
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        22, // on "fix", goto 21
        23, // on "fun", goto 22
        24, // on "head", goto 23
        25, // on "if", goto 24
        0, // on "in", error
        26, // on "let", goto 25
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 1
        //     Add = Add (*) "+" Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Add = Add (*) "-" Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Cmp = Add (*) [")", "else", "end", "in", "then", EOF]
        //     Cmp = Add (*) "<" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = Add (*) "=" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = Add (*) ">" Cmp [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -19, // on ")", reduce `Cmp = Add => ActionFn(11);`
        0, // on "*", error
        29, // on "+", goto 28
        0, // on ",", error
        30, // on "-", goto 29
        0, // on "->", error
        0, // on "::", error
        31, // on "<", goto 30
        0, // on "<-", error
        32, // on "=", goto 31
        33, // on ">", goto 32
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        -19, // on "else", reduce `Cmp = Add => ActionFn(11);`
        0, // on "empty", error
        0, // on "empty?", error
        -19, // on "end", reduce `Cmp = Add => ActionFn(11);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -19, // on "in", reduce `Cmp = Add => ActionFn(11);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -19, // on "then", reduce `Cmp = Add => ActionFn(11);`
        0, // on "true", error
        // State 2
        //     App = App (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Get = App (*) [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        14, // on "(", goto 13
        -29, // on ")", reduce `Get = App => ActionFn(21);`
        -29, // on "*", reduce `Get = App => ActionFn(21);`
        -29, // on "+", reduce `Get = App => ActionFn(21);`
        -29, // on ",", reduce `Get = App => ActionFn(21);`
        -29, // on "-", reduce `Get = App => ActionFn(21);`
        0, // on "->", error
        -29, // on "::", reduce `Get = App => ActionFn(21);`
        -29, // on "<", reduce `Get = App => ActionFn(21);`
        0, // on "<-", error
        -29, // on "=", reduce `Get = App => ActionFn(21);`
        -29, // on ">", reduce `Get = App => ActionFn(21);`
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        -29, // on "]", reduce `Get = App => ActionFn(21);`
        0, // on "array", error
        0, // on "begin", error
        -29, // on "else", reduce `Get = App => ActionFn(21);`
        19, // on "empty", goto 18
        0, // on "empty?", error
        -29, // on "end", reduce `Get = App => ActionFn(21);`
        21, // on "false", goto 20
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -29, // on "in", reduce `Get = App => ActionFn(21);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -29, // on "then", reduce `Get = App => ActionFn(21);`
        28, // on "true", goto 27
        // State 3
        //     App = Atom (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        -9, // on "(", reduce `App = Atom => ActionFn(27);`
        -9, // on ")", reduce `App = Atom => ActionFn(27);`
        -9, // on "*", reduce `App = Atom => ActionFn(27);`
        -9, // on "+", reduce `App = Atom => ActionFn(27);`
        -9, // on ",", reduce `App = Atom => ActionFn(27);`
        -9, // on "-", reduce `App = Atom => ActionFn(27);`
        0, // on "->", error
        -9, // on "::", reduce `App = Atom => ActionFn(27);`
        -9, // on "<", reduce `App = Atom => ActionFn(27);`
        0, // on "<-", error
        -9, // on "=", reduce `App = Atom => ActionFn(27);`
        -9, // on ">", reduce `App = Atom => ActionFn(27);`
        -9, // on "Iden", reduce `App = Atom => ActionFn(27);`
        -9, // on "Num", reduce `App = Atom => ActionFn(27);`
        0, // on "[", error
        -9, // on "]", reduce `App = Atom => ActionFn(27);`
        0, // on "array", error
        0, // on "begin", error
        -9, // on "else", reduce `App = Atom => ActionFn(27);`
        -9, // on "empty", reduce `App = Atom => ActionFn(27);`
        0, // on "empty?", error
        -9, // on "end", reduce `App = Atom => ActionFn(27);`
        -9, // on "false", reduce `App = Atom => ActionFn(27);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -9, // on "in", reduce `App = Atom => ActionFn(27);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -9, // on "then", reduce `App = Atom => ActionFn(27);`
        -9, // on "true", reduce `App = Atom => ActionFn(27);`
        // State 4
        //     Expr = Cmp (*) [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -25, // on ")", reduce `Expr = Cmp => ActionFn(7);`
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        -25, // on "else", reduce `Expr = Cmp => ActionFn(7);`
        0, // on "empty", error
        0, // on "empty?", error
        -25, // on "end", reduce `Expr = Cmp => ActionFn(7);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -25, // on "in", reduce `Expr = Cmp => ActionFn(7);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -25, // on "then", reduce `Expr = Cmp => ActionFn(7);`
        0, // on "true", error
        // State 5
        //     Program = Expr (*) [EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 6
        //     List = Get (*) [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     List = Get (*) "::" List [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -33, // on ")", reduce `List = Get => ActionFn(18);`
        -33, // on "*", reduce `List = Get => ActionFn(18);`
        -33, // on "+", reduce `List = Get => ActionFn(18);`
        -33, // on ",", reduce `List = Get => ActionFn(18);`
        -33, // on "-", reduce `List = Get => ActionFn(18);`
        0, // on "->", error
        36, // on "::", goto 35
        -33, // on "<", reduce `List = Get => ActionFn(18);`
        0, // on "<-", error
        -33, // on "=", reduce `List = Get => ActionFn(18);`
        -33, // on ">", reduce `List = Get => ActionFn(18);`
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        -33, // on "]", reduce `List = Get => ActionFn(18);`
        0, // on "array", error
        0, // on "begin", error
        -33, // on "else", reduce `List = Get => ActionFn(18);`
        0, // on "empty", error
        0, // on "empty?", error
        -33, // on "end", reduce `List = Get => ActionFn(18);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -33, // on "in", reduce `List = Get => ActionFn(18);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -33, // on "then", reduce `List = Get => ActionFn(18);`
        0, // on "true", error
        // State 7
        //     Atom = Ident (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Get = Ident (*) "[" Add "]" [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Get = Ident (*) "[" Add "]" "<-" Atom [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        -10, // on "(", reduce `Atom = Ident => ActionFn(28);`
        -10, // on ")", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "*", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "+", reduce `Atom = Ident => ActionFn(28);`
        -10, // on ",", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "-", reduce `Atom = Ident => ActionFn(28);`
        0, // on "->", error
        -10, // on "::", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "<", reduce `Atom = Ident => ActionFn(28);`
        0, // on "<-", error
        -10, // on "=", reduce `Atom = Ident => ActionFn(28);`
        -10, // on ">", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "Iden", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "Num", reduce `Atom = Ident => ActionFn(28);`
        37, // on "[", goto 36
        -10, // on "]", reduce `Atom = Ident => ActionFn(28);`
        0, // on "array", error
        0, // on "begin", error
        -10, // on "else", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "empty", reduce `Atom = Ident => ActionFn(28);`
        0, // on "empty?", error
        -10, // on "end", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "false", reduce `Atom = Ident => ActionFn(28);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -10, // on "in", reduce `Atom = Ident => ActionFn(28);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -10, // on "then", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "true", reduce `Atom = Ident => ActionFn(28);`
        // State 8
        //     Atom = Int (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        -15, // on "(", reduce `Atom = Int => ActionFn(33);`
        -15, // on ")", reduce `Atom = Int => ActionFn(33);`
        -15, // on "*", reduce `Atom = Int => ActionFn(33);`
        -15, // on "+", reduce `Atom = Int => ActionFn(33);`
        -15, // on ",", reduce `Atom = Int => ActionFn(33);`
        -15, // on "-", reduce `Atom = Int => ActionFn(33);`
        0, // on "->", error
        -15, // on "::", reduce `Atom = Int => ActionFn(33);`
        -15, // on "<", reduce `Atom = Int => ActionFn(33);`
        0, // on "<-", error
        -15, // on "=", reduce `Atom = Int => ActionFn(33);`
        -15, // on ">", reduce `Atom = Int => ActionFn(33);`
        -15, // on "Iden", reduce `Atom = Int => ActionFn(33);`
        -15, // on "Num", reduce `Atom = Int => ActionFn(33);`
        0, // on "[", error
        -15, // on "]", reduce `Atom = Int => ActionFn(33);`
        0, // on "array", error
        0, // on "begin", error
        -15, // on "else", reduce `Atom = Int => ActionFn(33);`
        -15, // on "empty", reduce `Atom = Int => ActionFn(33);`
        0, // on "empty?", error
        -15, // on "end", reduce `Atom = Int => ActionFn(33);`
        -15, // on "false", reduce `Atom = Int => ActionFn(33);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -15, // on "in", reduce `Atom = Int => ActionFn(33);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -15, // on "then", reduce `Atom = Int => ActionFn(33);`
        -15, // on "true", reduce `Atom = Int => ActionFn(33);`
        // State 9
        //     Mul = List (*) [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -35, // on ")", reduce `Mul = List => ActionFn(16);`
        -35, // on "*", reduce `Mul = List => ActionFn(16);`
        -35, // on "+", reduce `Mul = List => ActionFn(16);`
        -35, // on ",", reduce `Mul = List => ActionFn(16);`
        -35, // on "-", reduce `Mul = List => ActionFn(16);`
        0, // on "->", error
        0, // on "::", error
        -35, // on "<", reduce `Mul = List => ActionFn(16);`
        0, // on "<-", error
        -35, // on "=", reduce `Mul = List => ActionFn(16);`
        -35, // on ">", reduce `Mul = List => ActionFn(16);`
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        -35, // on "]", reduce `Mul = List => ActionFn(16);`
        0, // on "array", error
        0, // on "begin", error
        -35, // on "else", reduce `Mul = List => ActionFn(16);`
        0, // on "empty", error
        0, // on "empty?", error
        -35, // on "end", reduce `Mul = List => ActionFn(16);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -35, // on "in", reduce `Mul = List => ActionFn(16);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -35, // on "then", reduce `Mul = List => ActionFn(16);`
        0, // on "true", error
        // State 10
        //     Add = Mul (*) [")", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Mul = Mul (*) "*" List [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -3, // on ")", reduce `Add = Mul => ActionFn(14);`
        38, // on "*", goto 37
        -3, // on "+", reduce `Add = Mul => ActionFn(14);`
        -3, // on ",", reduce `Add = Mul => ActionFn(14);`
        -3, // on "-", reduce `Add = Mul => ActionFn(14);`
        0, // on "->", error
        0, // on "::", error
        -3, // on "<", reduce `Add = Mul => ActionFn(14);`
        0, // on "<-", error
        -3, // on "=", reduce `Add = Mul => ActionFn(14);`
        -3, // on ">", reduce `Add = Mul => ActionFn(14);`
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        -3, // on "]", reduce `Add = Mul => ActionFn(14);`
        0, // on "array", error
        0, // on "begin", error
        -3, // on "else", reduce `Add = Mul => ActionFn(14);`
        0, // on "empty", error
        0, // on "empty?", error
        -3, // on "end", reduce `Add = Mul => ActionFn(14);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -3, // on "in", reduce `Add = Mul => ActionFn(14);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -3, // on "then", reduce `Add = Mul => ActionFn(14);`
        0, // on "true", error
        // State 11
        //     Int = Num (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        -31, // on "(", reduce `Int = Num => ActionFn(35);`
        -31, // on ")", reduce `Int = Num => ActionFn(35);`
        -31, // on "*", reduce `Int = Num => ActionFn(35);`
        -31, // on "+", reduce `Int = Num => ActionFn(35);`
        -31, // on ",", reduce `Int = Num => ActionFn(35);`
        -31, // on "-", reduce `Int = Num => ActionFn(35);`
        0, // on "->", error
        -31, // on "::", reduce `Int = Num => ActionFn(35);`
        -31, // on "<", reduce `Int = Num => ActionFn(35);`
        0, // on "<-", error
        -31, // on "=", reduce `Int = Num => ActionFn(35);`
        -31, // on ">", reduce `Int = Num => ActionFn(35);`
        -31, // on "Iden", reduce `Int = Num => ActionFn(35);`
        -31, // on "Num", reduce `Int = Num => ActionFn(35);`
        0, // on "[", error
        -31, // on "]", reduce `Int = Num => ActionFn(35);`
        0, // on "array", error
        0, // on "begin", error
        -31, // on "else", reduce `Int = Num => ActionFn(35);`
        -31, // on "empty", reduce `Int = Num => ActionFn(35);`
        0, // on "empty?", error
        -31, // on "end", reduce `Int = Num => ActionFn(35);`
        -31, // on "false", reduce `Int = Num => ActionFn(35);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -31, // on "in", reduce `Int = Num => ActionFn(35);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -31, // on "then", reduce `Int = Num => ActionFn(35);`
        -31, // on "true", reduce `Int = Num => ActionFn(35);`
        // State 12
        //     __Program = Program (*) [EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 13
        //     Add = (*) Add "+" Mul [")", "+", "-", "<", "=", ">"]
        //     Add = (*) Add "-" Mul [")", "+", "-", "<", "=", ">"]
        //     Add = (*) Mul [")", "+", "-", "<", "=", ">"]
        //     App = (*) App Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) "array" "(" Add "," Add ")" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) "head" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) Ident ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) Int ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = "(" (*) Exprs ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "false" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "true" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Cmp = (*) Add [")"]
        //     Cmp = (*) Add "<" Cmp [")"]
        //     Cmp = (*) Add "=" Cmp [")"]
        //     Cmp = (*) Add ">" Cmp [")"]
        //     Expr = (*) Cmp [")"]
        //     Expr = (*) "begin" Expr "end" [")"]
        //     Expr = (*) "fix" Ident "->" Expr [")"]
        //     Expr = (*) "fun" Ident "->" Expr [")"]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr [")"]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr [")"]
        //     Exprs = (*) Expr [")"]
        //     Get = (*) App [")", "*", "+", "-", "::", "<", "=", ">"]
        //     Get = (*) Ident "[" Add "]" [")", "*", "+", "-", "::", "<", "=", ">"]
        //     Get = (*) Ident "[" Add "]" "<-" Atom [")", "*", "+", "-", "::", "<", "=", ">"]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "[", "empty", "false", "true"]
        //     Int = (*) Num ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     List = (*) Get [")", "*", "+", "-", "<", "=", ">"]
        //     List = (*) Get "::" List [")", "*", "+", "-", "<", "=", ">"]
        //     Mul = (*) List [")", "*", "+", "-", "<", "=", ">"]
        //     Mul = (*) Mul "*" List [")", "*", "+", "-", "<", "=", ">"]
        //     Num = (*) "Num" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        18, // on "begin", goto 17
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        22, // on "fix", goto 21
        23, // on "fun", goto 22
        24, // on "head", goto 23
        25, // on "if", goto 24
        0, // on "in", error
        26, // on "let", goto 25
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 14
        //     Ident = "Iden" (*) ["(", ")", "*", "+", ",", "-", "->", "::", "<", "=", ">", "Iden", "Num", "[", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        -30, // on "(", reduce `Ident = "Iden" => ActionFn(36);`
        -30, // on ")", reduce `Ident = "Iden" => ActionFn(36);`
        -30, // on "*", reduce `Ident = "Iden" => ActionFn(36);`
        -30, // on "+", reduce `Ident = "Iden" => ActionFn(36);`
        -30, // on ",", reduce `Ident = "Iden" => ActionFn(36);`
        -30, // on "-", reduce `Ident = "Iden" => ActionFn(36);`
        -30, // on "->", reduce `Ident = "Iden" => ActionFn(36);`
        -30, // on "::", reduce `Ident = "Iden" => ActionFn(36);`
        -30, // on "<", reduce `Ident = "Iden" => ActionFn(36);`
        0, // on "<-", error
        -30, // on "=", reduce `Ident = "Iden" => ActionFn(36);`
        -30, // on ">", reduce `Ident = "Iden" => ActionFn(36);`
        -30, // on "Iden", reduce `Ident = "Iden" => ActionFn(36);`
        -30, // on "Num", reduce `Ident = "Iden" => ActionFn(36);`
        -30, // on "[", reduce `Ident = "Iden" => ActionFn(36);`
        -30, // on "]", reduce `Ident = "Iden" => ActionFn(36);`
        0, // on "array", error
        0, // on "begin", error
        -30, // on "else", reduce `Ident = "Iden" => ActionFn(36);`
        -30, // on "empty", reduce `Ident = "Iden" => ActionFn(36);`
        0, // on "empty?", error
        -30, // on "end", reduce `Ident = "Iden" => ActionFn(36);`
        -30, // on "false", reduce `Ident = "Iden" => ActionFn(36);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -30, // on "in", reduce `Ident = "Iden" => ActionFn(36);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -30, // on "then", reduce `Ident = "Iden" => ActionFn(36);`
        -30, // on "true", reduce `Ident = "Iden" => ActionFn(36);`
        // State 15
        //     Num = "Num" (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        -36, // on "(", reduce `Num = "Num" => ActionFn(37);`
        -36, // on ")", reduce `Num = "Num" => ActionFn(37);`
        -36, // on "*", reduce `Num = "Num" => ActionFn(37);`
        -36, // on "+", reduce `Num = "Num" => ActionFn(37);`
        -36, // on ",", reduce `Num = "Num" => ActionFn(37);`
        -36, // on "-", reduce `Num = "Num" => ActionFn(37);`
        0, // on "->", error
        -36, // on "::", reduce `Num = "Num" => ActionFn(37);`
        -36, // on "<", reduce `Num = "Num" => ActionFn(37);`
        0, // on "<-", error
        -36, // on "=", reduce `Num = "Num" => ActionFn(37);`
        -36, // on ">", reduce `Num = "Num" => ActionFn(37);`
        -36, // on "Iden", reduce `Num = "Num" => ActionFn(37);`
        -36, // on "Num", reduce `Num = "Num" => ActionFn(37);`
        0, // on "[", error
        -36, // on "]", reduce `Num = "Num" => ActionFn(37);`
        0, // on "array", error
        0, // on "begin", error
        -36, // on "else", reduce `Num = "Num" => ActionFn(37);`
        -36, // on "empty", reduce `Num = "Num" => ActionFn(37);`
        0, // on "empty?", error
        -36, // on "end", reduce `Num = "Num" => ActionFn(37);`
        -36, // on "false", reduce `Num = "Num" => ActionFn(37);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -36, // on "in", reduce `Num = "Num" => ActionFn(37);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -36, // on "then", reduce `Num = "Num" => ActionFn(37);`
        -36, // on "true", reduce `Num = "Num" => ActionFn(37);`
        // State 16
        //     App = "array" (*) "(" Add "," Add ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        41, // on "(", goto 40
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 17
        //     Add = (*) Add "+" Mul ["+", "-", "<", "=", ">", "end"]
        //     Add = (*) Add "-" Mul ["+", "-", "<", "=", ">", "end"]
        //     Add = (*) Mul ["+", "-", "<", "=", ">", "end"]
        //     App = (*) App Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "end", "false", "true"]
        //     App = (*) Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "end", "false", "true"]
        //     App = (*) "array" "(" Add "," Add ")" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "end", "false", "true"]
        //     App = (*) "empty?" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "end", "false", "true"]
        //     App = (*) "head" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "end", "false", "true"]
        //     App = (*) "tail" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "end", "false", "true"]
        //     Atom = (*) Ident ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "end", "false", "true"]
        //     Atom = (*) Int ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "end", "false", "true"]
        //     Atom = (*) "(" Exprs ")" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "end", "false", "true"]
        //     Atom = (*) "empty" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "end", "false", "true"]
        //     Atom = (*) "false" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "end", "false", "true"]
        //     Atom = (*) "true" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "end", "false", "true"]
        //     Cmp = (*) Add ["end"]
        //     Cmp = (*) Add "<" Cmp ["end"]
        //     Cmp = (*) Add "=" Cmp ["end"]
        //     Cmp = (*) Add ">" Cmp ["end"]
        //     Expr = (*) Cmp ["end"]
        //     Expr = (*) "begin" Expr "end" ["end"]
        //     Expr = "begin" (*) Expr "end" [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "fix" Ident "->" Expr ["end"]
        //     Expr = (*) "fun" Ident "->" Expr ["end"]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr ["end"]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr ["end"]
        //     Get = (*) App ["*", "+", "-", "::", "<", "=", ">", "end"]
        //     Get = (*) Ident "[" Add "]" ["*", "+", "-", "::", "<", "=", ">", "end"]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["*", "+", "-", "::", "<", "=", ">", "end"]
        //     Ident = (*) "Iden" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "[", "empty", "end", "false", "true"]
        //     Int = (*) Num ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "end", "false", "true"]
        //     List = (*) Get ["*", "+", "-", "<", "=", ">", "end"]
        //     List = (*) Get "::" List ["*", "+", "-", "<", "=", ">", "end"]
        //     Mul = (*) List ["*", "+", "-", "<", "=", ">", "end"]
        //     Mul = (*) Mul "*" List ["*", "+", "-", "<", "=", ">", "end"]
        //     Num = (*) "Num" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "end", "false", "true"]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        18, // on "begin", goto 17
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        22, // on "fix", goto 21
        23, // on "fun", goto 22
        24, // on "head", goto 23
        25, // on "if", goto 24
        0, // on "in", error
        26, // on "let", goto 25
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 18
        //     Atom = "empty" (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        -13, // on "(", reduce `Atom = "empty" => ActionFn(31);`
        -13, // on ")", reduce `Atom = "empty" => ActionFn(31);`
        -13, // on "*", reduce `Atom = "empty" => ActionFn(31);`
        -13, // on "+", reduce `Atom = "empty" => ActionFn(31);`
        -13, // on ",", reduce `Atom = "empty" => ActionFn(31);`
        -13, // on "-", reduce `Atom = "empty" => ActionFn(31);`
        0, // on "->", error
        -13, // on "::", reduce `Atom = "empty" => ActionFn(31);`
        -13, // on "<", reduce `Atom = "empty" => ActionFn(31);`
        0, // on "<-", error
        -13, // on "=", reduce `Atom = "empty" => ActionFn(31);`
        -13, // on ">", reduce `Atom = "empty" => ActionFn(31);`
        -13, // on "Iden", reduce `Atom = "empty" => ActionFn(31);`
        -13, // on "Num", reduce `Atom = "empty" => ActionFn(31);`
        0, // on "[", error
        -13, // on "]", reduce `Atom = "empty" => ActionFn(31);`
        0, // on "array", error
        0, // on "begin", error
        -13, // on "else", reduce `Atom = "empty" => ActionFn(31);`
        -13, // on "empty", reduce `Atom = "empty" => ActionFn(31);`
        0, // on "empty?", error
        -13, // on "end", reduce `Atom = "empty" => ActionFn(31);`
        -13, // on "false", reduce `Atom = "empty" => ActionFn(31);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -13, // on "in", reduce `Atom = "empty" => ActionFn(31);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -13, // on "then", reduce `Atom = "empty" => ActionFn(31);`
        -13, // on "true", reduce `Atom = "empty" => ActionFn(31);`
        // State 19
        //     App = "empty?" (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        19, // on "empty", goto 18
        0, // on "empty?", error
        0, // on "end", error
        21, // on "false", goto 20
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        28, // on "true", goto 27
        // State 20
        //     Atom = "false" (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        -12, // on "(", reduce `Atom = "false" => ActionFn(30);`
        -12, // on ")", reduce `Atom = "false" => ActionFn(30);`
        -12, // on "*", reduce `Atom = "false" => ActionFn(30);`
        -12, // on "+", reduce `Atom = "false" => ActionFn(30);`
        -12, // on ",", reduce `Atom = "false" => ActionFn(30);`
        -12, // on "-", reduce `Atom = "false" => ActionFn(30);`
        0, // on "->", error
        -12, // on "::", reduce `Atom = "false" => ActionFn(30);`
        -12, // on "<", reduce `Atom = "false" => ActionFn(30);`
        0, // on "<-", error
        -12, // on "=", reduce `Atom = "false" => ActionFn(30);`
        -12, // on ">", reduce `Atom = "false" => ActionFn(30);`
        -12, // on "Iden", reduce `Atom = "false" => ActionFn(30);`
        -12, // on "Num", reduce `Atom = "false" => ActionFn(30);`
        0, // on "[", error
        -12, // on "]", reduce `Atom = "false" => ActionFn(30);`
        0, // on "array", error
        0, // on "begin", error
        -12, // on "else", reduce `Atom = "false" => ActionFn(30);`
        -12, // on "empty", reduce `Atom = "false" => ActionFn(30);`
        0, // on "empty?", error
        -12, // on "end", reduce `Atom = "false" => ActionFn(30);`
        -12, // on "false", reduce `Atom = "false" => ActionFn(30);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -12, // on "in", reduce `Atom = "false" => ActionFn(30);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -12, // on "then", reduce `Atom = "false" => ActionFn(30);`
        -12, // on "true", reduce `Atom = "false" => ActionFn(30);`
        // State 21
        //     Expr = "fix" (*) Ident "->" Expr [")", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" ["->"]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 22
        //     Expr = "fun" (*) Ident "->" Expr [")", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" ["->"]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 23
        //     App = "head" (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        19, // on "empty", goto 18
        0, // on "empty?", error
        0, // on "end", error
        21, // on "false", goto 20
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        28, // on "true", goto 27
        // State 24
        //     Add = (*) Add "+" Mul ["+", "-", "<", "=", ">", "then"]
        //     Add = (*) Add "-" Mul ["+", "-", "<", "=", ">", "then"]
        //     Add = (*) Mul ["+", "-", "<", "=", ">", "then"]
        //     App = (*) App Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     App = (*) Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     App = (*) "array" "(" Add "," Add ")" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     App = (*) "empty?" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     App = (*) "head" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     App = (*) "tail" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Atom = (*) Ident ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Atom = (*) Int ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Atom = (*) "(" Exprs ")" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Atom = (*) "empty" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Atom = (*) "false" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Atom = (*) "true" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Cmp = (*) Add ["then"]
        //     Cmp = (*) Add "<" Cmp ["then"]
        //     Cmp = (*) Add "=" Cmp ["then"]
        //     Cmp = (*) Add ">" Cmp ["then"]
        //     Expr = (*) Cmp ["then"]
        //     Expr = (*) "begin" Expr "end" ["then"]
        //     Expr = (*) "fix" Ident "->" Expr ["then"]
        //     Expr = (*) "fun" Ident "->" Expr ["then"]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr ["then"]
        //     Expr = "if" (*) Expr "then" Expr "else" Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr ["then"]
        //     Get = (*) App ["*", "+", "-", "::", "<", "=", ">", "then"]
        //     Get = (*) Ident "[" Add "]" ["*", "+", "-", "::", "<", "=", ">", "then"]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["*", "+", "-", "::", "<", "=", ">", "then"]
        //     Ident = (*) "Iden" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "[", "empty", "false", "then", "true"]
        //     Int = (*) Num ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     List = (*) Get ["*", "+", "-", "<", "=", ">", "then"]
        //     List = (*) Get "::" List ["*", "+", "-", "<", "=", ">", "then"]
        //     Mul = (*) List ["*", "+", "-", "<", "=", ">", "then"]
        //     Mul = (*) Mul "*" List ["*", "+", "-", "<", "=", ">", "then"]
        //     Num = (*) "Num" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        18, // on "begin", goto 17
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        22, // on "fix", goto 21
        23, // on "fun", goto 22
        24, // on "head", goto 23
        25, // on "if", goto 24
        0, // on "in", error
        26, // on "let", goto 25
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 25
        //     Expr = "let" (*) Ident "=" Expr "in" Expr [")", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" ["="]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 26
        //     App = "tail" (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        19, // on "empty", goto 18
        0, // on "empty?", error
        0, // on "end", error
        21, // on "false", goto 20
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        28, // on "true", goto 27
        // State 27
        //     Atom = "true" (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        -11, // on "(", reduce `Atom = "true" => ActionFn(29);`
        -11, // on ")", reduce `Atom = "true" => ActionFn(29);`
        -11, // on "*", reduce `Atom = "true" => ActionFn(29);`
        -11, // on "+", reduce `Atom = "true" => ActionFn(29);`
        -11, // on ",", reduce `Atom = "true" => ActionFn(29);`
        -11, // on "-", reduce `Atom = "true" => ActionFn(29);`
        0, // on "->", error
        -11, // on "::", reduce `Atom = "true" => ActionFn(29);`
        -11, // on "<", reduce `Atom = "true" => ActionFn(29);`
        0, // on "<-", error
        -11, // on "=", reduce `Atom = "true" => ActionFn(29);`
        -11, // on ">", reduce `Atom = "true" => ActionFn(29);`
        -11, // on "Iden", reduce `Atom = "true" => ActionFn(29);`
        -11, // on "Num", reduce `Atom = "true" => ActionFn(29);`
        0, // on "[", error
        -11, // on "]", reduce `Atom = "true" => ActionFn(29);`
        0, // on "array", error
        0, // on "begin", error
        -11, // on "else", reduce `Atom = "true" => ActionFn(29);`
        -11, // on "empty", reduce `Atom = "true" => ActionFn(29);`
        0, // on "empty?", error
        -11, // on "end", reduce `Atom = "true" => ActionFn(29);`
        -11, // on "false", reduce `Atom = "true" => ActionFn(29);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -11, // on "in", reduce `Atom = "true" => ActionFn(29);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -11, // on "then", reduce `Atom = "true" => ActionFn(29);`
        -11, // on "true", reduce `Atom = "true" => ActionFn(29);`
        // State 28
        //     Add = Add "+" (*) Mul [")", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Get = (*) App [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "[", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     List = (*) Get [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     List = (*) Get "::" List [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        0, // on "begin", error
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        0, // on "fix", error
        0, // on "fun", error
        24, // on "head", goto 23
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 29
        //     Add = Add "-" (*) Mul [")", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Get = (*) App [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "[", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     List = (*) Get [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     List = (*) Get "::" List [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        0, // on "begin", error
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        0, // on "fix", error
        0, // on "fun", error
        24, // on "head", goto 23
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 30
        //     Add = (*) Add "+" Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Add = (*) Add "-" Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Add = (*) Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Cmp = (*) Add [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add "<" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = Add "<" (*) Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add "=" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add ">" Cmp [")", "else", "end", "in", "then", EOF]
        //     Get = (*) App [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "[", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     List = (*) Get [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     List = (*) Get "::" List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        0, // on "begin", error
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        0, // on "fix", error
        0, // on "fun", error
        24, // on "head", goto 23
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 31
        //     Add = (*) Add "+" Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Add = (*) Add "-" Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Add = (*) Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Cmp = (*) Add [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add "<" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add "=" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = Add "=" (*) Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add ">" Cmp [")", "else", "end", "in", "then", EOF]
        //     Get = (*) App [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "[", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     List = (*) Get [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     List = (*) Get "::" List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        0, // on "begin", error
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        0, // on "fix", error
        0, // on "fun", error
        24, // on "head", goto 23
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 32
        //     Add = (*) Add "+" Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Add = (*) Add "-" Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Add = (*) Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Cmp = (*) Add [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add "<" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add "=" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add ">" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = Add ">" (*) Cmp [")", "else", "end", "in", "then", EOF]
        //     Get = (*) App [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "[", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     List = (*) Get [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     List = (*) Get "::" List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        0, // on "begin", error
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        0, // on "fix", error
        0, // on "fun", error
        24, // on "head", goto 23
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 33
        //     App = App Atom (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        -8, // on "(", reduce `App = App, Atom => ActionFn(26);`
        -8, // on ")", reduce `App = App, Atom => ActionFn(26);`
        -8, // on "*", reduce `App = App, Atom => ActionFn(26);`
        -8, // on "+", reduce `App = App, Atom => ActionFn(26);`
        -8, // on ",", reduce `App = App, Atom => ActionFn(26);`
        -8, // on "-", reduce `App = App, Atom => ActionFn(26);`
        0, // on "->", error
        -8, // on "::", reduce `App = App, Atom => ActionFn(26);`
        -8, // on "<", reduce `App = App, Atom => ActionFn(26);`
        0, // on "<-", error
        -8, // on "=", reduce `App = App, Atom => ActionFn(26);`
        -8, // on ">", reduce `App = App, Atom => ActionFn(26);`
        -8, // on "Iden", reduce `App = App, Atom => ActionFn(26);`
        -8, // on "Num", reduce `App = App, Atom => ActionFn(26);`
        0, // on "[", error
        -8, // on "]", reduce `App = App, Atom => ActionFn(26);`
        0, // on "array", error
        0, // on "begin", error
        -8, // on "else", reduce `App = App, Atom => ActionFn(26);`
        -8, // on "empty", reduce `App = App, Atom => ActionFn(26);`
        0, // on "empty?", error
        -8, // on "end", reduce `App = App, Atom => ActionFn(26);`
        -8, // on "false", reduce `App = App, Atom => ActionFn(26);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -8, // on "in", reduce `App = App, Atom => ActionFn(26);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -8, // on "then", reduce `App = App, Atom => ActionFn(26);`
        -8, // on "true", reduce `App = App, Atom => ActionFn(26);`
        // State 34
        //     Atom = Ident (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        -10, // on "(", reduce `Atom = Ident => ActionFn(28);`
        -10, // on ")", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "*", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "+", reduce `Atom = Ident => ActionFn(28);`
        -10, // on ",", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "-", reduce `Atom = Ident => ActionFn(28);`
        0, // on "->", error
        -10, // on "::", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "<", reduce `Atom = Ident => ActionFn(28);`
        0, // on "<-", error
        -10, // on "=", reduce `Atom = Ident => ActionFn(28);`
        -10, // on ">", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "Iden", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "Num", reduce `Atom = Ident => ActionFn(28);`
        0, // on "[", error
        -10, // on "]", reduce `Atom = Ident => ActionFn(28);`
        0, // on "array", error
        0, // on "begin", error
        -10, // on "else", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "empty", reduce `Atom = Ident => ActionFn(28);`
        0, // on "empty?", error
        -10, // on "end", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "false", reduce `Atom = Ident => ActionFn(28);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -10, // on "in", reduce `Atom = Ident => ActionFn(28);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -10, // on "then", reduce `Atom = Ident => ActionFn(28);`
        -10, // on "true", reduce `Atom = Ident => ActionFn(28);`
        // State 35
        //     App = (*) App Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Get = (*) App [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "[", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     List = (*) Get [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     List = (*) Get "::" List [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     List = Get "::" (*) List [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        0, // on "begin", error
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        0, // on "fix", error
        0, // on "fun", error
        24, // on "head", goto 23
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 36
        //     Add = (*) Add "+" Mul ["+", "-", "]"]
        //     Add = (*) Add "-" Mul ["+", "-", "]"]
        //     Add = (*) Mul ["+", "-", "]"]
        //     App = (*) App Atom ["(", "*", "+", "-", "::", "Iden", "Num", "]", "empty", "false", "true"]
        //     App = (*) Atom ["(", "*", "+", "-", "::", "Iden", "Num", "]", "empty", "false", "true"]
        //     App = (*) "array" "(" Add "," Add ")" ["(", "*", "+", "-", "::", "Iden", "Num", "]", "empty", "false", "true"]
        //     App = (*) "empty?" Atom ["(", "*", "+", "-", "::", "Iden", "Num", "]", "empty", "false", "true"]
        //     App = (*) "head" Atom ["(", "*", "+", "-", "::", "Iden", "Num", "]", "empty", "false", "true"]
        //     App = (*) "tail" Atom ["(", "*", "+", "-", "::", "Iden", "Num", "]", "empty", "false", "true"]
        //     Atom = (*) Ident ["(", "*", "+", "-", "::", "Iden", "Num", "]", "empty", "false", "true"]
        //     Atom = (*) Int ["(", "*", "+", "-", "::", "Iden", "Num", "]", "empty", "false", "true"]
        //     Atom = (*) "(" Exprs ")" ["(", "*", "+", "-", "::", "Iden", "Num", "]", "empty", "false", "true"]
        //     Atom = (*) "empty" ["(", "*", "+", "-", "::", "Iden", "Num", "]", "empty", "false", "true"]
        //     Atom = (*) "false" ["(", "*", "+", "-", "::", "Iden", "Num", "]", "empty", "false", "true"]
        //     Atom = (*) "true" ["(", "*", "+", "-", "::", "Iden", "Num", "]", "empty", "false", "true"]
        //     Get = (*) App ["*", "+", "-", "::", "]"]
        //     Get = (*) Ident "[" Add "]" ["*", "+", "-", "::", "]"]
        //     Get = Ident "[" (*) Add "]" [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["*", "+", "-", "::", "]"]
        //     Get = Ident "[" (*) Add "]" "<-" Atom [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", "*", "+", "-", "::", "Iden", "Num", "[", "]", "empty", "false", "true"]
        //     Int = (*) Num ["(", "*", "+", "-", "::", "Iden", "Num", "]", "empty", "false", "true"]
        //     List = (*) Get ["*", "+", "-", "]"]
        //     List = (*) Get "::" List ["*", "+", "-", "]"]
        //     Mul = (*) List ["*", "+", "-", "]"]
        //     Mul = (*) Mul "*" List ["*", "+", "-", "]"]
        //     Num = (*) "Num" ["(", "*", "+", "-", "::", "Iden", "Num", "]", "empty", "false", "true"]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        0, // on "begin", error
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        0, // on "fix", error
        0, // on "fun", error
        24, // on "head", goto 23
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 37
        //     App = (*) App Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Get = (*) App [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "[", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     List = (*) Get [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     List = (*) Get "::" List [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Mul = Mul "*" (*) List [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        0, // on "begin", error
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        0, // on "fix", error
        0, // on "fun", error
        24, // on "head", goto 23
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 38
        //     Exprs = Expr (*) [")"]
        0, // on "(", error
        -26, // on ")", reduce `Exprs = Expr => ActionFn(34);`
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 39
        //     Atom = "(" Exprs (*) ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        0, // on "(", error
        58, // on ")", goto 57
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 40
        //     Add = (*) Add "+" Mul ["+", ",", "-"]
        //     Add = (*) Add "-" Mul ["+", ",", "-"]
        //     Add = (*) Mul ["+", ",", "-"]
        //     App = (*) App Atom ["(", "*", "+", ",", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) Atom ["(", "*", "+", ",", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) "array" "(" Add "," Add ")" ["(", "*", "+", ",", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     App = "array" "(" (*) Add "," Add ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", "*", "+", ",", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) "head" Atom ["(", "*", "+", ",", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) "tail" Atom ["(", "*", "+", ",", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) Ident ["(", "*", "+", ",", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) Int ["(", "*", "+", ",", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "(" Exprs ")" ["(", "*", "+", ",", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "empty" ["(", "*", "+", ",", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "false" ["(", "*", "+", ",", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "true" ["(", "*", "+", ",", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     Get = (*) App ["*", "+", ",", "-", "::"]
        //     Get = (*) Ident "[" Add "]" ["*", "+", ",", "-", "::"]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["*", "+", ",", "-", "::"]
        //     Ident = (*) "Iden" ["(", "*", "+", ",", "-", "::", "Iden", "Num", "[", "empty", "false", "true"]
        //     Int = (*) Num ["(", "*", "+", ",", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     List = (*) Get ["*", "+", ",", "-"]
        //     List = (*) Get "::" List ["*", "+", ",", "-"]
        //     Mul = (*) List ["*", "+", ",", "-"]
        //     Mul = (*) Mul "*" List ["*", "+", ",", "-"]
        //     Num = (*) "Num" ["(", "*", "+", ",", "-", "::", "Iden", "Num", "empty", "false", "true"]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        0, // on "begin", error
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        0, // on "fix", error
        0, // on "fun", error
        24, // on "head", goto 23
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 41
        //     Expr = "begin" Expr (*) "end" [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        60, // on "end", goto 59
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 42
        //     App = "empty?" Atom (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        -7, // on "(", reduce `App = "empty?", Atom => ActionFn(25);`
        -7, // on ")", reduce `App = "empty?", Atom => ActionFn(25);`
        -7, // on "*", reduce `App = "empty?", Atom => ActionFn(25);`
        -7, // on "+", reduce `App = "empty?", Atom => ActionFn(25);`
        -7, // on ",", reduce `App = "empty?", Atom => ActionFn(25);`
        -7, // on "-", reduce `App = "empty?", Atom => ActionFn(25);`
        0, // on "->", error
        -7, // on "::", reduce `App = "empty?", Atom => ActionFn(25);`
        -7, // on "<", reduce `App = "empty?", Atom => ActionFn(25);`
        0, // on "<-", error
        -7, // on "=", reduce `App = "empty?", Atom => ActionFn(25);`
        -7, // on ">", reduce `App = "empty?", Atom => ActionFn(25);`
        -7, // on "Iden", reduce `App = "empty?", Atom => ActionFn(25);`
        -7, // on "Num", reduce `App = "empty?", Atom => ActionFn(25);`
        0, // on "[", error
        -7, // on "]", reduce `App = "empty?", Atom => ActionFn(25);`
        0, // on "array", error
        0, // on "begin", error
        -7, // on "else", reduce `App = "empty?", Atom => ActionFn(25);`
        -7, // on "empty", reduce `App = "empty?", Atom => ActionFn(25);`
        0, // on "empty?", error
        -7, // on "end", reduce `App = "empty?", Atom => ActionFn(25);`
        -7, // on "false", reduce `App = "empty?", Atom => ActionFn(25);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -7, // on "in", reduce `App = "empty?", Atom => ActionFn(25);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -7, // on "then", reduce `App = "empty?", Atom => ActionFn(25);`
        -7, // on "true", reduce `App = "empty?", Atom => ActionFn(25);`
        // State 43
        //     Expr = "fix" Ident (*) "->" Expr [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        61, // on "->", goto 60
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 44
        //     Expr = "fun" Ident (*) "->" Expr [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        62, // on "->", goto 61
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 45
        //     App = "head" Atom (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        -5, // on "(", reduce `App = "head", Atom => ActionFn(23);`
        -5, // on ")", reduce `App = "head", Atom => ActionFn(23);`
        -5, // on "*", reduce `App = "head", Atom => ActionFn(23);`
        -5, // on "+", reduce `App = "head", Atom => ActionFn(23);`
        -5, // on ",", reduce `App = "head", Atom => ActionFn(23);`
        -5, // on "-", reduce `App = "head", Atom => ActionFn(23);`
        0, // on "->", error
        -5, // on "::", reduce `App = "head", Atom => ActionFn(23);`
        -5, // on "<", reduce `App = "head", Atom => ActionFn(23);`
        0, // on "<-", error
        -5, // on "=", reduce `App = "head", Atom => ActionFn(23);`
        -5, // on ">", reduce `App = "head", Atom => ActionFn(23);`
        -5, // on "Iden", reduce `App = "head", Atom => ActionFn(23);`
        -5, // on "Num", reduce `App = "head", Atom => ActionFn(23);`
        0, // on "[", error
        -5, // on "]", reduce `App = "head", Atom => ActionFn(23);`
        0, // on "array", error
        0, // on "begin", error
        -5, // on "else", reduce `App = "head", Atom => ActionFn(23);`
        -5, // on "empty", reduce `App = "head", Atom => ActionFn(23);`
        0, // on "empty?", error
        -5, // on "end", reduce `App = "head", Atom => ActionFn(23);`
        -5, // on "false", reduce `App = "head", Atom => ActionFn(23);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -5, // on "in", reduce `App = "head", Atom => ActionFn(23);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -5, // on "then", reduce `App = "head", Atom => ActionFn(23);`
        -5, // on "true", reduce `App = "head", Atom => ActionFn(23);`
        // State 46
        //     Expr = "if" Expr (*) "then" Expr "else" Expr [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        63, // on "then", goto 62
        0, // on "true", error
        // State 47
        //     Expr = "let" Ident (*) "=" Expr "in" Expr [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        64, // on "=", goto 63
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 48
        //     App = "tail" Atom (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        -6, // on "(", reduce `App = "tail", Atom => ActionFn(24);`
        -6, // on ")", reduce `App = "tail", Atom => ActionFn(24);`
        -6, // on "*", reduce `App = "tail", Atom => ActionFn(24);`
        -6, // on "+", reduce `App = "tail", Atom => ActionFn(24);`
        -6, // on ",", reduce `App = "tail", Atom => ActionFn(24);`
        -6, // on "-", reduce `App = "tail", Atom => ActionFn(24);`
        0, // on "->", error
        -6, // on "::", reduce `App = "tail", Atom => ActionFn(24);`
        -6, // on "<", reduce `App = "tail", Atom => ActionFn(24);`
        0, // on "<-", error
        -6, // on "=", reduce `App = "tail", Atom => ActionFn(24);`
        -6, // on ">", reduce `App = "tail", Atom => ActionFn(24);`
        -6, // on "Iden", reduce `App = "tail", Atom => ActionFn(24);`
        -6, // on "Num", reduce `App = "tail", Atom => ActionFn(24);`
        0, // on "[", error
        -6, // on "]", reduce `App = "tail", Atom => ActionFn(24);`
        0, // on "array", error
        0, // on "begin", error
        -6, // on "else", reduce `App = "tail", Atom => ActionFn(24);`
        -6, // on "empty", reduce `App = "tail", Atom => ActionFn(24);`
        0, // on "empty?", error
        -6, // on "end", reduce `App = "tail", Atom => ActionFn(24);`
        -6, // on "false", reduce `App = "tail", Atom => ActionFn(24);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -6, // on "in", reduce `App = "tail", Atom => ActionFn(24);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -6, // on "then", reduce `App = "tail", Atom => ActionFn(24);`
        -6, // on "true", reduce `App = "tail", Atom => ActionFn(24);`
        // State 49
        //     Add = Add "+" Mul (*) [")", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Mul = Mul (*) "*" List [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -2, // on ")", reduce `Add = Add, "+", Mul => ActionFn(13);`
        38, // on "*", goto 37
        -2, // on "+", reduce `Add = Add, "+", Mul => ActionFn(13);`
        -2, // on ",", reduce `Add = Add, "+", Mul => ActionFn(13);`
        -2, // on "-", reduce `Add = Add, "+", Mul => ActionFn(13);`
        0, // on "->", error
        0, // on "::", error
        -2, // on "<", reduce `Add = Add, "+", Mul => ActionFn(13);`
        0, // on "<-", error
        -2, // on "=", reduce `Add = Add, "+", Mul => ActionFn(13);`
        -2, // on ">", reduce `Add = Add, "+", Mul => ActionFn(13);`
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        -2, // on "]", reduce `Add = Add, "+", Mul => ActionFn(13);`
        0, // on "array", error
        0, // on "begin", error
        -2, // on "else", reduce `Add = Add, "+", Mul => ActionFn(13);`
        0, // on "empty", error
        0, // on "empty?", error
        -2, // on "end", reduce `Add = Add, "+", Mul => ActionFn(13);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -2, // on "in", reduce `Add = Add, "+", Mul => ActionFn(13);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -2, // on "then", reduce `Add = Add, "+", Mul => ActionFn(13);`
        0, // on "true", error
        // State 50
        //     Add = Add "-" Mul (*) [")", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Mul = Mul (*) "*" List [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -1, // on ")", reduce `Add = Add, "-", Mul => ActionFn(12);`
        38, // on "*", goto 37
        -1, // on "+", reduce `Add = Add, "-", Mul => ActionFn(12);`
        -1, // on ",", reduce `Add = Add, "-", Mul => ActionFn(12);`
        -1, // on "-", reduce `Add = Add, "-", Mul => ActionFn(12);`
        0, // on "->", error
        0, // on "::", error
        -1, // on "<", reduce `Add = Add, "-", Mul => ActionFn(12);`
        0, // on "<-", error
        -1, // on "=", reduce `Add = Add, "-", Mul => ActionFn(12);`
        -1, // on ">", reduce `Add = Add, "-", Mul => ActionFn(12);`
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        -1, // on "]", reduce `Add = Add, "-", Mul => ActionFn(12);`
        0, // on "array", error
        0, // on "begin", error
        -1, // on "else", reduce `Add = Add, "-", Mul => ActionFn(12);`
        0, // on "empty", error
        0, // on "empty?", error
        -1, // on "end", reduce `Add = Add, "-", Mul => ActionFn(12);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -1, // on "in", reduce `Add = Add, "-", Mul => ActionFn(12);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -1, // on "then", reduce `Add = Add, "-", Mul => ActionFn(12);`
        0, // on "true", error
        // State 51
        //     Cmp = Add "<" Cmp (*) [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -17, // on ")", reduce `Cmp = Add, "<", Cmp => ActionFn(9);`
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        -17, // on "else", reduce `Cmp = Add, "<", Cmp => ActionFn(9);`
        0, // on "empty", error
        0, // on "empty?", error
        -17, // on "end", reduce `Cmp = Add, "<", Cmp => ActionFn(9);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -17, // on "in", reduce `Cmp = Add, "<", Cmp => ActionFn(9);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -17, // on "then", reduce `Cmp = Add, "<", Cmp => ActionFn(9);`
        0, // on "true", error
        // State 52
        //     Cmp = Add "=" Cmp (*) [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -16, // on ")", reduce `Cmp = Add, "=", Cmp => ActionFn(8);`
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        -16, // on "else", reduce `Cmp = Add, "=", Cmp => ActionFn(8);`
        0, // on "empty", error
        0, // on "empty?", error
        -16, // on "end", reduce `Cmp = Add, "=", Cmp => ActionFn(8);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -16, // on "in", reduce `Cmp = Add, "=", Cmp => ActionFn(8);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -16, // on "then", reduce `Cmp = Add, "=", Cmp => ActionFn(8);`
        0, // on "true", error
        // State 53
        //     Cmp = Add ">" Cmp (*) [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -18, // on ")", reduce `Cmp = Add, ">", Cmp => ActionFn(10);`
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        -18, // on "else", reduce `Cmp = Add, ">", Cmp => ActionFn(10);`
        0, // on "empty", error
        0, // on "empty?", error
        -18, // on "end", reduce `Cmp = Add, ">", Cmp => ActionFn(10);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -18, // on "in", reduce `Cmp = Add, ">", Cmp => ActionFn(10);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -18, // on "then", reduce `Cmp = Add, ">", Cmp => ActionFn(10);`
        0, // on "true", error
        // State 54
        //     List = Get "::" List (*) [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -32, // on ")", reduce `List = Get, "::", List => ActionFn(17);`
        -32, // on "*", reduce `List = Get, "::", List => ActionFn(17);`
        -32, // on "+", reduce `List = Get, "::", List => ActionFn(17);`
        -32, // on ",", reduce `List = Get, "::", List => ActionFn(17);`
        -32, // on "-", reduce `List = Get, "::", List => ActionFn(17);`
        0, // on "->", error
        0, // on "::", error
        -32, // on "<", reduce `List = Get, "::", List => ActionFn(17);`
        0, // on "<-", error
        -32, // on "=", reduce `List = Get, "::", List => ActionFn(17);`
        -32, // on ">", reduce `List = Get, "::", List => ActionFn(17);`
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        -32, // on "]", reduce `List = Get, "::", List => ActionFn(17);`
        0, // on "array", error
        0, // on "begin", error
        -32, // on "else", reduce `List = Get, "::", List => ActionFn(17);`
        0, // on "empty", error
        0, // on "empty?", error
        -32, // on "end", reduce `List = Get, "::", List => ActionFn(17);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -32, // on "in", reduce `List = Get, "::", List => ActionFn(17);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -32, // on "then", reduce `List = Get, "::", List => ActionFn(17);`
        0, // on "true", error
        // State 55
        //     Add = Add (*) "+" Mul ["+", "-", "]"]
        //     Add = Add (*) "-" Mul ["+", "-", "]"]
        //     Get = Ident "[" Add (*) "]" [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Get = Ident "[" Add (*) "]" "<-" Atom [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        29, // on "+", goto 28
        0, // on ",", error
        30, // on "-", goto 29
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        65, // on "]", goto 64
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 56
        //     Mul = Mul "*" List (*) [")", "*", "+", ",", "-", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -34, // on ")", reduce `Mul = Mul, "*", List => ActionFn(15);`
        -34, // on "*", reduce `Mul = Mul, "*", List => ActionFn(15);`
        -34, // on "+", reduce `Mul = Mul, "*", List => ActionFn(15);`
        -34, // on ",", reduce `Mul = Mul, "*", List => ActionFn(15);`
        -34, // on "-", reduce `Mul = Mul, "*", List => ActionFn(15);`
        0, // on "->", error
        0, // on "::", error
        -34, // on "<", reduce `Mul = Mul, "*", List => ActionFn(15);`
        0, // on "<-", error
        -34, // on "=", reduce `Mul = Mul, "*", List => ActionFn(15);`
        -34, // on ">", reduce `Mul = Mul, "*", List => ActionFn(15);`
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        -34, // on "]", reduce `Mul = Mul, "*", List => ActionFn(15);`
        0, // on "array", error
        0, // on "begin", error
        -34, // on "else", reduce `Mul = Mul, "*", List => ActionFn(15);`
        0, // on "empty", error
        0, // on "empty?", error
        -34, // on "end", reduce `Mul = Mul, "*", List => ActionFn(15);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -34, // on "in", reduce `Mul = Mul, "*", List => ActionFn(15);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -34, // on "then", reduce `Mul = Mul, "*", List => ActionFn(15);`
        0, // on "true", error
        // State 57
        //     Atom = "(" Exprs ")" (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        -14, // on "(", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        -14, // on ")", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        -14, // on "*", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        -14, // on "+", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        -14, // on ",", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        -14, // on "-", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        0, // on "->", error
        -14, // on "::", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        -14, // on "<", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        0, // on "<-", error
        -14, // on "=", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        -14, // on ">", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        -14, // on "Iden", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        -14, // on "Num", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        0, // on "[", error
        -14, // on "]", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        0, // on "array", error
        0, // on "begin", error
        -14, // on "else", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        -14, // on "empty", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        0, // on "empty?", error
        -14, // on "end", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        -14, // on "false", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -14, // on "in", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -14, // on "then", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        -14, // on "true", reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        // State 58
        //     Add = Add (*) "+" Mul ["+", ",", "-"]
        //     Add = Add (*) "-" Mul ["+", ",", "-"]
        //     App = "array" "(" Add (*) "," Add ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        29, // on "+", goto 28
        66, // on ",", goto 65
        30, // on "-", goto 29
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 59
        //     Expr = "begin" Expr "end" (*) [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -24, // on ")", reduce `Expr = "begin", Expr, "end" => ActionFn(6);`
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        -24, // on "else", reduce `Expr = "begin", Expr, "end" => ActionFn(6);`
        0, // on "empty", error
        0, // on "empty?", error
        -24, // on "end", reduce `Expr = "begin", Expr, "end" => ActionFn(6);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -24, // on "in", reduce `Expr = "begin", Expr, "end" => ActionFn(6);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -24, // on "then", reduce `Expr = "begin", Expr, "end" => ActionFn(6);`
        0, // on "true", error
        // State 60
        //     Add = (*) Add "+" Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Add = (*) Add "-" Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Add = (*) Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Cmp = (*) Add [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add "<" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add "=" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add ">" Cmp [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) Cmp [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "begin" Expr "end" [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "fix" Ident "->" Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = "fix" Ident "->" (*) Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "fun" Ident "->" Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr [")", "else", "end", "in", "then", EOF]
        //     Get = (*) App [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "[", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     List = (*) Get [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     List = (*) Get "::" List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        18, // on "begin", goto 17
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        22, // on "fix", goto 21
        23, // on "fun", goto 22
        24, // on "head", goto 23
        25, // on "if", goto 24
        0, // on "in", error
        26, // on "let", goto 25
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 61
        //     Add = (*) Add "+" Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Add = (*) Add "-" Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Add = (*) Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Cmp = (*) Add [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add "<" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add "=" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add ">" Cmp [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) Cmp [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "begin" Expr "end" [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "fix" Ident "->" Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "fun" Ident "->" Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = "fun" Ident "->" (*) Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr [")", "else", "end", "in", "then", EOF]
        //     Get = (*) App [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "[", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     List = (*) Get [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     List = (*) Get "::" List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        18, // on "begin", goto 17
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        22, // on "fix", goto 21
        23, // on "fun", goto 22
        24, // on "head", goto 23
        25, // on "if", goto 24
        0, // on "in", error
        26, // on "let", goto 25
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 62
        //     Add = (*) Add "+" Mul ["+", "-", "<", "=", ">", "else"]
        //     Add = (*) Add "-" Mul ["+", "-", "<", "=", ">", "else"]
        //     Add = (*) Mul ["+", "-", "<", "=", ">", "else"]
        //     App = (*) App Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     App = (*) Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     App = (*) "array" "(" Add "," Add ")" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     App = (*) "empty?" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     App = (*) "head" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     App = (*) "tail" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Atom = (*) Ident ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Atom = (*) Int ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Atom = (*) "(" Exprs ")" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Atom = (*) "empty" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Atom = (*) "false" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Atom = (*) "true" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Cmp = (*) Add ["else"]
        //     Cmp = (*) Add "<" Cmp ["else"]
        //     Cmp = (*) Add "=" Cmp ["else"]
        //     Cmp = (*) Add ">" Cmp ["else"]
        //     Expr = (*) Cmp ["else"]
        //     Expr = (*) "begin" Expr "end" ["else"]
        //     Expr = (*) "fix" Ident "->" Expr ["else"]
        //     Expr = (*) "fun" Ident "->" Expr ["else"]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr ["else"]
        //     Expr = "if" Expr "then" (*) Expr "else" Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr ["else"]
        //     Get = (*) App ["*", "+", "-", "::", "<", "=", ">", "else"]
        //     Get = (*) Ident "[" Add "]" ["*", "+", "-", "::", "<", "=", ">", "else"]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["*", "+", "-", "::", "<", "=", ">", "else"]
        //     Ident = (*) "Iden" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "[", "else", "empty", "false", "true"]
        //     Int = (*) Num ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     List = (*) Get ["*", "+", "-", "<", "=", ">", "else"]
        //     List = (*) Get "::" List ["*", "+", "-", "<", "=", ">", "else"]
        //     Mul = (*) List ["*", "+", "-", "<", "=", ">", "else"]
        //     Mul = (*) Mul "*" List ["*", "+", "-", "<", "=", ">", "else"]
        //     Num = (*) "Num" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        18, // on "begin", goto 17
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        22, // on "fix", goto 21
        23, // on "fun", goto 22
        24, // on "head", goto 23
        25, // on "if", goto 24
        0, // on "in", error
        26, // on "let", goto 25
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 63
        //     Add = (*) Add "+" Mul ["+", "-", "<", "=", ">", "in"]
        //     Add = (*) Add "-" Mul ["+", "-", "<", "=", ">", "in"]
        //     Add = (*) Mul ["+", "-", "<", "=", ">", "in"]
        //     App = (*) App Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     App = (*) Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     App = (*) "array" "(" Add "," Add ")" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     App = (*) "empty?" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     App = (*) "head" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     App = (*) "tail" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Atom = (*) Ident ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Atom = (*) Int ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Atom = (*) "(" Exprs ")" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Atom = (*) "empty" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Atom = (*) "false" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Atom = (*) "true" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Cmp = (*) Add ["in"]
        //     Cmp = (*) Add "<" Cmp ["in"]
        //     Cmp = (*) Add "=" Cmp ["in"]
        //     Cmp = (*) Add ">" Cmp ["in"]
        //     Expr = (*) Cmp ["in"]
        //     Expr = (*) "begin" Expr "end" ["in"]
        //     Expr = (*) "fix" Ident "->" Expr ["in"]
        //     Expr = (*) "fun" Ident "->" Expr ["in"]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr ["in"]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr ["in"]
        //     Expr = "let" Ident "=" (*) Expr "in" Expr [")", "else", "end", "in", "then", EOF]
        //     Get = (*) App ["*", "+", "-", "::", "<", "=", ">", "in"]
        //     Get = (*) Ident "[" Add "]" ["*", "+", "-", "::", "<", "=", ">", "in"]
        //     Get = (*) Ident "[" Add "]" "<-" Atom ["*", "+", "-", "::", "<", "=", ">", "in"]
        //     Ident = (*) "Iden" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "[", "empty", "false", "in", "true"]
        //     Int = (*) Num ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     List = (*) Get ["*", "+", "-", "<", "=", ">", "in"]
        //     List = (*) Get "::" List ["*", "+", "-", "<", "=", ">", "in"]
        //     Mul = (*) List ["*", "+", "-", "<", "=", ">", "in"]
        //     Mul = (*) Mul "*" List ["*", "+", "-", "<", "=", ">", "in"]
        //     Num = (*) "Num" ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        18, // on "begin", goto 17
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        22, // on "fix", goto 21
        23, // on "fun", goto 22
        24, // on "head", goto 23
        25, // on "if", goto 24
        0, // on "in", error
        26, // on "let", goto 25
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 64
        //     Get = Ident "[" Add "]" (*) [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Get = Ident "[" Add "]" (*) "<-" Atom [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -28, // on ")", reduce `Get = Ident, "[", Add, "]" => ActionFn(20);`
        -28, // on "*", reduce `Get = Ident, "[", Add, "]" => ActionFn(20);`
        -28, // on "+", reduce `Get = Ident, "[", Add, "]" => ActionFn(20);`
        -28, // on ",", reduce `Get = Ident, "[", Add, "]" => ActionFn(20);`
        -28, // on "-", reduce `Get = Ident, "[", Add, "]" => ActionFn(20);`
        0, // on "->", error
        -28, // on "::", reduce `Get = Ident, "[", Add, "]" => ActionFn(20);`
        -28, // on "<", reduce `Get = Ident, "[", Add, "]" => ActionFn(20);`
        71, // on "<-", goto 70
        -28, // on "=", reduce `Get = Ident, "[", Add, "]" => ActionFn(20);`
        -28, // on ">", reduce `Get = Ident, "[", Add, "]" => ActionFn(20);`
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        -28, // on "]", reduce `Get = Ident, "[", Add, "]" => ActionFn(20);`
        0, // on "array", error
        0, // on "begin", error
        -28, // on "else", reduce `Get = Ident, "[", Add, "]" => ActionFn(20);`
        0, // on "empty", error
        0, // on "empty?", error
        -28, // on "end", reduce `Get = Ident, "[", Add, "]" => ActionFn(20);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -28, // on "in", reduce `Get = Ident, "[", Add, "]" => ActionFn(20);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -28, // on "then", reduce `Get = Ident, "[", Add, "]" => ActionFn(20);`
        0, // on "true", error
        // State 65
        //     Add = (*) Add "+" Mul [")", "+", "-"]
        //     Add = (*) Add "-" Mul [")", "+", "-"]
        //     Add = (*) Mul [")", "+", "-"]
        //     App = (*) App Atom ["(", ")", "*", "+", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) Atom ["(", ")", "*", "+", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) "array" "(" Add "," Add ")" ["(", ")", "*", "+", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     App = "array" "(" Add "," (*) Add ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) "head" Atom ["(", ")", "*", "+", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) Ident ["(", ")", "*", "+", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) Int ["(", ")", "*", "+", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "empty" ["(", ")", "*", "+", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "false" ["(", ")", "*", "+", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "true" ["(", ")", "*", "+", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     Get = (*) App [")", "*", "+", "-", "::"]
        //     Get = (*) Ident "[" Add "]" [")", "*", "+", "-", "::"]
        //     Get = (*) Ident "[" Add "]" "<-" Atom [")", "*", "+", "-", "::"]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", "-", "::", "Iden", "Num", "[", "empty", "false", "true"]
        //     Int = (*) Num ["(", ")", "*", "+", "-", "::", "Iden", "Num", "empty", "false", "true"]
        //     List = (*) Get [")", "*", "+", "-"]
        //     List = (*) Get "::" List [")", "*", "+", "-"]
        //     Mul = (*) List [")", "*", "+", "-"]
        //     Mul = (*) Mul "*" List [")", "*", "+", "-"]
        //     Num = (*) "Num" ["(", ")", "*", "+", "-", "::", "Iden", "Num", "empty", "false", "true"]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        0, // on "begin", error
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        0, // on "fix", error
        0, // on "fun", error
        24, // on "head", goto 23
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 66
        //     Expr = "fix" Ident "->" Expr (*) [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -23, // on ")", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(5);`
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        -23, // on "else", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(5);`
        0, // on "empty", error
        0, // on "empty?", error
        -23, // on "end", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(5);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -23, // on "in", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(5);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -23, // on "then", reduce `Expr = "fix", Ident, "->", Expr => ActionFn(5);`
        0, // on "true", error
        // State 67
        //     Expr = "fun" Ident "->" Expr (*) [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -22, // on ")", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(4);`
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        -22, // on "else", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(4);`
        0, // on "empty", error
        0, // on "empty?", error
        -22, // on "end", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(4);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -22, // on "in", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(4);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -22, // on "then", reduce `Expr = "fun", Ident, "->", Expr => ActionFn(4);`
        0, // on "true", error
        // State 68
        //     Expr = "if" Expr "then" Expr (*) "else" Expr [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        73, // on "else", goto 72
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 69
        //     Expr = "let" Ident "=" Expr (*) "in" Expr [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        74, // on "in", goto 73
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 70
        //     Atom = (*) Ident [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Atom = (*) Int [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Atom = (*) "(" Exprs ")" [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Atom = (*) "empty" [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Atom = (*) "false" [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Atom = (*) "true" [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Get = Ident "[" Add "]" "<-" (*) Atom [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Int = (*) Num [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        //     Num = (*) "Num" [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        19, // on "empty", goto 18
        0, // on "empty?", error
        0, // on "end", error
        21, // on "false", goto 20
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        28, // on "true", goto 27
        // State 71
        //     Add = Add (*) "+" Mul [")", "+", "-"]
        //     Add = Add (*) "-" Mul [")", "+", "-"]
        //     App = "array" "(" Add "," Add (*) ")" ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        0, // on "(", error
        76, // on ")", goto 75
        0, // on "*", error
        29, // on "+", goto 28
        0, // on ",", error
        30, // on "-", goto 29
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "end", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 72
        //     Add = (*) Add "+" Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Add = (*) Add "-" Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Add = (*) Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Cmp = (*) Add [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add "<" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add "=" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add ">" Cmp [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) Cmp [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "begin" Expr "end" [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "fix" Ident "->" Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "fun" Ident "->" Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = "if" Expr "then" Expr "else" (*) Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr [")", "else", "end", "in", "then", EOF]
        //     Get = (*) App [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "[", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     List = (*) Get [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     List = (*) Get "::" List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        18, // on "begin", goto 17
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        22, // on "fix", goto 21
        23, // on "fun", goto 22
        24, // on "head", goto 23
        25, // on "if", goto 24
        0, // on "in", error
        26, // on "let", goto 25
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 73
        //     Add = (*) Add "+" Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Add = (*) Add "-" Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Add = (*) Mul [")", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "array" "(" Add "," Add ")" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exprs ")" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Cmp = (*) Add [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add "<" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add "=" Cmp [")", "else", "end", "in", "then", EOF]
        //     Cmp = (*) Add ">" Cmp [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) Cmp [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "begin" Expr "end" [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "fix" Ident "->" Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "fun" Ident "->" Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "if" Expr "then" Expr "else" Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = (*) "let" Ident "=" Expr "in" Expr [")", "else", "end", "in", "then", EOF]
        //     Expr = "let" Ident "=" Expr "in" (*) Expr [")", "else", "end", "in", "then", EOF]
        //     Get = (*) App [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Get = (*) Ident "[" Add "]" "<-" Atom [")", "*", "+", "-", "::", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "[", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        //     List = (*) Get [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     List = (*) Get "::" List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", "-", "<", "=", ">", "else", "end", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "end", "false", "in", "then", "true", EOF]
        14, // on "(", goto 13
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        15, // on "Iden", goto 14
        16, // on "Num", goto 15
        0, // on "[", error
        0, // on "]", error
        17, // on "array", goto 16
        18, // on "begin", goto 17
        0, // on "else", error
        19, // on "empty", goto 18
        20, // on "empty?", goto 19
        0, // on "end", error
        21, // on "false", goto 20
        22, // on "fix", goto 21
        23, // on "fun", goto 22
        24, // on "head", goto 23
        25, // on "if", goto 24
        0, // on "in", error
        26, // on "let", goto 25
        0, // on "set", error
        27, // on "tail", goto 26
        0, // on "then", error
        28, // on "true", goto 27
        // State 74
        //     Get = Ident "[" Add "]" "<-" Atom (*) [")", "*", "+", ",", "-", "::", "<", "=", ">", "]", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -27, // on ")", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(19);`
        -27, // on "*", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(19);`
        -27, // on "+", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(19);`
        -27, // on ",", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(19);`
        -27, // on "-", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(19);`
        0, // on "->", error
        -27, // on "::", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(19);`
        -27, // on "<", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(19);`
        0, // on "<-", error
        -27, // on "=", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(19);`
        -27, // on ">", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(19);`
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        -27, // on "]", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(19);`
        0, // on "array", error
        0, // on "begin", error
        -27, // on "else", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(19);`
        0, // on "empty", error
        0, // on "empty?", error
        -27, // on "end", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(19);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -27, // on "in", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(19);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -27, // on "then", reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(19);`
        0, // on "true", error
        // State 75
        //     App = "array" "(" Add "," Add ")" (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "]", "else", "empty", "end", "false", "in", "then", "true", EOF]
        -4, // on "(", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        -4, // on ")", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        -4, // on "*", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        -4, // on "+", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        -4, // on ",", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        -4, // on "-", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        0, // on "->", error
        -4, // on "::", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        -4, // on "<", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        0, // on "<-", error
        -4, // on "=", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        -4, // on ">", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        -4, // on "Iden", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        -4, // on "Num", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        0, // on "[", error
        -4, // on "]", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        0, // on "array", error
        0, // on "begin", error
        -4, // on "else", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        -4, // on "empty", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        0, // on "empty?", error
        -4, // on "end", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        -4, // on "false", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -4, // on "in", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -4, // on "then", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        -4, // on "true", reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        // State 76
        //     Expr = "if" Expr "then" Expr "else" Expr (*) [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -20, // on ")", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        -20, // on "else", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        0, // on "empty", error
        0, // on "empty?", error
        -20, // on "end", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -20, // on "in", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -20, // on "then", reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        0, // on "true", error
        // State 77
        //     Expr = "let" Ident "=" Expr "in" Expr (*) [")", "else", "end", "in", "then", EOF]
        0, // on "(", error
        -21, // on ")", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on "::", error
        0, // on "<", error
        0, // on "<-", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "[", error
        0, // on "]", error
        0, // on "array", error
        0, // on "begin", error
        -21, // on "else", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        0, // on "empty", error
        0, // on "empty?", error
        -21, // on "end", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -21, // on "in", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        0, // on "let", error
        0, // on "set", error
        0, // on "tail", error
        -21, // on "then", reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
        0, // on "true", error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -19, // on EOF, reduce `Cmp = Add => ActionFn(11);`
        -29, // on EOF, reduce `Get = App => ActionFn(21);`
        -9, // on EOF, reduce `App = Atom => ActionFn(27);`
        -25, // on EOF, reduce `Expr = Cmp => ActionFn(7);`
        -37, // on EOF, reduce `Program = Expr => ActionFn(1);`
        -33, // on EOF, reduce `List = Get => ActionFn(18);`
        -10, // on EOF, reduce `Atom = Ident => ActionFn(28);`
        -15, // on EOF, reduce `Atom = Int => ActionFn(33);`
        -35, // on EOF, reduce `Mul = List => ActionFn(16);`
        -3, // on EOF, reduce `Add = Mul => ActionFn(14);`
        -31, // on EOF, reduce `Int = Num => ActionFn(35);`
        -38, // on EOF, reduce `__Program = Program => ActionFn(0);`
        0, // on EOF, error
        -30, // on EOF, reduce `Ident = "Iden" => ActionFn(36);`
        -36, // on EOF, reduce `Num = "Num" => ActionFn(37);`
        0, // on EOF, error
        0, // on EOF, error
        -13, // on EOF, reduce `Atom = "empty" => ActionFn(31);`
        0, // on EOF, error
        -12, // on EOF, reduce `Atom = "false" => ActionFn(30);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -11, // on EOF, reduce `Atom = "true" => ActionFn(29);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -8, // on EOF, reduce `App = App, Atom => ActionFn(26);`
        -10, // on EOF, reduce `Atom = Ident => ActionFn(28);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -7, // on EOF, reduce `App = "empty?", Atom => ActionFn(25);`
        0, // on EOF, error
        0, // on EOF, error
        -5, // on EOF, reduce `App = "head", Atom => ActionFn(23);`
        0, // on EOF, error
        0, // on EOF, error
        -6, // on EOF, reduce `App = "tail", Atom => ActionFn(24);`
        -2, // on EOF, reduce `Add = Add, "+", Mul => ActionFn(13);`
        -1, // on EOF, reduce `Add = Add, "-", Mul => ActionFn(12);`
        -17, // on EOF, reduce `Cmp = Add, "<", Cmp => ActionFn(9);`
        -16, // on EOF, reduce `Cmp = Add, "=", Cmp => ActionFn(8);`
        -18, // on EOF, reduce `Cmp = Add, ">", Cmp => ActionFn(10);`
        -32, // on EOF, reduce `List = Get, "::", List => ActionFn(17);`
        0, // on EOF, error
        -34, // on EOF, reduce `Mul = Mul, "*", List => ActionFn(15);`
        -14, // on EOF, reduce `Atom = "(", Exprs, ")" => ActionFn(32);`
        0, // on EOF, error
        -24, // on EOF, reduce `Expr = "begin", Expr, "end" => ActionFn(6);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -28, // on EOF, reduce `Get = Ident, "[", Add, "]" => ActionFn(20);`
        0, // on EOF, error
        -23, // on EOF, reduce `Expr = "fix", Ident, "->", Expr => ActionFn(5);`
        -22, // on EOF, reduce `Expr = "fun", Ident, "->", Expr => ActionFn(4);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -27, // on EOF, reduce `Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(19);`
        -4, // on EOF, reduce `App = "array", "(", Add, ",", Add, ")" => ActionFn(22);`
        -20, // on EOF, reduce `Expr = "if", Expr, "then", Expr, "else", Expr => ActionFn(2);`
        -21, // on EOF, reduce `Expr = "let", Ident, "=", Expr, "in", Expr => ActionFn(3);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        6, // on Expr, goto 5
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        11, // on Mul, goto 10
        12, // on Num, goto 11
        13, // on Program, goto 12
        0, // on __Program, error
        // State 1
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 2
        0, // on Add, error
        0, // on App, error
        34, // on Atom, goto 33
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        35, // on Ident, goto 34
        9, // on Int, goto 8
        0, // on List, error
        0, // on Mul, error
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 3
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 4
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 5
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 6
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 7
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 8
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 9
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 10
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 11
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 12
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 13
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        39, // on Expr, goto 38
        40, // on Exprs, goto 39
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        11, // on Mul, goto 10
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 14
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 15
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 16
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 17
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        42, // on Expr, goto 41
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        11, // on Mul, goto 10
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 18
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 19
        0, // on Add, error
        0, // on App, error
        43, // on Atom, goto 42
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        35, // on Ident, goto 34
        9, // on Int, goto 8
        0, // on List, error
        0, // on Mul, error
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 20
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 21
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        44, // on Ident, goto 43
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 22
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        45, // on Ident, goto 44
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 23
        0, // on Add, error
        0, // on App, error
        46, // on Atom, goto 45
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        35, // on Ident, goto 34
        9, // on Int, goto 8
        0, // on List, error
        0, // on Mul, error
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 24
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        47, // on Expr, goto 46
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        11, // on Mul, goto 10
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 25
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        48, // on Ident, goto 47
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 26
        0, // on Add, error
        0, // on App, error
        49, // on Atom, goto 48
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        35, // on Ident, goto 34
        9, // on Int, goto 8
        0, // on List, error
        0, // on Mul, error
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 27
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 28
        0, // on Add, error
        3, // on App, goto 2
        4, // on Atom, goto 3
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        50, // on Mul, goto 49
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 29
        0, // on Add, error
        3, // on App, goto 2
        4, // on Atom, goto 3
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        51, // on Mul, goto 50
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 30
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        52, // on Cmp, goto 51
        0, // on Expr, error
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        11, // on Mul, goto 10
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 31
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        53, // on Cmp, goto 52
        0, // on Expr, error
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        11, // on Mul, goto 10
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 32
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        54, // on Cmp, goto 53
        0, // on Expr, error
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        11, // on Mul, goto 10
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 33
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 34
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 35
        0, // on Add, error
        3, // on App, goto 2
        4, // on Atom, goto 3
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        55, // on List, goto 54
        0, // on Mul, error
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 36
        56, // on Add, goto 55
        3, // on App, goto 2
        4, // on Atom, goto 3
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        11, // on Mul, goto 10
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 37
        0, // on Add, error
        3, // on App, goto 2
        4, // on Atom, goto 3
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        57, // on List, goto 56
        0, // on Mul, error
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 38
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 39
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 40
        59, // on Add, goto 58
        3, // on App, goto 2
        4, // on Atom, goto 3
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        11, // on Mul, goto 10
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 41
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 42
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 43
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 44
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 45
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 46
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 47
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 48
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 49
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 50
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 51
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 52
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 53
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 54
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 55
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 56
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 57
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 58
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 59
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 60
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        67, // on Expr, goto 66
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        11, // on Mul, goto 10
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 61
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        68, // on Expr, goto 67
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        11, // on Mul, goto 10
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 62
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        69, // on Expr, goto 68
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        11, // on Mul, goto 10
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 63
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        70, // on Expr, goto 69
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        11, // on Mul, goto 10
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 64
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 65
        72, // on Add, goto 71
        3, // on App, goto 2
        4, // on Atom, goto 3
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        11, // on Mul, goto 10
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 66
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 67
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 68
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 69
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 70
        0, // on Add, error
        0, // on App, error
        75, // on Atom, goto 74
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        35, // on Ident, goto 34
        9, // on Int, goto 8
        0, // on List, error
        0, // on Mul, error
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 71
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 72
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        77, // on Expr, goto 76
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        11, // on Mul, goto 10
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 73
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        78, // on Expr, goto 77
        0, // on Exprs, error
        7, // on Get, goto 6
        8, // on Ident, goto 7
        9, // on Int, goto 8
        10, // on List, goto 9
        11, // on Mul, goto 10
        12, // on Num, goto 11
        0, // on Program, error
        0, // on __Program, error
        // State 74
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 75
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 76
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 77
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Expr, error
        0, // on Exprs, error
        0, // on Get, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
    ];
    pub fn parse_Program<
        'input,
        __TOKEN: __ToTriple<'input, Error=tok::Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        text: &'input str,
        __tokens0: __TOKENS,
    ) -> Result<Box<Expr>, __lalrpop_util::ParseError<usize,Tok<'input>,tok::Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            let __integer = match __lookahead {
                (_, Tok::LParen, _) if true => 0,
                (_, Tok::RParen, _) if true => 1,
                (_, Tok::Mul, _) if true => 2,
                (_, Tok::Plus, _) if true => 3,
                (_, Tok::Comma, _) if true => 4,
                (_, Tok::Minus, _) if true => 5,
                (_, Tok::RArrow, _) if true => 6,
                (_, Tok::Cons, _) if true => 7,
                (_, Tok::Lt, _) if true => 8,
                (_, Tok::LArrow, _) if true => 9,
                (_, Tok::Eq, _) if true => 10,
                (_, Tok::Gt, _) if true => 11,
                (_, Tok::Ident(_), _) if true => 12,
                (_, Tok::Num(_), _) if true => 13,
                (_, Tok::LBracket, _) if true => 14,
                (_, Tok::RBracket, _) if true => 15,
                (_, Tok::Array, _) if true => 16,
                (_, Tok::Begin, _) if true => 17,
                (_, Tok::Else, _) if true => 18,
                (_, Tok::Empty, _) if true => 19,
                (_, Tok::Emptyq, _) if true => 20,
                (_, Tok::End, _) if true => 21,
                (_, Tok::False, _) if true => 22,
                (_, Tok::Fix, _) if true => 23,
                (_, Tok::Fun, _) if true => 24,
                (_, Tok::Head, _) if true => 25,
                (_, Tok::If, _) if true => 26,
                (_, Tok::In, _) if true => 27,
                (_, Tok::Let, _) if true => 28,
                (_, Tok::Set, _) if true => 29,
                (_, Tok::Tail, _) if true => 30,
                (_, Tok::Then, _) if true => 31,
                (_, Tok::True, _) if true => 32,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 33 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Tok::LParen => __Symbol::Term_22_28_22(__tok),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Tok::RParen => __Symbol::Term_22_29_22(__tok),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Tok::Mul => __Symbol::Term_22_2a_22(__tok),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Tok::Plus => __Symbol::Term_22_2b_22(__tok),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Tok::Comma => __Symbol::Term_22_2c_22(__tok),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Tok::Minus => __Symbol::Term_22_2d_22(__tok),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Tok::RArrow => __Symbol::Term_22_2d_3e_22(__tok),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            __tok @ Tok::Cons => __Symbol::Term_22_3a_3a_22(__tok),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Tok::Lt => __Symbol::Term_22_3c_22(__tok),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            __tok @ Tok::LArrow => __Symbol::Term_22_3c_2d_22(__tok),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            __tok @ Tok::Eq => __Symbol::Term_22_3d_22(__tok),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Tok::Gt => __Symbol::Term_22_3e_22(__tok),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Tok::Ident(__tok0) => __Symbol::Term_22Iden_22(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            Tok::Num(__tok0) => __Symbol::Term_22Num_22(__tok0),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            __tok @ Tok::LBracket => __Symbol::Term_22_5b_22(__tok),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            __tok @ Tok::RBracket => __Symbol::Term_22_5d_22(__tok),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            __tok @ Tok::Array => __Symbol::Term_22array_22(__tok),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            __tok @ Tok::Begin => __Symbol::Term_22begin_22(__tok),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            __tok @ Tok::Else => __Symbol::Term_22else_22(__tok),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            __tok @ Tok::Empty => __Symbol::Term_22empty_22(__tok),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            __tok @ Tok::Emptyq => __Symbol::Term_22empty_3f_22(__tok),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            __tok @ Tok::End => __Symbol::Term_22end_22(__tok),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            __tok @ Tok::False => __Symbol::Term_22false_22(__tok),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            __tok @ Tok::Fix => __Symbol::Term_22fix_22(__tok),
                            _ => unreachable!(),
                        },
                        24 => match __lookahead.1 {
                            __tok @ Tok::Fun => __Symbol::Term_22fun_22(__tok),
                            _ => unreachable!(),
                        },
                        25 => match __lookahead.1 {
                            __tok @ Tok::Head => __Symbol::Term_22head_22(__tok),
                            _ => unreachable!(),
                        },
                        26 => match __lookahead.1 {
                            __tok @ Tok::If => __Symbol::Term_22if_22(__tok),
                            _ => unreachable!(),
                        },
                        27 => match __lookahead.1 {
                            __tok @ Tok::In => __Symbol::Term_22in_22(__tok),
                            _ => unreachable!(),
                        },
                        28 => match __lookahead.1 {
                            __tok @ Tok::Let => __Symbol::Term_22let_22(__tok),
                            _ => unreachable!(),
                        },
                        29 => match __lookahead.1 {
                            __tok @ Tok::Set => __Symbol::Term_22set_22(__tok),
                            _ => unreachable!(),
                        },
                        30 => match __lookahead.1 {
                            __tok @ Tok::Tail => __Symbol::Term_22tail_22(__tok),
                            _ => unreachable!(),
                        },
                        31 => match __lookahead.1 {
                            __tok @ Tok::Then => __Symbol::Term_22then_22(__tok),
                            _ => unreachable!(),
                        },
                        32 => match __lookahead.1 {
                            __tok @ Tok::True => __Symbol::Term_22true_22(__tok),
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
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
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
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
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
    ) -> Option<Result<Box<Expr>,__lalrpop_util::ParseError<usize,Tok<'input>,tok::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Add = Add, "-", Mul => ActionFn(12);
                let __sym2 = __pop_NtMul(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAdd(__nt), __end));
                0
            }
            2 => {
                // Add = Add, "+", Mul => ActionFn(13);
                let __sym2 = __pop_NtMul(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action13::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAdd(__nt), __end));
                0
            }
            3 => {
                // Add = Mul => ActionFn(14);
                let __sym0 = __pop_NtMul(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAdd(__nt), __end));
                0
            }
            4 => {
                // App = "array", "(", Add, ",", Add, ")" => ActionFn(22);
                let __sym5 = __pop_Term_22_29_22(__symbols);
                let __sym4 = __pop_NtAdd(__symbols);
                let __sym3 = __pop_Term_22_2c_22(__symbols);
                let __sym2 = __pop_NtAdd(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22array_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action22::<>(text, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtApp(__nt), __end));
                1
            }
            5 => {
                // App = "head", Atom => ActionFn(23);
                let __sym1 = __pop_NtAtom(__symbols);
                let __sym0 = __pop_Term_22head_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtApp(__nt), __end));
                1
            }
            6 => {
                // App = "tail", Atom => ActionFn(24);
                let __sym1 = __pop_NtAtom(__symbols);
                let __sym0 = __pop_Term_22tail_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtApp(__nt), __end));
                1
            }
            7 => {
                // App = "empty?", Atom => ActionFn(25);
                let __sym1 = __pop_NtAtom(__symbols);
                let __sym0 = __pop_Term_22empty_3f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action25::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtApp(__nt), __end));
                1
            }
            8 => {
                // App = App, Atom => ActionFn(26);
                let __sym1 = __pop_NtAtom(__symbols);
                let __sym0 = __pop_NtApp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtApp(__nt), __end));
                1
            }
            9 => {
                // App = Atom => ActionFn(27);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtApp(__nt), __end));
                1
            }
            10 => {
                // Atom = Ident => ActionFn(28);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                2
            }
            11 => {
                // Atom = "true" => ActionFn(29);
                let __sym0 = __pop_Term_22true_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                2
            }
            12 => {
                // Atom = "false" => ActionFn(30);
                let __sym0 = __pop_Term_22false_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                2
            }
            13 => {
                // Atom = "empty" => ActionFn(31);
                let __sym0 = __pop_Term_22empty_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                2
            }
            14 => {
                // Atom = "(", Exprs, ")" => ActionFn(32);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExprs(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action32::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                2
            }
            15 => {
                // Atom = Int => ActionFn(33);
                let __sym0 = __pop_NtInt(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                2
            }
            16 => {
                // Cmp = Add, "=", Cmp => ActionFn(8);
                let __sym2 = __pop_NtCmp(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtCmp(__nt), __end));
                3
            }
            17 => {
                // Cmp = Add, "<", Cmp => ActionFn(9);
                let __sym2 = __pop_NtCmp(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtCmp(__nt), __end));
                3
            }
            18 => {
                // Cmp = Add, ">", Cmp => ActionFn(10);
                let __sym2 = __pop_NtCmp(__symbols);
                let __sym1 = __pop_Term_22_3e_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtCmp(__nt), __end));
                3
            }
            19 => {
                // Cmp = Add => ActionFn(11);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCmp(__nt), __end));
                3
            }
            20 => {
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
            21 => {
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
            22 => {
                // Expr = "fun", Ident, "->", Expr => ActionFn(4);
                let __sym3 = __pop_NtExpr(__symbols);
                let __sym2 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22fun_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action4::<>(text, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                4
            }
            23 => {
                // Expr = "fix", Ident, "->", Expr => ActionFn(5);
                let __sym3 = __pop_NtExpr(__symbols);
                let __sym2 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22fix_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action5::<>(text, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                4
            }
            24 => {
                // Expr = "begin", Expr, "end" => ActionFn(6);
                let __sym2 = __pop_Term_22end_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22begin_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                4
            }
            25 => {
                // Expr = Cmp => ActionFn(7);
                let __sym0 = __pop_NtCmp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                4
            }
            26 => {
                // Exprs = Expr => ActionFn(34);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprs(__nt), __end));
                5
            }
            27 => {
                // Get = Ident, "[", Add, "]", "<-", Atom => ActionFn(19);
                let __sym5 = __pop_NtAtom(__symbols);
                let __sym4 = __pop_Term_22_3c_2d_22(__symbols);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtAdd(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action19::<>(text, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtGet(__nt), __end));
                6
            }
            28 => {
                // Get = Ident, "[", Add, "]" => ActionFn(20);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtAdd(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action20::<>(text, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtGet(__nt), __end));
                6
            }
            29 => {
                // Get = App => ActionFn(21);
                let __sym0 = __pop_NtApp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtGet(__nt), __end));
                6
            }
            30 => {
                // Ident = "Iden" => ActionFn(36);
                let __sym0 = __pop_Term_22Iden_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent(__nt), __end));
                7
            }
            31 => {
                // Int = Num => ActionFn(35);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInt(__nt), __end));
                8
            }
            32 => {
                // List = Get, "::", List => ActionFn(17);
                let __sym2 = __pop_NtList(__symbols);
                let __sym1 = __pop_Term_22_3a_3a_22(__symbols);
                let __sym0 = __pop_NtGet(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action17::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                9
            }
            33 => {
                // List = Get => ActionFn(18);
                let __sym0 = __pop_NtGet(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                9
            }
            34 => {
                // Mul = Mul, "*", List => ActionFn(15);
                let __sym2 = __pop_NtList(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtMul(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtMul(__nt), __end));
                10
            }
            35 => {
                // Mul = List => ActionFn(16);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMul(__nt), __end));
                10
            }
            36 => {
                // Num = "Num" => ActionFn(37);
                let __sym0 = __pop_Term_22Num_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                11
            }
            37 => {
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
            38 => {
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
    fn __pop_NtCmp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCmp(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtList(__v), __r) => (__l, __v, __r),
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
pub fn __action0<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action2<
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
pub fn __action3<
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
pub fn __action4<
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
pub fn __action5<
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
pub fn __action6<
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
pub fn __action7<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action8<
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
pub fn __action9<
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
pub fn __action10<
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
pub fn __action11<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action12<
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
pub fn __action13<
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
pub fn __action14<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action15<
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
pub fn __action16<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    text: &'input str,
    (_, hd, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, tl, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Cons(hd, tl))
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action19<
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
pub fn __action20<
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
pub fn __action21<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action22<
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
pub fn __action23<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, l, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Head(l))
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, l, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Tail(l))
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, l, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(IsEmpty(l))
}

#[allow(unused_variables)]
pub fn __action26<
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
pub fn __action27<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action28<
    'input,
>(
    text: &'input str,
    (_, id, _): (usize, Id, usize),
) -> Box<Expr>
{
    Box::new(Var(id))
}

#[allow(unused_variables)]
pub fn __action29<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Box<Expr>
{
    Box::new(Const(Bool(true)))
}

#[allow(unused_variables)]
pub fn __action30<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Box<Expr>
{
    Box::new(Const(Bool(true)))
}

#[allow(unused_variables)]
pub fn __action31<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Box<Expr>
{
    Box::new(Empty)
}

#[allow(unused_variables)]
pub fn __action32<
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
pub fn __action33<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action34<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action35<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, i64, usize),
) -> Box<Expr>
{
    Box::new(Const(Int(__0)))
}

#[allow(unused_variables)]
pub fn __action36<
    'input,
>(
    text: &'input str,
    (_, id, _): (usize, &'input str, usize),
) -> Id
{
    String::from(id)
}

#[allow(unused_variables)]
pub fn __action37<
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
