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
};

// --- Structs ---

struct Fish {
    x: f64,
    y: f64,
    speed: f64,
    v_speed: f64,     // Added vertical speed
    direction: i32,   // 1 for right, -1 for left
    v_direction: i32, // 1 for down, -1 for up
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
    &["><>"],                 // 1. Classic Small
    &["<°)))><"],             // 2. Long
    &["(Q)"],                 // 3. Puffer
    &["><(('>"],              // 4. Medium
    &["<###-<"],              // 5. Skeleton/Striped
    &["*<"],                  // 6. Tiny
];

const COLORS: &[Color] = &[
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
    Color::White, // Added White for variety
];

// --- Implementation ---

impl Fish {
    fn new(w: u16, h: u16) -> Self {
        let mut rng = rand::thread_rng();
        Fish {
            x: rng.gen_range(1.0..(w as f64 - 10.0)),
            y: rng.gen_range(1.0..(h as f64 - 2.0)),
            speed: rng.gen_range(0.2..0.7),
            v_speed: rng.gen_range(0.05..0.2), // Vertical movement is generally slower
            direction: if rng.gen_bool(0.5) { 1 } else { -1 },
            v_direction: if rng.gen_bool(0.5) { 1 } else { -1 },
            color: COLORS[rng.gen_range(0..COLORS.len())],
            fish_type: rng.gen_range(0..FISH_SPRITES.len()),
        }
    }

    fn update(&mut self, w: u16, h: u16) {
        // Horizontal Movement
        self.x += self.speed * self.direction as f64;

        // Bounce off side walls
        let sprite_len = FISH_SPRITES[self.fish_type][0].len() as f64;
        
        if self.x <= 1.0 {
            self.direction = 1;
            self.x = 1.0;
        } else if self.x + sprite_len >= w as f64 {
            self.direction = -1;
            self.x = (w as f64) - sprite_len;
        }

        // Vertical Movement
        self.y += self.v_speed * self.v_direction as f64;

        // Bounce off top and bottom (Surface and Floor)
        // 1.0 is top margin, h-2.0 is bottom margin (leaving room for floor)
        if self.y <= 1.0 {
            self.v_direction = 1; // Go down
            self.y = 1.0;
        } else if self.y >= (h as f64 - 2.0) {
            self.v_direction = -1; // Go up
            self.y = (h as f64) - 2.0;
        }

        // Add a small chance to randomly change vertical direction 
        // This makes movement look more "organic" and less like a DVD screensaver
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.02) {
            self.v_direction *= -1;
        }
    }

    fn draw<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let sprite = FISH_SPRITES[self.fish_type][0];
        
        // Flip sprite if moving left (basic mirroring)
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
            speed: rng.gen_range(0.1..0.4),
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
    
    // INCREASED FISH COUNT HERE: 30 Fish
    let mut fishes: Vec<Fish> = (0..30).map(|_| Fish::new(cols, rows)).collect();
    
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
            fish.update(cols, rows); // Passed 'rows' here for vertical bounds
        }

        // Add new bubbles randomly
        if rng.gen_bool(0.2) { // Increased bubble frequency slightly
            bubbles.push(Bubble::new(cols, rows));
        }

        // Update bubbles and remove those that hit the surface
        for bubble in &mut bubbles {
            bubble.update();
        }
        bubbles.retain(|b| b.y > 1.0);

        // Render
        stdout.queue(Clear(ClearType::All))?;

        // Draw Water Background
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
    println!("Goodbye! Hope you enjoyed the crowded aquarium.");
    Ok(())
}
