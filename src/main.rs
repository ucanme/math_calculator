mod lex;
mod ast;
mod error;
mod exec;

fn main() {
  let str = "min(min(1,2)*(3+4),min(3,4))+4.12";
    let p = lex::Parser{
        source: str.to_string(),
        ch: 'm',
        offset: 0,
        error: Default::default(),
    };
   let result=p.parse().unwrap();
    let ast = ast::Ast::new(result.as_slice());
    match ast {
        Ok(mut ast) => {
            let result = ast.parse_expression();
            match result {
                Ok(node) => {
                    let val = exec::exec(&node);
                    let formatted_num = format!("{:?}", val.unwrap());
                    let trimmed = formatted_num.trim_end_matches('0');
                    // 如果小数点后全是零，也去掉小数点
                    let trimmed = trimmed.trim_end_matches('.');
                    print!("expression : {} ,", str);
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
