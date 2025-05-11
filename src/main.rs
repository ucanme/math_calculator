mod lex;
mod ast;
mod error;
mod exec;

fn main() {
  let str = "min(min(1,2)*(3+4),min(3,4))";
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
                    println!("ast: {:?}",node);
                    let val = exec::exec(&node);
                    println!("val: {:?}",val)
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
