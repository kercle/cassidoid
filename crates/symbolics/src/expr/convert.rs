use numbers::Number;
use parser::ast::{DIV_HEAD, ParserAst, SUB_HEAD};

use crate::{
    atom::Atom,
    expr::Expr,
    parser::ast::{
        ADD_HEAD, EQ_HEAD, GE_HEAD, GT_HEAD, LE_HEAD, LT_HEAD, MUL_HEAD, POW_HEAD,
        ParserAst as ParserAstOld,
    },
};

impl<A, T: Into<Atom>> From<T> for Expr<A>
where
    A: Default,
{
    fn from(x: T) -> Self {
        Expr::new_atom(x.into())
    }
}

impl<A: Default> From<ParserAst> for Expr<A> {
    fn from(ast: ParserAst) -> Self {
        use ParserAst::*;
        match ast {
            Constant { value } => Self::new_number(value),
            Symbol { name } => Self::new_symbol(name),
            LesserThan { lhs, rhs } => {
                Self::new_binary_node(LT_HEAD, Self::from(*lhs), Self::from(*rhs))
            }
            LesserEq { lhs, rhs } => {
                Self::new_binary_node(LE_HEAD, Self::from(*lhs), Self::from(*rhs))
            }
            Equals { lhs, rhs } => {
                Self::new_binary_node(EQ_HEAD, Self::from(*lhs), Self::from(*rhs))
            }
            GreaterEq { lhs, rhs } => {
                Self::new_binary_node(GE_HEAD, Self::from(*lhs), Self::from(*rhs))
            }
            GreaterThan { lhs, rhs } => {
                Self::new_binary_node(GT_HEAD, Self::from(*lhs), Self::from(*rhs))
            }
            Add { lhs, rhs } => Self::new_binary_node(ADD_HEAD, Self::from(*lhs), Self::from(*rhs)),
            Sub { lhs, rhs } => Self::new_binary_node(SUB_HEAD, Self::from(*lhs), Self::from(*rhs)),
            Negation { arg } => Self::new_unary_node(SUB_HEAD, Self::from(*arg)),
            Mul { lhs, rhs } => Self::new_binary_node(MUL_HEAD, Self::from(*lhs), Self::from(*rhs)),
            Div { lhs, rhs } => Self::new_binary_node(DIV_HEAD, Self::from(*lhs), Self::from(*rhs)),
            Pow { lhs, rhs } => Self::new_binary_node(POW_HEAD, Self::from(*lhs), Self::from(*rhs)),
            FunctionCall { name, args } => {
                let head = Self::new_symbol(name);
                let args = args.into_iter().map(|node| Self::from(node)).collect();

                Self::new_node(head, args)
            }
            Block { .. } => todo!(),
        }
    }
}

impl<A> Expr<A>
where
    A: Default + Clone + PartialEq,
{
    pub fn from_i64(value: i64) -> Self {
        Self::new_number(Number::from_i64(value))
    }

    pub fn from_parser_ast(parser_ast: ParserAstOld<A>) -> Self {
        match parser_ast {
            ParserAstOld::Constant { value, annotation } => {
                Self::new_number(value).with_annotation(annotation)
            }
            ParserAstOld::Symbol { name, annotation } => {
                Self::new_symbol(name).with_annotation(annotation)
            }
            ParserAstOld::Add { nodes, annotation } => {
                let head = Self::new_symbol(ADD_HEAD);
                let args = nodes
                    .into_iter()
                    .map(|node| Self::from_parser_ast(node))
                    .collect();
                Self::new_node(head, args).with_annotation(annotation.clone())
            }
            ParserAstOld::Sub {
                lhs,
                rhs,
                annotation,
            } => {
                let head = Self::new_symbol(ADD_HEAD);
                let lhs = Self::from_parser_ast(*lhs);
                let rhs = Self::from_parser_ast(*rhs);

                Self::new_node(
                    head,
                    vec![
                        lhs,
                        Self::new_node(
                            Self::new_symbol(MUL_HEAD),
                            vec![Self::new_number(Number::from_i64(-1)), rhs],
                        ),
                    ],
                )
                .with_annotation(annotation.clone())
            }
            ParserAstOld::Negation { arg, annotation } => {
                let arg = Self::from_parser_ast(*arg);
                Self::new_node(
                    Self::new_symbol(MUL_HEAD),
                    vec![Self::new_number(Number::from_i64(-1)), arg],
                )
                .with_annotation(annotation.clone())
            }
            ParserAstOld::Mul { nodes, annotation } => {
                let head = Self::new_symbol(MUL_HEAD);
                let args = nodes
                    .into_iter()
                    .map(|node| Self::from_parser_ast(node))
                    .collect();
                Self::new_node(head, args).with_annotation(annotation.clone())
            }
            ParserAstOld::Div {
                lhs,
                rhs,
                annotation,
            } => {
                let head = Self::new_symbol(MUL_HEAD);
                let lhs = Self::from_parser_ast(*lhs);
                let rhs = Self::from_parser_ast(*rhs);

                Self::new_node(
                    head,
                    vec![
                        lhs,
                        Self::new_node(
                            Self::new_symbol(POW_HEAD),
                            vec![rhs, Self::new_number(Number::from_i64(-1))],
                        ),
                    ],
                )
                .with_annotation(annotation.clone())
            }
            ParserAstOld::Pow {
                lhs,
                rhs,
                annotation,
            } => {
                let head = Self::new_symbol(POW_HEAD);
                let lhs = Self::from_parser_ast(*lhs);
                let rhs = Self::from_parser_ast(*rhs);

                Self::new_node(head, vec![lhs, rhs]).with_annotation(annotation.clone())
            }
            ParserAstOld::FunctionCall {
                name,
                args,
                annotation,
            } => {
                let head = Self::new_symbol(name);
                let args = args
                    .into_iter()
                    .map(|node| Self::from_parser_ast(node))
                    .collect();

                Self::new_node(head, args).with_annotation(annotation.clone())
            }
            ParserAstOld::Block { .. } => todo!(),
        }
    }
}
