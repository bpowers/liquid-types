use common::Id;
use common::Op2::*;
use common::Const::*;
use implicit::Exp;
use implicit::Exp::*;
use tok::{self, Tok};
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use common::Id;
    use common::Op2::*;
    use common::Const::*;
    use implicit::Exp;
    use implicit::Exp::*;
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
        Term_22_2e1_22(Tok<'input>),
        Term_22_2e2_22(Tok<'input>),
        Term_22_3a_3a_22(Tok<'input>),
        Term_22_3c_22(Tok<'input>),
        Term_22_3d_22(Tok<'input>),
        Term_22_3e_22(Tok<'input>),
        Term_22Iden_22(&'input str),
        Term_22Num_22(i64),
        Term_22else_22(Tok<'input>),
        Term_22empty_22(Tok<'input>),
        Term_22empty_3f_22(Tok<'input>),
        Term_22false_22(Tok<'input>),
        Term_22fix_22(Tok<'input>),
        Term_22fun_22(Tok<'input>),
        Term_22head_22(Tok<'input>),
        Term_22if_22(Tok<'input>),
        Term_22in_22(Tok<'input>),
        Term_22let_22(Tok<'input>),
        Term_22tail_22(Tok<'input>),
        Term_22then_22(Tok<'input>),
        Term_22true_22(Tok<'input>),
        NtAdd(Box<Exp>),
        NtApp(Box<Exp>),
        NtAtom(Box<Exp>),
        NtCmp(Box<Exp>),
        NtExp(Box<Exp>),
        NtExps(Box<Exp>),
        NtIdent(Id),
        NtInt(Box<Exp>),
        NtList(Box<Exp>),
        NtMul(Box<Exp>),
        NtNum(i64),
        NtProgram(Box<Exp>),
        Nt____Program(Box<Exp>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        //     Add = (*) Add "+" Mul ["+", "-", "<", "=", ">", EOF]
        //     Add = (*) Add "-" Mul ["+", "-", "<", "=", ">", EOF]
        //     Add = (*) Mul ["+", "-", "<", "=", ">", EOF]
        //     App = (*) App Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     App = (*) Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     App = (*) "empty?" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     App = (*) "head" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     App = (*) "tail" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Atom = (*) Atom ".1" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Atom = (*) Atom ".2" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Atom = (*) Ident ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Atom = (*) Int ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Atom = (*) "(" Exps ")" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Atom = (*) "empty" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Atom = (*) "false" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Atom = (*) "true" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Cmp = (*) Add [EOF]
        //     Cmp = (*) Add "<" Cmp [EOF]
        //     Cmp = (*) Add "=" Cmp [EOF]
        //     Cmp = (*) Add ">" Cmp [EOF]
        //     Exp = (*) Cmp [EOF]
        //     Exp = (*) "fix" Ident "->" Exp [EOF]
        //     Exp = (*) "fun" Ident "->" Exp [EOF]
        //     Exp = (*) "if" Exp "then" Exp "else" Exp [EOF]
        //     Exp = (*) "let" Ident "=" Exp "in" Exp [EOF]
        //     Ident = (*) "Iden" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Int = (*) Num ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     List = (*) App ["*", "+", "-", "<", "=", ">", EOF]
        //     List = (*) App "::" List ["*", "+", "-", "<", "=", ">", EOF]
        //     Mul = (*) List ["*", "+", "-", "<", "=", ">", EOF]
        //     Mul = (*) Mul "*" List ["*", "+", "-", "<", "=", ">", EOF]
        //     Num = (*) "Num" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true", EOF]
        //     Program = (*) Exp [EOF]
        //     __Program = (*) Program [EOF]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        19, // on "fix", goto 18
        20, // on "fun", goto 19
        21, // on "head", goto 20
        22, // on "if", goto 21
        0, // on "in", error
        23, // on "let", goto 22
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 1
        //     Add = Add (*) "+" Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Add = Add (*) "-" Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Cmp = Add (*) [")", ",", "else", "in", "then", EOF]
        //     Cmp = Add (*) "<" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = Add (*) "=" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = Add (*) ">" Cmp [")", ",", "else", "in", "then", EOF]
        0, // on "(", error
        -20, // on ")", reduce `Cmp = Add => ActionFn(10);`
        0, // on "*", error
        26, // on "+", goto 25
        -20, // on ",", reduce `Cmp = Add => ActionFn(10);`
        27, // on "-", goto 26
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        28, // on "<", goto 27
        29, // on "=", goto 28
        30, // on ">", goto 29
        0, // on "Iden", error
        0, // on "Num", error
        -20, // on "else", reduce `Cmp = Add => ActionFn(10);`
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -20, // on "in", reduce `Cmp = Add => ActionFn(10);`
        0, // on "let", error
        0, // on "tail", error
        -20, // on "then", reduce `Cmp = Add => ActionFn(10);`
        0, // on "true", error
        // State 2
        //     App = App (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     List = App (*) [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     List = App (*) "::" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        13, // on "(", goto 12
        -31, // on ")", reduce `List = App => ActionFn(17);`
        -31, // on "*", reduce `List = App => ActionFn(17);`
        -31, // on "+", reduce `List = App => ActionFn(17);`
        -31, // on ",", reduce `List = App => ActionFn(17);`
        -31, // on "-", reduce `List = App => ActionFn(17);`
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        32, // on "::", goto 31
        -31, // on "<", reduce `List = App => ActionFn(17);`
        -31, // on "=", reduce `List = App => ActionFn(17);`
        -31, // on ">", reduce `List = App => ActionFn(17);`
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        -31, // on "else", reduce `List = App => ActionFn(17);`
        16, // on "empty", goto 15
        0, // on "empty?", error
        18, // on "false", goto 17
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -31, // on "in", reduce `List = App => ActionFn(17);`
        0, // on "let", error
        0, // on "tail", error
        -31, // on "then", reduce `List = App => ActionFn(17);`
        25, // on "true", goto 24
        // State 3
        //     App = Atom (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = Atom (*) ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = Atom (*) ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        -8, // on "(", reduce `App = Atom => ActionFn(22);`
        -8, // on ")", reduce `App = Atom => ActionFn(22);`
        -8, // on "*", reduce `App = Atom => ActionFn(22);`
        -8, // on "+", reduce `App = Atom => ActionFn(22);`
        -8, // on ",", reduce `App = Atom => ActionFn(22);`
        -8, // on "-", reduce `App = Atom => ActionFn(22);`
        0, // on "->", error
        33, // on ".1", goto 32
        34, // on ".2", goto 33
        -8, // on "::", reduce `App = Atom => ActionFn(22);`
        -8, // on "<", reduce `App = Atom => ActionFn(22);`
        -8, // on "=", reduce `App = Atom => ActionFn(22);`
        -8, // on ">", reduce `App = Atom => ActionFn(22);`
        -8, // on "Iden", reduce `App = Atom => ActionFn(22);`
        -8, // on "Num", reduce `App = Atom => ActionFn(22);`
        -8, // on "else", reduce `App = Atom => ActionFn(22);`
        -8, // on "empty", reduce `App = Atom => ActionFn(22);`
        0, // on "empty?", error
        -8, // on "false", reduce `App = Atom => ActionFn(22);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -8, // on "in", reduce `App = Atom => ActionFn(22);`
        0, // on "let", error
        0, // on "tail", error
        -8, // on "then", reduce `App = Atom => ActionFn(22);`
        -8, // on "true", reduce `App = Atom => ActionFn(22);`
        // State 4
        //     Exp = Cmp (*) [")", ",", "else", "in", "then", EOF]
        0, // on "(", error
        -25, // on ")", reduce `Exp = Cmp => ActionFn(6);`
        0, // on "*", error
        0, // on "+", error
        -25, // on ",", reduce `Exp = Cmp => ActionFn(6);`
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        -25, // on "else", reduce `Exp = Cmp => ActionFn(6);`
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -25, // on "in", reduce `Exp = Cmp => ActionFn(6);`
        0, // on "let", error
        0, // on "tail", error
        -25, // on "then", reduce `Exp = Cmp => ActionFn(6);`
        0, // on "true", error
        // State 5
        //     Program = Exp (*) [EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 6
        //     Atom = Ident (*) ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        -9, // on "(", reduce `Atom = Ident => ActionFn(23);`
        -9, // on ")", reduce `Atom = Ident => ActionFn(23);`
        -9, // on "*", reduce `Atom = Ident => ActionFn(23);`
        -9, // on "+", reduce `Atom = Ident => ActionFn(23);`
        -9, // on ",", reduce `Atom = Ident => ActionFn(23);`
        -9, // on "-", reduce `Atom = Ident => ActionFn(23);`
        0, // on "->", error
        -9, // on ".1", reduce `Atom = Ident => ActionFn(23);`
        -9, // on ".2", reduce `Atom = Ident => ActionFn(23);`
        -9, // on "::", reduce `Atom = Ident => ActionFn(23);`
        -9, // on "<", reduce `Atom = Ident => ActionFn(23);`
        -9, // on "=", reduce `Atom = Ident => ActionFn(23);`
        -9, // on ">", reduce `Atom = Ident => ActionFn(23);`
        -9, // on "Iden", reduce `Atom = Ident => ActionFn(23);`
        -9, // on "Num", reduce `Atom = Ident => ActionFn(23);`
        -9, // on "else", reduce `Atom = Ident => ActionFn(23);`
        -9, // on "empty", reduce `Atom = Ident => ActionFn(23);`
        0, // on "empty?", error
        -9, // on "false", reduce `Atom = Ident => ActionFn(23);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -9, // on "in", reduce `Atom = Ident => ActionFn(23);`
        0, // on "let", error
        0, // on "tail", error
        -9, // on "then", reduce `Atom = Ident => ActionFn(23);`
        -9, // on "true", reduce `Atom = Ident => ActionFn(23);`
        // State 7
        //     Atom = Int (*) ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        -16, // on "(", reduce `Atom = Int => ActionFn(30);`
        -16, // on ")", reduce `Atom = Int => ActionFn(30);`
        -16, // on "*", reduce `Atom = Int => ActionFn(30);`
        -16, // on "+", reduce `Atom = Int => ActionFn(30);`
        -16, // on ",", reduce `Atom = Int => ActionFn(30);`
        -16, // on "-", reduce `Atom = Int => ActionFn(30);`
        0, // on "->", error
        -16, // on ".1", reduce `Atom = Int => ActionFn(30);`
        -16, // on ".2", reduce `Atom = Int => ActionFn(30);`
        -16, // on "::", reduce `Atom = Int => ActionFn(30);`
        -16, // on "<", reduce `Atom = Int => ActionFn(30);`
        -16, // on "=", reduce `Atom = Int => ActionFn(30);`
        -16, // on ">", reduce `Atom = Int => ActionFn(30);`
        -16, // on "Iden", reduce `Atom = Int => ActionFn(30);`
        -16, // on "Num", reduce `Atom = Int => ActionFn(30);`
        -16, // on "else", reduce `Atom = Int => ActionFn(30);`
        -16, // on "empty", reduce `Atom = Int => ActionFn(30);`
        0, // on "empty?", error
        -16, // on "false", reduce `Atom = Int => ActionFn(30);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -16, // on "in", reduce `Atom = Int => ActionFn(30);`
        0, // on "let", error
        0, // on "tail", error
        -16, // on "then", reduce `Atom = Int => ActionFn(30);`
        -16, // on "true", reduce `Atom = Int => ActionFn(30);`
        // State 8
        //     Mul = List (*) [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        0, // on "(", error
        -33, // on ")", reduce `Mul = List => ActionFn(15);`
        -33, // on "*", reduce `Mul = List => ActionFn(15);`
        -33, // on "+", reduce `Mul = List => ActionFn(15);`
        -33, // on ",", reduce `Mul = List => ActionFn(15);`
        -33, // on "-", reduce `Mul = List => ActionFn(15);`
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        -33, // on "<", reduce `Mul = List => ActionFn(15);`
        -33, // on "=", reduce `Mul = List => ActionFn(15);`
        -33, // on ">", reduce `Mul = List => ActionFn(15);`
        0, // on "Iden", error
        0, // on "Num", error
        -33, // on "else", reduce `Mul = List => ActionFn(15);`
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -33, // on "in", reduce `Mul = List => ActionFn(15);`
        0, // on "let", error
        0, // on "tail", error
        -33, // on "then", reduce `Mul = List => ActionFn(15);`
        0, // on "true", error
        // State 9
        //     Add = Mul (*) [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = Mul (*) "*" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        0, // on "(", error
        -3, // on ")", reduce `Add = Mul => ActionFn(13);`
        35, // on "*", goto 34
        -3, // on "+", reduce `Add = Mul => ActionFn(13);`
        -3, // on ",", reduce `Add = Mul => ActionFn(13);`
        -3, // on "-", reduce `Add = Mul => ActionFn(13);`
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        -3, // on "<", reduce `Add = Mul => ActionFn(13);`
        -3, // on "=", reduce `Add = Mul => ActionFn(13);`
        -3, // on ">", reduce `Add = Mul => ActionFn(13);`
        0, // on "Iden", error
        0, // on "Num", error
        -3, // on "else", reduce `Add = Mul => ActionFn(13);`
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -3, // on "in", reduce `Add = Mul => ActionFn(13);`
        0, // on "let", error
        0, // on "tail", error
        -3, // on "then", reduce `Add = Mul => ActionFn(13);`
        0, // on "true", error
        // State 10
        //     Int = Num (*) ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        -29, // on "(", reduce `Int = Num => ActionFn(33);`
        -29, // on ")", reduce `Int = Num => ActionFn(33);`
        -29, // on "*", reduce `Int = Num => ActionFn(33);`
        -29, // on "+", reduce `Int = Num => ActionFn(33);`
        -29, // on ",", reduce `Int = Num => ActionFn(33);`
        -29, // on "-", reduce `Int = Num => ActionFn(33);`
        0, // on "->", error
        -29, // on ".1", reduce `Int = Num => ActionFn(33);`
        -29, // on ".2", reduce `Int = Num => ActionFn(33);`
        -29, // on "::", reduce `Int = Num => ActionFn(33);`
        -29, // on "<", reduce `Int = Num => ActionFn(33);`
        -29, // on "=", reduce `Int = Num => ActionFn(33);`
        -29, // on ">", reduce `Int = Num => ActionFn(33);`
        -29, // on "Iden", reduce `Int = Num => ActionFn(33);`
        -29, // on "Num", reduce `Int = Num => ActionFn(33);`
        -29, // on "else", reduce `Int = Num => ActionFn(33);`
        -29, // on "empty", reduce `Int = Num => ActionFn(33);`
        0, // on "empty?", error
        -29, // on "false", reduce `Int = Num => ActionFn(33);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -29, // on "in", reduce `Int = Num => ActionFn(33);`
        0, // on "let", error
        0, // on "tail", error
        -29, // on "then", reduce `Int = Num => ActionFn(33);`
        -29, // on "true", reduce `Int = Num => ActionFn(33);`
        // State 11
        //     __Program = Program (*) [EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 12
        //     Add = (*) Add "+" Mul [")", "+", ",", "-", "<", "=", ">"]
        //     Add = (*) Add "-" Mul [")", "+", ",", "-", "<", "=", ">"]
        //     Add = (*) Mul [")", "+", ",", "-", "<", "=", ">"]
        //     App = (*) App Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) "head" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = "(" (*) Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Cmp = (*) Add [")", ","]
        //     Cmp = (*) Add "<" Cmp [")", ","]
        //     Cmp = (*) Add "=" Cmp [")", ","]
        //     Cmp = (*) Add ">" Cmp [")", ","]
        //     Exp = (*) Cmp [")", ","]
        //     Exp = (*) "fix" Ident "->" Exp [")", ","]
        //     Exp = (*) "fun" Ident "->" Exp [")", ","]
        //     Exp = (*) "if" Exp "then" Exp "else" Exp [")", ","]
        //     Exp = (*) "let" Ident "=" Exp "in" Exp [")", ","]
        //     Exps = (*) Exp [")"]
        //     Exps = (*) Exp "," Exp [")"]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     List = (*) App [")", "*", "+", ",", "-", "<", "=", ">"]
        //     List = (*) App "::" List [")", "*", "+", ",", "-", "<", "=", ">"]
        //     Mul = (*) List [")", "*", "+", ",", "-", "<", "=", ">"]
        //     Mul = (*) Mul "*" List [")", "*", "+", ",", "-", "<", "=", ">"]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        19, // on "fix", goto 18
        20, // on "fun", goto 19
        21, // on "head", goto 20
        22, // on "if", goto 21
        0, // on "in", error
        23, // on "let", goto 22
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 13
        //     Ident = "Iden" (*) ["(", ")", "*", "+", ",", "-", "->", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        -28, // on "(", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on ")", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on "*", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on "+", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on ",", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on "-", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on "->", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on ".1", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on ".2", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on "::", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on "<", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on "=", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on ">", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on "Iden", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on "Num", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on "else", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on "empty", reduce `Ident = "Iden" => ActionFn(34);`
        0, // on "empty?", error
        -28, // on "false", reduce `Ident = "Iden" => ActionFn(34);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -28, // on "in", reduce `Ident = "Iden" => ActionFn(34);`
        0, // on "let", error
        0, // on "tail", error
        -28, // on "then", reduce `Ident = "Iden" => ActionFn(34);`
        -28, // on "true", reduce `Ident = "Iden" => ActionFn(34);`
        // State 14
        //     Num = "Num" (*) ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        -34, // on "(", reduce `Num = "Num" => ActionFn(35);`
        -34, // on ")", reduce `Num = "Num" => ActionFn(35);`
        -34, // on "*", reduce `Num = "Num" => ActionFn(35);`
        -34, // on "+", reduce `Num = "Num" => ActionFn(35);`
        -34, // on ",", reduce `Num = "Num" => ActionFn(35);`
        -34, // on "-", reduce `Num = "Num" => ActionFn(35);`
        0, // on "->", error
        -34, // on ".1", reduce `Num = "Num" => ActionFn(35);`
        -34, // on ".2", reduce `Num = "Num" => ActionFn(35);`
        -34, // on "::", reduce `Num = "Num" => ActionFn(35);`
        -34, // on "<", reduce `Num = "Num" => ActionFn(35);`
        -34, // on "=", reduce `Num = "Num" => ActionFn(35);`
        -34, // on ">", reduce `Num = "Num" => ActionFn(35);`
        -34, // on "Iden", reduce `Num = "Num" => ActionFn(35);`
        -34, // on "Num", reduce `Num = "Num" => ActionFn(35);`
        -34, // on "else", reduce `Num = "Num" => ActionFn(35);`
        -34, // on "empty", reduce `Num = "Num" => ActionFn(35);`
        0, // on "empty?", error
        -34, // on "false", reduce `Num = "Num" => ActionFn(35);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -34, // on "in", reduce `Num = "Num" => ActionFn(35);`
        0, // on "let", error
        0, // on "tail", error
        -34, // on "then", reduce `Num = "Num" => ActionFn(35);`
        -34, // on "true", reduce `Num = "Num" => ActionFn(35);`
        // State 15
        //     Atom = "empty" (*) ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        -12, // on "(", reduce `Atom = "empty" => ActionFn(26);`
        -12, // on ")", reduce `Atom = "empty" => ActionFn(26);`
        -12, // on "*", reduce `Atom = "empty" => ActionFn(26);`
        -12, // on "+", reduce `Atom = "empty" => ActionFn(26);`
        -12, // on ",", reduce `Atom = "empty" => ActionFn(26);`
        -12, // on "-", reduce `Atom = "empty" => ActionFn(26);`
        0, // on "->", error
        -12, // on ".1", reduce `Atom = "empty" => ActionFn(26);`
        -12, // on ".2", reduce `Atom = "empty" => ActionFn(26);`
        -12, // on "::", reduce `Atom = "empty" => ActionFn(26);`
        -12, // on "<", reduce `Atom = "empty" => ActionFn(26);`
        -12, // on "=", reduce `Atom = "empty" => ActionFn(26);`
        -12, // on ">", reduce `Atom = "empty" => ActionFn(26);`
        -12, // on "Iden", reduce `Atom = "empty" => ActionFn(26);`
        -12, // on "Num", reduce `Atom = "empty" => ActionFn(26);`
        -12, // on "else", reduce `Atom = "empty" => ActionFn(26);`
        -12, // on "empty", reduce `Atom = "empty" => ActionFn(26);`
        0, // on "empty?", error
        -12, // on "false", reduce `Atom = "empty" => ActionFn(26);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -12, // on "in", reduce `Atom = "empty" => ActionFn(26);`
        0, // on "let", error
        0, // on "tail", error
        -12, // on "then", reduce `Atom = "empty" => ActionFn(26);`
        -12, // on "true", reduce `Atom = "empty" => ActionFn(26);`
        // State 16
        //     App = "empty?" (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        0, // on "empty?", error
        18, // on "false", goto 17
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "tail", error
        0, // on "then", error
        25, // on "true", goto 24
        // State 17
        //     Atom = "false" (*) ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        -11, // on "(", reduce `Atom = "false" => ActionFn(25);`
        -11, // on ")", reduce `Atom = "false" => ActionFn(25);`
        -11, // on "*", reduce `Atom = "false" => ActionFn(25);`
        -11, // on "+", reduce `Atom = "false" => ActionFn(25);`
        -11, // on ",", reduce `Atom = "false" => ActionFn(25);`
        -11, // on "-", reduce `Atom = "false" => ActionFn(25);`
        0, // on "->", error
        -11, // on ".1", reduce `Atom = "false" => ActionFn(25);`
        -11, // on ".2", reduce `Atom = "false" => ActionFn(25);`
        -11, // on "::", reduce `Atom = "false" => ActionFn(25);`
        -11, // on "<", reduce `Atom = "false" => ActionFn(25);`
        -11, // on "=", reduce `Atom = "false" => ActionFn(25);`
        -11, // on ">", reduce `Atom = "false" => ActionFn(25);`
        -11, // on "Iden", reduce `Atom = "false" => ActionFn(25);`
        -11, // on "Num", reduce `Atom = "false" => ActionFn(25);`
        -11, // on "else", reduce `Atom = "false" => ActionFn(25);`
        -11, // on "empty", reduce `Atom = "false" => ActionFn(25);`
        0, // on "empty?", error
        -11, // on "false", reduce `Atom = "false" => ActionFn(25);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -11, // on "in", reduce `Atom = "false" => ActionFn(25);`
        0, // on "let", error
        0, // on "tail", error
        -11, // on "then", reduce `Atom = "false" => ActionFn(25);`
        -11, // on "true", reduce `Atom = "false" => ActionFn(25);`
        // State 18
        //     Exp = "fix" (*) Ident "->" Exp [")", ",", "else", "in", "then", EOF]
        //     Ident = (*) "Iden" ["->"]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        0, // on "Num", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 19
        //     Exp = "fun" (*) Ident "->" Exp [")", ",", "else", "in", "then", EOF]
        //     Ident = (*) "Iden" ["->"]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        0, // on "Num", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 20
        //     App = "head" (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        0, // on "empty?", error
        18, // on "false", goto 17
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "tail", error
        0, // on "then", error
        25, // on "true", goto 24
        // State 21
        //     Add = (*) Add "+" Mul ["+", "-", "<", "=", ">", "then"]
        //     Add = (*) Add "-" Mul ["+", "-", "<", "=", ">", "then"]
        //     Add = (*) Mul ["+", "-", "<", "=", ">", "then"]
        //     App = (*) App Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     App = (*) Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     App = (*) "empty?" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     App = (*) "head" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     App = (*) "tail" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Atom = (*) Atom ".1" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Atom = (*) Atom ".2" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Atom = (*) Ident ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Atom = (*) Int ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Atom = (*) "(" Exps ")" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Atom = (*) "empty" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Atom = (*) "false" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Atom = (*) "true" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Cmp = (*) Add ["then"]
        //     Cmp = (*) Add "<" Cmp ["then"]
        //     Cmp = (*) Add "=" Cmp ["then"]
        //     Cmp = (*) Add ">" Cmp ["then"]
        //     Exp = (*) Cmp ["then"]
        //     Exp = (*) "fix" Ident "->" Exp ["then"]
        //     Exp = (*) "fun" Ident "->" Exp ["then"]
        //     Exp = (*) "if" Exp "then" Exp "else" Exp ["then"]
        //     Exp = "if" (*) Exp "then" Exp "else" Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "let" Ident "=" Exp "in" Exp ["then"]
        //     Ident = (*) "Iden" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     Int = (*) Num ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        //     List = (*) App ["*", "+", "-", "<", "=", ">", "then"]
        //     List = (*) App "::" List ["*", "+", "-", "<", "=", ">", "then"]
        //     Mul = (*) List ["*", "+", "-", "<", "=", ">", "then"]
        //     Mul = (*) Mul "*" List ["*", "+", "-", "<", "=", ">", "then"]
        //     Num = (*) "Num" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "then", "true"]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        19, // on "fix", goto 18
        20, // on "fun", goto 19
        21, // on "head", goto 20
        22, // on "if", goto 21
        0, // on "in", error
        23, // on "let", goto 22
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 22
        //     Exp = "let" (*) Ident "=" Exp "in" Exp [")", ",", "else", "in", "then", EOF]
        //     Ident = (*) "Iden" ["="]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        0, // on "Num", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 23
        //     App = "tail" (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        0, // on "empty?", error
        18, // on "false", goto 17
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "tail", error
        0, // on "then", error
        25, // on "true", goto 24
        // State 24
        //     Atom = "true" (*) ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        -10, // on "(", reduce `Atom = "true" => ActionFn(24);`
        -10, // on ")", reduce `Atom = "true" => ActionFn(24);`
        -10, // on "*", reduce `Atom = "true" => ActionFn(24);`
        -10, // on "+", reduce `Atom = "true" => ActionFn(24);`
        -10, // on ",", reduce `Atom = "true" => ActionFn(24);`
        -10, // on "-", reduce `Atom = "true" => ActionFn(24);`
        0, // on "->", error
        -10, // on ".1", reduce `Atom = "true" => ActionFn(24);`
        -10, // on ".2", reduce `Atom = "true" => ActionFn(24);`
        -10, // on "::", reduce `Atom = "true" => ActionFn(24);`
        -10, // on "<", reduce `Atom = "true" => ActionFn(24);`
        -10, // on "=", reduce `Atom = "true" => ActionFn(24);`
        -10, // on ">", reduce `Atom = "true" => ActionFn(24);`
        -10, // on "Iden", reduce `Atom = "true" => ActionFn(24);`
        -10, // on "Num", reduce `Atom = "true" => ActionFn(24);`
        -10, // on "else", reduce `Atom = "true" => ActionFn(24);`
        -10, // on "empty", reduce `Atom = "true" => ActionFn(24);`
        0, // on "empty?", error
        -10, // on "false", reduce `Atom = "true" => ActionFn(24);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -10, // on "in", reduce `Atom = "true" => ActionFn(24);`
        0, // on "let", error
        0, // on "tail", error
        -10, // on "then", reduce `Atom = "true" => ActionFn(24);`
        -10, // on "true", reduce `Atom = "true" => ActionFn(24);`
        // State 25
        //     Add = Add "+" (*) Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     List = (*) App [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     List = (*) App "::" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        0, // on "fix", error
        0, // on "fun", error
        21, // on "head", goto 20
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 26
        //     Add = Add "-" (*) Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     List = (*) App [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     List = (*) App "::" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        0, // on "fix", error
        0, // on "fun", error
        21, // on "head", goto 20
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 27
        //     Add = (*) Add "+" Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Add = (*) Add "-" Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Add = (*) Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Cmp = (*) Add [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add "<" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = Add "<" (*) Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add "=" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add ">" Cmp [")", ",", "else", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     List = (*) App [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     List = (*) App "::" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        0, // on "fix", error
        0, // on "fun", error
        21, // on "head", goto 20
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 28
        //     Add = (*) Add "+" Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Add = (*) Add "-" Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Add = (*) Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Cmp = (*) Add [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add "<" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add "=" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = Add "=" (*) Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add ">" Cmp [")", ",", "else", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     List = (*) App [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     List = (*) App "::" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        0, // on "fix", error
        0, // on "fun", error
        21, // on "head", goto 20
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 29
        //     Add = (*) Add "+" Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Add = (*) Add "-" Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Add = (*) Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Cmp = (*) Add [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add "<" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add "=" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add ">" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = Add ">" (*) Cmp [")", ",", "else", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     List = (*) App [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     List = (*) App "::" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        0, // on "fix", error
        0, // on "fun", error
        21, // on "head", goto 20
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 30
        //     App = App Atom (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = Atom (*) ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = Atom (*) ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        -7, // on "(", reduce `App = App, Atom => ActionFn(21);`
        -7, // on ")", reduce `App = App, Atom => ActionFn(21);`
        -7, // on "*", reduce `App = App, Atom => ActionFn(21);`
        -7, // on "+", reduce `App = App, Atom => ActionFn(21);`
        -7, // on ",", reduce `App = App, Atom => ActionFn(21);`
        -7, // on "-", reduce `App = App, Atom => ActionFn(21);`
        0, // on "->", error
        33, // on ".1", goto 32
        34, // on ".2", goto 33
        -7, // on "::", reduce `App = App, Atom => ActionFn(21);`
        -7, // on "<", reduce `App = App, Atom => ActionFn(21);`
        -7, // on "=", reduce `App = App, Atom => ActionFn(21);`
        -7, // on ">", reduce `App = App, Atom => ActionFn(21);`
        -7, // on "Iden", reduce `App = App, Atom => ActionFn(21);`
        -7, // on "Num", reduce `App = App, Atom => ActionFn(21);`
        -7, // on "else", reduce `App = App, Atom => ActionFn(21);`
        -7, // on "empty", reduce `App = App, Atom => ActionFn(21);`
        0, // on "empty?", error
        -7, // on "false", reduce `App = App, Atom => ActionFn(21);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -7, // on "in", reduce `App = App, Atom => ActionFn(21);`
        0, // on "let", error
        0, // on "tail", error
        -7, // on "then", reduce `App = App, Atom => ActionFn(21);`
        -7, // on "true", reduce `App = App, Atom => ActionFn(21);`
        // State 31
        //     App = (*) App Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     List = (*) App [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     List = (*) App "::" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     List = App "::" (*) List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        0, // on "fix", error
        0, // on "fun", error
        21, // on "head", goto 20
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 32
        //     Atom = Atom ".1" (*) ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        -13, // on "(", reduce `Atom = Atom, ".1" => ActionFn(27);`
        -13, // on ")", reduce `Atom = Atom, ".1" => ActionFn(27);`
        -13, // on "*", reduce `Atom = Atom, ".1" => ActionFn(27);`
        -13, // on "+", reduce `Atom = Atom, ".1" => ActionFn(27);`
        -13, // on ",", reduce `Atom = Atom, ".1" => ActionFn(27);`
        -13, // on "-", reduce `Atom = Atom, ".1" => ActionFn(27);`
        0, // on "->", error
        -13, // on ".1", reduce `Atom = Atom, ".1" => ActionFn(27);`
        -13, // on ".2", reduce `Atom = Atom, ".1" => ActionFn(27);`
        -13, // on "::", reduce `Atom = Atom, ".1" => ActionFn(27);`
        -13, // on "<", reduce `Atom = Atom, ".1" => ActionFn(27);`
        -13, // on "=", reduce `Atom = Atom, ".1" => ActionFn(27);`
        -13, // on ">", reduce `Atom = Atom, ".1" => ActionFn(27);`
        -13, // on "Iden", reduce `Atom = Atom, ".1" => ActionFn(27);`
        -13, // on "Num", reduce `Atom = Atom, ".1" => ActionFn(27);`
        -13, // on "else", reduce `Atom = Atom, ".1" => ActionFn(27);`
        -13, // on "empty", reduce `Atom = Atom, ".1" => ActionFn(27);`
        0, // on "empty?", error
        -13, // on "false", reduce `Atom = Atom, ".1" => ActionFn(27);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -13, // on "in", reduce `Atom = Atom, ".1" => ActionFn(27);`
        0, // on "let", error
        0, // on "tail", error
        -13, // on "then", reduce `Atom = Atom, ".1" => ActionFn(27);`
        -13, // on "true", reduce `Atom = Atom, ".1" => ActionFn(27);`
        // State 33
        //     Atom = Atom ".2" (*) ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        -14, // on "(", reduce `Atom = Atom, ".2" => ActionFn(28);`
        -14, // on ")", reduce `Atom = Atom, ".2" => ActionFn(28);`
        -14, // on "*", reduce `Atom = Atom, ".2" => ActionFn(28);`
        -14, // on "+", reduce `Atom = Atom, ".2" => ActionFn(28);`
        -14, // on ",", reduce `Atom = Atom, ".2" => ActionFn(28);`
        -14, // on "-", reduce `Atom = Atom, ".2" => ActionFn(28);`
        0, // on "->", error
        -14, // on ".1", reduce `Atom = Atom, ".2" => ActionFn(28);`
        -14, // on ".2", reduce `Atom = Atom, ".2" => ActionFn(28);`
        -14, // on "::", reduce `Atom = Atom, ".2" => ActionFn(28);`
        -14, // on "<", reduce `Atom = Atom, ".2" => ActionFn(28);`
        -14, // on "=", reduce `Atom = Atom, ".2" => ActionFn(28);`
        -14, // on ">", reduce `Atom = Atom, ".2" => ActionFn(28);`
        -14, // on "Iden", reduce `Atom = Atom, ".2" => ActionFn(28);`
        -14, // on "Num", reduce `Atom = Atom, ".2" => ActionFn(28);`
        -14, // on "else", reduce `Atom = Atom, ".2" => ActionFn(28);`
        -14, // on "empty", reduce `Atom = Atom, ".2" => ActionFn(28);`
        0, // on "empty?", error
        -14, // on "false", reduce `Atom = Atom, ".2" => ActionFn(28);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -14, // on "in", reduce `Atom = Atom, ".2" => ActionFn(28);`
        0, // on "let", error
        0, // on "tail", error
        -14, // on "then", reduce `Atom = Atom, ".2" => ActionFn(28);`
        -14, // on "true", reduce `Atom = Atom, ".2" => ActionFn(28);`
        // State 34
        //     App = (*) App Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     List = (*) App [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     List = (*) App "::" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = Mul "*" (*) List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        0, // on "fix", error
        0, // on "fun", error
        21, // on "head", goto 20
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 35
        //     Exps = Exp (*) [")"]
        //     Exps = Exp (*) "," Exp [")"]
        0, // on "(", error
        -27, // on ")", reduce `Exps = Exp => ActionFn(32);`
        0, // on "*", error
        0, // on "+", error
        52, // on ",", goto 51
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 36
        //     Atom = "(" Exps (*) ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        0, // on "(", error
        53, // on ")", goto 52
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 37
        //     App = "empty?" Atom (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = Atom (*) ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = Atom (*) ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        -6, // on "(", reduce `App = "empty?", Atom => ActionFn(20);`
        -6, // on ")", reduce `App = "empty?", Atom => ActionFn(20);`
        -6, // on "*", reduce `App = "empty?", Atom => ActionFn(20);`
        -6, // on "+", reduce `App = "empty?", Atom => ActionFn(20);`
        -6, // on ",", reduce `App = "empty?", Atom => ActionFn(20);`
        -6, // on "-", reduce `App = "empty?", Atom => ActionFn(20);`
        0, // on "->", error
        33, // on ".1", goto 32
        34, // on ".2", goto 33
        -6, // on "::", reduce `App = "empty?", Atom => ActionFn(20);`
        -6, // on "<", reduce `App = "empty?", Atom => ActionFn(20);`
        -6, // on "=", reduce `App = "empty?", Atom => ActionFn(20);`
        -6, // on ">", reduce `App = "empty?", Atom => ActionFn(20);`
        -6, // on "Iden", reduce `App = "empty?", Atom => ActionFn(20);`
        -6, // on "Num", reduce `App = "empty?", Atom => ActionFn(20);`
        -6, // on "else", reduce `App = "empty?", Atom => ActionFn(20);`
        -6, // on "empty", reduce `App = "empty?", Atom => ActionFn(20);`
        0, // on "empty?", error
        -6, // on "false", reduce `App = "empty?", Atom => ActionFn(20);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -6, // on "in", reduce `App = "empty?", Atom => ActionFn(20);`
        0, // on "let", error
        0, // on "tail", error
        -6, // on "then", reduce `App = "empty?", Atom => ActionFn(20);`
        -6, // on "true", reduce `App = "empty?", Atom => ActionFn(20);`
        // State 38
        //     Exp = "fix" Ident (*) "->" Exp [")", ",", "else", "in", "then", EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        54, // on "->", goto 53
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 39
        //     Exp = "fun" Ident (*) "->" Exp [")", ",", "else", "in", "then", EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        55, // on "->", goto 54
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 40
        //     App = "head" Atom (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = Atom (*) ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = Atom (*) ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        -4, // on "(", reduce `App = "head", Atom => ActionFn(18);`
        -4, // on ")", reduce `App = "head", Atom => ActionFn(18);`
        -4, // on "*", reduce `App = "head", Atom => ActionFn(18);`
        -4, // on "+", reduce `App = "head", Atom => ActionFn(18);`
        -4, // on ",", reduce `App = "head", Atom => ActionFn(18);`
        -4, // on "-", reduce `App = "head", Atom => ActionFn(18);`
        0, // on "->", error
        33, // on ".1", goto 32
        34, // on ".2", goto 33
        -4, // on "::", reduce `App = "head", Atom => ActionFn(18);`
        -4, // on "<", reduce `App = "head", Atom => ActionFn(18);`
        -4, // on "=", reduce `App = "head", Atom => ActionFn(18);`
        -4, // on ">", reduce `App = "head", Atom => ActionFn(18);`
        -4, // on "Iden", reduce `App = "head", Atom => ActionFn(18);`
        -4, // on "Num", reduce `App = "head", Atom => ActionFn(18);`
        -4, // on "else", reduce `App = "head", Atom => ActionFn(18);`
        -4, // on "empty", reduce `App = "head", Atom => ActionFn(18);`
        0, // on "empty?", error
        -4, // on "false", reduce `App = "head", Atom => ActionFn(18);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -4, // on "in", reduce `App = "head", Atom => ActionFn(18);`
        0, // on "let", error
        0, // on "tail", error
        -4, // on "then", reduce `App = "head", Atom => ActionFn(18);`
        -4, // on "true", reduce `App = "head", Atom => ActionFn(18);`
        // State 41
        //     Exp = "if" Exp (*) "then" Exp "else" Exp [")", ",", "else", "in", "then", EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "tail", error
        56, // on "then", goto 55
        0, // on "true", error
        // State 42
        //     Exp = "let" Ident (*) "=" Exp "in" Exp [")", ",", "else", "in", "then", EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        57, // on "=", goto 56
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 43
        //     App = "tail" Atom (*) ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = Atom (*) ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = Atom (*) ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        -5, // on "(", reduce `App = "tail", Atom => ActionFn(19);`
        -5, // on ")", reduce `App = "tail", Atom => ActionFn(19);`
        -5, // on "*", reduce `App = "tail", Atom => ActionFn(19);`
        -5, // on "+", reduce `App = "tail", Atom => ActionFn(19);`
        -5, // on ",", reduce `App = "tail", Atom => ActionFn(19);`
        -5, // on "-", reduce `App = "tail", Atom => ActionFn(19);`
        0, // on "->", error
        33, // on ".1", goto 32
        34, // on ".2", goto 33
        -5, // on "::", reduce `App = "tail", Atom => ActionFn(19);`
        -5, // on "<", reduce `App = "tail", Atom => ActionFn(19);`
        -5, // on "=", reduce `App = "tail", Atom => ActionFn(19);`
        -5, // on ">", reduce `App = "tail", Atom => ActionFn(19);`
        -5, // on "Iden", reduce `App = "tail", Atom => ActionFn(19);`
        -5, // on "Num", reduce `App = "tail", Atom => ActionFn(19);`
        -5, // on "else", reduce `App = "tail", Atom => ActionFn(19);`
        -5, // on "empty", reduce `App = "tail", Atom => ActionFn(19);`
        0, // on "empty?", error
        -5, // on "false", reduce `App = "tail", Atom => ActionFn(19);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -5, // on "in", reduce `App = "tail", Atom => ActionFn(19);`
        0, // on "let", error
        0, // on "tail", error
        -5, // on "then", reduce `App = "tail", Atom => ActionFn(19);`
        -5, // on "true", reduce `App = "tail", Atom => ActionFn(19);`
        // State 44
        //     Add = Add "+" Mul (*) [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = Mul (*) "*" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        0, // on "(", error
        -2, // on ")", reduce `Add = Add, "+", Mul => ActionFn(12);`
        35, // on "*", goto 34
        -2, // on "+", reduce `Add = Add, "+", Mul => ActionFn(12);`
        -2, // on ",", reduce `Add = Add, "+", Mul => ActionFn(12);`
        -2, // on "-", reduce `Add = Add, "+", Mul => ActionFn(12);`
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        -2, // on "<", reduce `Add = Add, "+", Mul => ActionFn(12);`
        -2, // on "=", reduce `Add = Add, "+", Mul => ActionFn(12);`
        -2, // on ">", reduce `Add = Add, "+", Mul => ActionFn(12);`
        0, // on "Iden", error
        0, // on "Num", error
        -2, // on "else", reduce `Add = Add, "+", Mul => ActionFn(12);`
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -2, // on "in", reduce `Add = Add, "+", Mul => ActionFn(12);`
        0, // on "let", error
        0, // on "tail", error
        -2, // on "then", reduce `Add = Add, "+", Mul => ActionFn(12);`
        0, // on "true", error
        // State 45
        //     Add = Add "-" Mul (*) [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = Mul (*) "*" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        0, // on "(", error
        -1, // on ")", reduce `Add = Add, "-", Mul => ActionFn(11);`
        35, // on "*", goto 34
        -1, // on "+", reduce `Add = Add, "-", Mul => ActionFn(11);`
        -1, // on ",", reduce `Add = Add, "-", Mul => ActionFn(11);`
        -1, // on "-", reduce `Add = Add, "-", Mul => ActionFn(11);`
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        -1, // on "<", reduce `Add = Add, "-", Mul => ActionFn(11);`
        -1, // on "=", reduce `Add = Add, "-", Mul => ActionFn(11);`
        -1, // on ">", reduce `Add = Add, "-", Mul => ActionFn(11);`
        0, // on "Iden", error
        0, // on "Num", error
        -1, // on "else", reduce `Add = Add, "-", Mul => ActionFn(11);`
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -1, // on "in", reduce `Add = Add, "-", Mul => ActionFn(11);`
        0, // on "let", error
        0, // on "tail", error
        -1, // on "then", reduce `Add = Add, "-", Mul => ActionFn(11);`
        0, // on "true", error
        // State 46
        //     Cmp = Add "<" Cmp (*) [")", ",", "else", "in", "then", EOF]
        0, // on "(", error
        -18, // on ")", reduce `Cmp = Add, "<", Cmp => ActionFn(8);`
        0, // on "*", error
        0, // on "+", error
        -18, // on ",", reduce `Cmp = Add, "<", Cmp => ActionFn(8);`
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        -18, // on "else", reduce `Cmp = Add, "<", Cmp => ActionFn(8);`
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -18, // on "in", reduce `Cmp = Add, "<", Cmp => ActionFn(8);`
        0, // on "let", error
        0, // on "tail", error
        -18, // on "then", reduce `Cmp = Add, "<", Cmp => ActionFn(8);`
        0, // on "true", error
        // State 47
        //     Cmp = Add "=" Cmp (*) [")", ",", "else", "in", "then", EOF]
        0, // on "(", error
        -17, // on ")", reduce `Cmp = Add, "=", Cmp => ActionFn(7);`
        0, // on "*", error
        0, // on "+", error
        -17, // on ",", reduce `Cmp = Add, "=", Cmp => ActionFn(7);`
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        -17, // on "else", reduce `Cmp = Add, "=", Cmp => ActionFn(7);`
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -17, // on "in", reduce `Cmp = Add, "=", Cmp => ActionFn(7);`
        0, // on "let", error
        0, // on "tail", error
        -17, // on "then", reduce `Cmp = Add, "=", Cmp => ActionFn(7);`
        0, // on "true", error
        // State 48
        //     Cmp = Add ">" Cmp (*) [")", ",", "else", "in", "then", EOF]
        0, // on "(", error
        -19, // on ")", reduce `Cmp = Add, ">", Cmp => ActionFn(9);`
        0, // on "*", error
        0, // on "+", error
        -19, // on ",", reduce `Cmp = Add, ">", Cmp => ActionFn(9);`
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        -19, // on "else", reduce `Cmp = Add, ">", Cmp => ActionFn(9);`
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -19, // on "in", reduce `Cmp = Add, ">", Cmp => ActionFn(9);`
        0, // on "let", error
        0, // on "tail", error
        -19, // on "then", reduce `Cmp = Add, ">", Cmp => ActionFn(9);`
        0, // on "true", error
        // State 49
        //     List = App "::" List (*) [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        0, // on "(", error
        -30, // on ")", reduce `List = App, "::", List => ActionFn(16);`
        -30, // on "*", reduce `List = App, "::", List => ActionFn(16);`
        -30, // on "+", reduce `List = App, "::", List => ActionFn(16);`
        -30, // on ",", reduce `List = App, "::", List => ActionFn(16);`
        -30, // on "-", reduce `List = App, "::", List => ActionFn(16);`
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        -30, // on "<", reduce `List = App, "::", List => ActionFn(16);`
        -30, // on "=", reduce `List = App, "::", List => ActionFn(16);`
        -30, // on ">", reduce `List = App, "::", List => ActionFn(16);`
        0, // on "Iden", error
        0, // on "Num", error
        -30, // on "else", reduce `List = App, "::", List => ActionFn(16);`
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -30, // on "in", reduce `List = App, "::", List => ActionFn(16);`
        0, // on "let", error
        0, // on "tail", error
        -30, // on "then", reduce `List = App, "::", List => ActionFn(16);`
        0, // on "true", error
        // State 50
        //     Mul = Mul "*" List (*) [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        0, // on "(", error
        -32, // on ")", reduce `Mul = Mul, "*", List => ActionFn(14);`
        -32, // on "*", reduce `Mul = Mul, "*", List => ActionFn(14);`
        -32, // on "+", reduce `Mul = Mul, "*", List => ActionFn(14);`
        -32, // on ",", reduce `Mul = Mul, "*", List => ActionFn(14);`
        -32, // on "-", reduce `Mul = Mul, "*", List => ActionFn(14);`
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        -32, // on "<", reduce `Mul = Mul, "*", List => ActionFn(14);`
        -32, // on "=", reduce `Mul = Mul, "*", List => ActionFn(14);`
        -32, // on ">", reduce `Mul = Mul, "*", List => ActionFn(14);`
        0, // on "Iden", error
        0, // on "Num", error
        -32, // on "else", reduce `Mul = Mul, "*", List => ActionFn(14);`
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -32, // on "in", reduce `Mul = Mul, "*", List => ActionFn(14);`
        0, // on "let", error
        0, // on "tail", error
        -32, // on "then", reduce `Mul = Mul, "*", List => ActionFn(14);`
        0, // on "true", error
        // State 51
        //     Add = (*) Add "+" Mul [")", "+", "-", "<", "=", ">"]
        //     Add = (*) Add "-" Mul [")", "+", "-", "<", "=", ">"]
        //     Add = (*) Mul [")", "+", "-", "<", "=", ">"]
        //     App = (*) App Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) "head" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) Ident ["(", ")", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) Int ["(", ")", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "empty" ["(", ")", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "false" ["(", ")", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Atom = (*) "true" ["(", ")", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Cmp = (*) Add [")"]
        //     Cmp = (*) Add "<" Cmp [")"]
        //     Cmp = (*) Add "=" Cmp [")"]
        //     Cmp = (*) Add ">" Cmp [")"]
        //     Exp = (*) Cmp [")"]
        //     Exp = (*) "fix" Ident "->" Exp [")"]
        //     Exp = (*) "fun" Ident "->" Exp [")"]
        //     Exp = (*) "if" Exp "then" Exp "else" Exp [")"]
        //     Exp = (*) "let" Ident "=" Exp "in" Exp [")"]
        //     Exps = Exp "," (*) Exp [")"]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     Int = (*) Num ["(", ")", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        //     List = (*) App [")", "*", "+", "-", "<", "=", ">"]
        //     List = (*) App "::" List [")", "*", "+", "-", "<", "=", ">"]
        //     Mul = (*) List [")", "*", "+", "-", "<", "=", ">"]
        //     Mul = (*) Mul "*" List [")", "*", "+", "-", "<", "=", ">"]
        //     Num = (*) "Num" ["(", ")", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "true"]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        19, // on "fix", goto 18
        20, // on "fun", goto 19
        21, // on "head", goto 20
        22, // on "if", goto 21
        0, // on "in", error
        23, // on "let", goto 22
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 52
        //     Atom = "(" Exps ")" (*) ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        -15, // on "(", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        -15, // on ")", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        -15, // on "*", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        -15, // on "+", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        -15, // on ",", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        -15, // on "-", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        0, // on "->", error
        -15, // on ".1", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        -15, // on ".2", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        -15, // on "::", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        -15, // on "<", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        -15, // on "=", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        -15, // on ">", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        -15, // on "Iden", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        -15, // on "Num", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        -15, // on "else", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        -15, // on "empty", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        0, // on "empty?", error
        -15, // on "false", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -15, // on "in", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        0, // on "let", error
        0, // on "tail", error
        -15, // on "then", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        -15, // on "true", reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        // State 53
        //     Add = (*) Add "+" Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Add = (*) Add "-" Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Add = (*) Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Cmp = (*) Add [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add "<" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add "=" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add ">" Cmp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) Cmp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "fix" Ident "->" Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = "fix" Ident "->" (*) Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "fun" Ident "->" Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "if" Exp "then" Exp "else" Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "let" Ident "=" Exp "in" Exp [")", ",", "else", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     List = (*) App [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     List = (*) App "::" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        19, // on "fix", goto 18
        20, // on "fun", goto 19
        21, // on "head", goto 20
        22, // on "if", goto 21
        0, // on "in", error
        23, // on "let", goto 22
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 54
        //     Add = (*) Add "+" Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Add = (*) Add "-" Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Add = (*) Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Cmp = (*) Add [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add "<" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add "=" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add ">" Cmp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) Cmp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "fix" Ident "->" Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "fun" Ident "->" Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = "fun" Ident "->" (*) Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "if" Exp "then" Exp "else" Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "let" Ident "=" Exp "in" Exp [")", ",", "else", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     List = (*) App [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     List = (*) App "::" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        19, // on "fix", goto 18
        20, // on "fun", goto 19
        21, // on "head", goto 20
        22, // on "if", goto 21
        0, // on "in", error
        23, // on "let", goto 22
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 55
        //     Add = (*) Add "+" Mul ["+", "-", "<", "=", ">", "else"]
        //     Add = (*) Add "-" Mul ["+", "-", "<", "=", ">", "else"]
        //     Add = (*) Mul ["+", "-", "<", "=", ">", "else"]
        //     App = (*) App Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     App = (*) Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     App = (*) "empty?" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     App = (*) "head" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     App = (*) "tail" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Atom = (*) Atom ".1" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Atom = (*) Atom ".2" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Atom = (*) Ident ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Atom = (*) Int ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Atom = (*) "(" Exps ")" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Atom = (*) "empty" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Atom = (*) "false" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Atom = (*) "true" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Cmp = (*) Add ["else"]
        //     Cmp = (*) Add "<" Cmp ["else"]
        //     Cmp = (*) Add "=" Cmp ["else"]
        //     Cmp = (*) Add ">" Cmp ["else"]
        //     Exp = (*) Cmp ["else"]
        //     Exp = (*) "fix" Ident "->" Exp ["else"]
        //     Exp = (*) "fun" Ident "->" Exp ["else"]
        //     Exp = (*) "if" Exp "then" Exp "else" Exp ["else"]
        //     Exp = "if" Exp "then" (*) Exp "else" Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "let" Ident "=" Exp "in" Exp ["else"]
        //     Ident = (*) "Iden" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     Int = (*) Num ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        //     List = (*) App ["*", "+", "-", "<", "=", ">", "else"]
        //     List = (*) App "::" List ["*", "+", "-", "<", "=", ">", "else"]
        //     Mul = (*) List ["*", "+", "-", "<", "=", ">", "else"]
        //     Mul = (*) Mul "*" List ["*", "+", "-", "<", "=", ">", "else"]
        //     Num = (*) "Num" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "true"]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        19, // on "fix", goto 18
        20, // on "fun", goto 19
        21, // on "head", goto 20
        22, // on "if", goto 21
        0, // on "in", error
        23, // on "let", goto 22
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 56
        //     Add = (*) Add "+" Mul ["+", "-", "<", "=", ">", "in"]
        //     Add = (*) Add "-" Mul ["+", "-", "<", "=", ">", "in"]
        //     Add = (*) Mul ["+", "-", "<", "=", ">", "in"]
        //     App = (*) App Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     App = (*) Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     App = (*) "empty?" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     App = (*) "head" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     App = (*) "tail" Atom ["(", "*", "+", "-", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Atom = (*) Atom ".1" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Atom = (*) Atom ".2" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Atom = (*) Ident ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Atom = (*) Int ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Atom = (*) "(" Exps ")" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Atom = (*) "empty" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Atom = (*) "false" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Atom = (*) "true" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Cmp = (*) Add ["in"]
        //     Cmp = (*) Add "<" Cmp ["in"]
        //     Cmp = (*) Add "=" Cmp ["in"]
        //     Cmp = (*) Add ">" Cmp ["in"]
        //     Exp = (*) Cmp ["in"]
        //     Exp = (*) "fix" Ident "->" Exp ["in"]
        //     Exp = (*) "fun" Ident "->" Exp ["in"]
        //     Exp = (*) "if" Exp "then" Exp "else" Exp ["in"]
        //     Exp = (*) "let" Ident "=" Exp "in" Exp ["in"]
        //     Exp = "let" Ident "=" (*) Exp "in" Exp [")", ",", "else", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     Int = (*) Num ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        //     List = (*) App ["*", "+", "-", "<", "=", ">", "in"]
        //     List = (*) App "::" List ["*", "+", "-", "<", "=", ">", "in"]
        //     Mul = (*) List ["*", "+", "-", "<", "=", ">", "in"]
        //     Mul = (*) Mul "*" List ["*", "+", "-", "<", "=", ">", "in"]
        //     Num = (*) "Num" ["(", "*", "+", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "empty", "false", "in", "true"]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        19, // on "fix", goto 18
        20, // on "fun", goto 19
        21, // on "head", goto 20
        22, // on "if", goto 21
        0, // on "in", error
        23, // on "let", goto 22
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 57
        //     Exps = Exp "," Exp (*) [")"]
        0, // on "(", error
        -26, // on ")", reduce `Exps = Exp, ",", Exp => ActionFn(31);`
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 58
        //     Exp = "fix" Ident "->" Exp (*) [")", ",", "else", "in", "then", EOF]
        0, // on "(", error
        -24, // on ")", reduce `Exp = "fix", Ident, "->", Exp => ActionFn(5);`
        0, // on "*", error
        0, // on "+", error
        -24, // on ",", reduce `Exp = "fix", Ident, "->", Exp => ActionFn(5);`
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        -24, // on "else", reduce `Exp = "fix", Ident, "->", Exp => ActionFn(5);`
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -24, // on "in", reduce `Exp = "fix", Ident, "->", Exp => ActionFn(5);`
        0, // on "let", error
        0, // on "tail", error
        -24, // on "then", reduce `Exp = "fix", Ident, "->", Exp => ActionFn(5);`
        0, // on "true", error
        // State 59
        //     Exp = "fun" Ident "->" Exp (*) [")", ",", "else", "in", "then", EOF]
        0, // on "(", error
        -23, // on ")", reduce `Exp = "fun", Ident, "->", Exp => ActionFn(4);`
        0, // on "*", error
        0, // on "+", error
        -23, // on ",", reduce `Exp = "fun", Ident, "->", Exp => ActionFn(4);`
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        -23, // on "else", reduce `Exp = "fun", Ident, "->", Exp => ActionFn(4);`
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -23, // on "in", reduce `Exp = "fun", Ident, "->", Exp => ActionFn(4);`
        0, // on "let", error
        0, // on "tail", error
        -23, // on "then", reduce `Exp = "fun", Ident, "->", Exp => ActionFn(4);`
        0, // on "true", error
        // State 60
        //     Exp = "if" Exp "then" Exp (*) "else" Exp [")", ",", "else", "in", "then", EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        63, // on "else", goto 62
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        0, // on "in", error
        0, // on "let", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 61
        //     Exp = "let" Ident "=" Exp (*) "in" Exp [")", ",", "else", "in", "then", EOF]
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        0, // on "else", error
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        64, // on "in", goto 63
        0, // on "let", error
        0, // on "tail", error
        0, // on "then", error
        0, // on "true", error
        // State 62
        //     Add = (*) Add "+" Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Add = (*) Add "-" Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Add = (*) Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Cmp = (*) Add [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add "<" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add "=" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add ">" Cmp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) Cmp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "fix" Ident "->" Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "fun" Ident "->" Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "if" Exp "then" Exp "else" Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = "if" Exp "then" Exp "else" (*) Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "let" Ident "=" Exp "in" Exp [")", ",", "else", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     List = (*) App [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     List = (*) App "::" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        19, // on "fix", goto 18
        20, // on "fun", goto 19
        21, // on "head", goto 20
        22, // on "if", goto 21
        0, // on "in", error
        23, // on "let", goto 22
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 63
        //     Add = (*) Add "+" Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Add = (*) Add "-" Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Add = (*) Mul [")", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     App = (*) App Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "empty?" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "head" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     App = (*) "tail" Atom ["(", ")", "*", "+", ",", "-", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".1" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Atom ".2" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Ident ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) Int ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "(" Exps ")" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "empty" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "false" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Atom = (*) "true" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Cmp = (*) Add [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add "<" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add "=" Cmp [")", ",", "else", "in", "then", EOF]
        //     Cmp = (*) Add ">" Cmp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) Cmp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "fix" Ident "->" Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "fun" Ident "->" Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "if" Exp "then" Exp "else" Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = (*) "let" Ident "=" Exp "in" Exp [")", ",", "else", "in", "then", EOF]
        //     Exp = "let" Ident "=" Exp "in" (*) Exp [")", ",", "else", "in", "then", EOF]
        //     Ident = (*) "Iden" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     Int = (*) Num ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        //     List = (*) App [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     List = (*) App "::" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Mul = (*) Mul "*" List [")", "*", "+", ",", "-", "<", "=", ">", "else", "in", "then", EOF]
        //     Num = (*) "Num" ["(", ")", "*", "+", ",", "-", ".1", ".2", "::", "<", "=", ">", "Iden", "Num", "else", "empty", "false", "in", "then", "true", EOF]
        13, // on "(", goto 12
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        14, // on "Iden", goto 13
        15, // on "Num", goto 14
        0, // on "else", error
        16, // on "empty", goto 15
        17, // on "empty?", goto 16
        18, // on "false", goto 17
        19, // on "fix", goto 18
        20, // on "fun", goto 19
        21, // on "head", goto 20
        22, // on "if", goto 21
        0, // on "in", error
        23, // on "let", goto 22
        24, // on "tail", goto 23
        0, // on "then", error
        25, // on "true", goto 24
        // State 64
        //     Exp = "if" Exp "then" Exp "else" Exp (*) [")", ",", "else", "in", "then", EOF]
        0, // on "(", error
        -21, // on ")", reduce `Exp = "if", Exp, "then", Exp, "else", Exp => ActionFn(2);`
        0, // on "*", error
        0, // on "+", error
        -21, // on ",", reduce `Exp = "if", Exp, "then", Exp, "else", Exp => ActionFn(2);`
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        -21, // on "else", reduce `Exp = "if", Exp, "then", Exp, "else", Exp => ActionFn(2);`
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -21, // on "in", reduce `Exp = "if", Exp, "then", Exp, "else", Exp => ActionFn(2);`
        0, // on "let", error
        0, // on "tail", error
        -21, // on "then", reduce `Exp = "if", Exp, "then", Exp, "else", Exp => ActionFn(2);`
        0, // on "true", error
        // State 65
        //     Exp = "let" Ident "=" Exp "in" Exp (*) [")", ",", "else", "in", "then", EOF]
        0, // on "(", error
        -22, // on ")", reduce `Exp = "let", Ident, "=", Exp, "in", Exp => ActionFn(3);`
        0, // on "*", error
        0, // on "+", error
        -22, // on ",", reduce `Exp = "let", Ident, "=", Exp, "in", Exp => ActionFn(3);`
        0, // on "-", error
        0, // on "->", error
        0, // on ".1", error
        0, // on ".2", error
        0, // on "::", error
        0, // on "<", error
        0, // on "=", error
        0, // on ">", error
        0, // on "Iden", error
        0, // on "Num", error
        -22, // on "else", reduce `Exp = "let", Ident, "=", Exp, "in", Exp => ActionFn(3);`
        0, // on "empty", error
        0, // on "empty?", error
        0, // on "false", error
        0, // on "fix", error
        0, // on "fun", error
        0, // on "head", error
        0, // on "if", error
        -22, // on "in", reduce `Exp = "let", Ident, "=", Exp, "in", Exp => ActionFn(3);`
        0, // on "let", error
        0, // on "tail", error
        -22, // on "then", reduce `Exp = "let", Ident, "=", Exp, "in", Exp => ActionFn(3);`
        0, // on "true", error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -20, // on EOF, reduce `Cmp = Add => ActionFn(10);`
        -31, // on EOF, reduce `List = App => ActionFn(17);`
        -8, // on EOF, reduce `App = Atom => ActionFn(22);`
        -25, // on EOF, reduce `Exp = Cmp => ActionFn(6);`
        -35, // on EOF, reduce `Program = Exp => ActionFn(1);`
        -9, // on EOF, reduce `Atom = Ident => ActionFn(23);`
        -16, // on EOF, reduce `Atom = Int => ActionFn(30);`
        -33, // on EOF, reduce `Mul = List => ActionFn(15);`
        -3, // on EOF, reduce `Add = Mul => ActionFn(13);`
        -29, // on EOF, reduce `Int = Num => ActionFn(33);`
        -36, // on EOF, reduce `__Program = Program => ActionFn(0);`
        0, // on EOF, error
        -28, // on EOF, reduce `Ident = "Iden" => ActionFn(34);`
        -34, // on EOF, reduce `Num = "Num" => ActionFn(35);`
        -12, // on EOF, reduce `Atom = "empty" => ActionFn(26);`
        0, // on EOF, error
        -11, // on EOF, reduce `Atom = "false" => ActionFn(25);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -10, // on EOF, reduce `Atom = "true" => ActionFn(24);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -7, // on EOF, reduce `App = App, Atom => ActionFn(21);`
        0, // on EOF, error
        -13, // on EOF, reduce `Atom = Atom, ".1" => ActionFn(27);`
        -14, // on EOF, reduce `Atom = Atom, ".2" => ActionFn(28);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -6, // on EOF, reduce `App = "empty?", Atom => ActionFn(20);`
        0, // on EOF, error
        0, // on EOF, error
        -4, // on EOF, reduce `App = "head", Atom => ActionFn(18);`
        0, // on EOF, error
        0, // on EOF, error
        -5, // on EOF, reduce `App = "tail", Atom => ActionFn(19);`
        -2, // on EOF, reduce `Add = Add, "+", Mul => ActionFn(12);`
        -1, // on EOF, reduce `Add = Add, "-", Mul => ActionFn(11);`
        -18, // on EOF, reduce `Cmp = Add, "<", Cmp => ActionFn(8);`
        -17, // on EOF, reduce `Cmp = Add, "=", Cmp => ActionFn(7);`
        -19, // on EOF, reduce `Cmp = Add, ">", Cmp => ActionFn(9);`
        -30, // on EOF, reduce `List = App, "::", List => ActionFn(16);`
        -32, // on EOF, reduce `Mul = Mul, "*", List => ActionFn(14);`
        0, // on EOF, error
        -15, // on EOF, reduce `Atom = "(", Exps, ")" => ActionFn(29);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -24, // on EOF, reduce `Exp = "fix", Ident, "->", Exp => ActionFn(5);`
        -23, // on EOF, reduce `Exp = "fun", Ident, "->", Exp => ActionFn(4);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -21, // on EOF, reduce `Exp = "if", Exp, "then", Exp, "else", Exp => ActionFn(2);`
        -22, // on EOF, reduce `Exp = "let", Ident, "=", Exp, "in", Exp => ActionFn(3);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        6, // on Exp, goto 5
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        9, // on List, goto 8
        10, // on Mul, goto 9
        11, // on Num, goto 10
        12, // on Program, goto 11
        0, // on __Program, error
        // State 1
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
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
        31, // on Atom, goto 30
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        0, // on List, error
        0, // on Mul, error
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 3
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 12
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        36, // on Exp, goto 35
        37, // on Exps, goto 36
        7, // on Ident, goto 6
        8, // on Int, goto 7
        9, // on List, goto 8
        10, // on Mul, goto 9
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 13
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 14
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        38, // on Atom, goto 37
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        0, // on List, error
        0, // on Mul, error
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 17
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 18
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        39, // on Ident, goto 38
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 19
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        40, // on Ident, goto 39
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 20
        0, // on Add, error
        0, // on App, error
        41, // on Atom, goto 40
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        0, // on List, error
        0, // on Mul, error
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 21
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        42, // on Exp, goto 41
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        9, // on List, goto 8
        10, // on Mul, goto 9
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 22
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        43, // on Ident, goto 42
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 23
        0, // on Add, error
        0, // on App, error
        44, // on Atom, goto 43
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        0, // on List, error
        0, // on Mul, error
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 24
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 25
        0, // on Add, error
        3, // on App, goto 2
        4, // on Atom, goto 3
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        9, // on List, goto 8
        45, // on Mul, goto 44
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 26
        0, // on Add, error
        3, // on App, goto 2
        4, // on Atom, goto 3
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        9, // on List, goto 8
        46, // on Mul, goto 45
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 27
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        47, // on Cmp, goto 46
        0, // on Exp, error
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        9, // on List, goto 8
        10, // on Mul, goto 9
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 28
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        48, // on Cmp, goto 47
        0, // on Exp, error
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        9, // on List, goto 8
        10, // on Mul, goto 9
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 29
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        49, // on Cmp, goto 48
        0, // on Exp, error
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        9, // on List, goto 8
        10, // on Mul, goto 9
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 30
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 31
        0, // on Add, error
        3, // on App, goto 2
        4, // on Atom, goto 3
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        50, // on List, goto 49
        0, // on Mul, error
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 32
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 33
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 34
        0, // on Add, error
        3, // on App, goto 2
        4, // on Atom, goto 3
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        51, // on List, goto 50
        0, // on Mul, error
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 35
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 36
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 37
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 38
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 40
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 41
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 51
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        58, // on Exp, goto 57
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        9, // on List, goto 8
        10, // on Mul, goto 9
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 52
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 53
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        59, // on Exp, goto 58
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        9, // on List, goto 8
        10, // on Mul, goto 9
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 54
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        60, // on Exp, goto 59
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        9, // on List, goto 8
        10, // on Mul, goto 9
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 55
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        61, // on Exp, goto 60
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        9, // on List, goto 8
        10, // on Mul, goto 9
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 56
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        62, // on Exp, goto 61
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        9, // on List, goto 8
        10, // on Mul, goto 9
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 57
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
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
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 60
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 61
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 62
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        65, // on Exp, goto 64
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        9, // on List, goto 8
        10, // on Mul, goto 9
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 63
        2, // on Add, goto 1
        3, // on App, goto 2
        4, // on Atom, goto 3
        5, // on Cmp, goto 4
        66, // on Exp, goto 65
        0, // on Exps, error
        7, // on Ident, goto 6
        8, // on Int, goto 7
        9, // on List, goto 8
        10, // on Mul, goto 9
        11, // on Num, goto 10
        0, // on Program, error
        0, // on __Program, error
        // State 64
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
        0, // on Ident, error
        0, // on Int, error
        0, // on List, error
        0, // on Mul, error
        0, // on Num, error
        0, // on Program, error
        0, // on __Program, error
        // State 65
        0, // on Add, error
        0, // on App, error
        0, // on Atom, error
        0, // on Cmp, error
        0, // on Exp, error
        0, // on Exps, error
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
    ) -> Result<Box<Exp>, __lalrpop_util::ParseError<usize,Tok<'input>,tok::Error>>
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
                (_, Tok::Dot1, _) if true => 7,
                (_, Tok::Dot2, _) if true => 8,
                (_, Tok::Cons, _) if true => 9,
                (_, Tok::Lt, _) if true => 10,
                (_, Tok::Eq, _) if true => 11,
                (_, Tok::Gt, _) if true => 12,
                (_, Tok::Ident(_), _) if true => 13,
                (_, Tok::Num(_), _) if true => 14,
                (_, Tok::Else, _) if true => 15,
                (_, Tok::Empty, _) if true => 16,
                (_, Tok::Emptyq, _) if true => 17,
                (_, Tok::False, _) if true => 18,
                (_, Tok::Fix, _) if true => 19,
                (_, Tok::Fun, _) if true => 20,
                (_, Tok::Head, _) if true => 21,
                (_, Tok::If, _) if true => 22,
                (_, Tok::In, _) if true => 23,
                (_, Tok::Let, _) if true => 24,
                (_, Tok::Tail, _) if true => 25,
                (_, Tok::Then, _) if true => 26,
                (_, Tok::True, _) if true => 27,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 28 + __integer];
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
                            __tok @ Tok::Dot1 => __Symbol::Term_22_2e1_22(__tok),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Tok::Dot2 => __Symbol::Term_22_2e2_22(__tok),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            __tok @ Tok::Cons => __Symbol::Term_22_3a_3a_22(__tok),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            __tok @ Tok::Lt => __Symbol::Term_22_3c_22(__tok),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Tok::Eq => __Symbol::Term_22_3d_22(__tok),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            __tok @ Tok::Gt => __Symbol::Term_22_3e_22(__tok),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            Tok::Ident(__tok0) => __Symbol::Term_22Iden_22(__tok0),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            Tok::Num(__tok0) => __Symbol::Term_22Num_22(__tok0),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            __tok @ Tok::Else => __Symbol::Term_22else_22(__tok),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            __tok @ Tok::Empty => __Symbol::Term_22empty_22(__tok),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            __tok @ Tok::Emptyq => __Symbol::Term_22empty_3f_22(__tok),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            __tok @ Tok::False => __Symbol::Term_22false_22(__tok),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            __tok @ Tok::Fix => __Symbol::Term_22fix_22(__tok),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            __tok @ Tok::Fun => __Symbol::Term_22fun_22(__tok),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            __tok @ Tok::Head => __Symbol::Term_22head_22(__tok),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            __tok @ Tok::If => __Symbol::Term_22if_22(__tok),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            __tok @ Tok::In => __Symbol::Term_22in_22(__tok),
                            _ => unreachable!(),
                        },
                        24 => match __lookahead.1 {
                            __tok @ Tok::Let => __Symbol::Term_22let_22(__tok),
                            _ => unreachable!(),
                        },
                        25 => match __lookahead.1 {
                            __tok @ Tok::Tail => __Symbol::Term_22tail_22(__tok),
                            _ => unreachable!(),
                        },
                        26 => match __lookahead.1 {
                            __tok @ Tok::Then => __Symbol::Term_22then_22(__tok),
                            _ => unreachable!(),
                        },
                        27 => match __lookahead.1 {
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
    ) -> Option<Result<Box<Exp>,__lalrpop_util::ParseError<usize,Tok<'input>,tok::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Add = Add, "-", Mul => ActionFn(11);
                let __sym2 = __pop_NtMul(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAdd(__nt), __end));
                0
            }
            2 => {
                // Add = Add, "+", Mul => ActionFn(12);
                let __sym2 = __pop_NtMul(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAdd(__nt), __end));
                0
            }
            3 => {
                // Add = Mul => ActionFn(13);
                let __sym0 = __pop_NtMul(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAdd(__nt), __end));
                0
            }
            4 => {
                // App = "head", Atom => ActionFn(18);
                let __sym1 = __pop_NtAtom(__symbols);
                let __sym0 = __pop_Term_22head_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtApp(__nt), __end));
                1
            }
            5 => {
                // App = "tail", Atom => ActionFn(19);
                let __sym1 = __pop_NtAtom(__symbols);
                let __sym0 = __pop_Term_22tail_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtApp(__nt), __end));
                1
            }
            6 => {
                // App = "empty?", Atom => ActionFn(20);
                let __sym1 = __pop_NtAtom(__symbols);
                let __sym0 = __pop_Term_22empty_3f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtApp(__nt), __end));
                1
            }
            7 => {
                // App = App, Atom => ActionFn(21);
                let __sym1 = __pop_NtAtom(__symbols);
                let __sym0 = __pop_NtApp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtApp(__nt), __end));
                1
            }
            8 => {
                // App = Atom => ActionFn(22);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtApp(__nt), __end));
                1
            }
            9 => {
                // Atom = Ident => ActionFn(23);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                2
            }
            10 => {
                // Atom = "true" => ActionFn(24);
                let __sym0 = __pop_Term_22true_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                2
            }
            11 => {
                // Atom = "false" => ActionFn(25);
                let __sym0 = __pop_Term_22false_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                2
            }
            12 => {
                // Atom = "empty" => ActionFn(26);
                let __sym0 = __pop_Term_22empty_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                2
            }
            13 => {
                // Atom = Atom, ".1" => ActionFn(27);
                let __sym1 = __pop_Term_22_2e1_22(__symbols);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action27::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                2
            }
            14 => {
                // Atom = Atom, ".2" => ActionFn(28);
                let __sym1 = __pop_Term_22_2e2_22(__symbols);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(text, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                2
            }
            15 => {
                // Atom = "(", Exps, ")" => ActionFn(29);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExps(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action29::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                2
            }
            16 => {
                // Atom = Int => ActionFn(30);
                let __sym0 = __pop_NtInt(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                2
            }
            17 => {
                // Cmp = Add, "=", Cmp => ActionFn(7);
                let __sym2 = __pop_NtCmp(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtCmp(__nt), __end));
                3
            }
            18 => {
                // Cmp = Add, "<", Cmp => ActionFn(8);
                let __sym2 = __pop_NtCmp(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtCmp(__nt), __end));
                3
            }
            19 => {
                // Cmp = Add, ">", Cmp => ActionFn(9);
                let __sym2 = __pop_NtCmp(__symbols);
                let __sym1 = __pop_Term_22_3e_22(__symbols);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtCmp(__nt), __end));
                3
            }
            20 => {
                // Cmp = Add => ActionFn(10);
                let __sym0 = __pop_NtAdd(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCmp(__nt), __end));
                3
            }
            21 => {
                // Exp = "if", Exp, "then", Exp, "else", Exp => ActionFn(2);
                let __sym5 = __pop_NtExp(__symbols);
                let __sym4 = __pop_Term_22else_22(__symbols);
                let __sym3 = __pop_NtExp(__symbols);
                let __sym2 = __pop_Term_22then_22(__symbols);
                let __sym1 = __pop_NtExp(__symbols);
                let __sym0 = __pop_Term_22if_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action2::<>(text, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtExp(__nt), __end));
                4
            }
            22 => {
                // Exp = "let", Ident, "=", Exp, "in", Exp => ActionFn(3);
                let __sym5 = __pop_NtExp(__symbols);
                let __sym4 = __pop_Term_22in_22(__symbols);
                let __sym3 = __pop_NtExp(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22let_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action3::<>(text, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtExp(__nt), __end));
                4
            }
            23 => {
                // Exp = "fun", Ident, "->", Exp => ActionFn(4);
                let __sym3 = __pop_NtExp(__symbols);
                let __sym2 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22fun_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action4::<>(text, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExp(__nt), __end));
                4
            }
            24 => {
                // Exp = "fix", Ident, "->", Exp => ActionFn(5);
                let __sym3 = __pop_NtExp(__symbols);
                let __sym2 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22fix_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action5::<>(text, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExp(__nt), __end));
                4
            }
            25 => {
                // Exp = Cmp => ActionFn(6);
                let __sym0 = __pop_NtCmp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExp(__nt), __end));
                4
            }
            26 => {
                // Exps = Exp, ",", Exp => ActionFn(31);
                let __sym2 = __pop_NtExp(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtExp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action31::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExps(__nt), __end));
                5
            }
            27 => {
                // Exps = Exp => ActionFn(32);
                let __sym0 = __pop_NtExp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExps(__nt), __end));
                5
            }
            28 => {
                // Ident = "Iden" => ActionFn(34);
                let __sym0 = __pop_Term_22Iden_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent(__nt), __end));
                6
            }
            29 => {
                // Int = Num => ActionFn(33);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInt(__nt), __end));
                7
            }
            30 => {
                // List = App, "::", List => ActionFn(16);
                let __sym2 = __pop_NtList(__symbols);
                let __sym1 = __pop_Term_22_3a_3a_22(__symbols);
                let __sym0 = __pop_NtApp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                8
            }
            31 => {
                // List = App => ActionFn(17);
                let __sym0 = __pop_NtApp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtList(__nt), __end));
                8
            }
            32 => {
                // Mul = Mul, "*", List => ActionFn(14);
                let __sym2 = __pop_NtList(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtMul(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(text, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtMul(__nt), __end));
                9
            }
            33 => {
                // Mul = List => ActionFn(15);
                let __sym0 = __pop_NtList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMul(__nt), __end));
                9
            }
            34 => {
                // Num = "Num" => ActionFn(35);
                let __sym0 = __pop_Term_22Num_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                10
            }
            35 => {
                // Program = Exp => ActionFn(1);
                let __sym0 = __pop_NtExp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(text, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                11
            }
            36 => {
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
        let __next_state = __GOTO[__state * 13 + __nonterminal] - 1;
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
    fn __pop_Term_22_2e1_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2e1_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2e2_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2e2_22(__v), __r) => (__l, __v, __r),
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
    ) -> (usize, Box<Exp>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAdd(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtApp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Exp>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtApp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAtom<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Exp>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAtom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCmp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Exp>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCmp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Exp>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExps<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Exp>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExps(__v), __r) => (__l, __v, __r),
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
    ) -> (usize, Box<Exp>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInt(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Exp>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMul<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Exp>, usize) {
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
    ) -> (usize, Box<Exp>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Exp>, usize) {
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
    (_, __0, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, c, _): (usize, Box<Exp>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, t, _): (usize, Box<Exp>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, f, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
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
    (_, v, _): (usize, Box<Exp>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, e, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
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
    (_, e, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
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
    (_, e, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    Box::new(Fix(id, e))
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Exp>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    Box::new(Op2(Eq, l, r))
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Exp>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    Box::new(Op2(LT, l, r))
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Exp>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    Box::new(Op2(GT, l, r))
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Exp>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    Box::new(Op2(Sub, l, r))
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Exp>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    Box::new(Op2(Add, l, r))
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Exp>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    Box::new(Op2(Mul, l, r))
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    text: &'input str,
    (_, hd, _): (usize, Box<Exp>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, tl, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    Box::new(Cons(hd, tl))
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, l, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    Box::new(Head(l))
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, l, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    Box::new(Tail(l))
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, l, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    Box::new(IsEmpty(l))
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
>(
    text: &'input str,
    (_, a, _): (usize, Box<Exp>, usize),
    (_, b, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    Box::new(App(a, b))
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action23<
    'input,
>(
    text: &'input str,
    (_, id, _): (usize, Id, usize),
) -> Box<Exp>
{
    Box::new(Var(id))
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Box<Exp>
{
    Box::new(Const(Bool(true)))
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Box<Exp>
{
    Box::new(Const(Bool(true)))
}

#[allow(unused_variables)]
pub fn __action26<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> Box<Exp>
{
    Box::new(Empty)
}

#[allow(unused_variables)]
pub fn __action27<
    'input,
>(
    text: &'input str,
    (_, p, _): (usize, Box<Exp>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Box<Exp>
{
    Box::new(ProjL(p))
}

#[allow(unused_variables)]
pub fn __action28<
    'input,
>(
    text: &'input str,
    (_, p, _): (usize, Box<Exp>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Box<Exp>
{
    Box::new(ProjR(p))
}

#[allow(unused_variables)]
pub fn __action29<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __0, _): (usize, Box<Exp>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Box<Exp>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action30<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action31<
    'input,
>(
    text: &'input str,
    (_, l, _): (usize, Box<Exp>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, r, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    Box::new(Pair(l, r))
}

#[allow(unused_variables)]
pub fn __action32<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Box<Exp>, usize),
) -> Box<Exp>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action33<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, i64, usize),
) -> Box<Exp>
{
    Box::new(Const(Int(__0)))
}

#[allow(unused_variables)]
pub fn __action34<
    'input,
>(
    text: &'input str,
    (_, id, _): (usize, &'input str, usize),
) -> Id
{
    String::from(id)
}

#[allow(unused_variables)]
pub fn __action35<
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
