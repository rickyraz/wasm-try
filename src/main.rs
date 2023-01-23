use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    // create a wasmtime engine
    let engine = Engine::default();
    // load the module
    let module = Module::from_file(&engine, "hello.wat")?;
    // create a store and module instance
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    // locate the exported function
    let exported_run = instance.get_typed_func::<(), i32>(&mut store, "run")?;
    // call and print the result
    let res = exported_run.call(&mut store, ())?;
    println!("WebAssembly says - {}", res);
    Ok(())
}
