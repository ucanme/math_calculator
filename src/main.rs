mod lex;
mod ast;
mod error;

fn main() {
  let str = "1+2*3+4+5";
    let p = lex::Parser{
        source: str.to_string(),
        ch: '1',
        offset: 0,
        error: Default::default(),
    };
   let result=p.parse().unwrap();
    let ast = ast::Ast::new_ast(&result);
    match ast {
        Ok(mut ast) => {
            let result = ast.parse_expression();
        },
        Err(e) => {
            println!("{:?}",e);
        }
    }
}
