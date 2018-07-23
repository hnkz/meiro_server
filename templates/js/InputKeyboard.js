function InputKeyboard() {
  var _input_key_buffer = null;

  function KeyDownFunc(e) {
    _input_key_buffer[e.keyCode] = true;
  }
  function KeyUpFunc(e) {
    _input_key_buffer[e.keyCode] = false;
  }
  function BlurFunc(e) {
    _input_key_buffer.length = 0;
  }

  this.isDown = function(key_code) {
    if (_input_key_buffer[key_code]) {
      return true;
    }
    return false;
  };

  this.release = function() {
    if (window.removeEventListener) {
      document.removeEventListener("keydown", KeyDownFunc);
      document.removeEventListener("keyup", KeyUpFunc);
      window.removeEventListener("blur", BlurFunc);
    } else if (window.detachEvent) {
      document.detachEvent("onkeydown", KeyDownFunc);
      document.detachEvent("onkeyup", KeyUpFunc);
      window.detachEvent("onblur", BlurFunc);
    }
  };

  (function() {
    _input_key_buffer = new Array();

    if (window.addEventListener) {
      document.addEventListener("keydown", KeyDownFunc);
      document.addEventListener("keyup", KeyUpFunc);
      window.addEventListener("blur", BlurFunc);
    } else if (window.attachEvent) {
      document.attachEvent("onkeydown", KeyDownFunc);
      document.attachEvent("onkeyup", KeyUpFunc);
      window.attachEvent("onblur", BlurFunc);
    }
  })();
}
