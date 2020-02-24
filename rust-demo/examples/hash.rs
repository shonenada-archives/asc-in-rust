use wasmer_runtime::{error, imports, instantiate, Array, Func, WasmPtr};
use wasmer_wasi;
use wasmer_wasi::WasiVersion;

const WASI_VERSION: WasiVersion = WasiVersion::Snapshot0;

// Our entry point to our application
fn main() -> error::Result<()> {
    let state = wasmer_wasi::state::WasiState::new("Hash").build().unwrap();

    let wasm_bytes = include_bytes!("wasm/hash.wasm");

    let import_object = wasmer_wasi::generate_import_object_from_state(state, WASI_VERSION);

    let instance = instantiate(wasm_bytes, &import_object)?;

    // Lets get the context and memory of our Wasm Instance
    let wasm_instance_context = instance.context();
    let wasm_instance_memory = wasm_instance_context.memory(0);

    let get_ptr: Func<(), WasmPtr<u8, Array>> =
        instance.func("getMemoryPtr").expect("getMemoryPtr");
    let buffer_ptr = get_ptr.call().unwrap();

    let raw = "Hello World";

    let memory_writer = buffer_ptr
        .deref(wasm_instance_memory, 0, raw.len() as u32)
        .unwrap();
    for (i, b) in raw.bytes().enumerate() {
        memory_writer[i].set(b);
    }

    print!("String in memory: ");
    let print_memory: Func<u32, u32> = instance.func("printMemory").expect("printMemory");
    let _len = print_memory.call(raw.len() as u32).unwrap();

    let b64encode: Func<u32, u32> = instance.func("b64encode")?;
    let new_len = b64encode.call(raw.len() as u32)?;
    println!("called b64encode ");

    let new_buffer_ptr = get_ptr.call().unwrap();
    let encoded = new_buffer_ptr
        .get_utf8_string(wasm_instance_memory, new_len)
        .unwrap();

    println!("Base64 Encode of `{}` is: \"{}\"", raw, encoded);

    Ok(())
}
