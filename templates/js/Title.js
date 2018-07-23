class Title {
  constructor() {
    this.cv = document.getElementById('title');
    this.ct = this.cv.getContext('2d');
    this.input_key = new InputKeyboard();
    this.image = new Image();
    this.image.src = './image/title.png';
    this.x = 0;
    this.y = 0;
  }

  update(pos) {
    const RETURN_KEY = 13;  // Enter

    if (this.input_key.isDown(RETURN_KEY)) {
      this.clear();
      gameState++;
    }
  }

  clear() {
    this.ct.clearRect(this.x, this.y, this.cv.width, this.cv.height);
  }

  draw() {
    this.ct.drawImage(this.image, this.x, this.y, this.cv.width, this.cv.height);
  }
}
