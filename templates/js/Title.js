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

    if (this.input_key.isDown(RETURN_KEY) && gameState == 2) {
      this.clear();
      gameState++;
    }
  }

  clear() {
    this.ct.clearRect(this.x, this.y, this.cv.width, this.cv.height);
  }

  draw() {
    this.ct.drawImage(this.image, this.x, this.y, this.cv.width, this.cv.height);

    if (gameState == 2) {
      this.ct.font = "64px cursive";
      this.ct.textAlign = "center";
      this.ct.textBaseline = "top";
      this.ct.fillStyle = "white";
      this.ct.fillText("Press Enter Key", this.cv.width / 2, this.cv.height / 2 + 30);
    }
  }
}
