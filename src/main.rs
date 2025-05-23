mod lex;
mod ast;
mod error;
mod exec;

fn main() {
    let str = "123+89-0.9";
    println!("expression : {} ,", str);
    let p = lex::Parser::new(str.to_string()).unwrap();
    let result=p.parse().unwrap();
    let ast = ast::Ast::new(result.as_slice());
    match ast {
        Ok(mut ast) => {
            println!("ast tree: {:?}",ast);
            let result = ast.parse_expression();
            match result {
                Ok(node) => {
                    let val = exec::exec(&node);
                    let formatted_num = format!("{:?}", val.unwrap());
                    let trimmed = formatted_num.trim_end_matches('0');
                    // 如果小数点后全是零，也去掉小数点
                    let trimmed = trimmed.trim_end_matches('.');
                    println!("expression : {} ,", str);
                    println!("exec result: {}",trimmed);

                },
                Err(e) => {
                    println!("{:?}",e);
                }
            }
        },
        Err(e) => {
            println!("{:?}",e);
        }
    }
}
