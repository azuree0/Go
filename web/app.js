const BOARD_SIZE = 19;
const CELL_SIZE = 30;
const BOARD_PADDING = 15;
const STONE_RADIUS = 13;

let board = [];
let currentPlayer = 1; // 1 = Black, 2 = White
let gameOver = false;
let goGame = null;

const canvas = document.getElementById('goBoard');
const ctx = canvas.getContext('2d');

// Initialize board
function initBoard() {
    board = Array(BOARD_SIZE).fill(null).map(() => Array(BOARD_SIZE).fill(0));
    currentPlayer = 1;
    gameOver = false;
    updateUI();
    drawBoard();
    // Update score display after initialization
    if (goGame) {
        updateUI();
    }
}

// Draw board immediately on page load
initBoard();

// Draw the Go board
function drawBoard() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    // Draw board background
    ctx.fillStyle = '#deb887';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    
    // Draw grid lines
    ctx.strokeStyle = '#000';
    ctx.lineWidth = 1;
    
    for (let i = 0; i < BOARD_SIZE; i++) {
        const pos = BOARD_PADDING + i * CELL_SIZE;
        
        // Vertical lines
        ctx.beginPath();
        ctx.moveTo(pos, BOARD_PADDING);
        ctx.lineTo(pos, BOARD_PADDING + (BOARD_SIZE - 1) * CELL_SIZE);
        ctx.stroke();
        
        // Horizontal lines
        ctx.beginPath();
        ctx.moveTo(BOARD_PADDING, pos);
        ctx.lineTo(BOARD_PADDING + (BOARD_SIZE - 1) * CELL_SIZE, pos);
        ctx.stroke();
    }
    
    // Draw star points (hoshi)
    const starPoints = [3, 9, 15];
    ctx.fillStyle = '#000';
    for (let x of starPoints) {
        for (let y of starPoints) {
            ctx.beginPath();
            ctx.arc(
                BOARD_PADDING + x * CELL_SIZE,
                BOARD_PADDING + y * CELL_SIZE,
                3, 0, 2 * Math.PI
            );
            ctx.fill();
        }
    }
    
    // Draw stones
    if (goGame) {
        const boardState = goGame.getBoard();
        for (let y = 0; y < BOARD_SIZE; y++) {
            for (let x = 0; x < BOARD_SIZE; x++) {
                const color = boardState[y][x];
                if (color !== 0) {
                    drawStone(x, y, color);
                }
            }
        }
    }
}

// Draw a stone
function drawStone(x, y, color) {
    const centerX = BOARD_PADDING + x * CELL_SIZE;
    const centerY = BOARD_PADDING + y * CELL_SIZE;
    
    ctx.beginPath();
    ctx.arc(centerX, centerY, STONE_RADIUS, 0, 2 * Math.PI);
    
    if (color === 1) { // Black
        ctx.fillStyle = '#000';
    } else { // White
        ctx.fillStyle = '#fff';
    }
    
    ctx.fill();
    ctx.strokeStyle = '#333';
    ctx.lineWidth = 1;
    ctx.stroke();
    
    // Add highlight for white stones
    if (color === 2) {
        ctx.beginPath();
        ctx.arc(centerX - 3, centerY - 3, 4, 0, 2 * Math.PI);
        ctx.fillStyle = 'rgba(255, 255, 255, 0.6)';
        ctx.fill();
    }
}

// Get board coordinates from mouse position
function getBoardCoords(event) {
    const rect = canvas.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;
    
    const boardX = Math.round((x - BOARD_PADDING) / CELL_SIZE);
    const boardY = Math.round((y - BOARD_PADDING) / CELL_SIZE);
    
    if (boardX >= 0 && boardX < BOARD_SIZE && boardY >= 0 && boardY < BOARD_SIZE) {
        return { x: boardX, y: boardY };
    }
    return null;
}

// Handle mouse click on board
canvas.addEventListener('click', (event) => {
    if (gameOver) return;
    
    const coords = getBoardCoords(event);
    if (!coords) return;
    
    // If WebAssembly is loaded, use it; otherwise use local board state
    if (goGame) {
        const success = goGame.makeMove(coords.x, coords.y);
        if (success) {
            updateGameState();
            drawBoard();
        } else {
            showStatus('Invalid move!', 'error');
        }
    } else {
        // Fallback: use local board state for placing stones
        if (board[coords.y][coords.x] === 0) {
            const playerName = currentPlayer === 1 ? 'Black' : 'White';
            board[coords.y][coords.x] = currentPlayer;
            currentPlayer = currentPlayer === 1 ? 2 : 1;
            updateUI();
            drawBoard();
            showStatus(`Placed ${playerName} stone`, 'success');
        } else {
            showStatus('Invalid move! Space already occupied.', 'error');
        }
    }
});

// Update game state from WebAssembly
function updateGameState() {
    if (!goGame) return;
    
    currentPlayer = goGame.getCurrentPlayer();
    gameOver = goGame.isGameOver();
    updateUI();
    
    if (gameOver) {
        if (goGame) {
            const score = goGame.getScore();
            const blackScore = score.black;
            const whiteScore = score.white;
            let winner = '';
            if (blackScore > whiteScore) {
                winner = `Black wins!`;
            } else if (whiteScore > blackScore) {
                winner = `White wins!`;
            } else {
                winner = `Tie!`;
            }
            // Show score in status box - this will persist
            const statusEl = document.getElementById('status');
            statusEl.textContent = `${winner} | Black: ${blackScore} - White: ${whiteScore}`;
            statusEl.className = 'success';
            // Clear any pending timeout
            if (statusEl._timeout) {
                clearTimeout(statusEl._timeout);
                statusEl._timeout = null;
            }
        } else {
            const statusEl = document.getElementById('status');
            statusEl.className = 'success';
            if (statusEl._timeout) {
                clearTimeout(statusEl._timeout);
                statusEl._timeout = null;
            }
        }
    }
}

// Update UI elements
function updateUI() {
    const playerText = currentPlayer === 1 ? 'Black' : 'White';
    document.getElementById('currentPlayer').textContent = playerText;
    
    // Disable game buttons when game is over, but keep "New Game" enabled
    const undoBtn = document.getElementById('undoBtn');
    const passBtn = document.getElementById('passBtn');
    
    if (undoBtn) undoBtn.disabled = gameOver;
    if (passBtn) passBtn.disabled = gameOver;
    
    // New Game button should always be enabled
}

// Show status message
function showStatus(message, type = 'info', persist = false) {
    const statusEl = document.getElementById('status');
    statusEl.textContent = message;
    statusEl.className = type;
    
    // Clear previous timeout if any
    if (statusEl._timeout) {
        clearTimeout(statusEl._timeout);
        statusEl._timeout = null;
    }
    
    // Only auto-clear if not persisting and not an error
    if (!persist && type !== 'error') {
        statusEl._timeout = setTimeout(() => {
            statusEl.textContent = '';
            statusEl._timeout = null;
        }, 3000);
    }
}

// New game button
document.getElementById('newGameBtn').addEventListener('click', () => {
    if (goGame) {
        goGame.newGame();
        initBoard();
        showStatus('New game started!', 'success');
    } else {
        // Even if WebAssembly isn't loaded, reset local board state
        initBoard();
        showStatus('New game started!', 'success');
    }
});

// Undo button
document.getElementById('undoBtn').addEventListener('click', () => {
    if (gameOver || !goGame) return;
    
    const success = goGame.undo();
    if (success) {
        updateGameState();
        drawBoard();
        showStatus('Move undone', 'info');
    } else {
        showStatus('No moves to undo', 'error');
    }
});

// Pass button
document.getElementById('passBtn').addEventListener('click', () => {
    if (gameOver || !goGame) return;
    
    const success = goGame.pass();
    if (success) {
        updateGameState();
        // Only show "Passed" if game is not over
        if (!goGame.isGameOver()) {
            showStatus('Passed', 'info');
        }
        // If game is over, updateGameState() will show the score
    }
});

// Initialize when WebAssembly is loaded
async function init() {
    showStatus('Loading WebAssembly...', 'loading');
    
    try {
        // Load Rust WebAssembly module (wasm-bindgen generated)
        const wasmModule = await import('./wasm_exec.js');
        
        // Initialize WASM module - default export loads the wasm file
        // wasm-bindgen looks for go_game_bg.wasm by default, or we can specify it
        await wasmModule.default();
        
        // Initialize the game
        wasmModule.init_game();
        
        // Create goGame object that wraps Rust functions
        goGame = {
            makeMove: (x, y) => {
                return wasmModule.make_move(x, y);
            },
            getBoard: () => {
                // Convert js_sys::Array to regular JavaScript array
                const boardArray = wasmModule.get_board();
                const result = [];
                for (let i = 0; i < boardArray.length; i++) {
                    const row = boardArray[i];
                    const rowArray = [];
                    for (let j = 0; j < row.length; j++) {
                        rowArray.push(row[j]);
                    }
                    result.push(rowArray);
                }
                return result;
            },
            getCurrentPlayer: () => {
                return wasmModule.get_current_player();
            },
            isGameOver: () => {
                return wasmModule.is_game_over();
            },
            newGame: () => {
                wasmModule.new_game();
                return true;
            },
            undo: () => {
                return wasmModule.undo_move();
            },
            pass: () => {
                return wasmModule.pass_move();
            },
            getScore: () => {
                return wasmModule.get_score();
            }
        };
        
        initBoard();
        showStatus('New game', 'success');
    } catch (error) {
        showStatus('Error loading WebAssembly: ' + error.message + '. Please run build.bat or build.sh first.', 'error');
        console.error('WebAssembly loading error:', error);
        // Fallback: allow playing without WebAssembly
        initBoard();
        showStatus('Running in fallback mode (no WebAssembly). Some features may be limited.', 'info');
    }
}

// Start initialization when page loads
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}

