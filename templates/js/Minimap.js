class MiniMap {
  constructor() {
    this.cv = document.getElementById('miniMap');
    this.ct = this.cv.getContext('2d');
    this.input_key = new InputKeyboard();
    this.centerX = 0;
    this.centerY = 0;
  }

  convertMapPosX(x) {
    return (x + 70 - this.centerX) * 2;
  }

  convertMapPosY(y) {
    return (y + 30 - this.centerY) * 2;
  }

  update(pos) {
    const POS_KEY = 80;    // P

    this.centerX = pos.x;
    this.centerY = pos.z;

    if (this.input_key.isDown(POS_KEY)) {
      console.log(pos);
    }
  }

  clearMiniMap() {
    this.ct.clearRect(0, 0, this.cv.width, this.cv.height);
  }

  drawMiniMap() {
    this.ct.globalAlpha = 0.6;

    this.ct.fillStyle = '#ccffcc';
    this.ct.fillRect(0, 0, this.cv.width, this.cv.height);

    this.ct.strokeStyle = '#00ff00';
    this.ct.strokeRect(0, 0, this.cv.width, this.cv.height);
  }

  drawObj(obj) {
    let x = this.convertMapPosX(obj.pos.x);
    let y = this.convertMapPosY(obj.pos.z);
    let sizeX = 20;
    let sizeY = 20;
    this.ct.globalAlpha = 1.0;

    // player
    if (obj.objID == PLAYER) {
      this.ct.fillStyle = '#ff6666';
      sizeX = 10;
      sizeY = 10;
    } else if (obj.objID == ITEM) {
      this.ct.fillStyle = '#338833';
      sizeX = 10;
      sizeY = 10;
    } else {
      this.ct.fillStyle = '#ccccff';
    }
    this.ct.fillRect(x, y, sizeX, sizeY);
  }
}
