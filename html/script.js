'use strict';

console.log("load complete");

var wsSocket = new WebSocket("ws://127.0.0.1:2794", "rust-websocket");
wsSocket.onmessage = onMessage;
wsSocket.onopen = onOpen;
wsSocket.onerror = onError;
wsSocket.onclose = onClose;

function sendData() {
    var elm = document.getElementById("comment");
    var comment = elm.value;
    elm.value = "";

    let message =
        `{
            "player": {
                "player01": "${comment}",
                "shizutaro": [30, 3111, 212.0021, 4.8888]
            }
        }`;

    wsSocket.send(message);
}

function onMessage(event) {
    if (event && event.data) {
        var box = document.getElementById("home");
        box.innerHTML = event.data + "<br>" + box.innerHTML;
    }
}

function onOpen() {
    // wsSocket.send("Hello Client!<br>");
}

function onError(error) {
    console.log(error);
    // console.log(error.code);
    // console.log(error.reason);
}

function onClose(event) {
    console.log(event);
}