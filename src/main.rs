use macroquad::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    // Variables for the players' paddle.
    let mut player: Block = Block {
        x: 0.0,
        y: screen_height() - 40.0,
        width: 150.0,
        height: 30.0,
        color: RED,
    };

    let mut ball: Block = Block {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        width: 30.0,
        height: 30.0,
        color: WHITE,
    };
    let mut ball_vel_x: f32 = 3.0;
    let mut ball_vel_y: f32 = 3.0;

    // Some random variables!
    let mut lives: i8 = 3;

    // Variables for the block generation.
    let color_array: [Color; 5] = [LIME, YELLOW, RED, BLUE, GREEN];
    let mut blocks: Vec<Block> = Vec::new();
    const BLOCKS_X: u128 = 13;
    const BLOCKS_Y: u128 = 8;
    const BLOCK_HEIGHT: f32 = 30.0;
    let mut blocks_alive: i128 = BLOCKS_X as i128 * BLOCKS_Y as i128;

    for y in 0..BLOCKS_Y {
        for x in 0..BLOCKS_X {
            let c: Color = *color_array
                .get(((x + y) % color_array.len() as u128) as usize)
                .unwrap();
            let b: Block = Block {
                x: screen_width() / BLOCKS_X as f32 * x as f32,
                y: BLOCK_HEIGHT * y as f32,
                width: screen_width() / BLOCKS_X as f32,
                height: BLOCK_HEIGHT,
                color: c,
            };
            blocks.push(b);
        }
    }

    // Main loop
    loop {
        // Logic section.
        // Getting the mouse position for the x-axis and setting the player's x coordinate.
        let (x, _): (f32, _) = mouse_position();
        player.x = x - player.width / 2.0;
        // Clamping the players' x so the paddle won't go off the screen.
        if player.x + player.width > screen_width() {
            player.x = screen_width() - player.width;
        } else if player.x < 0.0 {
            player.x = 0.0;
        }
        // Letting the ball move.
        ball.x += ball_vel_x;
        ball.y += ball_vel_y;

        // Letting the ball bounce.
        if ball.x > screen_width() || ball.x < 0.0 {
            ball_vel_x = -ball_vel_x;
        } else if ball.y < 0.0 {
            ball_vel_y = -ball_vel_y;
        }

        // Player with ball collision.
        if player.x < ball.x + ball.width
            && player.x + player.width > ball.x
            && player.y < ball.y + ball.height
            && player.y + player.height > ball.y
        {
            ball_vel_y = -ball_vel_y;
        }
        // Checking if a live is lost.
        if ball.y > screen_height() {
            ball.x = screen_width() / 2.0;
            ball.y = screen_height() / 2.0;
            lives -= 1;
        }

        // Checking for blocks' collision with ball.
        for mut block in blocks.iter_mut() {
            let side: Side = collide(&ball, block);
            if side == Side::Bottom || side == Side::Top {
                ball_vel_y = -ball_vel_y;
            } else if side == Side::Left || side == Side::Right {
                ball_vel_x = -ball_vel_x;
            }

            if side != Side::None {
                // Moves block offscreen.
                block.x = 0.0 - block.width * 2.0;
                // Used for playing sound when block is hit.
                let mut player =
                    winaudio::wave::Player::from_file("assets/sounds/hit.wav").unwrap();
                player.play().unwrap();
            }
        }

        // Render section.
        clear_background(BLACK);
        // Drawing the paddle and the ball.
        draw_rectangle(
            player.x,
            player.y,
            player.width,
            player.height,
            player.color,
        );
        draw_rectangle(ball.x, ball.y, ball.width, ball.height, ball.color);

        // Drawing the blocks
        for block in blocks.iter() {
            draw_rectangle(block.x, block.y, block.width, block.height, block.color);
        }

        // Drawing the amount of lives in the top-left.
        draw_text(&lives.to_string(), 10.0, 10.0, 24.0, WHITE);

        println!("FPS: {}", get_fps());
        next_frame().await
    }
}

struct Block {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
}

#[derive(PartialEq)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
    None,
}

fn collide(r1: &Block, r2: &Block) -> Side {
    let dx = (r1.x + r1.width / 2.0) - (r2.x + r2.width / 2.0);
    let dy = (r1.y + r1.height / 2.0) - (r2.y + r2.height / 2.0);
    let width = (r1.width + r2.width) / 2.0;
    let height = (r1.height + r2.height) / 2.0;
    let cross_width = width * dy;
    let cross_height = height * dx;
    let mut collision: Side = Side::None;
    //
    if dx.abs() <= width && dy.abs() <= height {
        if cross_width > cross_height {
            collision = if cross_width > (-cross_height) {
                Side::Bottom
            } else {
                Side::Left
            };
        } else {
            collision = if cross_width > -(cross_height) {
                Side::Right
            } else {
                Side::Top
            };
        }
    }
    return collision;
}
