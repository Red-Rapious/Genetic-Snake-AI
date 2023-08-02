import * as gm from "lib-game-wasm";

//const NB_VISUALISATION = 4;

var games = new gm.Games();
var viewport = document.getElementById("viewport");

const viewportWidth = viewport.width;
const viewportHeight = viewport.height;

const viewportScale = window.devicePixelRatio || 1;

viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;

viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';

const width = games.games()[0].width;
const height = games.games()[0].height;
const side_w = viewportWidth / width;
const side_h = viewportHeight / height;

const ctxt = viewport.getContext('2d');
ctxt.scale(viewportScale, viewportScale);
ctxt.fillStyle = 'rgb(255, 255, 255)';

function redraw() {
    var snake = games.games()[0].snake;

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

    //requestAnimationFrame(redraw);
}

redraw();