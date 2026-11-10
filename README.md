# Leap of Legend 🎮

A real-time, terminal-based 2D evasion game written in Rust[cite: 4]. Players control an on-screen character, navigating vertically to evade oncoming waves of enemies while accumulating points[cite: 4]. The project demonstrates raw terminal control, non-blocking asynchronous event polling, and game-loop architectures[cite: 4, 9].

---

## 🚀 Key Features

* **Terminal UI Manipulation:** Configures raw terminal mode, custom canvas dimensions (`78x22`), and dynamic cursor positioning using `crossterm`[cite: 4, 5, 6].
* **Real-time Event Polling:** Uses non-blocking input handling via `event::poll` to allow smooth gameplay and responsive movement[cite: 9].
* **Collision Detection:** Computes player-enemy coordinate intersections to trigger game-over states and terminal alerts[cite: 6, 7, 8].
* **State Management:** Tracks current session score alongside dynamic high-score persistence during continuous retry loops[cite: 4, 7, 9, 10].
* **Color & Audio Feedback:** Uses styled foreground/background rendering with colored characters and terminal bell audio alerts on impact[cite: 4, 8, 10].

---

## 🕹️ Controls

| Key | Action |
| :--- | :--- |
| `Space` | Start game from the title screen[cite: 6, 7] |
| `w` | Move character up[cite: 4, 9] |
| `s` | Move character down[cite: 4, 9] |
| `Esc` | Exit active gameplay session[cite: 9] |
| `r` | Retry after Game Over[cite: 4, 10] |
| `q` | Quit program[cite: 4, 6, 10] |

---

## 🛠️ Dependencies

* **[Rust](https://www.rust-lang.org/)** (2021 edition or newer)
* **[crossterm](https://crates.io/crates/crossterm)**: Cross-platform terminal manipulation (cursor, styling, raw mode, event listening)[cite: 4].
* **[rand](https://crates.io/crates/rand)**: Random number generation for dynamic enemy positioning[cite: 4, 7].

Add them to your `Cargo.toml`:

```toml
[dependencies]
crossterm = "0.27"
rand = "0.8"
