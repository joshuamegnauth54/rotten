use gl_generator::{Api, DebugStructGenerator, Fallbacks, Profile, Registry, StructGenerator};
use std::{env, fs::File, path::Path};

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file_bindings = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    let registry = Registry::new(Api::Gl, (4, 6), Profile::Core, Fallbacks::All, []);

    // StructGenerator is preferred over the default GlobalGenerator because it's safer.
    // Instead of a global set of functions that may not be initialized, StructGenerator returns
    // a Gl struct with the functions once the procedures are loaded.
    // Source: https://github.com/brendanzab/gl-rs/tree/master/gl_generator
    if env::var("CARGO_FEATURE_DEBUG").is_ok() {
        registry
            .write_bindings(DebugStructGenerator, &mut file_bindings)
            .unwrap()
    } else {
        registry
            .write_bindings(StructGenerator, &mut file_bindings)
            .unwrap()
    }
}
