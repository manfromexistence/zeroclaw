//! ASCII art splash screen with rainbow colors and train animations

use crate::effects::RainbowEffect;
use owo_colors::OwoColorize;
use rand::seq::SliceRandom;
use std::io::{self, Write};
use terminal_size::{Height, Width, terminal_size};

// 10 hardcoded DX logos selected by user
const DX_LOGOS: [&str; 10] = [
    // Bloody
    r#"▓█████▄ ▒██   ██▒
▒██▀ ██▌▒▒ █ █ ▒░
░██   █▌░░  █   ░
░▓█▄   ▌ ░ █ █ ▒ 
░▒████▓ ▒██▒ ▒██▒
 ▒▒▓  ▒ ▒▒ ░ ░▓ ░
 ░ ▒  ▒ ░░   ░▒ ░
 ░ ░  ░  ░    ░  
   ░     ░    ░  
 ░               "#,
    // 3d
    r#" ███████   ██     ██
░██░░░░██ ░░██   ██ 
░██    ░██ ░░██ ██  
░██    ░██  ░░███   
░██    ░██   ██░██  
░██    ██   ██ ░░██ 
░███████   ██   ░░██
░░░░░░░   ░░     ░░ "#,
    // Doh
    r#"                                          
DDDDDDDDDDDDD        XXXXXXX       XXXXXXX
D::::::::::::DDD     X:::::X       X:::::X
D:::::::::::::::DD   X:::::X       X:::::X
DDD:::::DDDDD:::::D  X::::::X     X::::::X
  D:::::D    D:::::D XXX:::::X   X:::::XXX
  D:::::D     D:::::D   X:::::X X:::::X   
  D:::::D     D:::::D    X:::::X:::::X    
  D:::::D     D:::::D     X:::::::::X     
  D:::::D     D:::::D     X:::::::::X     
  D:::::D     D:::::D    X:::::X:::::X    
  D:::::D     D:::::D   X:::::X X:::::X   
  D:::::D    D:::::D XXX:::::X   X:::::XXX
DDD:::::DDDDD:::::D  X::::::X     X::::::X
D:::::::::::::::DD   X:::::X       X:::::X
D::::::::::::DDD     X:::::X       X:::::X
DDDDDDDDDDDDD        XXXXXXX       XXXXXXX"#,
    // Diamond
    r#"/\\\\\    /\\      /\\
/\\   /\\  /\\   /\\  
/\\    /\\  /\\ /\\   
/\\    /\\    /\\     
/\\    /\\  /\\ /\\   
/\\   /\\  /\\   /\\  
/\\\\\    /\\      /\\"#,
    // Electronic
    r#" ▄▄▄▄▄▄▄▄▄▄   ▄       ▄ 
▐░░░░░░░░░░▌ ▐░▌     ▐░▌
▐░█▀▀▀▀▀▀▀█░▌ ▐░▌   ▐░▌ 
▐░▌       ▐░▌  ▐░▌ ▐░▌  
▐░▌       ▐░▌   ▐░▐░▌   
▐░▌       ▐░▌    ▐░▌    
▐░▌       ▐░▌   ▐░▌░▌   
▐░▌       ▐░▌  ▐░▌ ▐░▌  
▐░█▄▄▄▄▄▄▄█░▌ ▐░▌   ▐░▌ 
▐░░░░░░░░░░▌ ▐░▌     ▐░▌
 ▀▀▀▀▀▀▀▀▀▀   ▀       ▀ "#,
    // Fraktur
    r#"       ....                       ..   
   .xH888888Hx.         .H88x.  :~)88: 
 .H8888888888888:      x888888X ~:8888 
 888*"""?""*88888X    ~   "8888X  %88" 
'f     d8x.   ^%88k        X8888       
'>    <88888X   '?8     .xxX8888xxxd>  
 `:..:`888888>    8>   :88888888888"   
        `"*88     X    ~   '8888       
   .xHHhx.."      !   xx.  X8888:    . 
  X88888888hx. ..!   X888  X88888x.x"  
 !   "*888888888"    X88% : '%8888"    
        ^"***"`       "*=~    `""      "#,
    // Marquee
    r#".:::::    .::      .::
.::   .::  .::   .::  
.::    .::  .:: .::   
.::    .::    .::     
.::    .::  .:: .::   
.::   .::  .::   .::  
.:::::    .::      .::"#,
    // Reverse
    r#"====================
=       ===   ==   =
=  ====  ===  ==  ==
=  ====  ===  ==  ==
=  ====  ====    ===
=  ====  =====  ====
=  ====  ====    ===
=  ====  ===  ==  ==
=  ====  ===  ==  ==
=       ===  ====  =
===================="#,
    // Stellar
    r#"`.....    `..      `..
`..   `..  `..   `..  
`..    `..  `.. `..   
`..    `..    `..     
`..    `..  `.. `..   
`..   `..  `..   `..  
`.....    `..      `.."#,
    // Tubular
    r#"O~~~~~    O~~      O~~
O~~   O~~  O~~   O~~  
O~~    O~~  O~~ O~~   
O~~    O~~    O~~     
O~~    O~~  O~~ O~~   
O~~   O~~  O~~   O~~  
O~~~~~    O~~      O~~"#,
];

pub fn render_dx_logo(rainbow: &RainbowEffect) -> io::Result<()> {
    // Pick a random logo from the 10 hardcoded options
    let mut rng = rand::thread_rng();
    let logo = DX_LOGOS.choose(&mut rng).unwrap_or(&DX_LOGOS[0]);

    // Render with rainbow colors
    for (line_idx, line) in logo.lines().enumerate() {
        for (char_idx, ch) in line.chars().enumerate() {
            let color_idx = char_idx + line_idx * 5;
            let color = rainbow.color_at(color_idx);
            print!("{}", ch.to_string().truecolor(color.r, color.g, color.b));
        }
        println!();
    }

    println!();

    // Description text with rainbow colors
    let description = "Enhanced Development Experience";
    for (char_idx, ch) in description.chars().enumerate() {
        let color_idx = char_idx + 50;
        let color = rainbow.color_at(color_idx);
        print!("{}", ch.to_string().truecolor(color.r, color.g, color.b));
    }
    println!();

    io::stdout().flush()?;
    Ok(())
}

pub fn render_train_animation(rainbow: &RainbowEffect, frame: usize) -> io::Result<()> {
    // Use terminal_size crate to get ACTUAL terminal width
    let size = terminal_size();
    let terminal_width = if let Some((Width(w), Height(_h))) = size {
        w as usize
    } else {
        120 // fallback
    };

    let elapsed_ms = frame * 200;
    let train_width = 55;

    // Train moves from right to left across the full terminal width
    let total_travel = terminal_width + train_width + 10;
    let cycle_duration = 3000;
    let progress = (elapsed_ms % cycle_duration) as f32 / cycle_duration as f32;
    let x_pos = (terminal_width as f32 + 10.0 - progress * total_travel as f32) as i32;

    let train = vec![
        "      ====        ________                ___________",
        "  _D _|  |_______/        \\__I_I_____===__|_________|",
        "   |(_)---  |   H\\________/ |   |        =|___ ___|",
        "   /     |  |   H  |  |     |   |         ||_| |_||",
        "  |      |  |   H  |__--------------------| [___] |",
        "  | ________|___H__/__|_____/[][]~\\_______|       |",
        "  |/ |   |-----------I_____I [][] []  D   |=======|",
        "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|",
        " |/-=|___|=O=====O=====O=====O   |_____/~\\___/",
        "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/",
    ];

    // Add smoke that animates above the train
    let smoke_frames: Vec<&[&str]> = vec![
        &["    (  )", "   (    )", "  (      )"],
        &["   (   )", "  (     )", " (       )"],
        &["  (    )", " (      )", "(        )"],
    ];
    let smoke_frame_idx = (elapsed_ms / 300) % smoke_frames.len();
    let smoke = smoke_frames[smoke_frame_idx];

    // Render smoke above the train - positioned relative to train
    let smoke_x_offset = x_pos + 6;
    for smoke_line in smoke {
        // Clear the line first
        print!("{}", " ".repeat(terminal_width));
        print!("\r");

        if smoke_x_offset >= -(smoke_line.len() as i32) && smoke_x_offset < terminal_width as i32 {
            if smoke_x_offset >= 0 {
                print!("{}", " ".repeat(smoke_x_offset as usize));
                for (ci, ch) in smoke_line.chars().enumerate() {
                    if smoke_x_offset as usize + ci < terminal_width {
                        let color_idx = (ci + (elapsed_ms / 200)) % 50;
                        let color = rainbow.color_at(color_idx);
                        print!("{}", ch.to_string().truecolor(color.r, color.g, color.b));
                    }
                }
            } else {
                let visible_start = (-smoke_x_offset) as usize;
                if visible_start < smoke_line.len() {
                    for (ci, ch) in smoke_line[visible_start..].chars().enumerate() {
                        if ci < terminal_width {
                            let color_idx = (ci + visible_start + (elapsed_ms / 200)) % 50;
                            let color = rainbow.color_at(color_idx);
                            print!("{}", ch.to_string().truecolor(color.r, color.g, color.b));
                        }
                    }
                }
            }
        }
        println!();
    }

    // Render train - ensure it uses the full terminal width
    for (line_idx, line) in train.iter().enumerate() {
        // Clear the line first
        print!("{}", " ".repeat(terminal_width));
        print!("\r");

        if x_pos >= -(train_width as i32) && x_pos < terminal_width as i32 {
            if x_pos >= 0 {
                // Train is fully or partially visible from the left
                print!("{}", " ".repeat(x_pos as usize));
                for (char_idx, ch) in line.chars().enumerate() {
                    if x_pos as usize + char_idx < terminal_width {
                        let color_idx = (char_idx + line_idx * 3 + (elapsed_ms / 150)) % 50;
                        let color = rainbow.color_at(color_idx);
                        print!("{}", ch.to_string().truecolor(color.r, color.g, color.b));
                    }
                }
            } else {
                // Train is partially off-screen to the left
                let visible_start = (-x_pos) as usize;
                if visible_start < line.len() {
                    for (char_idx, ch) in line[visible_start..].chars().enumerate() {
                        if char_idx < terminal_width {
                            let color_idx =
                                (char_idx + visible_start + line_idx * 3 + (elapsed_ms / 150)) % 50;
                            let color = rainbow.color_at(color_idx);
                            print!("{}", ch.to_string().truecolor(color.r, color.g, color.b));
                        }
                    }
                }
            }
        }
        println!();
    }

    // Render tracks across full terminal width
    print!("{}", " ".repeat(terminal_width));
    print!("\r");
    for x in 0..terminal_width {
        let ch = if (x + (elapsed_ms / 300)).is_multiple_of(4) {
            '╫'
        } else {
            '═'
        };
        let color_idx = (x + (elapsed_ms / 300)) % 50;
        let color = rainbow.color_at(color_idx);
        print!("{}", ch.to_string().truecolor(color.r, color.g, color.b));
    }
    println!();

    io::stdout().flush()?;
    Ok(())
}
