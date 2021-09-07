extern crate lalrpop;

fn main() {
    println!("cargo:rerun-if-changed=src/parsing/jodin_grammar.lalrpop");
    println!("cargo:rerun-if-changed=src/parsing/jodin_grammar.rs");
    // lalrpop::process_root().unwrap()
    lalrpop::Configuration::new()
        // .generate_in_source_tree()
        .always_use_colors()
        // .emit_report(true)
        .process_current_dir()
        .unwrap()
}
