use vervainc::ast::lexer;

fn main() {
    let code = r#"
        import io {
            print
        }

        @private func print_hello() {
            print("Hello, world!")
        }

        print_hello()
    "#;

    for tok in lexer::lex(&code) {
        println!("{:?}", tok);
    }
}