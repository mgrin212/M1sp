use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0},
    combinator::{map, map_res, recognize},
    multi::many0,
    sequence::{delimited, pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Prim1 {
    Add1,
    Sub1,
    IsZero,
    IsNum,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum Prim2 {
    Plus,
    Minus,
    Eq,
    Lt,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(i64),
    Id(String),
    True,
    False,
    Nil,
    UnPrim(Prim1, Box<Expr>),
    BinPrim(Prim2, Box<Expr>, Box<Expr>),
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
        |s: &str| {
            match s {
                "true" => Expr::True,
                "false" => Expr::False,
                _ => Expr::Id(s.to_string()),
            }
        },
    )(input)
}

fn parse_true(input: &str) -> IResult<&str, Expr> {
    map(tag("true"), |_| Expr::True)(input)
}

fn parse_false(input: &str) -> IResult<&str, Expr> {
    map(tag("false"), |_| Expr::False)(input)
}

fn parse_nil(input: &str) -> IResult<&str, Expr> {
    map(tag("()"), |_| Expr::Nil)(input)
}

fn parse_un_prim(input: &str) -> IResult<&str, Prim1> {
    alt((
        map(tag("add1"), |_| Prim1::Add1),
        map(tag("sub1"), |_| Prim1::Sub1),
        map(tag("zero?"), |_| Prim1::IsZero),
        map(tag("num?"), |_| Prim1::IsNum),
        map(tag("not"), |_| Prim1::Not),
    ))(input)
}

fn parse_bin_prim(input: &str) -> IResult<&str, Prim2> {
    alt((
        map(char('+'), |_| Prim2::Plus),
        map(char('-'), |_| Prim2::Minus),
        map(char('='), |_| Prim2::Eq),
        map(char('<'), |_| Prim2::Lt),
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

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_num,
        parse_id,
        parse_nil,
        parse_un_prim_expr,
        parse_bin_prim_expr,
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
                Err(format!("Parsing incomplete. Remaining input: '{}'", remaining))
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
            Ok(vec![Expr::UnPrim(Prim1::Add1, Box::new(Expr::Num(5)))])
        );
        assert_eq!(
            parse("(+ 3 4)"),
            Ok(vec![Expr::BinPrim(
                Prim2::Plus,
                Box::new(Expr::Num(3)),
                Box::new(Expr::Num(4))
            )])
        );
    }

    #[test]
    fn test_parse_multiple_expressions() {
        assert_eq!(
            parse("42 x true"),
            Ok(vec![
                Expr::Num(42),
                Expr::Id("x".to_string()),
                Expr::True
            ])
        );
        assert_eq!(
            parse("(add1 5) (+ 3 4)"),
            Ok(vec![
                Expr::UnPrim(Prim1::Add1, Box::new(Expr::Num(5))),
                Expr::BinPrim(
                    Prim2::Plus,
                    Box::new(Expr::Num(3)),
                    Box::new(Expr::Num(4))
                )
            ])
        );
    }

    #[test]
    fn test_parse_with_whitespace() {
        assert_eq!(
            parse("  42  \n  x  \t true  "),
            Ok(vec![
                Expr::Num(42),
                Expr::Id("x".to_string()),
                Expr::True
            ])
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