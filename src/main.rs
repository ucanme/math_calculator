mod lex;
mod ast;
mod error;

fn main() {
  let str = "min(min(1,2)*(3+4),max(3,4))";
    let p = lex::Parser{
        source: str.to_string(),
        ch: 'm',
        offset: 0,
        error: Default::default(),
    };
   let result=p.parse().unwrap();
    let ast = ast::Ast::new_ast(&result);
    match ast {
        Ok(mut ast) => {
            let result = ast.parse_expression();
            match result {
                Ok(node) => {
                    println!("{:?}",node);
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
