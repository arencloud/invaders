use invaders::frame::{fill_starry_background, new_frame, write_text, Drawable, Frame};
use invaders::invaders::Invaders;
use invaders::player::Player;
use invaders::NUM_ROWS;

fn print_frame(frame: &Frame) {
    for y in 0..NUM_ROWS {
        let mut line = String::with_capacity(frame.len());
        for col in frame {
            line.push_str(col[y]);
        }
        println!("{line}");
    }
}

fn main() {
    let mut frame = new_frame();
    fill_starry_background(&mut frame, 42);

    let player = Player::new();
    let invaders = Invaders::new();
    player.draw(&mut frame);
    invaders.draw(&mut frame);

    write_text(&mut frame, 1, 0, "SCORE: 01200  HIGH: 03400  INVADERS: 18");
    write_text(
        &mut frame,
        0,
        NUM_ROWS - 1,
        "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
    );
    write_text(
        &mut frame,
        0,
        NUM_ROWS - 1,
        "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
    );

    print_frame(&frame);
}
