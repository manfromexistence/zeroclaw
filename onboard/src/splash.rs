//! ASCII art splash screen with rainbow colors

use crate::effects::RainbowEffect;
use figlet_rs::FIGfont;
use owo_colors::OwoColorize;
use terminal_size::{Width, Height, terminal_size};
use rand;
use std::io::{self, Write};

pub fn render_dx_logo(rainbow: &RainbowEffect) -> io::Result<()> {
    // Pick a random font for DX title each time
    let all_fonts = get_valid_fonts();
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    let selected_font = all_fonts.choose(&mut rng).unwrap_or(&"Block");

    // Render DX with the randomly selected font
    let dx_figlet_lines = if let Ok(font_data) = dx_font::figlet::read_font(selected_font)
        && let Ok(font_str) = String::from_utf8(font_data)
        && let Ok(font) = FIGfont::from_content(&font_str)
        && let Some(figure) = font.convert("DX")
    {
        figure.to_string().lines().map(|s| s.to_string()).collect()
    } else {
        // Fallback ASCII art
        vec![
            "██████╗ ██╗  ██╗".to_string(),
            "██╔══██╗╚██╗██╔╝".to_string(),
            "██║  ██║ ╚███╔╝ ".to_string(),
            "██║  ██║ ██╔██╗ ".to_string(),
            "██████╔╝██╔╝ ██╗".to_string(),
            "╚═════╝ ╚═╝  ╚═╝".to_string(),
        ]
    };

    // Render DX title with rainbow colors
    for (line_idx, line) in dx_figlet_lines.iter().enumerate() {
        for (char_idx, ch) in line.chars().enumerate() {
            let color_idx = char_idx + line_idx * 5;
            let color = rainbow.color_at(color_idx);
            print!("{}", ch.to_string().truecolor(color.r, color.g, color.b));
        }
        println!();
    }

    println!();

    // Description text at the bottom with rainbow colors
    let description = "Enhanced Development Experience";
    for (char_idx, ch) in description.chars().enumerate() {
        let color_idx = char_idx + 50; // Different offset for description
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
    let smoke_frame_idx = ((elapsed_ms / 300) as usize) % smoke_frames.len();
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
                        let color_idx = (ci + (elapsed_ms / 200) as usize) % 50;
                        let color = rainbow.color_at(color_idx);
                        print!("{}", ch.to_string().truecolor(color.r, color.g, color.b));
                    }
                }
            } else {
                let visible_start = (-smoke_x_offset) as usize;
                if visible_start < smoke_line.len() {
                    for (ci, ch) in smoke_line[visible_start..].chars().enumerate() {
                        if ci < terminal_width {
                            let color_idx = (ci + visible_start + (elapsed_ms / 200) as usize) % 50;
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
                        let color_idx = (char_idx + line_idx * 3 + (elapsed_ms / 150) as usize) % 50;
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
                            let color_idx = (char_idx + visible_start + line_idx * 3 + (elapsed_ms / 150) as usize) % 50;
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
        let ch = if (x + (elapsed_ms / 300) as usize) % 4 == 0 { '╫' } else { '═' };
        let color_idx = (x + (elapsed_ms / 300) as usize) % 50;
        let color = rainbow.color_at(color_idx);
        print!("{}", ch.to_string().truecolor(color.r, color.g, color.b));
    }
    println!();

    io::stdout().flush()?;
    Ok(())
}

fn get_valid_fonts() -> Vec<&'static str> {
    vec![
        // Fonts verified to work with figlet-rs
        "Block",
        "Colossal", 
        "Banner3",
        "Doom",
        "Epic",
        "Graffiti",
        "Isometric1",
        "Isometric2",
        "Ogre",
        "Slant",
        "Shadow",
        "3d",
        "Broadway",
        "Chunky",
        "Cyberlarge",
        "Doh",
        "Gothic",
        "Graceful",
        "Gradient",
        "Hollywood",
        "Lean",
        "Mini",
        "Rounded",
        "Small",
        "Speed",
        "Stellar",
        "Thick",
        "Thin",
        "ansi_shadow",
        "big_chief",
        "banner3_d",
        "Bloody",
        "Bolger",
        "Braced",
        "Bright",
        "Bulbhead",
        "Caligraphy",
        "Cards",
        "Catwalk",
        "Computer",
        "Contrast",
        "Crawford",
        "Cricket",
        "Cursive",
        "Cybersmall",
        "Cygnet",
        "DANC4",
        "Decimal",
        "Diamond",
        "Double",
        "Electronic",
        "Elite",
        "Fender",
        "Fraktur",
        "Fuzzy",
        "Goofy",
        "Hex",
        "Invita",
        "Italic",
        "Jazmine",
        "Jerusalem",
        "Katakana",
        "Keyboard",
        "LCD",
        "Letters",
        "Linux",
        "Madrid",
        "Marquee",
        "Mike",
        "Mirror",
        "Mnemonic",
        "Moscow1",
        "NScript",
        "Nancyj",
        "O8",
        "OS2",
        "Octal",
        "Pawp",
        "Peaks",
        "Pebbles",
        "Pepper",
        "Poison",
        "Puffy",
        "Puzzle",
        "Rectangles",
        "Relief",
        "Relief2",
        "Reverse",
        "Roman",
        "Rozzo",
        "Runic",
        "Script",
        "Serifcap",
        "Shimrod",
        "Short",
        "Slide",
        "Stacey",
        "Stampate",
        "Stop",
        "Straight",
        "Swan",
        "THIS",
        "Tanja",
        "Tengwar",
        "Test1",
        "Ticks",
        "Tiles",
        "Tombstone",
        "Trek",
        "Tubular",
        "Univers",
        "Weird",
        "Whimsy",
    ]
}