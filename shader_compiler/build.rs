// build.rs
// This is a build script that Cargo will execute before compiling our crate.
// Its job is to find our .lalrpop grammar file and generate the corresponding
// Rust parser code from it.

fn main() {
    // This tells Cargo to run lalrpop on our grammar file.
    // The generated Rust code will be placed in the target directory
    // and included by our main.rs.
    lalrpop::process_root().unwrap();
}
