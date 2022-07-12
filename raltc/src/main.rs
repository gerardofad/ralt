use raltc_mod_parser::parser::*;

fn main() {
    let mut moduler: Moduler = Moduler {
        main_module: Module::new(),
        modules:     vec![],
    };

    parser("../TEST/main.mod", &mut moduler);
}
