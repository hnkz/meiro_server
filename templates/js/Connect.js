var connection = new WebSocket("ws://192.168.0.87:2794", "rust-websocket");

// receive positions
var playerPos = [
    [0, 4, 0],
    [0, 4, 0],
    [0, 4, 0],
    [0, 4, 0]
];
var mapPos = new Array();
var itemType = new Array();
var itemPos = new Array();

// game state
var gameState = 0;

// receive IDs
var playerID = 0;
var getItemID = -1;
var getPlayerID = false;

connection.onopen = onOpen;
connection.onclose = onClose;
connection.onerror = onError;
connection.onmessage = onMessage;

function onOpen() {
    console.log("connection success");
}

function onClose(event) {
    console.log(event);
}

function onError(error) {
    console.log(error);
}

function onMessage(message) {
    let data = JSON.parse(message.data);
    //console.log(data);

    // game state
    if (gameState == 0) {
        gameState++;
    }

    // id
    if (data.id !== undefined && !getPlayerID) {
        playerID = data.id;
        getPlayerID = true;
    }

    // map
    if (data.map) {
        for (i = 0; i < data.map.length; i++) {
            mapPos[i] = data.map[i];
        }
    }

    // player
    if (data.player) {
        for (i = 0; i < data.player.length; i++) {
            playerPos[i][0] = data.player[i].x;
            playerPos[i][1] = data.player[i].y;
            playerPos[i][2] = data.player[i].z;
        }
    }

    // item
    if (data.item) {
        for (i = 0; i < data.item.length; i++) {
            itemType[i] = data.item[i].type;
            itemPos[i] = data.item[i].pos;
        }
    }

    // get
    getItemID = -1;
    if (data.get) {
        getItemID = data.get;
    }

    // chat
    if (data.chat) {
        let content = document.getElementById("text");
        let newContent = "Player" + data.chat.id + ": " + data.chat.content + "<br>";
        content.innerHTML = newContent + content.innerHTML;
        document.getElementById("chat").value = "";
    }
}

function sendPos(pos) {
    let message = `{ "pos": [${pos.x}, ${pos.y}, ${pos.z}] }`;
    connection.send(message);
}

function sendGet(id) {
    let message = `{ "get": ${id} }`;
    connection.send(message);
}

function sendChat() {
    let content = document.getElementById("chat").value;
    let message = `{ "chat": { "id": ${playerID}, "content": "${content}" } }`;
    connection.send(message);
}