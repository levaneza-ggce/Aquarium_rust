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
    v_speed: f64,
    direction: i32,   // 1 for right, -1 for left
    v_direction: i32, // 1 for down, -1 for up
    color: Color,
    fish_type: usize,
}

struct Shark {
    x: f64,
    y: f64,
    speed: f64,
    v_speed: f64,
    direction: i32,
    v_direction: i32,
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
    sharks: Vec<Shark>,
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

// Simple ASCII Shark
const SHARK_SPRITE: &str = r"____/^\____<"; 

const COLORS: &[Color] = &[
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
    Color::White,
];

// --- Implementation ---

impl Shark {
    fn new(w: u16, h: u16) -> Self {
        let mut rng = rand::thread_rng();
        Shark {
            x: rng.gen_range(1.0..(w as f64 - 15.0)),
            y: rng.gen_range(1.0..(h as f64 - 5.0)),
            speed: rng.gen_range(0.4..0.8), // Sharks are generally faster
            v_speed: rng.gen_range(0.05..0.2),
            direction: if rng.gen_bool(0.5) { 1 } else { -1 },
            v_direction: if rng.gen_bool(0.5) { 1 } else { -1 },
        }
    }

    fn update(&mut self, w: u16, h: u16) {
        self.x += self.speed * self.direction as f64;
        self.y += self.v_speed * self.v_direction as f64;

        let sprite_len = SHARK_SPRITE.len() as f64;

        // Bounce horizontal
        if self.x <= 1.0 {
            self.direction = 1;
            self.x = 1.0;
        } else if self.x + sprite_len >= w as f64 {
            self.direction = -1;
            self.x = (w as f64) - sprite_len;
        }

        // Bounce vertical
        if self.y <= 2.0 {
            self.v_direction = 1;
            self.y = 2.0;
        } else if self.y >= (h as f64 - 3.0) {
            self.v_direction = -1;
            self.y = (h as f64) - 3.0;
        }

        // Randomly change vertical direction less frequently
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.01) {
            self.v_direction *= -1;
        }
    }

    fn draw<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let sprite = SHARK_SPRITE;
        
        // Flip sprite if moving right (default sprite faces left)
        let final_sprite = if self.direction == 1 {
             sprite.chars().rev().map(|c| {
                match c {
                    '<' => '>',
                    '>' => '<',
                    '/' => '\\',
                    '\\' => '/',
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
        out.queue(SetForegroundColor(Color::DarkGrey))?;
        out.queue(Print(final_sprite))?;
        Ok(())
    }
}

impl Fish {
    fn new(w: u16, h: u16) -> Self {
        let mut rng = rand::thread_rng();
        Fish {
            x: rng.gen_range(1.0..(w as f64 - 10.0)),
            y: rng.gen_range(1.0..(h as f64 - 2.0)),
            speed: rng.gen_range(0.2..0.7),
            v_speed: rng.gen_range(0.05..0.2), 
            direction: if rng.gen_bool(0.5) { 1 } else { -1 },
            v_direction: if rng.gen_bool(0.5) { 1 } else { -1 },
            color: COLORS[rng.gen_range(0..COLORS.len())],
            fish_type: rng.gen_range(0..FISH_SPRITES.len()),
        }
    }

    fn update(&mut self, w: u16, h: u16) {
        self.x += self.speed * self.direction as f64;
        self.y += self.v_speed * self.v_direction as f64;

        let sprite_len = FISH_SPRITES[self.fish_type][0].len() as f64;
        
        if self.x <= 1.0 {
            self.direction = 1;
            self.x = 1.0;
        } else if self.x + sprite_len >= w as f64 {
            self.direction = -1;
            self.x = (w as f64) - sprite_len;
        }

        if self.y <= 1.0 {
            self.v_direction = 1; 
            self.y = 1.0;
        } else if self.y >= (h as f64 - 2.0) {
            self.v_direction = -1; 
            self.y = (h as f64) - 2.0;
        }

        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.02) {
            self.v_direction *= -1;
        }
    }

    fn draw<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let sprite = FISH_SPRITES[self.fish_type][0];
        
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
            y: h as f64 - 1.0, 
            speed: rng.gen_range(0.1..0.4),
        }
    }

    fn update(&mut self) {
        self.y -= self.speed;
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
    
    // Initial Spawn
    let mut fishes: Vec<Fish> = (0..30).map(|_| Fish::new(cols, rows)).collect();
    let mut sharks: Vec<Shark> = (0..2).map(|_| Shark::new(cols, rows)).collect(); // 2 Sharks
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

        // --- Logic Updates ---

        // Update Fishes
        for fish in &mut fishes {
            fish.update(cols, rows);
        }

        // Update Sharks
        for shark in &mut sharks {
            shark.update(cols, rows);
        }

        // Update Bubbles
        if rng.gen_bool(0.2) {
            bubbles.push(Bubble::new(cols, rows));
        }
        for bubble in &mut bubbles {
            bubble.update();
        }
        bubbles.retain(|b| b.y > 1.0);

        // --- Collision Logic (Shark Eats Fish) ---
        for shark in &sharks {
            fishes.retain(|fish| {
                let dx = (fish.x - shark.x).abs();
                let dy = (fish.y - shark.y).abs();
                
                // If collision happens (Shark is big, so tolerance is higher)
                // Shark mouth is near the front. Shark is ~12 chars wide.
                // Simple box collision
                let collision = dx < 8.0 && dy < 2.0;
                
                !collision // Keep fish if NO collision
            });
        }
        
        // Optional: Respawn fish if population gets too low
        if fishes.len() < 5 {
             fishes.push(Fish::new(cols, rows));
        }


        // --- Render ---
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

        for shark in &sharks {
            shark.draw(&mut stdout)?;
        }

        stdout.queue(ResetColor)?;
        stdout.flush()?;
    }

    // Cleanup
    stdout.execute(Show)?;
    stdout.execute(Clear(ClearType::All))?;
    disable_raw_mode()?;
    println!("Goodbye!");
    Ok(())
}
