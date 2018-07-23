class Player extends GameObject {
  constructor() {
    super();
    this.objID = PLAYER;
    this.isVisible = true;
    this.y_rot_speed = 3;
    this.move_speed = 0.5;
    this.forward = new THREE.Vector3( 1, 0, 0 );
    this.input_key = new InputKeyboard();
    // create sphere for hit check
    this.sphere = new THREE.Sphere( new THREE.Vector3( 0, 0, 0 ), 2);
    this.oldPos =this.pos.clone();
    this.y_speed = 0;
    this.jumpCount = 100;
    this.mapCount = 10000;
  }

  start(pos) {
    super.start(pos);
    this.pos = pos.clone();
    this.updatePos(pos);
  }

  // update player position
  updatePos(currentPos) {
    this.oldPos.set(this.pos.x, this.pos.y, this.pos.z);
    this.pos.set(currentPos.x, currentPos.y, currentPos.z);
    this.sphere.center.set(this.pos.x, this.pos.y, this.pos.z);
  }

  setPos(setPos) {
    this.updatePos(setPos);
  }

  update() {
    super.update();
    const FORWARD_KEY = 87;     // W
    const BACK_KEY = 83;        // S
    const LEFT_TURN_KEY = 65;   // A
    const RIGHT_TURN_KEY = 68;  // D
    const JUMP_KEY = 74;        // J

    let currentPos = this.pos.clone();
    if (this.input_key.isDown(FORWARD_KEY)) {
      let move = this.forward.clone();
      move.multiplyScalar(this.move_speed);
      currentPos.add(move);
    }

    if (this.input_key.isDown(BACK_KEY)) {
      let move = this.forward.clone();
      move.multiplyScalar(this.move_speed);
      currentPos.sub(move);
    }

    if (this.input_key.isDown(LEFT_TURN_KEY)) {
      this.forward.applyAxisAngle(new THREE.Vector3(0, 1, 0), this.y_rot_speed*Math.PI/180.0);
    }

    if (this.input_key.isDown(RIGHT_TURN_KEY)) {
      this.forward.applyAxisAngle(new THREE.Vector3(0, -1, 0), this.y_rot_speed*Math.PI/180.0);
    }

    if (this.input_key.isDown(JUMP_KEY) && this.jumpCount > 0) {
      if (this.y_speed == 0) {
        this.y_speed = 2;
        this.jumpCount--;
      }
    }

    this.y_speed -= (9.8 / 120);
    currentPos.y += this.y_speed;
    this.updatePos(currentPos);

    if (this.mapCount > 0) {
      this.mapCount--;
    }
  }

  // get lool point
  getLookAtVector() {
    let lookAtVector = this.pos.clone();
    lookAtVector.add(this.forward);
    return lookAtVector;
  }

  hitCheck(obj) {
    // player
    if (obj.objID == PLAYER || obj.mesh.geometry.boundingBox == null || obj.mesh.geometry.boundingSphere == null) {
      return;
    }

    // other object
    let min = GameUtil.toWorldPoint(obj.pos, obj.mesh.geometry.boundingBox.min);
    let max = GameUtil.toWorldPoint(obj.pos, obj.mesh.geometry.boundingBox.max);
    let nearPoint = GameUtil.calcNearPointOnAABB(this.sphere.center, min, max);
    let length = nearPoint.distanceTo(this.pos);
    if (length < this.sphere.radius) {
      obj.onHit(this, nearPoint);
    }
  }
}
