let canvas = document.getElementById('pong');
let score = document.getElementById('score');

// For now, initialize to default
let grid = [];
for(let i = 0; i < 30 * 30; ++i) {
  if(i % 30 < 15) {
    grid[i] = 1;
  } else {
    grid[i] = 0;
  }
}

let balls = [
  [7, 14],
  [22, 14],
];
let ballVels = [
  [1, -1],
  [-1, 1],
];

function update() {
  for(let ball = 0; ball < 2; ++ball) {
    let pos = balls[ball];
    let newPos = [
      pos[0] + ballVels[ball][0],
      pos[1] + ballVels[ball][1],
    ];
    let edgeX = false;
    let edgeY = false;
    let hitX = false;
    let hitY = false;
    if(newPos[0] < 0 || newPos[0] >= 30) {
      hitX = true;
      edgeX = true;
    } else if(grid[pos[1] * 30 + newPos[0]] === ball) {
      grid[pos[1] * 30 + newPos[0]] = 1 - ball;
      hitX = true;
    }
    if(newPos[1] < 0 || newPos[1] >= 30) {
      hitY = true;
      edgeY = true;
    } else if(grid[newPos[1] * 30 + pos[0]] === ball) {
      grid[newPos[1] * 30 + pos[0]] = 1 - ball;
      hitY = true;
    }

    if(!edgeX && !edgeY) {
      if(grid[newPos[1] * 30 + newPos[0]] === ball) {
        grid[newPos[1] * 30 + newPos[0]] = 1 - ball;
        hitX = true;
        //hitY = true;
      }
    }

    if(hitX) {
      ballVels[ball][0] *= -1;
    }
    if(hitY) {
      ballVels[ball][1] *= -1;
    }

    balls[ball] = [
      pos[0] + ballVels[ball][0],
      pos[1] + ballVels[ball][1],
    ];
  }
}
setInterval(update, 20);

function draw() {
  let counts = [0, 0];
  let ctx = canvas.getContext('2d');

  for(let color = 0; color < 2; ++color) {
    if(color === 0) {
      ctx.fillStyle = '#007700';
    } else {
      ctx.fillStyle = '#77bb77';
    }

    for(let y = 0; y < 30; ++y) {
      for(let x = 0; x < 30; ++x) {
        if(grid[y * 30 + x] === color) {
          ctx.fillRect(30 * x, 30 * y, 30, 30);
          counts[color] += 1;
        }
      }
    }
  }

  ctx.fillStyle = '#007700';
  ctx.fillRect(30 * balls[0][0], 30 * balls[0][1], 30, 30);
  ctx.fillStyle = '#77bb77';
  ctx.fillRect(30 * balls[1][0], 30 * balls[1][1], 30, 30);

  score.innerText = 'day ' + counts[0] + ' | night ' + counts[1];

  window.requestAnimationFrame(draw);
}
draw();