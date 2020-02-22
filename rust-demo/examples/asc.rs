#[macro_use]
extern crate wasmer_runtime;
use std::error::Error;
use wasmer_as::{AsmScriptString, AsmScriptStringPtr};
use wasmer_runtime::{error, imports, instantiate, Ctx, Func};

// Our entry point to our application
fn main() -> error::Result<()> {
    // Let's get the .wasm file as bytes
    let wasm_bytes = include_bytes!(env!("WASM_PATH"));

    // Our import object, that allows exposing functions to our Wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {
        "env" => {
            "abort" => func!(abort),
        },
    };

    // Let's create an instance of Wasm module running in the wasmer-runtime
    let instance = instantiate(wasm_bytes, &import_object)?;

    // Let's get `add_one` as a function which takes one `u32` and returns one `u32`
    let add_one: Func<(u32, u32), u32> = instance.func("add")?;
    let result = add_one.call(50, 12)?;

    // Log the new value
    println!("Result: {}", result);

    // Asserting that the returned value from the function is our expected value.
    assert_eq!(result, 62);

    // Return OK since everything executed successfully!
    Ok(())
}

fn abort(
    ctx: &mut Ctx,
    message: AsmScriptStringPtr,
    filename: AsmScriptStringPtr,
    line: i32,
    col: i32,
) {
    let memory = ctx.memory(0);
    let message = message.get_as_string(memory).unwrap();
    let filename = filename.get_as_string(memory).unwrap();
    eprintln!("Error: {} at {}:{} col: {}", message, filename, line, col);
}
