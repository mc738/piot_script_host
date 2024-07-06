use std::error::Error;
use wasmtime::{Engine, Module, Store, Instance};

fn hello_world() -> Result<(), Box<dyn Error>> {
    // https://docs.wasmtime.dev/lang-rust.html
    let engine = Engine::default();

    let module = Module::from_file(&engine, "hello.wat")?;

    let mut store = Store::new(&engine, ());

    let instance = Instance::new(&mut store, &module, &[])?;

    let answer = instance.get_func(&mut store, "answer")
        .expect("`answer` was not an exported function");

    let answer = answer.typed::<(), i32>(&store)?;

    let result = answer.call(&mut store, ());

    println!("Answer: {:?}", result);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    hello_world()
}