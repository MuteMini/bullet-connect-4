import { Board, Hud } from "bullet-connect-4";

const board = Board.new();
const width = board.width();

const TOKEN_RADIUS = 50;
const PADDING = 15;

//const hud = Hud.new();

const canvas = document.getElementById("connect-4-canvas");
const ctx = canvas.getContext("2d");

canvas.addEventListener("click", event => {
    if( !board.game_won() ){
        const boundRect = canvas.getBoundingClientRect();
        board.take_input( (event.clientX - boundRect.left) * (canvas.width / boundRect.width) );
    }

    board.draw_game();

    if( board.game_won() ) {
        ctx.font = "48px serif";
        ctx.fillStyle = "#F9CCCA";
        ctx.fillText("SOMEONE WON OMG", canvas.width/2 - 200, canvas.height/2);
    }
});

const clientTimerDiv = document.getElementById("client-timer");
const serverTimerDiv = document.getElementById("server-timer");

let timerTick = setInterval(() => {
    board.tick_time();

    clientTimerDiv.textContent = (board.client_time()/1000.0).toFixed(2).toString();
    serverTimerDiv.textContent = (board.server_time()/1000.0).toFixed(2).toString();
    
    if( board.game_won() ) clearInterval( timerTick );
}, 10);

board.draw_game();