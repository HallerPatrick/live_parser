// An expression can be everything which yiealds a "result"
// E.g:
// 1 + 1 -> Arithemtic
// 1 * 2 + 1
// foo.bar() -> Method calls
// baz()     -> Function Calls
// Class::foo() -> Function calls of classes (static methods)
//
// Parenthese show order of evaluation
// 1 * (1 + 2)
//
// For now expressions are only lexed
// where only literals are being parsed but not evaluated

// If we want to parse them most of those have to be parsed for:
// None
// Binary
// Grouping
// Unary
// And
// Or
//
//
// Dot
// This
// Super
//
pub mod binary;
pub mod call;

use crate::parser::{
    literals::{parse_literal, sp, Literal},
    parse_variable,
    tokens::{
        dot, left_bracket, left_paren, parse_binary_operator, parse_unary_operator, right_bracket,
        right_paren, Operator, UnOperator, BINOP_PRECEDENCE, UNOPS,
    },
    Res, Variable,
};

use crate::parser::expression::binary::{BinaryOp, UnaryOp};
use call::{parse_call, Call, MemberCall};

use nom::{
    branch::alt,
    combinator::map,
    error::context,
    multi::many0,
    sequence::{delimited, preceded, tuple},
};

// Represent a token or literal in a expression
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Call(Call),
    MemberCall(MemberCall),
    BinaryOp(Box<BinaryOp>),
    UnaryOp(Box<UnaryOp>),
    PrefixExpr(Box<PrefixExpr>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expression2 {
    Literal(Literal),
    PrefixExpr(PrefixExpr),
}

fn parse_flatexp(input: &str) -> Res<&str, FlatExpr> {
    context("FlatExpr", tuple((parse_head, parse_bin_op_chain)))(input)
        .map(|(next_input, res)| (next_input, flat_expr_from_components(res.0, res.1)))
}

pub fn parse_expression(input: &str) -> Res<&str, Expression> {
    map(parse_flatexp, Expression::from)(input)
}

pub fn parse_expression2(input: &str) -> Res<&str, Expression2> {
    context(
        "Expression2",
        preceded(
            sp,
            alt((
                // The order is important
                // TODO: Figure out if we need to parse literals, this is done by prefixexpr
                map(prefixexpr, Expression2::PrefixExpr),
                map(parse_literal, Expression2::Literal),
            )),
        ),
    )(input)
}

#[derive(Clone, PartialEq, Debug)]
struct ExprHead {
    un_ops: Vec<UnOperator>,
    expr: Expression2,
}

fn parse_head(input: &str) -> Res<&str, ExprHead> {
    context(
        "NestedExpression",
        preceded(
            sp,
            tuple((many0(parse_unary_operator), preceded(sp, parse_expression2))),
        ),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            ExprHead {
                un_ops: res.0,
                expr: res.1,
            },
        )
    })
}

fn parse_bin_op_chain(input: &str) -> Res<&str, Vec<(Operator, ExprHead)>> {
    context(
        "OpChain",
        preceded(
            sp,
            many0(tuple((parse_binary_operator, preceded(sp, parse_head)))),
        ),
    )(input)
}

#[derive(Clone, PartialEq, Debug)]
pub struct PrefixExpr {
    pub prefix: ExprOrVarname,
    pub suffix_chain: Vec<ExprSuffix>,
}

/// This parser deals with all kind of suffix expressions which are part
/// of a single expression
///
/// E.g. tbl.method()[idx](call, parameter)
pub fn prefixexpr(input: &str) -> Res<&str, PrefixExpr> {
    context(
        "PrefixExpr",
        preceded(sp, tuple((prefixexpr2, many0(prefixexpr3)))),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            PrefixExpr {
                prefix: res.0,
                suffix_chain: res.1,
            },
        )
    })
}

#[derive(Clone, PartialEq, Debug)]
pub enum ExprOrVarname {
    Exp(Expression),
    Varname(Variable),
}

struct FlatExpr(Vec<OpOrExp2>);

#[derive(Clone, Debug, PartialEq)]
pub enum OpOrExp2 {
    Op(UnOrBinOp),
    Exp2(Expression2),
}

#[derive(Debug)]
enum OpOrExp {
    Op(UnOrBinOp),
    Exp(Expression),
}

impl OpOrExp {
    fn is_binop(&self) -> bool {
        matches!(self, OpOrExp::Op(UnOrBinOp::BinOp(_)))
    }

    fn is_unop(&self) -> bool {
        matches!(self, OpOrExp::Op(UnOrBinOp::UnOp(_)))
    }

    fn is_exp(&self) -> bool {
        matches!(self, OpOrExp::Exp(_))
    }
}

impl From<FlatExpr> for Expression {
    fn from(fe: FlatExpr) -> Expression {
        // Helper function. Expects a,b to be Exps and o to be a BinOp
        fn merge_nodes_binop(a: OpOrExp, o: OpOrExp, b: OpOrExp) -> OpOrExp {
            match (a, o, b) {
                (OpOrExp::Exp(a), OpOrExp::Op(UnOrBinOp::BinOp(o)), OpOrExp::Exp(b)) => {
                    let merged_exp = Expression::BinaryOp(Box::new(BinaryOp {
                        left: a,
                        op: o,
                        right: b,
                    }));
                    OpOrExp::Exp(merged_exp)
                }
                _ => panic!("unexpected input variants in merge_nodes_binop"),
            }
        }

        // Helper function. Expects o to be a UnOp and a to be an Exp
        fn merge_nodes_unop(o: OpOrExp, a: OpOrExp) -> OpOrExp {
            match (o, a) {
                (OpOrExp::Op(UnOrBinOp::UnOp(o)), OpOrExp::Exp(a)) => {
                    let merged_exp = Expression::UnaryOp(Box::new(UnaryOp { op: o, operand: a }));
                    OpOrExp::Exp(merged_exp)
                }
                _ => panic!("unexpected input variants in merge_nodes_unop"),
            }
        }

        // TODO: make this more efficient
        fn merge_all_binops(explist: &mut Vec<OpOrExp>, binops: &[Operator]) {
            loop {
                let mut tojoin_idx: Option<usize> = None;
                for (i, oe) in explist
                    .iter()
                    .enumerate()
                    .filter(|&(_, ref oe)| oe.is_binop())
                {
                    match *oe {
                        OpOrExp::Op(UnOrBinOp::BinOp(ref o)) => {
                            // Found something to join
                            if binops.binary_search(o).is_ok() {
                                assert!(i > 0);
                                assert!(explist[i - 1].is_exp());
                                assert!(i.checked_add(1).is_some());
                                let next = explist.get(i + 1).unwrap();

                                // If UnOps haven't been merged yet, ignore them. If there are two
                                // subsequent binops, that's an error. Otherwise we have a $ b
                                // where a and b are Exps and $ is a BinOp
                                match *next {
                                    OpOrExp::Op(UnOrBinOp::UnOp(_)) => continue,
                                    OpOrExp::Op(UnOrBinOp::BinOp(_)) => {
                                        panic!("encountered two binops next to each other");
                                    }
                                    OpOrExp::Exp(_) => {
                                        tojoin_idx = Some(i);
                                        break;
                                    }
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                if let Some(i) = tojoin_idx {
                    let a = explist.remove(i - 1);
                    let o = explist.remove(i - 1);
                    let b = explist.remove(i - 1);
                    let merged = merge_nodes_binop(a, o, b);
                    explist.insert(i - 1, merged);
                }
                // Joined everything we could. Break
                else {
                    break;
                }
            }
        }

        fn merge_all_unops(explist: &mut Vec<OpOrExp>, unops: &[UnOperator]) {
            loop {
                let mut tojoin_idx: Option<usize> = None;
                // Reverse iterate, since we want to apply stacked unary operators right-to-left
                for (i, oe) in explist
                    .iter()
                    .enumerate()
                    .filter(|&(_, ref oe)| oe.is_unop())
                    .rev()
                {
                    match *oe {
                        OpOrExp::Op(UnOrBinOp::UnOp(ref o)) => {
                            // Found something to join
                            if unops.binary_search(o).is_ok() {
                                assert!(i.checked_add(1).is_some());
                                let next = explist.get(i + 1).unwrap();
                                assert!(next.is_exp());

                                tojoin_idx = Some(i);
                                break;
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                if let Some(i) = tojoin_idx {
                    let o = explist.remove(i);
                    let a = explist.remove(i);
                    let merged = merge_nodes_unop(o, a);
                    explist.insert(i, merged);
                }
                // Joined everything we could. Break
                else {
                    break;
                }
            }
        }

        // First apply the highest-precedent binop (^) where applicable. This will miss cases
        // like 10 ^ #"hi" == 100. So then apply all unops, then apply all binops, starting again
        // from the highest-precedent one.

        let mut explist: Vec<OpOrExp> =
            fe.0.into_iter()
                .map(|oe| match oe {
                    OpOrExp2::Op(o) => OpOrExp::Op(o),
                    OpOrExp2::Exp2(e) => OpOrExp::Exp(Expression::from(e)),
                })
                .collect();

        // First pass: find all triplets of the form a $ b where a and b are Exps and $ is a binop
        // of the highest precedence
        merge_all_binops(&mut explist, &*BINOP_PRECEDENCE[0]);
        merge_all_unops(&mut explist, &*UNOPS);

        for binops in BINOP_PRECEDENCE.iter() {
            merge_all_binops(&mut explist, &*binops);
        }

        assert_eq!(explist.len(), 1, "Exp tree construction didn't complete");
        match explist.pop().unwrap() {
            OpOrExp::Exp(e) => e,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnOrBinOp {
    UnOp(UnOperator),
    BinOp(Operator),
}

fn tuple_to_flat_vec(head: ExprHead) -> Vec<OpOrExp2> {
    let mut v: Vec<OpOrExp2> = head
        .un_ops
        .into_iter()
        .map(UnOrBinOp::UnOp)
        .map(OpOrExp2::Op)
        .collect();
    v.push(OpOrExp2::Exp2(head.expr));
    v
}

fn flat_expr_from_components(head: ExprHead, binop_chain: Vec<(Operator, ExprHead)>) -> FlatExpr {
    let acc = tuple_to_flat_vec(head);
    let res = binop_chain.into_iter().fold(acc, |mut a, head| {
        a.push(OpOrExp2::Op(UnOrBinOp::BinOp(head.0)));
        a.extend_from_slice(&*tuple_to_flat_vec(head.1));
        a
    });

    FlatExpr(res)
}

fn prefixexpr2(input: &str) -> Res<&str, ExprOrVarname> {
    preceded(
        sp,
        alt((
            map(
                delimited(left_paren, parse_expression, right_paren),
                ExprOrVarname::Exp,
            ),
            map(parse_variable, ExprOrVarname::Varname),
        )),
    )(input)
}

#[derive(Clone, PartialEq, Debug)]
pub enum ExprSuffix {
    TableDot(Variable),
    TableIdx(Expression),
    FuncCall(Call),
}

fn prefixexpr3(input: &str) -> Res<&str, ExprSuffix> {
    preceded(
        sp,
        alt((
            map(preceded(dot, parse_variable), ExprSuffix::TableDot),
            map(
                delimited(left_bracket, parse_expression, right_bracket),
                ExprSuffix::TableIdx,
            ),
            map(parse_call, ExprSuffix::FuncCall),
        )),
    )(input)
}

impl From<Expression2> for Expression {
    fn from(e: Expression2) -> Expression {
        match e {
            Expression2::Literal(l) => Expression::Literal(l),
            Expression2::PrefixExpr(p) => Expression::PrefixExpr(Box::new(p)),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::parser::Variable;

    #[test]
    fn test_parse_head() {
        let string = "-hello and test";
        let res = parse_head(string);
        println!("{:?}", res);

        assert_eq!(
            res,
            Ok((
                " and test",
                ExprHead {
                    un_ops: vec![UnOperator::Sub],
                    expr: Expression2::PrefixExpr(PrefixExpr {
                        prefix: ExprOrVarname::Varname(Variable {
                            name: String::from("hello")
                        }),
                        suffix_chain: vec![]
                    })
                }
            ))
        )
    }

    #[test]
    fn test_parse_bin_op_chain() {
        let string = "< hello() - 3";
        let res = parse_bin_op_chain(string);
        println!("{:?}", res);
    }

    #[test]
    fn test_expr_lexer6() {
        let string = "hello()";
        let res = parse_expression(string);
        assert_eq!(
            res,
            Ok((
                "",
                Expression::PrefixExpr(Box::new(PrefixExpr {
                    prefix: ExprOrVarname::Varname(Variable {
                        name: String::from("hello")
                    }),
                    suffix_chain: vec![ExprSuffix::FuncCall(Call {
                        args: vec![],
                        callee: None
                    })]
                }))
            ))
        );
    }

    #[test]
    fn test_expr_lexer7() {
        let string = "hello()";
        let res = prefixexpr(string);
        assert_eq!(
            res,
            Ok((
                "",
                PrefixExpr {
                    prefix: ExprOrVarname::Varname(Variable {
                        name: String::from("hello")
                    }),
                    suffix_chain: vec![ExprSuffix::FuncCall(Call {
                        args: vec![],
                        callee: None
                    })]
                }
            ))
        );
    }

    #[test]
    fn test_expr_lexer4() {
        let string = "3";
        let res = parse_expression(string);
        assert_eq!(res, Ok(("", Expression::Literal(Literal::Num(3.0)))))
    }

    #[test]
    fn test_expr_lexer3() {
        let string = "3 < x";
        let res = parse_expression(string);
        assert_eq!(
            res,
            Ok((
                "",
                Expression::BinaryOp(Box::new(BinaryOp {
                    op: Operator::Lt,
                    left: Expression::Literal(Literal::Num(3.0)),
                    right: Expression::PrefixExpr(Box::new(PrefixExpr {
                        prefix: ExprOrVarname::Varname(Variable {
                            name: String::from("x")
                        }),
                        suffix_chain: vec![]
                    }))
                }))
            ))
        )
    }

    #[test]
    fn test_expr_lexer5() {
        let string = "(3 < x) and Nil == Nil";
        let res = parse_expression(string);

        let left = Expression::PrefixExpr(Box::new(PrefixExpr {
            prefix: ExprOrVarname::Exp(Expression::BinaryOp(Box::new(BinaryOp {
                op: Operator::Lt,
                left: Expression::Literal(Literal::Num(3.0)),
                right: Expression::PrefixExpr(Box::new(PrefixExpr {
                    prefix: ExprOrVarname::Varname(Variable {
                        name: String::from("x"),
                    }),
                    suffix_chain: vec![],
                })),
            }))),
            suffix_chain: vec![],
        }));

        let right = Expression::BinaryOp(Box::new(BinaryOp {
            op: Operator::EQ,
            left: Expression::PrefixExpr(Box::new(PrefixExpr {
                prefix: ExprOrVarname::Varname(Variable {
                    name: String::from("Nil"),
                }),
                suffix_chain: vec![],
            })),
            right: Expression::PrefixExpr(Box::new(PrefixExpr {
                prefix: ExprOrVarname::Varname(Variable {
                    name: String::from("Nil"),
                }),
                suffix_chain: vec![],
            })),
        }));

        assert_eq!(
            res,
            Ok((
                "",
                Expression::BinaryOp(Box::new(BinaryOp {
                    left: left,
                    op: Operator::And,
                    right: right
                }))
            ))
        )
    }

    #[test]
    fn test_expr_lexer2() {
        let string = "(3 + 4)";
        let res = parse_expression(string);
        assert_eq!(
            res,
            Ok((
                "",
                Expression::PrefixExpr(Box::new(PrefixExpr {
                    prefix: ExprOrVarname::Exp(Expression::BinaryOp(Box::new(BinaryOp {
                        op: Operator::Add,
                        left: Expression::Literal(Literal::Num(3.0)),
                        right: Expression::Literal(Literal::Num(4.0))
                    }))),
                    suffix_chain: vec![]
                }))
            ))
        )
    }
}