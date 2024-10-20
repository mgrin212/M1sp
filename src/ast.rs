use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0},
    combinator::{map, map_res, recognize},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub enum UnPrim {
    Add1,
    Sub1,
    IsZero,
    IsNum,
    Not,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinPrim {
    Plus,
    Minus,
    Eq,
    Lt,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Num(i64),
    Id(String),
    True,
    False,
    Nil,
    UnPrim(UnPrim, Box<Expr>),
    BinPrim(BinPrim, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Vec<(String, Box<Expr>)>, Box<Expr>),
    Do(Vec<Expr>),
    Define(String, Vec<String>, Box<Expr>),
    Call(String, Vec<Expr>),
}

fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

fn parse_num(input: &str) -> IResult<&str, Expr> {
    map_res(recognize(digit1), |digit_str: &str| {
        digit_str.parse::<i64>().map(Expr::Num)
    })(input)
}

fn parse_id(input: &str) -> IResult<&str, Expr> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s: &str| match s {
            "true" => Expr::True,
            "false" => Expr::False,
            _ => Expr::Id(s.to_string()),
        },
    )(input)
}

fn parse_nil(input: &str) -> IResult<&str, Expr> {
    map(tag("()"), |_| Expr::Nil)(input)
}

fn parse_un_prim(input: &str) -> IResult<&str, UnPrim> {
    alt((
        map(tag("add1"), |_| UnPrim::Add1),
        map(tag("sub1"), |_| UnPrim::Sub1),
        map(tag("zero?"), |_| UnPrim::IsZero),
        map(tag("num?"), |_| UnPrim::IsNum),
        map(tag("not"), |_| UnPrim::Not),
    ))(input)
}

fn parse_bin_prim(input: &str) -> IResult<&str, BinPrim> {
    alt((
        map(char('+'), |_| BinPrim::Plus),
        map(char('-'), |_| BinPrim::Minus),
        map(char('='), |_| BinPrim::Eq),
        map(char('<'), |_| BinPrim::Lt),
    ))(input)
}

fn parse_un_prim_expr(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            char('('),
            tuple((ws(parse_un_prim), ws(parse_expr))),
            char(')'),
        ),
        |(prim, expr)| Expr::UnPrim(prim, Box::new(expr)),
    )(input)
}

fn parse_bin_prim_expr(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            char('('),
            tuple((ws(parse_bin_prim), ws(parse_expr), ws(parse_expr))),
            char(')'),
        ),
        |(op, left, right)| Expr::BinPrim(op, Box::new(left), Box::new(right)),
    )(input)
}

fn parse_if_expr(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            char('('),
            preceded(
                ws(tag("if")),
                tuple((ws(parse_expr), ws(parse_expr), ws(parse_expr))),
            ),
            char(')'),
        ),
        |(cond, then_expr, else_expr)| {
            Expr::If(Box::new(cond), Box::new(then_expr), Box::new(else_expr))
        },
    )(input)
}

fn parse_call_expr(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            char('('),
            tuple((
                ws(map(parse_id, |expr| match expr {
                    Expr::Id(s) => s,
                    _ => unreachable!(),
                })),
                many0(ws(parse_expr)),
            )),
            char(')'),
        ),
        |(func, args)| Expr::Call(func, args),
    )(input)
}

fn parse_let_binding(input: &str) -> IResult<&str, (String, Box<Expr>)> {
    delimited(
        char('('),
        tuple((
            ws(map(parse_id, |expr| match expr {
                Expr::Id(s) => s,
                _ => unreachable!(),
            })),
            ws(map(parse_expr, Box::new)),
        )),
        char(')'),
    )(input)
}

fn parse_let_expr(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            char('('),
            preceded(
                ws(tag("let")),
                tuple((
                    delimited(char('('), many0(ws(parse_let_binding)), char(')')),
                    ws(parse_expr),
                )),
            ),
            char(')'),
        ),
        |(bindings, body)| Expr::Let(bindings, Box::new(body)),
    )(input)
}

fn parse_do_expr(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            char('('),
            preceded(ws(tag("do")), many1(ws(parse_expr))),
            char(')'),
        ),
        Expr::Do,
    )(input)
}

fn parse_def_expr(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            char('('),
            preceded(
                ws(tag("define")),
                tuple((
                    delimited(
                        char('('),
                        tuple((
                            ws(map(parse_id, |expr| match expr {
                                Expr::Id(s) => s,
                                _ => unreachable!(),
                            })),
                            many0(ws(map(parse_id, |expr| match expr {
                                Expr::Id(s) => s,
                                _ => unreachable!(),
                            }))),
                        )),
                        char(')'),
                    ),
                    ws(parse_expr),
                )),
            ),
            char(')'),
        ),
        |((name, params), body)| Expr::Define(name, params, Box::new(body)),
    )(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_num,
        parse_id,
        parse_nil,
        parse_un_prim_expr,
        parse_bin_prim_expr,
        parse_if_expr,
        parse_let_expr,
        parse_do_expr,
        parse_def_expr,
        parse_call_expr,
    ))(input)
}

fn parse_multiple_expr(input: &str) -> IResult<&str, Vec<Expr>> {
    many0(ws(parse_expr))(input)
}

pub fn parse(input: &str) -> Result<Vec<Expr>, String> {
    match parse_multiple_expr(input) {
        Ok((remaining, exprs)) => {
            if remaining.trim().is_empty() {
                Ok(exprs)
            } else {
                Err(format!(
                    "Parsing incomplete. Remaining input: '{}'",
                    remaining
                ))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_expressions() {
        assert_eq!(parse("42"), Ok(vec![Expr::Num(42)]));
        assert_eq!(parse("x"), Ok(vec![Expr::Id("x".to_string())]));
        assert_eq!(parse("true"), Ok(vec![Expr::True]));
        assert_eq!(parse("false"), Ok(vec![Expr::False]));
        assert_eq!(parse("()"), Ok(vec![Expr::Nil]));
        assert_eq!(
            parse("(add1 5)"),
            Ok(vec![Expr::UnPrim(UnPrim::Add1, Box::new(Expr::Num(5)))])
        );
        assert_eq!(
            parse("(+ 3 4)"),
            Ok(vec![Expr::BinPrim(
                BinPrim::Plus,
                Box::new(Expr::Num(3)),
                Box::new(Expr::Num(4))
            )])
        );
    }

    #[test]
    fn test_parse_if_expr() {
        assert_eq!(
            parse("(if true 1 2)"),
            Ok(vec![Expr::If(
                Box::new(Expr::True),
                Box::new(Expr::Num(1)),
                Box::new(Expr::Num(2))
            )])
        );
    }

    #[test]
    fn test_parse_let_expr() {
        assert_eq!(
            parse("(let ((x 1) (y 2)) (+ x y))"),
            Ok(vec![Expr::Let(
                vec![
                    ("x".to_string(), Box::new(Expr::Num(1))),
                    ("y".to_string(), Box::new(Expr::Num(2)))
                ],
                Box::new(Expr::BinPrim(
                    BinPrim::Plus,
                    Box::new(Expr::Id("x".to_string())),
                    Box::new(Expr::Id("y".to_string()))
                ))
            )])
        );
    }

    #[test]
    fn test_parse_do_expr() {
        assert_eq!(
            parse("(do 1 2 3)"),
            Ok(vec![Expr::Do(vec![
                Expr::Num(1),
                Expr::Num(2),
                Expr::Num(3)
            ])])
        );
    }

    #[test]
    fn test_parse_multiple_expressions() {
        assert_eq!(
            parse("42 x true"),
            Ok(vec![Expr::Num(42), Expr::Id("x".to_string()), Expr::True])
        );
    }

    #[test]
    fn test_parse_with_whitespace() {
        assert_eq!(
            parse("  42  \n  x  \t true  "),
            Ok(vec![Expr::Num(42), Expr::Id("x".to_string()), Expr::True])
        );
    }

    #[test]
    fn test_parse_empty_input() {
        assert_eq!(parse(""), Ok(vec![]));
        assert_eq!(parse("   \n  \t  "), Ok(vec![]));
    }

    #[test]
    fn test_parse_invalid_input() {
        assert!(parse("@").is_err());
        assert!(parse("42 @ true").is_err());
    }
}
