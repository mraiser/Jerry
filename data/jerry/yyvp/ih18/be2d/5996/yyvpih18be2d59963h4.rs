let mut o = DataObject::new();

let mut g = flowlang::datastore::DataStore::globals();
if !g.has("jerryservice") { g.put_object("jerryservice", DataObject::new()); }
let mut g = g.get_object("jerryservice");

let rootdir = jerryroot();
let paths = std::fs::read_dir(rootdir).unwrap();
for path in paths {
  let path = path.unwrap().path();
  let filename = path.file_name().unwrap().to_os_string().into_string().unwrap();
  if filename.starts_with("launch_"){
    let id = &path.file_stem().unwrap().to_os_string().into_string().unwrap()[7..];
    //let fullname = path.display().to_string();
    
    let mut d = get_service_metadata(id);
    d.put_string("id", id);
    
    if g.has(id) {
      let g = g.get_object(id);
      if g.has("running") {
        d.put_boolean("running", g.get_boolean("running"));
      }
    }
    
    if !d.has("name") { d.put_string("name", id); }
    if !d.has("url") { 
      if d.has("port"){
        let url = "http://localhost:".to_string() + &d.get_int("port").to_string();
        d.put_string("url", &url); 
      }
      else { d.put_string("url", "http://localhost:7860"); }
    }

    o.put_object(id, d);
  }
}


for (k,v) in g.objects(){
  
}


o
}

pub fn get_service_metadata(id:&str) -> DataObject {
  let f = DataStore::new().root.parent().unwrap().join("runtime").join("jerry").join("service").join(id.to_string()+".json");
  if f.exists() {
    let s = read_all_string(f.display().to_string());
    return DataObject::from_string(&s);
  }
  DataObject::new()
}

pub fn jerryroot() -> String {
  let mut g = flowlang::datastore::DataStore::globals();
  if g.has("jerryroot") {
    return g.get_string("jerryroot");
  }
  else {
    let j = g.get_object("system").get_object("apps"); 
    if j.has("jerry") && j.get_object("jerry").get_object("runtime").has("root"){ // FIXME - UGH
      return j.get_object("jerry").get_object("runtime").get_string("root");
    }
    else {
      let dir = DataStore::new().root.parent().unwrap().join("runtime").join("jerry").join("script");
      if !dir.exists() { let _x = std::fs::create_dir_all(&dir); }
      return dir.into_os_string().into_string().unwrap();
    }
  }