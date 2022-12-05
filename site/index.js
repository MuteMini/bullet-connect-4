import { Board, Token } from "bullet-connect-4";
import { memory } from "bullet-connect-4/bullet_connect_4_bg";

const TOKEN_RADIUS = 50;
const PADDING = 15;
const GRID_COLOR = "#0000FF";
const RED_COLOR = "#FF0000";
const YELLOW_COLOR = "#FFFF00";

const board = Board.new();
const width = board.width();
const height = board.height();

const canvas = document.getElementById("connect-4-canvas");
canvas.height = (TOKEN_RADIUS*2 + PADDING)*height + 1;
canvas.width = (TOKEN_RADIUS*2 + PADDING)*width + 1;

const ctx = canvas.getContext("2d");

const getIndex = ( row, column ) => {
    return row*width + column;
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= width; ++i) {
        ctx.moveTo( i*(TOKEN_RADIUS*2 + PADDING) + 1, 0 );
        ctx.lineTo( i*(TOKEN_RADIUS*2 + PADDING) + 1, (TOKEN_RADIUS*2 + PADDING)*height + 1 );
    }

    for (let j = 0; j <= height; ++j) {
        ctx.moveTo( 0, j*(TOKEN_RADIUS*2 + PADDING) + 1);
        ctx.lineTo( (TOKEN_RADIUS*2 + PADDING)*width + 1, j*(TOKEN_RADIUS*2 + PADDING) + 1 );
    }

    ctx.stroke();
};

const drawTokens = () => {
    const tokenPtr = board.token();
    const token = new Uint8Array( memory.buffer, tokenPtr, width*height );

    for (let row = 0; row < height; ++row) {
        for (let col = 0; col < width; ++col) {
            const idx = getIndex( row, col );

            ctx.beginPath();
            ctx.fillStyle = ( token[idx] === Token.Empty )
                ? GRID_COLOR
                : (( token[idx] === Token.Client )
                    ? YELLOW_COLOR
                    : RED_COLOR
                );
            
            ctx.arc(
                2*col*(TOKEN_RADIUS + PADDING/2) + TOKEN_RADIUS + PADDING/2,
                canvas.height - 2*(row + 1)*(TOKEN_RADIUS + PADDING/2) + TOKEN_RADIUS + PADDING/2,
                TOKEN_RADIUS, 
                0, 2*Math.PI
            );
            ctx.fill();
        }
    }
};

canvas.addEventListener("click", event => {

    if( !board.game_won() ){
        const boundRect = canvas.getBoundingClientRect();

        const scaleX = canvas.width / boundRect.width;
        const canvasLeft = (event.clientX - boundRect.left) * scaleX;

        const col = Math.min( Math.floor( canvasLeft / (TOKEN_RADIUS*2 + PADDING) ), width - 1 );

        board.take_input( col );
    }

    drawGrid();
    drawTokens();

    if( board.game_won() ) {
        ctx.font = "48px serif";
        ctx.fillStyle = "#F9CCCA";
        ctx.fillText("SOMEONE WON OMG", canvas.width/2 - 200, canvas.height/2);
    }
});

const clientTimerDiv = document.getElementById("client-timer");
const serverTimerDiv = document.getElementById("server-timer");

let timerTick = setInterval(() => {

    clientTimerDiv.textContent = (board.client_time()/1000.0).toFixed(2).toString();
    serverTimerDiv.textContent = (board.server_time()/1000.0).toFixed(2).toString();

    board.tick_time();
    
    if( board.game_won() ) clearInterval( timerTick );
}, 10);

drawGrid();
drawTokens();