extern crate lalrpop;

fn main() {
    println!("cargo:rerun-if-changed=src/parsing/jodin_grammar.lalrpop");
    println!("cargo:rerun-if-changed=src/parsing/jodin_grammar.rs");
    // lalrpop::process_root().unwrap()
    lalrpop::Configuration::new()
        .generate_in_source_tree()
        .process_current_dir()
        .unwrap()
}