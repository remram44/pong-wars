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

fn main() {
}
