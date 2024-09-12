use std::fs::File;
use std::io::Read;

mod ast;
mod parser;
mod semantic;
mod codegen;
mod target_code;

fn main() {
    // Read the source file
    let mut file = File::open("source.vyne").expect("Unable to open file");
    let mut source = String::new();
    file.read_to_string(&mut source).expect("Unable to read file");

    // Parse the source code
    let pairs = parser::VyneParser::parse(Rule::expression, &source)
        .expect("Failed to parse source code");

    let ast = generate_ast_from_pairs(pairs);

    // Perform semantic analysis
    semantic::check_semantics(&ast).expect("Semantic error");

    // Generate intermediate code
    let intermediate_code = codegen::generate_intermediate_code(&ast);

    // Generate target code
    let target_code = target_code::generate_target_code(&intermediate_code);

    // Output or save the target code
    println!("{}", target_code);
}

fn generate_ast_from_pairs(pairs: pest::iterators::Pairs<Rule>) -> ast::Program {
    // Convert Pest pairs into your AST structure
    ast::Program { expressions: vec![] }
}
