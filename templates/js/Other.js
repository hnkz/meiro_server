class Other extends GameObject {
  constructor() {
    super();
    this.objID = PLAYER;
    this.isVisible = true;
    this.mesh = new THREE.Mesh(
      new THREE.SphereGeometry(2),
      new THREE.MeshStandardMaterial({color: 0xffffff, roughness: 0.0})
    );
    scene.add(this.mesh);
  }

  start(pos) {
    super.start(pos);
    this.pos = pos.clone();
    this.mesh.position.set(this.pos.x, this.pos.y, this.pos.z);
  }

  // move updown
  update() {
    this.mesh.position.set(this.pos.x, this.pos.y, this.pos.z);
  }
}
