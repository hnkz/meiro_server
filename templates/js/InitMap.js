function initMap() {
  var wall = new Array();
  for (i = 0; i < mapPos.length; i++) {
    wall[i] = new Wall();
    wall[i].start(new THREE.Vector3(mapPos[i][0], mapPos[i][1], mapPos[i][2]));
  }
}
