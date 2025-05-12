use crate::ast::{AstNode, NumberAst, get_func};
use crate::error::error::CustomError;
use std::rc::Rc;

pub fn exec(ast_node: &AstNode) -> Result<f64, CustomError> {
    match ast_node {
        AstNode::Number(num) => Ok(num.val),
        AstNode::Binary(binary) => {
            let left = binary.left.as_ref();
            let right = binary.right.as_ref();
            match binary.op.as_str() {
                "+" => Ok(exec(left)? + exec(right)?),
                "-" => Ok(exec(left)? - exec(right)?),
                "*" => Ok(exec(left)? * exec(right)?),
                "/" => Ok(exec(left)? / exec(right)?),
                "%" => Ok(exec(left)? % exec(right)?),
                _ => Err(CustomError::InvalidOperator(binary.op.to_string())),
            }
        }
        AstNode::FunCaller(fun) => {
            let func = get_func(&fun.name);
            match func {
                Some(func) => {
                    let mut args: Vec<f64> = vec![];
                    fun.arg.as_slice().as_ref().iter().try_for_each(
                        |arg| -> Result<(), CustomError> {
                            let t = arg.as_ref();
                            match t {
                                AstNode::Number(num) => {
                                    args.push(num.val);
                                    Ok(())
                                }
                                AstNode::Binary(binary) => {
                                    let tmp = exec(arg)?;
                                    args.push(tmp);
                                    Ok(())
                                }

                                AstNode::FunCaller(fun) => {
                                    let tmp = exec(arg)?;
                                    args.push(tmp);
                                    Ok(())
                                }
                            }
                        },
                    )?;
                    let val = (func.func)(args.as_slice());
                    Ok(val)
                }
                None => Ok(0.0),
            }
        }
        _ => Ok(0.0),
    }
}
