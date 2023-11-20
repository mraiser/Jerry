let x = rand() as i64;

let body = "".to_string()+r#"{"client_id":"ff66f1d808bb48b384b2266a282be8f3","prompt":{"1":{"inputs":{"ckpt_name":"albedobaseXL_v11.safetensors"},"class_type":"CheckpointLoaderSimple"},"2":{"inputs":{"text":"a beautiful woman posing","clip":["1",1]},"class_type":"CLIPTextEncode"},"3":{"inputs":{"seed":1,"steps":20,"cfg":8,"sampler_name":"euler","scheduler":"normal","denoise":1,"model":["1",0],"positive":["2",0],"negative":["4",0],"latent_image":["5",0]},"class_type":"KSampler"},"4":{"inputs":{"text":"","clip":["1",1]},"class_type":"CLIPTextEncode"},"5":{"inputs":{"width":896,"height":1152,"batch_size":1},"class_type":"EmptyLatentImage"},"6":{"inputs":{"samples":["3",0],"vae":["1",2]},"class_type":"VAEDecode"},"7":{"inputs":{"filename_prefix":"ComfyUI","images":["6",0]},"class_type":"SaveImage"}},"extra_data":{"extra_pnginfo":{"workflow":{"last_node_id":7,"last_link_id":9,"nodes":[{"id":2,"type":"CLIPTextEncode","pos":[5,-316],"size":{"0":400,"1":200},"flags":{},"order":2,"mode":0,"inputs":[{"name":"clip","type":"CLIP","link":3}],"outputs":[{"name":"CONDITIONING","type":"CONDITIONING","links":[2],"shape":3,"slot_index":0}],"properties":{"Node name for S&R":"CLIPTextEncode"},"widgets_values":["a beautiful woman posing"]},{"id":4,"type":"CLIPTextEncode","pos":[9,-61],"size":{"0":400,"1":200},"flags":{},"order":3,"mode":0,"inputs":[{"name":"clip","type":"CLIP","link":5,"slot_index":0}],"outputs":[{"name":"CONDITIONING","type":"CONDITIONING","links":[4],"shape":3,"slot_index":0}],"properties":{"Node name for S&R":"CLIPTextEncode"},"widgets_values":[""]},{"id":5,"type":"EmptyLatentImage","pos":[91,197],"size":{"0":315,"1":106},"flags":{},"order":0,"mode":0,"outputs":[{"name":"LATENT","type":"LATENT","links":[6],"shape":3,"slot_index":0}],"properties":{"Node name for S&R":"EmptyLatentImage"},"widgets_values":[896,1152,1]},{"id":3,"type":"KSampler","pos":[563.9166746533606,-502.2036011751033],"size":{"0":315,"1":262},"flags":{},"order":4,"mode":0,"inputs":[{"name":"model","type":"MODEL","link":1},{"name":"positive","type":"CONDITIONING","link":2},{"name":"negative","type":"CONDITIONING","link":4},{"name":"latent_image","type":"LATENT","link":6}],"outputs":[{"name":"LATENT","type":"LATENT","links":[7],"shape":3,"slot_index":0}],"properties":{"Node name for S&R":"KSampler"},"widgets_values":[577928509232634,"randomize",20,8,"euler","normal",1]},{"id":1,"type":"CheckpointLoaderSimple","pos":[59,-496],"size":{"0":315,"1":98},"flags":{},"order":1,"mode":0,"outputs":[{"name":"MODEL","type":"MODEL","links":[1],"shape":3,"slot_index":0},{"name":"CLIP","type":"CLIP","links":[3,5],"shape":3,"slot_index":1},{"name":"VAE","type":"VAE","links":[8],"shape":3,"slot_index":2}],"properties":{"Node name for S&R":"CheckpointLoaderSimple"},"widgets_values":["albedobaseXL_v11.safetensors"]},{"id":6,"type":"VAEDecode","pos":[1043.255502570759,-488.9802720251163],"size":{"0":210,"1":46},"flags":{},"order":5,"mode":0,"inputs":[{"name":"samples","type":"LATENT","link":7},{"name":"vae","type":"VAE","link":8}],"outputs":[{"name":"IMAGE","type":"IMAGE","links":[9],"shape":3,"slot_index":0}],"properties":{"Node name for S&R":"VAEDecode"}},{"id":7,"type":"SaveImage","pos":[795,-148],"size":[390.4042779591457,483.8131267453027],"flags":{},"order":6,"mode":0,"inputs":[{"name":"images","type":"IMAGE","link":9}],"properties":{},"widgets_values":["ComfyUI"]}],"links":[[1,1,0,3,0,"MODEL"],[2,2,0,3,1,"CONDITIONING"],[3,1,1,2,0,"CLIP"],[4,4,0,3,2,"CONDITIONING"],[5,1,1,4,0,"CLIP"],[6,5,0,3,3,"LATENT"],[7,3,0,6,0,"LATENT"],[8,1,2,6,1,"VAE"],[9,6,0,7,0,"IMAGE"]],"groups":[],"config":{},"extra":{},"version":0.4,"seed_widgets":{"3":0}}}}}"#;

let o = DataObject::from_string(&body);
o.get_object("prompt").get_object("3").get_object("inputs").put_int("seed", x);
let body = o.to_string();

let resp = attohttpc::post("http://127.0.0.1:8188/prompt").text(body).send();
if resp.is_ok(){
  let o = DataObject::from_string(&resp.unwrap().text().unwrap());
  let id = o.get_string("prompt_id");

  let beat = Duration::from_millis(500);
  let mut i = 0;
  while i < 240 {
    thread::sleep(beat);
    
    let resp = attohttpc::get("http://127.0.0.1:8188/history/".to_string()+&id).send();
    if resp.is_ok(){
      let s = resp.unwrap().text().unwrap();
      let o = DataObject::from_string(&s);
      if o.has(&id) {
        let o = o.get_object(&id).get_object("outputs").get_object("7").get_array("images").get_object(0);
        return o;
      }
    }
    else {
      println!("ERROR {:?}", resp);
    }
    i += 1;
  }
  println!("DONE");
}
else {
  println!("ERROR {:?}", resp);
}

DataObject::new()