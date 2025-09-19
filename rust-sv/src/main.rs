use std::collections::HashMap;
use std::path::PathBuf;
use sv_parser::parse_sv;

fn main() {

    let path = "../examples/sdram_controller.v";
    let defines = HashMap::new();
    let includes: Vec<PathBuf> = Vec::new();
    let (syntax_tree, _) = parse_sv(&path, &defines, &includes, false, false).expect("failed to parse verilog");
    println!("{syntax_tree}");

    // It looks like there might not be any way to modify the SyntaxTree.
    // Here is someone asking about this: https://github.com/dalance/sv-parser/issues/108

    // I also have now found a way to turn a SyntaxTree back into Verilog.

}
