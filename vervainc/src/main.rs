use vervainc::ast::lexer;

fn main() {
    let code = r#"
        set x = 10
        if x >= 5
            print("hello")
        end
    "#;

    for tok in lexer::lex(&code).iter() {
        println!("{:?}", tok);
    }
}