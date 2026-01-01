// Import: WebAssembly module and game classes
import init, { GameState, Stone } from './pkg/go_game.js';

// Global: Current game state instance
let game = null;

// Init: Initialize WASM, create game, render board, and set up event listeners
async function run() {
    await init();
    game = new GameState();
    renderBoard();
    updateUI();
    setupEventListeners();
    
    // Export: Make test function available globally for console access
    window.testGuanzi = testGuanzi;
    console.log('testGuanzi() is now available in the console');
}

// Events: Set up click handlers for Pass and Reset buttons
function setupEventListeners() {
    document.getElementById('pass-btn').addEventListener('click', () => {
        if (!game || (typeof game.game_over === 'function' ? game.game_over() : game.game_over)) {
            return;
        }
        game.pass();
        renderBoard();
        updateUI();
    });

    document.getElementById('reset-btn').addEventListener('click', () => {
        if (!game) {
            return;
        }
        game.reset();
        renderBoard();
        updateUI();
    });
}

// Render: Build the 19x19 Go board with stones, labels, and intersections
function renderBoard() {
    const board = document.getElementById('game-board');
    board.innerHTML = '';
    
    const boardData = game.get_board_data();
    
    // Labels: Add coordinate labels at the top (A-S)
    const topLabels = document.createElement('div');
    topLabels.className = 'board-labels top-labels';
    topLabels.appendChild(document.createElement('div')); // Empty corner
    for (let col = 0; col < 19; col++) {
        const label = document.createElement('div');
        label.className = 'board-label';
        label.textContent = game.get_column_label(col);
        topLabels.appendChild(label);
    }
    board.appendChild(topLabels);
    
    // Grid: Create 19x19 board grid with intersections
    for (let row = 0; row < 19; row++) {
        const boardRow = document.createElement('div');
        boardRow.className = 'board-row';
        
        // Label: Add row label on the left (19-1)
        const leftLabel = document.createElement('div');
        leftLabel.className = 'board-label row-label';
        leftLabel.textContent = game.get_row_label(row);
        boardRow.appendChild(leftLabel);
        
        // Intersections: Create 19x19 grid intersections (clickable positions)
        for (let col = 0; col < 19; col++) {
            const index = row * 19 + col;
            const data = boardData[index];
            const intersection = document.createElement('div');
            intersection.className = 'intersection';
            intersection.dataset.row = data.row;
            intersection.dataset.col = data.col;
            
            // Stone: Add black or white stone if present
            if (data.stone === 1) {
                intersection.className += ' black-stone';
            } else if (data.stone === 2) {
                intersection.className += ' white-stone';
            }
            
            // Star Point: Add hoshi marker on empty intersections
            if (data.is_star_point && data.stone === 0) {
                const starPoint = document.createElement('div');
                starPoint.className = 'star-point';
                intersection.appendChild(starPoint);
            }
            
            // Last Move: Highlight the last played stone
            if (data.is_last_move) {
                intersection.className += ' last-move';
            }
            
            // Valid Move: Highlight intersections where moves are valid
            if (data.is_valid_move) {
                intersection.className += ' valid-move';
            }
            
            // Click: Add click handler to place stones
            const clickRow = row;
            const clickCol = col;
            intersection.addEventListener('click', () => {
                handleIntersectionClick(clickRow, clickCol);
            });
            boardRow.appendChild(intersection);
        }
        
        // Add row label on the right
        const rightLabel = document.createElement('div');
        rightLabel.className = 'board-label row-label';
        rightLabel.textContent = game.get_row_label(row);
        boardRow.appendChild(rightLabel);
        
        board.appendChild(boardRow);
    }
    
    // Labels: Add coordinate labels at the bottom (A-S)
    const bottomLabels = document.createElement('div');
    bottomLabels.className = 'board-labels bottom-labels';
    bottomLabels.appendChild(document.createElement('div')); // Empty corner
    for (let col = 0; col < 19; col++) {
        const label = document.createElement('div');
        label.className = 'board-label';
        label.textContent = game.get_column_label(col);
        bottomLabels.appendChild(label);
    }
    board.appendChild(bottomLabels);
}

// UI: Update player indicator, captured counts, and game status
function updateUI() {
    // Player: Update current player display
    const currentPlayer = typeof game.current_player === 'function' ? game.current_player() : game.current_player;
    const playerName = currentPlayer === Stone.Black ? 'Black' : 'White';
    const playerNameEl = document.getElementById('player-name');
    playerNameEl.textContent = playerName;
    playerNameEl.className = `player-${playerName.toLowerCase()}`;
    
    // Cursor: Update board cursor to show current player's stone
    const board = document.getElementById('game-board');
    board.classList.remove('cursor-black', 'cursor-white');
    if (currentPlayer === Stone.Black) {
        board.classList.add('cursor-black');
    } else {
        board.classList.add('cursor-white');
    }
    
    // Captured: Update captured stone counts
    const blackCaptured = typeof game.black_captured === 'function' ? game.black_captured() : game.black_captured;
    const whiteCaptured = typeof game.white_captured === 'function' ? game.white_captured() : game.white_captured;
    document.getElementById('black-captured').textContent = blackCaptured;
    document.getElementById('white-captured').textContent = whiteCaptured;
    
    // Status: Show game over message with scores if both players passed
    const statusEl = document.getElementById('status');
    const isGameOver = typeof game.game_over === 'function' ? game.game_over() : game.game_over;
    if (isGameOver) {
        // Calculate final scores with komi
        const [blackScore, whiteScore] = game.calculate_scores();
        const winner = blackScore > whiteScore ? 'Black' : whiteScore > blackScore ? 'White' : 'Tie';
        
        let statusText = `Game Over! `;
        statusText += `Black: ${blackScore.toFixed(1)} | White: ${whiteScore.toFixed(1)} (Komi: 6.5)`;
        
        if (winner === 'Tie') {
            statusText += ' - Tie!';
        } else {
            statusText += ` - ${winner} wins!`;
        }
        statusEl.style.color = '#212529'; // Black text for better readability
        
        statusEl.textContent = statusText;
        // Remove cursor classes when game is over
        board.classList.remove('cursor-black', 'cursor-white');
    } else {
        statusEl.textContent = '';
    }
}

// Click: Handle stone placement when intersection is clicked
function handleIntersectionClick(row, col) {
    if (!game) {
        return;
    }
    const isGameOver = typeof game.game_over === 'function' ? game.game_over() : game.game_over;
    if (isGameOver) {
        return;
    }
    
    const success = game.place_stone(row, col);
    
    if (success) {
        renderBoard();
        updateUI();
    }
}

// Test: Guānzǐ (官子) - Endgame test function
// Sets up an endgame scenario with ~100 white stones on left, ~100 black stones on right
// Call from console: testGuanzi()
function testGuanzi() {
    if (!game) {
        console.error('Game not initialized. Please wait for page to load.');
        return;
    }
    
    console.log('Setting up Guānzǐ (官子) endgame test scenario...');
    console.log('Placing ~100 white stones on left, ~100 black stones on right...');
    
    // Reset game (starts with Black to play)
    game.reset();
    
    // Generate moves to create endgame scenario
    // Left side (cols 0-9): ~100 white stones
    // Right side (cols 10-18): ~100 black stones
    // Middle area (around col 9-10): mostly empty for endgame moves
    
    // Collect positions for each side
    const blackMoves = []; // Right side (cols 10-18)
    const whiteMoves = []; // Left side (cols 0-9)
    
    // Black territory - right side of board (cols 10-18)
    for (let row = 0; row < 19; row++) {
        for (let col = 10; col < 19; col++) {
            // Skip some intersections to leave small gaps for endgame moves
            if ((row + col) % 3 !== 0 && (row * 19 + col) % 7 !== 0) {
                blackMoves.push([row, col]);
            }
        }
    }
    
    // White territory - left side of board (cols 0-9)
    for (let row = 0; row < 19; row++) {
        for (let col = 0; col < 10; col++) {
            // Skip some intersections to leave small gaps for endgame moves
            if ((row + col) % 3 !== 0 && (row * 19 + col) % 7 !== 0) {
                whiteMoves.push([row, col]);
            }
        }
    }
    
    // Add border stones
    for (let row = 0; row < 19; row++) {
        if (row % 2 === 0) {
            blackMoves.push([row, 10]); // Left border of black territory
            whiteMoves.push([row, 9]);  // Right border of white territory
        }
    }
    
    // Place moves ensuring correct player for each territory
    // Game starts with Black, so pattern is: Black (right), White (left), Black (right), White (left)...
    let successCount = 0;
    let blackCount = 0;
    let whiteCount = 0;
    let blackIndex = 0;
    let whiteIndex = 0;
    
    // Continue placing stones until we run out of moves for both sides
    while (blackIndex < blackMoves.length || whiteIndex < whiteMoves.length) {
        // Place black stone (right side) if it's Black's turn
        if (blackIndex < blackMoves.length && game.current_player === Stone.Black) {
            const [row, col] = blackMoves[blackIndex];
            // Double-check it's on the right side (cols 10-18)
            if (col >= 10 && col < 19) {
                if (game.place_stone(row, col)) {
                    successCount++;
                    blackCount++;
                    blackIndex++;
                } else {
                    console.warn(`Failed to place black stone at (${row}, ${col})`);
                    blackIndex++; // Skip this move and try next
                }
            } else {
                console.warn(`Black stone at wrong position: (${row}, ${col}) - skipping`);
                blackIndex++; // Skip invalid position
            }
        }
        
        // Place white stone (left side) if it's White's turn
        if (whiteIndex < whiteMoves.length && game.current_player === Stone.White) {
            const [row, col] = whiteMoves[whiteIndex];
            // Double-check it's on the left side (cols 0-9)
            if (col < 10) {
                if (game.place_stone(row, col)) {
                    successCount++;
                    whiteCount++;
                    whiteIndex++;
                } else {
                    console.warn(`Failed to place white stone at (${row}, ${col})`);
                    whiteIndex++; // Skip this move and try next
                }
            } else {
                console.warn(`White stone at wrong position: (${row}, ${col}) - skipping`);
                whiteIndex++; // Skip invalid position
            }
        }
        
        // Safety check: if we can't place either stone, break to avoid infinite loop
        if ((blackIndex >= blackMoves.length || game.current_player !== Stone.Black) &&
            (whiteIndex >= whiteMoves.length || game.current_player !== Stone.White)) {
            console.warn('Cannot place any more stones - breaking loop');
            break;
        }
    }
    
    // Render and update
    renderBoard();
    updateUI();
    
    console.log(`✓ Guānzǐ (官子) endgame scenario ready!`);
    console.log(`  - ${successCount} stones placed successfully`);
    console.log(`  - White stones (left side): ~${whiteCount}`);
    console.log(`  - Black stones (right side): ~${blackCount}`);
    console.log(`  - Board coverage: ~${Math.round((successCount / 361) * 100)}%`);
    console.log(`  - Current player: ${game.current_player === Stone.Black ? 'Black' : 'White'}`);
    console.log(`  - Middle area (cols 9-10) is mostly empty for endgame moves (官子)`);
    console.log(`  - Try playing small moves to secure territory!`);
}

// Export: Make test function available globally for console access
window.testGuanzi = testGuanzi;

// Start: Initialize the game when page loads
run().catch(console.error);