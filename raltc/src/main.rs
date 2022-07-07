use std::fs::File;
use std::io::Write;

use raltc_script::script::Script;
use raltc_cleaner::cleaner::cleaner;
use raltc_tokenizer::tokenizer::tokenizer;

fn main() {
    let mut script = Script::new_script("../Testing/test.rt");
    script.scan();

    cleaner(&mut script);
    tokenizer(&mut script);

    let mut rust_file = File::create("../Testing/testing/src/main.rs").unwrap();
    let mut rust_codegen: String = String::from("");
    
    while script.contains() {
        rust_codegen.push_str(script.get().value.as_str());
        rust_codegen.push(' ');
    }

    rust_file.write_all(rust_codegen.as_bytes()).unwrap();
    println!("Ready");
}
