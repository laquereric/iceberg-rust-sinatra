use rutie::{AnyObject, Class, Object, RString, VM};
use iceberg_rust::catalog::Catalog;
use std::collections::HashMap;
use anyhow::Result;
use tokio::runtime::Runtime;

#[derive(Debug)]
struct IcebergCatalog {
    catalog: Catalog,
}

impl IcebergCatalog {
    fn new(config: HashMap<String, String>) -> Result<Self> {
        let catalog = Catalog::new(&config)?;
        Ok(IcebergCatalog { catalog })
    }

    fn create_namespace(&self, namespace: &str) -> Result<()> {
        self.catalog.create_namespace(namespace, None)?;
        Ok(())
    }
}

rutie::class!(RubyCatalog);

methods!(
    RubyCatalog,
    rtself,
    fn ruby_catalog_new(config: AnyObject) -> AnyObject {
        let config_hash: HashMap<String, String> = config.try_into().unwrap();
        let catalog = IcebergCatalog::new(config_hash).unwrap();
        Class::from_existing("RubyCatalog").wrap_data(catalog, &*RACK_CATALOG)
    },
    fn ruby_catalog_create_namespace(namespace: RString) -> RString {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let catalog = rtself.get_data(&*RACK_CATALOG);
            catalog.create_namespace(&namespace.to_string()).unwrap();
        });
        namespace
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_iceberg_rust_sinatra() {
    let mut class = Class::new("RubyCatalog", None);
    class.def_self("new", ruby_catalog_new);
    class.def("create_namespace", ruby_catalog_create_namespace);
}
