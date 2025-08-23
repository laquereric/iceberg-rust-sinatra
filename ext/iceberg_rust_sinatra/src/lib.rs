use rutie::{Class, Object, RString, VM};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_iceberg_rust_sinatra() {
    Class::new("IcebergRustSinatra", None).define(|klass| {
        klass.def_self("hello", hello);
    });
}

fn hello(_: &Class) -> RString {
    RString::new_utf8("Hello from Rust!")
}
