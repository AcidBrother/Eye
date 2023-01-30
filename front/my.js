$.getJSON('http://localhost:8000/calls/ip/calced', function(data){
var ipiss = data
edges2 = []
var nodes2 = []
var c =0
var idc = 0
ipiss.calls.forEach(call => {
  c++
  foundip = false
  foundtar = false
  nodes2.forEach(nod => {
    if (call.ip == nod.label) {
      foundip = true
      nod.value = nod.value + 1
    };  
  });
  nodes2.forEach(nod => {
    if ((call.target_ip == nod.label)) {
      foundtar = true
      nod.value = nod.value + 1
    };  
  });
  if (foundip == false) {
    var nod={id:idc, value: 10, label: call.ip}
    nodes2.push(nod)
    idc = idc + 1
  }
  if (foundtar == false) {
    var nod={id:idc, value: 10, label: call.target_ip}
    nodes2.push(nod)
    idc = idc + 1
  } 
}); 

ipiss.calls.forEach(call => {
  var edge ={
     from: 1, to: 1, value:1, label: 1, arrows: "to" 
  };
  nodes2.forEach(nod => {
    if (nod.label ==call.ip){
      edge.from= nod.id
    }
    if (nod.label ==call.target_ip){
      edge.to= nod.id
    }
  });
  edge.label = call.count.toString()
  edge.value = call.count
  edges2.push(edge)
});




console.log(edges2)
// create an array with nodes
 var nodes = new vis.DataSet([
  { id: 1, label: "Node 1" },
  { id: 2, label: "Node 2" },
  { id: 3, label: "Node 3" },
  { id: 4, label: "Node 4" },
  { id: 5, label: "Node 5" },
  { id: 6, label: "Node 6" },
  { id: 7, label: "Node 7" },
  { id: 8, label: "192.168.1.72" },
]);


// create an array with edges
var edges = new vis.DataSet([
  { from: 1, to: 8, label: "nice", arrows: "to", dashes: true },
  { from: 8, to: 1, arrows: "to", dashes: true },
  { from: 8, to: 8, arrows: "to", dashes: true },
  { from: 1, to: 3, arrows: "to" },
  { from: 1, to: 2, arrows: "to, from" },
  { from: 2, to: 4, arrows: "to, middle" },
  { from: 2, to: 5, arrows: "to, middle, from" },
  { from: 5, to: 6,  value: 1, arrows: { to: { scaleFactor: -1 } } },
  {
    from: 6, to: 7, arrows: { middle: { scaleFactor: 0.5 }, from: true },
  },
]);

// create a network
var container = document.getElementById("mynetwork");
var data = {
  nodes: nodes2,
  edges: edges2,
};
var options = {
  nodes: {
    scaling: {
      label: {
        min: 20,
        max: 25,
      },
    },
  },
  physics: {
    solver:"repulsion",
    repulsion:
    {
      centralGravity: 0.001,
      springLength: 10,
      springConstant: 0.002
    }
  }
};
var network = new vis.Network(container, data, options);
});