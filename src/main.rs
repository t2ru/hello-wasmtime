use wasmtime::Val;

fn main() {
    let hello_wat = std::fs::read("./src/hello.wat").expect("no file");
    let hello_wasm = wat::parse_bytes(hello_wat.as_ref()).expect("wat parse error");

    let store = wasmtime::Store::default();
    let module = wasmtime::Module::new(&store, &hello_wasm).expect("make module");

    let exports = [wasmtime::Extern::Func(wasmtime::Func::wrap1(&store, print_number))];
    let instance = wasmtime::Instance::new(&module, &exports).expect("wasm instance");

    let add = instance
        .get_export("add")
        .and_then(|sym| sym.func())
        .unwrap();
    let result = add.call(&[Val::I32(2), Val::I32(3)]).expect("call add");
    println!("Result: {:?}", result);

    let add_print = instance.get_export("add_print").and_then(|sym| sym.func()).unwrap();
    let result = add_print.call(&[Val::I32(2), Val::I32(3)]).expect("call add_print");
    println!("Result: {:?}", result);
}

fn print_number(n: i32) {
    println!("numner = {}", n);
}
