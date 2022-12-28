import { Board } from "bullet-connect-4";

//Creates a new game board for the client.
const board = Board.new();

//Gets the client and adds mouse input into the game.
//Previously tried to move this into Rust using web-sys, but ran into borrowing issues.
const canvas = document.getElementById("connect-4-canvas");
canvas.addEventListener("click", event => {
    //If the game has not been won, take input given where the mouse event occured.
    if( !board.game_won() ){
        const boundRect = canvas.getBoundingClientRect();
        board.take_input( (event.clientX - boundRect.left) * (canvas.width / boundRect.width) );
    }

    //Redraw the game.
    board.draw_game();
});

//Ticks player's time left using JsDate. Will be moved into the server.
//The interval is only for looping the tick_time method: it does not tick the time itself.
//Previously tried to move this into Rust using web-sys, but ran into borrowing issues.
const clientTimerDiv = document.getElementById("client-timer");
const serverTimerDiv = document.getElementById("server-timer");
let timerTick = setInterval(() => {
    //Move the amount of time left using JsDate
    board.tick_time();

    //Set the div text content to be what the time left is.
    clientTimerDiv.textContent = (board.client_time()/1000.0).toFixed(2).toString();
    serverTimerDiv.textContent = (board.server_time()/1000.0).toFixed(2).toString();
    
    //If the game is won, remove the interval.
    if( board.game_won() ) clearInterval( timerTick );
}, 10);

board.draw_game();