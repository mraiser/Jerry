let mut out = DataObject::new();

let mut g = flowlang::datastore::DataStore::globals();
if g.has("jerryservice") { 
  let mut g = g.get_object("jerryservice");
  if g.has(&service) { 
    let mut g = g.get_object(&service);
    
    if g.has("out") {
      let b = g.get_bytes("out");
      let n = b.current_len();
      let ba = b.read(n);
      let s = std::str::from_utf8(&ba).unwrap();
      out.put_string("out", &s);
    }
    
    if g.has("err") {
      let b = g.get_bytes("err");
      let n = b.current_len();
      let ba = b.read(n);
      let s = std::str::from_utf8(&ba).unwrap();
      out.put_string("err", &s);
    }
  }
}

out