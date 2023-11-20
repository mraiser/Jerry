let mut o = DataArray::new();
o.push_string("kgx");
o.push_string("-e");
o.push_string("runtime/jerry/script/launch_comfyui.sh");
let _x = system_call(o);

"OK".to_string()