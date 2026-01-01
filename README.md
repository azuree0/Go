<img width="963" height="832" alt="G" src="https://github.com/user-attachments/assets/3e4ca094-6a33-4d85-ab28-e4c269a2c1f0" />

<br>

# Prerequisites

- Rust (latest stable version)
wasm-pack, install with: 
```bash
cargo install wasm-pack
```

### Build Steps

1. Build the WebAssembly module:
```bash
wasm-pack build --target web
```

2. Serve the files with a local web server (required for WebAssembly):
```bash
python -m http.server 8000
```

3. Open browser:
```bash
http://localhost:8000
```

<br>

# Game Rules

### Basic

- **Starting Player**: Black plays first, then players alternate turns.
- **Board**: 19×19 grid with 361 intersections.

1. **Place a Stone**: Click on an empty intersection to place your stone (black or white).
2. **Capture**: When you place a stone that removes all liberties (adjacent empty spaces) from an opponent's group, those stones are captured and removed from the board.
3. **End Turn**: After placing a stone, the turn automatically passes to the opponent.

1. **Placement**
   - Players take turns placing stones on empty intersections.
   - Black always moves first.
   - Once placed, stones cannot be moved (except when captured).

2. **Groups and Ki**
   - Stones of the same color that are connected horizontally or vertically form a **group**.
   - **Ki** are empty intersections adjacent to a group (horizontally or vertically).
   - A group must have at least one liberty to remain on the board.

3. **Capture**
   - When a group has no liberties, it is **captured** and removed from the board.
   - Captured stones are counted and displayed in the game interface.
   - You can capture multiple groups in a single move.

# Game Phase

### Opening (Bùjú 布局 / Fuseki 布石 / Poseok 포석)
- **Purpose**: Players establish their initial positions and claim territory on the board.
- **Strategy**: Focus on corner and side positions, as these are easier to secure than the center.
- **Common Patterns**: Players typically play in the corners first (at star points or 3-3, 4-4 points), then extend along the sides.
- **Balance**: The opening sets the framework for the entire game, balancing between claiming territory and maintaining flexibility.
- **Typical Moves**: The first 20-30 moves establish the basic territorial framework and influence patterns.

### Middle (Zhōngpán 中盤 /Chūban 中盤 /  Jungban 중반)
- **Purpose**: Players fight for territory, attack weak groups, and defend their own positions.
- **Strategy**: This is the most complex phase, involving tactical battles, capturing sequences, and strategic decisions about when to fight and when to compromise.
- **Key Elements**: 
  - **Fighting**: Direct confrontations between groups
  - **Invasion**: Entering opponent's territory to reduce their points
  - **Reduction**: Playing moves that reduce opponent's potential territory without full invasion
  - **Connection**: Securing groups and ensuring they have enough liberties
- **Complexity**: The middle game requires reading ahead, calculating capture sequences, and understanding life and death situations.
- **Duration**: Can last from move 30-40 until move 150-200, depending on the game's complexity.

### End (Guānzǐ 官子 / Yose ヨセ / Gwanja 관자)
- **Purpose**: Players secure remaining territory and maximize their final score.
- **Strategy**: Focus shifts from fighting to precise point-gaining moves along boundaries.
- **Key Elements**:
  - **Boundary Plays**: Securing the edges between territories
  - **Sente vs Gote**: Playing moves that force a response (sente) versus moves that allow the opponent to take the initiative (gote)
  - **Counting**: Estimating the score to determine if risky plays are necessary
- **Precision**: Small mistakes in the endgame can cost several points, making accuracy crucial.
- **Typical Duration**: Usually begins around move 150-200 and continues until both players pass, ending the game.

# Victory

- **Scoring**: After both players pass, the game ends and territory is counted.
- **Territory**: Empty intersections surrounded by your stones count as your territory.
- **Winner**: The player with more territory (including captured stones) wins.

<br>

# History

### Three Sovereigns and Five Emperors (c. 2852-2070 BCE)
- **Legendary Origin**: According to Chinese legend, Go (Weiqi 圍棋) was invented by Emperor Yao (堯, c. 2356-2255 BCE), one of the Five Emperors, as a way to teach strategic thinking and wisdom to his son Danzhu.
- **Mythological Significance**: The game's creation is attributed to this legendary period of Chinese prehistory, representing the transmission of knowledge and the cultivation of virtue.
- **Taoist Foundation**: Even in legend, the game embodies early Taoist principles of balance, natural order, and the harmony between opposing forces.

### Xia Dynasty (c. 2070-1600 BCE)
- **Early Development**: During China's first recorded dynasty, Go continued to develop as a strategic and philosophical tool.
- **Cultural Integration**: The game became associated with rulership and governance, used to teach principles of balance and strategic thinking to rulers and officials.

### Shang Dynasty (c. 1600-1046 BCE)
- **Archaeological Evidence**: While direct evidence is limited, the game's principles align with the strategic thinking evident in Shang military and political organization.
- **Oracle Bone Context**: The strategic complexity of Go reflects the sophisticated thinking of the Shang period, where divination and strategic planning were highly developed arts.

### Warring States Period (475-221 BCE)
- **Documented History**: The first reliable historical references to Go appear during this period of intense warfare and philosophical development.
- **Strategic Training**: Go was used as a training method for military strategy and tactical thinking during this era of constant warfare between the seven major states.
- **Social Context**: The game was played by nobles, officials, and military commanders.
- **Board Development**: During this period, the game's rules and board structure became more standardized, moving from various regional variations toward a more unified form.
- **Metaphor**: Historical texts from this period use Go as a metaphor for political strategy, diplomacy, and statecraft.

### Han Dynasty (206 BCE - 220 CE)
- **Popularization**: Go became a popular pastime among the educated elite and was established as one of the "Four Arts" (四藝) of the Chinese scholar, alongside calligraphy, painting, and music.
- **Philosophical Integration**: Taoist scholars used Go as a tool for teaching governance, strategy, and understanding the interconnectedness of all things.
- **Cultural Status**: The game was recognized as a mark of refinement and intellectual cultivation, played by emperors, officials, and scholars.
- **Board Development**: The game's rules and board size began to standardize during this period.

### Three Kingdoms Period (220-280 CE)
- **Strategic Culture**: During this period of warfare and political intrigue, Go was highly valued as a training tool for military strategy and tactical thinking.
- **Literary References**: The game appears in historical records and literature from this period, associated with famous generals and strategists.
- **Cultural Continuity**: Despite political fragmentation, Go maintained its status as a refined art and strategic discipline.
- **Spread to Korea**: Go was introduced to Korea during the Unified Silla period (668–935 CE), where it became known as Baduk (바둑) and became deeply integrated into Korean culture.

### Tang Dynasty (618-907 CE)
- **Golden Age**: Go flourished during this period of cultural and artistic renaissance in China.
- **Literary References**: The game appears frequently in Tang poetry and literature, celebrated for its beauty and strategic depth.
- **Professional Development**: The first systematic study of Go strategy and theory began to emerge, with players developing formal techniques and opening patterns.
- **Imperial Patronage**: Tang emperors and court officials were known to be avid players, further elevating the game's status.
- **Silk Road Spread**: Go spread to Tibet via the Silk Road during the Tang period, as trade and cultural exchange flourished along these routes with Central Asia.
- **Spread to Japan**: Go was introduced to Japan during the Nara period with firm historical records documenting its presence by 735 CE, with cultural transmission very likely began in the late Asuka period.  It became known as Igo (囲碁) and was initially played by the imperial court and envoy to Tang.

### Song Dynasty (960-1279 CE)
- **Theoretical Advancement**: This period saw the development of sophisticated Go theory and the compilation of strategic treatises.
- **Board Standardization**: The 19×19 board became the standard size, replacing various smaller board sizes that had been used previously.
- **Spread to Japan**: Go was introduced to Japan (where it became known as Igo 囲碁) during the Song period, likely through Buddhist monks and cultural exchanges.
- **Artistic Integration**: Go became integrated into Chinese painting, poetry, and philosophy, representing the ideal of the cultured scholar.
- **Go Academies**: Formal schools and academies dedicated to Go study began to emerge.

### Ming Dynasty (1368-1644 CE)
- **Master Players**: This period produced some of China's most celebrated Go masters, whose games and strategies were studied for generations.
- **Strategic Treatises**: Comprehensive Go manuals and strategy books were written, documenting opening patterns, tactical sequences, and positional judgment.
- **Cultural Refinement**: Go was further elevated as an art form, with emphasis on aesthetic beauty in play, proper etiquette, and philosophical understanding.
- **Regional Styles**: Distinct playing styles began to develop in different regions of China, contributing to the game's rich strategic diversity.
- **Go Tournaments**: Organized competitions and matches between masters became more formalized.

### Modern Period (20th Century - Present)
- **Professional Systems**: Modern professional Go systems were established in China, Japan, and Korea, with formal rankings (dan levels), tournaments, and professional players.
- **International Competition**: Go became a vehicle for cultural exchange and competition between East Asian nations, with international tournaments and championships.
- **Spread to the West**: Go was first introduced to Europe through the writings of Engelbert Kaempfer, a German physician and naturalist who traveled to Japan in the late 17th century. His posthumously published work "The History of Japan" (1727) included detailed descriptions of Go (which he called "Go-ban"), marking the first significant Western documentation of the game.

# Symbol

The board design reflects Taoist cosmological & theological symbolism:

- **19×19 Grid**: The board consists of 19 lines in each direction, creating 361 intersections (19² = 361). This structure represents the harmony between heaven and earth, with the central point (tengen, 天元) symbolizing the connection between the celestial and terrestrial realms.

- **Black and White Stones**: The two colors embody the fundamental Taoist principle of Yin and Yang (陰陽):
  - **Black (Yin 陰)**: Represents the receptive, passive, dark, and feminine principle—the earth, water, and the yielding aspect of nature.
  - **White (Yang 陽)**: Represents the creative, active, light, and masculine principle—heaven, fire, and the assertive aspect of nature.
  - Together, they demonstrate that all existence arises from the interplay and balance of these complementary opposites.

- **The Empty Board (無)**: Before play begins, the board is empty, representing Wu (無)—the void, nothingness, or non-being from which all things emerge. This mirrors the Taoist concept that the Tao itself is empty yet contains infinite potential.

- **Star Points (Hoshi 星)**: The nine star points on the board (at intersections 3-3, 3-9, 3-15, 9-3, 9-9, 9-15, 15-3, 15-9, 15-15) represent key cosmological positions:
  - The central star point (9-9, tengen) represents the center of the universe, the axis mundi connecting heaven and earth.
  - The four corner star points represent the four directions and the four seasons.
  - The four side star points represent the intermediate directions and the balance between extremes.

- **Circular Stones**: The round shape of the stones reflects the Taoist concept of the circle as a symbol of completeness, eternity, and the cyclical nature of existence. Like the Tao, the circle has no beginning or end.

- **Balance and Harmony**: The game's objective of surrounding territory while maintaining balance reflects the Taoist ideal of harmony (和)—finding equilibrium between opposing forces, neither too aggressive nor too passive, neither too focused on territory nor too focused on influence.

- **The Way (道)**: Just as the Tao is simple yet profound, Go has simple rules but infinite complexity. Each game is a unique expression of the Way, teaching that profound depth emerges from simplicity and that the best path is often the natural one that follows the flow of the position.

- **Interconnectedness**: The way stones connect to form groups reflects the Taoist understanding that all things are interconnected. A single stone's placement affects the entire board, demonstrating that individual actions are part of a greater whole.

- **Non-Attachment**: Players must learn to let go of captured stones and lost territory, embodying the Taoist principle of non-attachment (無執)—focusing on the whole rather than individual battles, accepting loss as part of the natural flow.

<br>

# Project Structure

```
.
├── Cargo.toml          # Rust project configuration
├── src/
│   └── lib.rs          # Main game logic
├── index.html          # Web interface
├── style.css           # Styling
├── index.js            # JavaScript bindings
├── build.bat           # Windows build script
└── README.md           # This file
```
