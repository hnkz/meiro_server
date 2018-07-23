// object manage table
const GameObjectTbl = new Array();

// hit judge table
const HitCheckTbl = new Array();

// create scene
const scene = new THREE.Scene();

// object ID
const FLOOR  = 0;
const WALL   = 1;
const PLAYER = 2;
const ITEM   = 3;

class GameObject {
  constructor() {
    GameObjectTbl.push(this);
    this.pos = new THREE.Vector3(0, 0, 0);
  }

  // initialize
  start() {
  }

  // update
  update() {
  }

  // hit
  onHit(player, hitPoint) {
  }
}
