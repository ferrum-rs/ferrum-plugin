extern crate ferrum_plugin;
extern crate void;

use void::Void;
use ferrum_plugin::{Extensible, Plugin, Pluggable};
use ferrum_plugin::typemap::{TypeMap, Key};

struct Struct {
    map: TypeMap
}

impl Extensible for Struct {
    fn extensions(&self) -> &TypeMap {
        &self.map
    }
    fn extensions_mut(&mut self) -> &mut TypeMap {
        &mut self.map
    }
}

impl Pluggable for Struct {}

#[derive(Clone, Debug)]
struct IntPlugin {
    field: i32
}

impl Key for IntPlugin { type Value = IntPlugin; }

impl Plugin<Struct> for IntPlugin {
    type Error = Void;

    fn eval(_: &mut Struct) -> Result<IntPlugin, Void> {
        Ok(IntPlugin { field: 7i32 })
    }
}

fn main() {
    let mut x = Struct { map: TypeMap::new() };
    println!("{:?}", x.get_ref::<IntPlugin>());
}
