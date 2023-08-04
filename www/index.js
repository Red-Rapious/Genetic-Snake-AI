import * as gm from "lib-game-wasm";

// Number of snakes displayed at the same time
//const NB_VISUALISATION = 4;

var games = new gm.Games();

// The `pause` checkbox pauses the simulation by stoping the main rendering function
var simulationPaused = false;
const pauseCheckbox = document.getElementById("pause");
pauseCheckbox.checked = simulationPaused;

pauseCheckbox.onclick = function() {
    simulationPaused = pauseCheckbox.checked;
    if (!simulationPaused) {
        redraw();
    }
}

// The restart buttons creates a new `Games` object
const restartBtn = document.getElementById("restart");
restartBtn.onclick = function() {
    games = new gm.Games();
}

// Maps the "Next Generation" button to `simulation.train()`
document.getElementById("train").onclick = function() {
    //simulation.train();
}

var viewport = document.getElementById("viewport");

// Adapat the viewport scale to avoid pixelized images.
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;

const viewportScale = window.devicePixelRatio || 1;

viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;

viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';

// Computes the side of one cell of the grid
const width = games.games()[0].width;
const height = games.games()[0].height;
const side_w = viewportWidth / width;
const side_h = viewportHeight / height;

// Canvas rendering context
const ctxt = viewport.getContext('2d');
ctxt.scale(viewportScale, viewportScale);

// Main rendering function
function redraw() {
    ctxt.clearRect(0, 0, viewportWidth, viewportHeight);
    games.step();

    var snake = games.games()[0].snake;

    ctxt.fillStyle = 'rgb(255, 255, 255)';
    for (var tail = 0; tail < snake.length; tail += 1) {
        const x = snake[tail][0];
        const y = snake[tail][1];

        ctxt.fillRect(x * side_w, y * side_h, side_w, side_h);
    }

    // Eyes
    const x0 = snake[0][0];
    const y0 = snake[0][1];

    ctxt.fillStyle = 'rgb(0, 0, 0)';
    ctxt.fillRect(x0 * side_w + side_w * 0.1, y0 * side_h + side_w * 0.1, side_w * 0.3, side_h * 0.3);
    ctxt.fillRect(x0 * side_w + side_w * 0.6, y0 * side_h + side_w * 0.1, side_w * 0.3, side_h * 0.3);

    if (!simulationPaused) {
        requestAnimationFrame(redraw);
    }
}

redraw();