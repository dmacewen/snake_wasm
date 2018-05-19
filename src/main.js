const { Game } = wasm_bindgen;
let lastKey = 37;
let game = false;
let gameArea = null;
let gameAreaContext = null;

function getKey() {
  return lastKey;
}

function setKey(event) {
  lastKey = event.keyCode;
}

function logValue(str) {
  console.log(str);
}

function drawFood(x, y) {
  var img = new Image()
  img.src = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAoAAAAKCAMAAAC67D+PAAAAD1BMVEUisUy5elftHCT/gID///8wiCrhAAAAMUlEQVR4AT3KwQkAUQxCQeOf/mteCGEfCHMwJAyWmWObaYm+btJn7SfSOywvcSLUzgc4DwENsWMNEAAAAABJRU5ErkJggg=="
  img.onload = () => gameAreaContext.drawImage(img, x - 5, y - 5);
}

function eraseFood(x, y) {
  gameAreaContext.clearRect(x - 5, y - 5, 10, 10);
}

function drawDot(x, y, radius) {
  gameAreaContext.beginPath();
  gameAreaContext.arc(x, y, radius - 1, 0, 2 * Math.PI);
  gameAreaContext.stroke();
}

function eraseDot(x, y, radius) {
  gameAreaContext.clearRect(x - 5, y - 5, radius, radius);
}

function getRandomCoord(gridSize, squareSize) {
  return Math.floor((Math.random() * gridSize) / squareSize) * squareSize;
}

function gameLoop() {
  if(game) {
    game.increment();
    setTimeout(gameLoop, 100)
  } else {
    gameAreaContext.clearRect(0, 0, gameArea.width, gameArea.height);
  }
}

function startGame() {
  if(game) {
    console.log('Game is already Running!');
    return;
  }

  console.log('Starting Snake...')
  window.onkeydown = (event) => setKey(event);
  lastKey = 37;
  gameArea = document.getElementById("gameArea");
  gameAreaContext = gameArea.getContext("2d");

  game = Game.new();
  gameLoop();
}

function endGame() {
  if(!game) {
    console.log('Game is already Over!');
    return;
  }

  console.log('Ending Snake...')
  game = false;
}

// Load and instantiate the wasm file, and we specify the source of the wasm
// file here. Once the returned promise is resolved we're ready to go and
// use our imports.
wasm_bindgen('../out/main_bg.wasm').then(startGame);
