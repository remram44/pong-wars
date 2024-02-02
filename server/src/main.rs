use std::fs::File;
use std::io::{Read, Write};
use std::time::{Duration, SystemTime};

const MOVE_INTERVAL: Duration = Duration::from_millis(20); // 50 times per second
const SAVE_INTERVAL: Duration = Duration::from_millis(30_000); // every 30 seconds

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

fn load_state<R: Read>(mut reader: R) -> std::io::Result<(GameState, SystemTime)> {
    let mut header = [0u8; 16];
    reader.read_exact(&mut header)?;

    // Read timestamp
    let time_secs =
        header[0] as u64
        + (header[1] as u64) << 8
        + (header[2] as u64) << 16
        + (header[3] as u64) << 24;
    let time_nanos =
        header[4] as u32
        + (header[5] as u32) << 8
        + (header[6] as u32) << 16
        + (header[7] as u32) << 24;
    let time = SystemTime::UNIX_EPOCH + Duration::new(time_secs, time_nanos);

    // Read ball positions
    let balls = [
        [header[8] as i8, header[9] as i8],
        [header[10] as i8, header[11] as i8],
    ];

    // Read ball velocities
    let ballVels = [
        [header[12] as i8, header[13] as i8],
        [header[14] as i8, header[15] as i8],
    ];

    // Read grid
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

    Ok((
        GameState {
            grid,
            balls,
            ballVels,
        },
        time,
    ))
}

fn save_state<W: Write>(mut writer: W, game: &GameState) -> std::io::Result<()> {
    // Write current timestamp
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let time_secs = time.as_secs();
    let time_nanos = time.subsec_nanos();
    writer.write_all(&[
        time_secs as u8,
        (time_secs >> 8) as u8,
        (time_secs >> 16) as u8,
        (time_secs >> 24) as u8,
    ])?;
    writer.write_all(&[
        time_nanos as u8,
        (time_nanos >> 8) as u8,
        (time_nanos >> 16) as u8,
        (time_nanos >> 24) as u8,
    ])?;

    // Write ball positions
    writer.write_all(&[game.balls[0][0] as u8])?;
    writer.write_all(&[game.balls[0][1] as u8])?;
    writer.write_all(&[game.balls[1][0] as u8])?;
    writer.write_all(&[game.balls[1][1] as u8])?;

    // Write ball velocities
    writer.write_all(&[game.ballVels[0][0] as u8])?;
    writer.write_all(&[game.ballVels[0][1] as u8])?;
    writer.write_all(&[game.ballVels[1][0] as u8])?;
    writer.write_all(&[game.ballVels[1][1] as u8])?;

    // Write grid
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
    let (mut game, mut last_update) = match File::open("game.bin") {
        Ok(file) => {
            // Load last checkpoint
            eprintln!("loading saved game");
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

            (game, SystemTime::now())
        }
        Err(e) => {
            panic!("reading saved game: {}", e);
        }
    };

    let mut last_save = last_update;

    loop {
        update(&mut game);

        let now = SystemTime::now();

        if last_save + SAVE_INTERVAL <= now {
            eprintln!("saving game");
            let save_to = File::create("game.bin").expect("saving game");
            save_state(save_to, &game).expect("saving game");

            last_save = now;
        }

        last_update = now;
        if last_update + MOVE_INTERVAL > now {
            std::thread::sleep(
                MOVE_INTERVAL
                - now.duration_since(last_update).unwrap()
            );
        }
    }
}
