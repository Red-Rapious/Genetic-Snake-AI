import * as gm from "lib-game-wasm";

// Number of snakes displayed at the same time
//const NB_VISUALISATION = 4;

// Minimum delay between each frame, in milliseconds.
const FRAME_DELAY = 0;
// The number of generations trained by one click on the "Train" button
const GENERATIONS_TRAIN = 80;
// Divide the displayed fitness by a big number to avoid showing huge fitnesses
const FITNESS_UNITS = 10000;

var games = new gm.Games();

// The `pause` checkbox pauses the game by stoping the main rendering function
var gamePaused = false;
const pauseCheckbox = document.getElementById("pause");
pauseCheckbox.checked = gamePaused;

pauseCheckbox.onclick = function() {
    gamePaused = pauseCheckbox.checked;
    if (!gamePaused) {
        redraw();
    }
}

// The restart buttons creates a new `Games` object
const restartBtn = document.getElementById("restart");
restartBtn.onclick = function() {
    games = new gm.Games();
    pauseCheckbox.checked = false;
    gamePaused = false;
    redraw();
}

// Maps the "Next Generation" button to `simulation.train()`
const trainBtn = document.getElementById("train");
trainBtn.onclick = function() {
    for (var i = 0; i < GENERATIONS_TRAIN; i += 1) {
        games.train();
    }
    trainBtn.innerHTML = "Train " + GENERATIONS_TRAIN + " Generation" + ((GENERATIONS_TRAIN > 1) ? "s" : "");
}

trainBtn.innerHTML = "Train " + GENERATIONS_TRAIN + " Generation" + ((GENERATIONS_TRAIN > 1) ? "s" : "");

const generationLabel = document.getElementById("generation");
const bestScoreLabel = document.getElementById("best-score");
const minFitnessLabel = document.getElementById("min-fitness");
const maxFitnessLabel = document.getElementById("max-fitness");
const averageFitnessLabel = document.getElementById("average-fitness");

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
const games_list = games.games();
const width = games_list[0].width;
const height = games_list[0].height;
const side_w = viewportWidth / width;
const side_h = viewportHeight / height;

// Canvas rendering context
const ctxt = viewport.getContext('2d');
ctxt.scale(viewportScale, viewportScale);

// Main rendering function
function redraw() {
    var games_list = games.games();
    ctxt.clearRect(0, 0, viewportWidth, viewportHeight);
    games.step();

    // Draw snake
    const snake = games_list[0].snake;

    ctxt.fillStyle = 'rgb(255, 255, 255)';
    for (var tail = 0; tail < snake.length; tail += 1) {
        const x = snake[tail][0];
        const y = snake[tail][1];

        const size_ratio = 0.9;
        ctxt.fillRect(
            x * side_w + side_w * (1-size_ratio) / 2, 
            y * side_h + side_h * (1-size_ratio) / 2, 
            side_w * size_ratio, 
            side_h * size_ratio
        );
    }

    // Draw eyes
    const direction = games_list[0].direction;
    ctxt.fillStyle = 'rgb(0, 0, 0)';

    const x0 = snake[0][0];
    const y0 = snake[0][1];

    if (direction == 0) {
        // eyes on the right
        ctxt.fillRect(x0 * side_w + side_w * 0.6, y0 * side_h + side_w * 0.1, side_w * 0.3, side_h * 0.3);
        ctxt.fillRect(x0 * side_w + side_w * 0.6, y0 * side_h + side_w * 0.6, side_w * 0.3, side_h * 0.3);
    }
    else if (direction == 1) {
        // eyes down
        ctxt.fillRect(x0 * side_w + side_w * 0.1, y0 * side_h + side_w * 0.5, side_w * 0.3, side_h * 0.3);
        ctxt.fillRect(x0 * side_w + side_w * 0.6, y0 * side_h + side_w * 0.5, side_w * 0.3, side_h * 0.3);
    }
    else if (direction == 2) {
        // eyes on the left
        ctxt.fillRect(x0 * side_w + side_w * 0.1, y0 * side_h + side_w * 0.1, side_w * 0.3, side_h * 0.3);
        ctxt.fillRect(x0 * side_w + side_w * 0.1, y0 * side_h + side_w * 0.6, side_w * 0.3, side_h * 0.3);
    }
    else if (direction == 3) {
        // eyes up
        ctxt.fillRect(x0 * side_w + side_w * 0.1, y0 * side_h + side_w * 0.1, side_w * 0.3, side_h * 0.3);
        ctxt.fillRect(x0 * side_w + side_w * 0.6, y0 * side_h + side_w * 0.1, side_w * 0.3, side_h * 0.3);
    } 

    // Draw apple
    const apple = games_list[0].apple;

    ctxt.fillStyle = 'rgb(255, 0, 0)';
    ctxt.beginPath();
    ctxt.roundRect(apple[0] * side_w, apple[1] * side_h, side_w, side_h, side_w * 0.1);
    ctxt.fill();

    // Update the labels
    generationLabel.textContent = "Generation: " + games.generation();
    bestScoreLabel.textContent = "Best Score (gen.): " + games.best_score();

    minFitnessLabel.textContent = "Min. Fitness: " + Math.round(games.min_fitness() / FITNESS_UNITS);
    maxFitnessLabel.textContent = "Max. Fitness: " + Math.round(games.max_fitness() / FITNESS_UNITS);
    averageFitnessLabel.textContent = "Average Fitness: " + Math.round(games.avg_fitness() / FITNESS_UNITS);

    if (!gamePaused) {
        setTimeout(function() { requestAnimationFrame(redraw) }, FRAME_DELAY);
    }
}

redraw();