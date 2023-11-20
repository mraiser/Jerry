var me = this; 
var ME = $('#'+me.UUID)[0];

me.uiReady = function(ui){
  me.ui = ui;
  $(ME).find('.wrap').css('display', 'block');
};

me.ready = function(){
  send_list_services(function(result){
    var servicelist = [];
    for (var i in result.data) {
      servicelist.push(result.data[i]);
    }

    var d = {
      list: servicelist,
      title: "Services",
      itemlib: "jerry",
      itemctl: "service_toggle"
    };
    installControl("#slgh", "app", "list", function(api){
      function xxx(){
        poll();
        setTimeout(xxx, 500);
      }

      setTimeout(xxx, 500);
    }, d);
  });
};

function poll(){
  $(ME).find('.servicetoggle:checked').closest('li').each(function(x,y){
    $(y).find('.item_cell')[0].api.fetch_log();
  });
}