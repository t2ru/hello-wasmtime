use wasmtime::Val;

fn main() {
    let hello_wat = std::fs::read("./src/hello.wat").expect("no file");
    let hello_wasm = wat::parse_bytes(hello_wat.as_ref()).expect("wat parse error");

    let store = wasmtime::Store::default();
    let module = wasmtime::Module::new(&store, &hello_wasm).unwrap();
    let instance = wasmtime::Instance::new(&module, &[]).expect("wasm instance");

    let add = instance
        .get_export("add")
        .and_then(|sym| sym.func())
        .unwrap();
    let result = add.call(&[Val::I32(2), Val::I32(3)]).unwrap();
    println!("Result: {:?}", result);
}
