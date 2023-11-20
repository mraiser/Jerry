use ndata::dataobject::*;
use ndata::dataarray::DataArray;
use flowlang::flowlang::system::system_call::system_call;

pub fn execute(_o: DataObject) -> DataObject {
let ax = start();
let mut o = DataObject::new();
o.put_string("a", &ax);
o
}

pub fn start() -> String {
let mut o = DataArray::new();
o.push_string("kgx");
o.push_string("-e");
o.push_string("runtime/jerry/script/launch_comfyui.sh");
let _x = system_call(o);

"OK".to_string()
}

