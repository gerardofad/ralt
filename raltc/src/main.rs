use raltc_codegen::codegen::codegen;

fn main() {
    codegen(
        "../TEST/main.rs",
        
        "../BUILD/main/src/main.rs"
    );

    println!("Ok");
}
