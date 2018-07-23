class Floor extends GameObject {
  constructor() {
    super();
    this.objID = FLOOR;
    this.isVisible = false;
    this.mesh = new THREE.Mesh(
      new THREE.BoxGeometry(2000, 1, 2000),
      new THREE.MeshStandardMaterial({color: 0x00FFFF, roughness: 0.0})
    );
    // create AABB
    this.mesh.geometry.computeBoundingBox();
    HitCheckTbl.push(this);
    scene.add(this.mesh);
  }

  // hit
  onHit(player, hitPoint) {
    // create vector (hitPoint to player)
    let dir = player.pos.clone().sub(hitPoint);
    // calc length
    let k = player.sphere.radius - dir.length() + 0.001;
    dir.normalize();
    player.setPos(player.pos.clone().add(dir.clone().multiplyScalar(k)));
    if (dir.dot(new THREE.Vector3(0, 1, 0)) > 0.99) {
      player.y_speed = 0;
    }
  }
}
