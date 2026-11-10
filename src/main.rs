use crossterm::{
    cursor::{Hide, MoveTo},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor, Print},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,QueueableCommand,
};
use rand::Rng;
use std::{
    error::Error,
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

const MAX_X: u16 = 78;
const MAX_Y: u16 = 22;

struct Enemy {
    name: &'static str,
    color: Color,
    active: (bool,u16, u16),
}

impl Enemy {
    fn new(name: &'static str, x: u16, y: u16, color: Color) -> Self {
        Self {
            name,
            color,
            active: (true,x, y),
        }
    }

    fn move_enemy(&mut self) {
        if !self.active.0 {
            return;
        }

        self.active.1 = (self.active.1 as i16 - 1).clamp(1, MAX_X as i16 - 2) as u16;

        if self.active.1 < 2 {
            self.active.0 = false;
        }
    }
    fn check_collision(&self, px: u16, py: u16) -> bool {
        self.active.0 && self.active.1 == px && self.active.2 == py
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    stdout.execute(Hide)?;
    terminal::enable_raw_mode()?;

    execute!(stdout, terminal::SetSize(MAX_X + 1, MAX_Y + 1))?;
    stdout.queue(Clear(ClearType::All)).unwrap();
    let label = b"Welcome to Leap Of Legend";
    let label2 = b"\nClick (Space bar) to Start!";
    let label3 = b"\nClick (q) to Quit!";
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.write(label).unwrap();
    stdout.write(label2).unwrap();
    stdout.write(label3).unwrap();
    stdout.flush().unwrap();
    stdout.flush()?;

    while let Event::Key(KeyEvent { code, .. }) = event::read()? {
        match code {
            KeyCode::Char(' ') => {
                break;
            }
            KeyCode::Char('q') => {
                std::process::exit(0); 
            }
            _ => {}
        }
    }
    let mut highscore = 0;


    loop {
        let mut score = 0;
        let mut enemies: Vec<Enemy> = Vec::new();
        let mut rng = rand::thread_rng();

        let  pos_x = 4;
        let mut pos_y = MAX_Y - 2;
        let mut game_over = false;

        // Game loop
        while !game_over {
            stdout.execute(Clear(ClearType::All))?;

            //enemies
            let num_enemies = 1 as usize;
            let spacing = 2;
            let start_y = 2;

            for i in 0..num_enemies {
                let x = MAX_X - (i as u16 * spacing);
                let y = start_y + (rng.gen_range(0..12) * 2);
                if x > 1 && y < MAX_Y - 1 {
                    enemies.push(Enemy::new("██", x, y, Color::Red));
                }
            }

            // Move enemies and check for collisions
            let mut new_enemies = Vec::new();
            let mut game_over_flag = false;

            for mut enemy in enemies.drain(..) {
                enemy.move_enemy();
                if enemy.check_collision(pos_x, pos_y) {
                    game_over_flag = true;
                    execute!(stdout, Print("\x07"))?; // Beep sound
                } else if enemy.active.0 {
                    new_enemies.push(enemy);
                }
            }

            enemies = new_enemies;
            game_over = game_over_flag;

            // Show score
            show_character(MAX_X - 15, 0, &format!("Score: {}", score), Color::White, Color::Red);

            // Show highscore
            show_character(0, 0, &format!("Highscore: {}", highscore), Color::White, Color::Red);

            // Show player character
            show_character(pos_x, pos_y, "Aa", Color::Green, Color::White);

            // Display active enemies
            for enemy in enemies.iter() {
                show_character(enemy.active.1, enemy.active.2, enemy.name, enemy.color, Color::Black);
            }

            // Display ground
            for i in 0..MAX_X {
                if i % 2 == 0 {
                    show_character(i, MAX_Y - 1, "#", Color::Black, Color::White);
                } else {
                    show_character(i, MAX_Y - 1, " ", Color::Black, Color::Black);
                }
            }

            score += 10;

            if let Ok(true) = event::poll(Duration::from_millis(50)) {
                if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
                    match code {
                        KeyCode::Char('w') => {
                            if pos_y >= 3 {
                                pos_y -= 1;
                            }
                        }
                        KeyCode::Char('s') => {
                            if pos_y < MAX_Y - 2 {
                                pos_y += 1;
                            }
                        }
                        KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }

            sleep(Duration::from_millis(100));
        }

        // Update highscore
        if score > highscore {
            highscore = score;
        }

        stdout.execute(Clear(ClearType::All))?;
        println!("\nGame Over!\nFinal Score: {}", score);
        println!("High Score: {}", highscore);
        println!("Press 'r' to retry or 'q' to quit.");

        //Retry or quit
        loop {
            if let Ok(true) = event::poll(Duration::from_millis(50)) {
                if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
                    match code {
                        KeyCode::Char('r') => break, 
                        KeyCode::Char('q') => return Ok(()),
                        _ => {}
                    }
                }
            }
        }
    }
}

fn show_character(x: u16, y: u16, character: &str, color: Color, bg: Color) {
    let mut stdout = stdout();
    stdout.execute(MoveTo(x, y)).unwrap();
    stdout.execute(SetForegroundColor(color)).unwrap();
    stdout.execute(SetBackgroundColor(bg)).unwrap();
    print!("{}", character);
    stdout.execute(ResetColor).unwrap();
}
