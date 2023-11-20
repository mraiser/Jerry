var me = this; 
var ME = $('#'+me.UUID)[0];

me.ready = function(){
  $(ME).find('.servicename').text(ME.DATA.item.name);

  let spanStatus = $(ME).find(".toggle-status")[0];
  let servicetoggle = $(ME).find('.servicetoggle');
  let blaunch = $(ME).find('.blaunch');
//  let getlog = $(ME).find('.getlog');
  let outlog = $(ME).find('.outlog');
  
  servicetoggle.change(function(){
    var b = this.checked;
    let nuval = b ? "ON" : "OFF";
    spanStatus.textContent = nuval;
    blaunch.css('display', b ? 'inline-block' : 'none');
//    getlog.css('display', b ? 'inline-block' : 'none');
    outlog.css('display', b ? 'block' : 'none');
    send_toggle(ME.DATA.item.id, nuval, function(result){
      console.log(JSON.stringify(result));
    });
  });

  send_toggle(ME.DATA.item.id, "STATUS", function(result){
    var b = result.data.running;
    let nuval = b ? "ON" : "OFF";
    spanStatus.textContent = nuval;
    servicetoggle.prop("checked", b);
    blaunch.css('display', b ? 'inline-block' : 'none').click(function(){
      window.open(ME.DATA.item.url, '_blank');
    });
//    getlog.css('display', b ? 'inline-block' : 'none').click(me.fetch_log);
    outlog.css('display', b ? 'block' : 'none');
  });
  
  me.fetch_log = function(){
    send_read_log(ME.DATA.item.id, function(result){
      var out = $('<pre class="stdout"/>');
      out[0].innerHTML = result.data.out;
      outlog.append(out);
      var err = $('<pre class="stderr"/>');
      err[0].innerHTML = result.data.err;
      outlog.append(err);
      outlog.scrollTop(outlog.get(0).scrollHeight);
    });
  }
};

                