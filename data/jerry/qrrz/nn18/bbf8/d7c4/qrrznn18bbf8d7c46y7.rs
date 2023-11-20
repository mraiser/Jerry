let mut out = DataObject::new();

let mut g = flowlang::datastore::DataStore::globals();
if !g.has("jerryservice") { g.put_object("jerryservice", DataObject::new()); }
let mut g = g.get_object("jerryservice");
if !g.has(&service) { 
  let mut s = DataObject::new();
  s.put_boolean("running", false);
  g.put_object(&service, s); 
}
let mut g = g.get_object(&service);

if value == "OFF" {
  g.put_boolean("running", false);

  if g.has("pid") {
    let id = g.get_int("pid");
    kill_my_process(id.to_string());
  }
  
  let pidfile = DataStore::new().root.parent().unwrap().join("runtime").join("jerry").join("pid").join(&service);
  if pidfile.exists() { std::fs::remove_file(pidfile); }
}
else if value == "ON" {
  if !g.has("running") || !g.get_boolean("running") {
    g.put_boolean("running", true);
    
    thread::spawn(move || {
      let mut comfyout = DataBytes::new();
      g.put_bytes("out", comfyout.clone());
      let mut comfyerr = DataBytes::new();
      g.put_bytes("err", comfyerr.clone());

      let cmd = jerryroot()+"/launch_"+&service+".sh";
      let mut args = Vec::new();
      args.push(&cmd);
      let cmd = Command::new("bash")
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();
      
      if cmd.is_err() {
        let msg = "Unable to execute system call ".to_string()+&service;
        println!("{}", msg);
      }
      else {
        let mut cmd = cmd.unwrap();
        let id = cmd.id();
        println!("STARTING {} as process #{}", &service, id);
        
        {
          let piddir = DataStore::new().root.parent().unwrap().join("runtime").join("jerry").join("pid");
          if !piddir.exists() { std::fs::create_dir_all(&piddir); }
          let pidfile = piddir.join(&service);
          let _x = std::fs::write(&pidfile, &id.to_string().as_bytes()).unwrap();
        }
        
        g.put_int("pid", id as i64);
        {
          let mut g = g.clone();
          let mut stdout = cmd.stdout.take().unwrap();
          //let service = service.clone();
          thread::spawn(move || {
            let mut buf: [u8; 1024] = [0; 1024];

            while g.get_boolean("running") {
              let n = stdout.read(&mut buf).unwrap();
              if n == 0 {
		        break;
              }
              else {
                comfyout.write(&buf[0..n]);
                //println!("OUT {}: {}", &service, std::str::from_utf8(&buf[0..n]).unwrap());
                let n = comfyout.current_len();
                if n > 1024 {
                  let _x = comfyout.read(n-1024);
                }
              }
            }
          });
        }
        
        {
          let mut g = g.clone();
          let mut stderr = cmd.stderr.take().unwrap();
          //let service = service.clone();
          thread::spawn(move || {
            let mut buf: [u8; 1024] = [0; 1024];

            while g.get_boolean("running") {
              let n = stderr.read(&mut buf).unwrap();
              if n == 0 {
		        break;
              }
              else {
                comfyerr.write(&buf[0..n]);
                //println!("ERR {}: {}", &service, std::str::from_utf8(&buf[0..n]).unwrap());
                let n = comfyerr.current_len();
                if n > 1024 {
                  let _x = comfyerr.read(n-1024);
                }
              }
            }
          });
        }
        
        //let beat = Duration::from_millis(500);
        //while g.get_boolean("running") { thread::sleep(beat); }
        
        cmd.wait().unwrap();
        println!("EXIT {} / {}", &service, id);
        g.put_boolean("running", false);
        
        let pidfile = DataStore::new().root.parent().unwrap().join("runtime").join("jerry").join("pid").join(&service);
        if pidfile.exists() { std::fs::remove_file(pidfile); }
      }    
    });
  }
}
else if value == "STATUS" {
  out.put_boolean("running", g.get_boolean("running"));
}

out
}

    
fn kill_my_process(id:String) {
  let mut args = Vec::new();
  args.push("-P");
  args.push(&id);
  let mut cmd = Command::new("pgrep")
  .args(args)
  .stdout(Stdio::piped())
  .spawn().unwrap();

  let mut stdout = cmd.stdout.take().unwrap();
  let reader = BufReader::new(stdout);
  let lines = reader.lines();
  for line in lines {
    if let Ok(ip) = line {
      kill_my_process(ip);
    }
  }
  cmd.wait();

  let mut args = Vec::new();
  args.push(&id);
  let mut cmd = Command::new("kill")
  .args(args)
  .spawn().unwrap().wait();