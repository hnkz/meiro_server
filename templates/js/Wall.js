class Wall extends GameObject {
  constructor() {
    super();
    this.objID = WALL;
    this.isVisible = true;
    this.mesh = new THREE.Mesh(
      new THREE.BoxGeometry(10, 10, 10),
      new THREE.MeshStandardMaterial({color: 0xFFFFFF, roughness: 0.0})
    );
    // create AABB
    this.mesh.geometry.computeBoundingBox();
    HitCheckTbl.push(this);
    scene.add(this.mesh);
  }

  start(pos) {
    super.start(pos);
    this.pos = pos.clone();
    this.mesh.position.set(this.pos.x, this.pos.y, this.pos.z);
  }

  // hit
  onHit(player, hitPoint) {
    // create vector (hitPoint to player)
    let dir = player.pos.clone().sub(hitPoint);
    // calc length
    let k = player.sphere.radius - dir.length();
    dir.normalize();
    player.setPos(player.pos.clone().add(dir.clone().multiplyScalar(k)));
    if (dir.dot(new THREE.Vector3(0, 1, 0)) > 0.99) {
      player.y_speed = 0;
    }
  }
}
