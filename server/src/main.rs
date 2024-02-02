use std::fs::File;
use std::io::{Read, Write};

struct GameState {
    grid: Vec<u8>,
    balls: [[i8; 2]; 2],
    ballVels: [[i8; 2]; 2],
}

fn update(game: &mut GameState) {
    let grid = &mut game.grid;
    let balls = &mut game.balls;
    let ballVels = &mut game.ballVels;

    for ball in 0..2 {
        let pos = balls[ball];
        let newPos = [
            pos[0] + ballVels[ball][0],
            pos[1] + ballVels[ball][1],
        ];
        let mut edgeX = false;
        let mut edgeY = false;
        let mut hitX = false;
        let mut hitY = false;
        if newPos[0] < 0 || newPos[0] >= 30 {
            hitX = true;
            edgeX = true;
        } else if grid[pos[1] as usize * 30 + newPos[0] as usize] as usize == ball {
            grid[pos[1] as usize * 30 + newPos[0] as usize] = (1 - ball) as u8;
            hitX = true;
        }
        if newPos[1] < 0 || newPos[1] >= 30 {
            hitY = true;
            edgeY = true;
        } else if grid[newPos[1] as usize * 30 + pos[0] as usize] as usize == ball {
            grid[newPos[1] as usize * 30 + pos[0] as usize] = (1 - ball) as u8;
            hitY = true;
        }

        if !edgeX && !edgeY {
            if grid[newPos[1] as usize * 30 + newPos[0] as usize] as usize == ball {
                grid[newPos[1] as usize * 30 + newPos[0] as usize] = (1 - ball) as u8;
                hitX = true;
                //hitY = true;
            }
        }

        if hitX {
            ballVels[ball][0] *= -1;
        }
        if hitY {
            ballVels[ball][1] *= -1;
        }

        balls[ball] = [
            pos[0] + ballVels[ball][0],
            pos[1] + ballVels[ball][1],
        ];
    }
}

fn load_state<R: Read>(mut reader: R) -> std::io::Result<GameState> {
    let mut grid = vec![0u8; 30 * 30];
    let mut b = 0;
    for i in 0..(30 * 30) {
        // Read a byte from the array every 8 bit
        if i % 8 == 0 {
            let mut buf = [0u8; 1];
            reader.read_exact(&mut buf)?;
            b = buf[0];
        }

        // Read a bit into the grid
        grid[i] = b >> 7;
        b = (b << 1) & 0xFF;
    }

    Ok(GameState {
        grid,
        balls: [
            [7, 14],
            [22, 14],
        ],
        ballVels: [
            [1, -1],
            [-1, 1],
        ],
    })
}

fn save_state<W: Write>(mut writer: W, game: &GameState) -> std::io::Result<()> {
    let mut b = 0;
    for i in 0..(30 * 30) {
        if i > 0 && i % 8 == 0 {
            writer.write_all(&[b])?;
            b = 0;
        }
        b = (b << 1) | game.grid[i];
    }
    if 30 * 30 % 8 != 0 {
        writer.write_all(&[b])?;
    }
    Ok(())
}

fn main() {
    // Get game
    let game = match File::open("game.bin") {
        Ok(file) => {
            // Load last checkpoint
            load_state(file).expect("reading saved game")
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Create new game
            eprintln!("initializing new game");
            let mut grid = vec![0; 30 * 30];
            for i in 0..(30 * 30) {
                if i % 30 < 15 {
                    grid[i] = 1;
                } else {
                    grid[i] = 0;
                }
            }
            let game = GameState {
                grid,
                balls: [
                    [7, 14],
                    [22, 14],
                ],
                ballVels: [
                    [1, -1],
                    [-1, 1],
                ],
            };

            // Save it immediately
            let save_to = File::create("game.bin").expect("saving game");
            save_state(save_to, &game).expect("saving game");

            game
        }
        Err(e) => {
            panic!("reading saved game: {}", e);
        }
    };
}
