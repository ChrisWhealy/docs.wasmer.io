// Import the Filesystem so we can read our .wasm file
use std::fs::File;
use std::io::prelude::*;

// Import the wasmer runtime so we can use it
use wasmer_runtime::{
    error,
    // Include the function macro
    func,
    imports,
    instantiate,
    // Include the Context for our WASM Instance for passing imported host functions
    Ctx,
    Func,
};

const WASM_FILE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/target/wasm32-unknown-unknown/release/early_exit_guest.wasm"
);

// Our entry point to our application
fn main() -> error::Result<()> {
    // Let's read in our .wasm file as bytes

    // Let's open the file.
    let mut file = File::open(WASM_FILE_PATH).expect(&format!("WASM file at {}", WASM_FILE_PATH));

    // Let's read the file into a Vec
    let mut wasm_vec = Vec::new();
    file.read_to_end(&mut wasm_vec)
        .expect("Error reading the WASM file");

    // Now that we have the WASM file as bytes, let's run it with the wasmer runtime

    // Let's define the import object used to import our function
    // into our webassembly sample application.
    //
    // Make sure to check your function signature (parameter and return types) carefully!
    let import_object = imports! {
        // Define the "env" namespace that was implicitly used
        // by our example rust WASM crate.
        "env" => {
            // Key should be the name of the imported function
            // Value should be the func! macro, with the function passed in.
            "interrupt_execution" => func!(interrupt_execution),
        },
    };

    // Let's create an instance of WASM module running in the wasmer-runtime
    let instance = instantiate(&wasm_vec, &import_object)?;

    // Let's call the exported "exit_early" function on the WASM module.
    let exit_early_func: Func<(), i32> = instance
        .func("exit_early")
        .expect("exit_early function not found");
    let response = exit_early_func.call();

    match response {
        Ok(value) => {
            // This should have thrown an error, return an error
            panic!("exit_early did not error. Returned the value: {}", value);
        }
        Err(e) => {
            // Log the error
            println!("Error from exit_early: {}", e);
        }
    }

    // Log a success message.
    println!("Success!");

    // Return OK since everything executed successfully!
    Ok(())
}

// Function that is imported into the guest WASM module, that will immediately stop execution
fn interrupt_execution(_ctx: &mut Ctx) -> Result<(), ()> {
    // Log that we were called
    println!("interrupt_execution called!");

    // Return an error, which will immediately stop execution of the WASM module
    Err(())
}
