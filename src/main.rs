mod lex;
mod ast;
mod error;

fn main() {
  let str = "12+2*(3+3)";
    let p = lex::Parser{
        source: str.to_string(),
        ch: '1',
        offset: 0,
        error: Default::default(),
    };
   let result=p.parse();
    println!("{:?}",result)
}
