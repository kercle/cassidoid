use numbers::RealScalar;

use crate::{
    ast::{AstNode, BinaryOp, UnaryOp},
    error::{BoxedError, ParseError},
    lex::{Token, TokenStream},
};

fn parse_named_value_or_function_call(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <named_value_or_function_call> ::= <identifier>
    //    | <identifier> "(" ")"
    //    | <identifier> "(" <expression> { "," <expression> }* ")"

    let identifier = stream.next_if_identifier();

    if identifier.is_none() {
        return Err(ParseError {
            message: "Expected an identifier".to_string(),
            at_token: stream.peek().cloned(),
        });
    }

    let identifier = identifier.unwrap().to_owned();

    if stream.next_if_matches_token(&Token::LeftParen).is_none() {
        return Ok(AstNode::NamedValue(identifier.to_string()));
    }

    if stream.next_if_matches_token(&Token::RightParen).is_some() {
        return Ok(AstNode::FunctionCall {
            name: identifier.to_string(),
            args: Vec::new(),
        });
    }

    let mut args = vec![parse_expression(stream)?];
    loop {
        if stream.next_if_matches_token(&Token::Comma).is_none() {
            break; // No more arguments
        }

        args.push(parse_expression(stream)?);
    }

    if stream.next_if_matches_token(&Token::RightParen).is_none() {
        return Err(ParseError {
            message: "Expected an identifier".to_string(),
            at_token: stream.peek().cloned(),
        });
    }

    Ok(AstNode::FunctionCall {
        name: identifier.to_string(),
        args,
    })
}

fn parse_atom(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <atom> ::= <number>
    //    | "(" <expression> ")"
    //    | <named_value_or_function_call>

    if stream.next_if_matches_token(&Token::LeftParen).is_some() {
        let expr = parse_expression(stream)?;

        if stream.next_if_matches_token(&Token::RightParen).is_some() {
            return Ok(expr);
        } else {
            return Err(ParseError {
                message: "Expected closing parenthesis ')'".to_string(),
                at_token: stream.peek().cloned(),
            });
        }
    } else if let Some(value) = stream.next_if_number() {
        let scalar = RealScalar::from_str(value).map_err(|e| ParseError {
            message: e,
            at_token: stream.peek().cloned(),
        })?;
        Ok(AstNode::Number(scalar))
    } else {
        parse_named_value_or_function_call(stream)
    }
}

fn parse_power(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <power> ::= <atom> { "^" <power> }

    let mut result = parse_atom(stream)?;

    if stream.next_if_matches_token(&Token::Caret).is_some() {
        result = AstNode::BinaryOp {
            op: BinaryOp::Pow,
            lhs: Box::new(result),
            rhs: Box::new(parse_power(stream)?),
        };
    }

    Ok(result)
}

fn parse_signed_power(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <signed_power> ::= { "+" | "-" }* power

    let mut negate_count = 0;

    while let Some(sign) =
        stream.next_if_matches(|token| matches!(token, Token::Plus | Token::Minus))
    {
        if *sign == Token::Minus {
            negate_count += 1;
        }
    }

    let ast = parse_power(stream)?;

    if negate_count % 2 == 1 {
        Ok(AstNode::UnaryOp {
            op: UnaryOp::Negate,
            expr: Box::new(ast),
        })
    } else {
        Ok(ast)
    }
}

fn parse_product(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <product> ::= <match_signed_power> { ("*"|"/") <match_signed_power> }*

    let mut result = parse_signed_power(stream)?;

    loop {
        let c = stream.next_if_matches(|token| matches!(token, Token::Asterisk | Token::Slash));

        if c.is_none() {
            break; // No more operators
        }

        result = AstNode::BinaryOp {
            op: match c {
                Some(Token::Asterisk) => BinaryOp::Mul,
                Some(Token::Slash) => BinaryOp::Div,
                _ => unreachable!(),
            },
            lhs: Box::new(result),
            rhs: Box::new(parse_signed_power(stream)?),
        };
    }

    Ok(result)
}

fn parse_sum(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <sum> ::= <product> { ("+"|"-") <product> }*

    let mut result = parse_product(stream)?;

    loop {
        let op = stream.next_if_matches(|token| matches!(token, Token::Plus | Token::Minus));
        if op.is_none() {
            break; // No more operators
        }

        result = AstNode::BinaryOp {
            op: match op {
                Some(Token::Plus) => BinaryOp::Add,
                Some(Token::Minus) => BinaryOp::Sub,
                _ => unreachable!(),
            },
            lhs: Box::new(result),
            rhs: Box::new(parse_product(stream)?),
        };
    }

    Ok(result)
}

fn parse_expression(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <expression> ::= <sum>

    parse_sum(stream)
}

pub fn parse(input: &str) -> Result<AstNode, BoxedError> {
    let mut stream = TokenStream::from_str(input)?;

    let ast = parse_expression(&mut stream);

    if ast.is_ok() && !stream.end_of_stream() {
        return Err(ParseError {
            message: "Unexpected tokens at the end of input".to_string(),
            at_token: stream.peek().cloned(),
        }
        .into());
    }

    match ast {
        Ok(node) => Ok(node),
        Err(err) => Err(err.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::AstNode;

    #[test]
    fn test_parse_expression_long() {
        let input = "3 + 4 * 2 - (1 + 5) ^ 2 ^ 3";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            AstNode::BinaryOp {
                op: BinaryOp::Sub,
                lhs: Box::new(AstNode::BinaryOp {
                    op: BinaryOp::Add,
                    lhs: Box::new(AstNode::Number(RealScalar::from_str("3").unwrap())),
                    rhs: Box::new(AstNode::BinaryOp {
                        op: BinaryOp::Mul,
                        lhs: Box::new(AstNode::Number(RealScalar::from_str("4").unwrap())),
                        rhs: Box::new(AstNode::Number(RealScalar::from_str("2").unwrap()))
                    })
                }),
                rhs: Box::new(AstNode::BinaryOp {
                    op: BinaryOp::Pow,
                    lhs: Box::new(AstNode::BinaryOp {
                        op: BinaryOp::Add,
                        lhs: Box::new(AstNode::Number(RealScalar::from_str("1").unwrap())),
                        rhs: Box::new(AstNode::Number(RealScalar::from_str("5").unwrap()))
                    }),
                    rhs: Box::new(AstNode::BinaryOp {
                        op: BinaryOp::Pow,
                        lhs: Box::new(AstNode::Number(RealScalar::from_str("2").unwrap())),
                        rhs: Box::new(AstNode::Number(RealScalar::from_str("3").unwrap()))
                    })
                })
            }
        );
    }

    #[test]
    fn test_parse_expression_short() {
        let input = "3+ 5 -  2";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            AstNode::BinaryOp {
                op: BinaryOp::Sub,
                lhs: Box::new(AstNode::BinaryOp {
                    op: BinaryOp::Add,
                    lhs: Box::new(AstNode::Number(RealScalar::from_str("3").unwrap())),
                    rhs: Box::new(AstNode::Number(RealScalar::from_str("5").unwrap()))
                }),
                rhs: Box::new(AstNode::Number(RealScalar::from_str("2").unwrap()))
            }
        );
    }

    #[test]
    fn test_parse_expression_with_float() {
        let input = "3.14 + 2.71";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            AstNode::BinaryOp {
                op: BinaryOp::Add,
                lhs: Box::new(AstNode::Number(RealScalar::from_str("3.14").unwrap())),
                rhs: Box::new(AstNode::Number(RealScalar::from_str("2.71").unwrap()))
            }
        );
    }

    #[test]
    fn test_parse_expression_with_invalid_input() {
        let input = "3 + * 5";
        let ast = parse(input);
        assert!(ast.is_err(), "Expected parse to fail with invalid input");
    }

    #[test]
    fn test_parse_expression_with_products() {
        let input = "3 + 5 * 2";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            AstNode::BinaryOp {
                op: BinaryOp::Add,
                lhs: Box::new(AstNode::Number(RealScalar::from_str("3").unwrap())),
                rhs: Box::new(AstNode::BinaryOp {
                    op: BinaryOp::Mul,
                    lhs: Box::new(AstNode::Number(RealScalar::from_str("5").unwrap())),
                    rhs: Box::new(AstNode::Number(RealScalar::from_str("2").unwrap()))
                })
            }
        );
    }

    #[test]
    fn test_parse_expression_with_powers() {
        let input = "7 * 2^3 + 4";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            AstNode::BinaryOp {
                op: BinaryOp::Add,
                lhs: Box::new(AstNode::BinaryOp {
                    op: BinaryOp::Mul,
                    lhs: Box::new(AstNode::Number(RealScalar::from_str("7").unwrap())),
                    rhs: Box::new(AstNode::BinaryOp {
                        op: BinaryOp::Pow,
                        lhs: Box::new(AstNode::Number(RealScalar::from_str("2").unwrap())),
                        rhs: Box::new(AstNode::Number(RealScalar::from_str("3").unwrap()))
                    })
                }),
                rhs: Box::new(AstNode::Number(RealScalar::from_str("4").unwrap()))
            }
        );
    }

    #[test]
    fn test_parse_expression_with_parentheses() {
        let input = "(3 + 5) * 2";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            AstNode::BinaryOp {
                op: BinaryOp::Mul,
                lhs: Box::new(AstNode::BinaryOp {
                    op: BinaryOp::Add,
                    lhs: Box::new(AstNode::Number(RealScalar::from_str("3").unwrap())),
                    rhs: Box::new(AstNode::Number(RealScalar::from_str("5").unwrap()))
                }),
                rhs: Box::new(AstNode::Number(RealScalar::from_str("2").unwrap()))
            }
        );
    }

    #[test]
    fn test_parse_expression_with_nested_parentheses() {
        let input = "3 + (2 * (5 - 1))^2";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            AstNode::BinaryOp {
                op: BinaryOp::Add,
                lhs: Box::new(AstNode::Number(RealScalar::from_str("3").unwrap())),
                rhs: Box::new(AstNode::BinaryOp {
                    op: BinaryOp::Pow,
                    lhs: Box::new(AstNode::BinaryOp {
                        op: BinaryOp::Mul,
                        lhs: Box::new(AstNode::Number(RealScalar::from_str("2").unwrap())),
                        rhs: Box::new(AstNode::BinaryOp {
                            op: BinaryOp::Sub,
                            lhs: Box::new(AstNode::Number(RealScalar::from_str("5").unwrap())),
                            rhs: Box::new(AstNode::Number(RealScalar::from_str("1").unwrap()))
                        })
                    }),
                    rhs: Box::new(AstNode::Number(RealScalar::from_str("2").unwrap()))
                })
            }
        );
    }

    #[test]
    fn test_parse_expression_with_function_call() {
        let input = "5*pi^2/4*cos(pi*x/2)*sin(pi*y/2)";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            AstNode::BinaryOp {
                op: BinaryOp::Mul,
                lhs: Box::new(AstNode::BinaryOp {
                    op: BinaryOp::Mul,
                    lhs: Box::new(AstNode::BinaryOp {
                        op: BinaryOp::Div,
                        lhs: Box::new(AstNode::BinaryOp {
                            op: BinaryOp::Mul,
                            lhs: Box::new(AstNode::Number(RealScalar::from_str("5").unwrap())),
                            rhs: Box::new(AstNode::BinaryOp {
                                op: BinaryOp::Pow,
                                lhs: Box::new(AstNode::NamedValue("pi".to_string())),
                                rhs: Box::new(AstNode::Number(RealScalar::from_str("2").unwrap()))
                            })
                        }),
                        rhs: Box::new(AstNode::Number(RealScalar::from_str("4").unwrap()))
                    }),
                    rhs: Box::new(AstNode::FunctionCall {
                        name: "cos".to_string(),
                        args: vec![AstNode::BinaryOp {
                            op: BinaryOp::Div,
                            lhs: Box::new(AstNode::BinaryOp {
                                op: BinaryOp::Mul,
                                lhs: Box::new(AstNode::NamedValue("pi".to_string())),
                                rhs: Box::new(AstNode::NamedValue("x".to_string()))
                            }),
                            rhs: Box::new(AstNode::Number(RealScalar::from_str("2").unwrap()))
                        }]
                    })
                }),
                rhs: Box::new(AstNode::FunctionCall {
                    name: "sin".to_string(),
                    args: vec![AstNode::BinaryOp {
                        op: BinaryOp::Div,
                        lhs: Box::new(AstNode::BinaryOp {
                            op: BinaryOp::Mul,
                            lhs: Box::new(AstNode::NamedValue("pi".to_string())),
                            rhs: Box::new(AstNode::NamedValue("y".to_string()))
                        }),
                        rhs: Box::new(AstNode::Number(RealScalar::from_str("2").unwrap()))
                    }]
                })
            }
        );
    }
}
