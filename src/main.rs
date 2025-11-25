use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size},
    ExecutableCommand, QueueableCommand,
};
use rand::Rng;
use std::{
    io::{stdout, Write},
    time::Duration,
    thread,
};

// --- Structs ---

struct Fish {
    x: f64,
    y: f64,
    speed: f64,
    direction: i32, // 1 for right, -1 for left
    color: Color,
    fish_type: usize,
}

struct Bubble {
    x: f64,
    y: f64,
    speed: f64,
}

struct Aquarium {
    width: u16,
    height: u16,
    fishes: Vec<Fish>,
    bubbles: Vec<Bubble>,
}

// --- Constants & ASCII Art ---

const FISH_SPRITES: &[&[&str]] = &[
    &["><>"],                 // Small fish
    &["<°)))><"],             // Long fish
    &["(Q)"],                 // Puffer fish logic (simplified)
];

const COLORS: &[Color] = &[
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
];

// --- Implementation ---

impl Fish {
    fn new(w: u16, h: u16) -> Self {
        let mut rng = rand::thread_rng();
        Fish {
            x: rng.gen_range(1.0..(w as f64 - 5.0)),
            y: rng.gen_range(1.0..(h as f64 - 2.0)),
            speed: rng.gen_range(0.2..0.6),
            direction: if rng.gen_bool(0.5) { 1 } else { -1 },
            color: COLORS[rng.gen_range(0..COLORS.len())],
            fish_type: rng.gen_range(0..FISH_SPRITES.len()),
        }
    }

    fn update(&mut self, w: u16) {
        self.x += self.speed * self.direction as f64;

        // Bounce off walls
        let sprite_len = FISH_SPRITES[self.fish_type][0].len() as f64;
        
        if self.x <= 1.0 {
            self.direction = 1;
            self.x = 1.0;
        } else if self.x + sprite_len >= w as f64 {
            self.direction = -1;
            self.x = (w as f64) - sprite_len;
        }
    }

    fn draw<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let sprite = FISH_SPRITES[self.fish_type][0];
        
        // Flip sprite if moving left
        let final_sprite = if self.direction == -1 {
            sprite.chars().rev().map(|c| {
                match c {
                    '<' => '>',
                    '>' => '<',
                    '(' => ')',
                    ')' => '(',
                    '{' => '}',
                    '}' => '{',
                    '[' => ']',
                    ']' => '[',
                    _ => c
                }
            }).collect::<String>()
        } else {
            sprite.to_string()
        };

        out.queue(MoveTo(self.x as u16, self.y as u16))?;
        out.queue(SetForegroundColor(self.color))?;
        out.queue(Print(final_sprite))?;
        Ok(())
    }
}

impl Bubble {
    fn new(w: u16, h: u16) -> Self {
        let mut rng = rand::thread_rng();
        Bubble {
            x: rng.gen_range(1.0..(w as f64 - 1.0)),
            y: h as f64 - 1.0, // Start at bottom
            speed: rng.gen_range(0.1..0.3),
        }
    }

    fn update(&mut self) {
        self.y -= self.speed;
        // Wiggle effect
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.3) {
            self.x += rng.gen_range(-0.5..0.5);
        }
    }

    fn draw<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        out.queue(MoveTo(self.x as u16, self.y as u16))?;
        out.queue(SetForegroundColor(Color::White))?;
        out.queue(Print("."))?;
        Ok(())
    }
}

// --- Main Loop ---

fn main() -> std::io::Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(Hide)?;
    stdout.execute(Clear(ClearType::All))?;

    let (mut cols, mut rows) = size()?;
    let mut fishes: Vec<Fish> = (0..10).map(|_| Fish::new(cols, rows)).collect();
    let mut bubbles: Vec<Bubble> = Vec::new();
    let mut rng = rand::thread_rng();

    // Game Loop
    loop {
        // Handle resizing and input
        if poll(Duration::from_millis(50))? {
            match read()? {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('q') || event.code == KeyCode::Esc {
                        break;
                    }
                }
                Event::Resize(w, h) => {
                    cols = w;
                    rows = h;
                    stdout.execute(Clear(ClearType::All))?;
                }
                _ => {}
            }
        }

        // Logic Updates
        for fish in &mut fishes {
            fish.update(cols);
        }

        // Add new bubbles randomly
        if rng.gen_bool(0.1) {
            bubbles.push(Bubble::new(cols, rows));
        }

        // Update bubbles and remove those that hit the surface
        for bubble in &mut bubbles {
            bubble.update();
        }
        bubbles.retain(|b| b.y > 1.0);

        // Render
        // We optimize by not clearing the whole screen every frame, 
        // but for simplicity in this demo, we clear to avoid trails.
        // A robust solution would clear only previous positions.
        stdout.queue(Clear(ClearType::All))?;

        // Draw Water Background (Simple blue tint bottom line)
        stdout.queue(MoveTo(0, rows-1))?;
        stdout.queue(SetForegroundColor(Color::DarkBlue))?;
        stdout.queue(Print("~".repeat(cols as usize)))?;

        for bubble in &bubbles {
            bubble.draw(&mut stdout)?;
        }
        
        for fish in &fishes {
            fish.draw(&mut stdout)?;
        }

        stdout.queue(ResetColor)?;
        stdout.flush()?;
    }

    // Cleanup
    stdout.execute(Show)?;
    stdout.execute(Clear(ClearType::All))?;
    disable_raw_mode()?;
    println!("Goodbye! Thanks for visiting the aquarium.");
    Ok(())
}
