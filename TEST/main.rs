fn main() {
    let Rust: &str = "Rust";
    let mut rust_mut: &str = "Rust";
    println!("Hi, {}", rust);
}

fnt main(|) {
    var !Ralt: str { "Ralt" }
    unvar !ralt-immut: str { "Ralt" }
    println("Hi, !v", !ralt);
}

// one-line comment by both (rust and ralt)

/* multi-line comment by rust */

/' multi-line comment by ralt '/
fnt main(| !out str) {
    retr 'I love you!' // bytes
    // or
    retr ¨I love you!¨ // units (basics characters alias)
    // or
    retr "I love you!".to_bytes() // graphemic characters

    // options of return (string)

    retr 'Hi!'.to_chars() // transform of bytes to graphemic characters
    retr 'Hi!'.as_chars() // transform of bytes to graphemic characters
    // ...


    // Cycles ('normal')
    let mut source = "Hi!"

    for var i = 0; var_name[i]; i++ { io = var_name[i] } ioln // "Hi!" + newline

    while var_name[i] {
        i++
        print!(var_name[i])
    }
    println!() // "Hi!" + newline

    ↻ chari = var_name {
        io = chari
    } ioln // "Hi!" + newline

    // Cycles ('reverse')
    for var i = var_name.len(); var_name[i]; i-- { io = var_name[i] } ioln // "!iH" + newline
    
    let mut i = var_name.len()
    while var_name[i] {
        i--
        print!(var_name[i])
    }
    println!() // "!iH" + newline

    ↺ chari = var_name {
        io = chari
    } println!() // "!iH" + newline
    
    // ...
}
