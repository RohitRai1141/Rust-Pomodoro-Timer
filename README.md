# 🍅 Pomodoro Timer

A beautiful, terminal-based Pomodoro timer written in Rust with customizable work sessions and break intervals.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)

## ✨ Features

- **Customizable Timers**: Set your own work duration, short break, and long break times
- **Session Tracking**: Configure total number of Pomodoro sessions
- **Big ASCII Timer Display**: Large, easy-to-read countdown timer
- **Break Prompts**: Manual break start - timer asks when you're ready
- **Desktop Notifications**: Get notified when sessions complete
- **Sound Alerts**: Audio notification on session transitions
- **Keyboard Controls**: Full keyboard navigation and control
- **Pause/Resume**: Pause and resume timers as needed
- **Time Adjustment**: Add or subtract minutes on the fly

## 🚀 Installation

### Prerequisites

- Rust 1.70 or higher
- `mpv` media player (for Linux/macOS sound alerts)

```bash
# Install mpv on Ubuntu/Debian
sudo apt install mpv

# Install mpv on macOS
brew install mpv
```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/pomodoro-timer.git
cd pomodoro-timer

# Build and run
cargo build --release
cargo run --release
```

## 🎮 Usage

### Setup Screen

When you first launch the timer, you'll see the setup screen:

```
Work Duration (minutes): 25
Short Break (minutes): 5
Long Break (minutes): 15
Total Sessions: 4
```

**Controls:**
- `TAB` or `↓` - Move to next field
- `↑` - Move to previous field
- Type numbers to input values
- `BACKSPACE` - Delete last digit
- `ENTER` - Start timer
- `q` - Quit

### Timer Screen

During work sessions and breaks, you'll see a large countdown timer.

**Controls:**
- `SPACE` - Pause/Resume timer
- `s` - Skip to next session
- `↑` - Add 1 minute
- `↓` - Subtract 1 minute
- `q` - Quit

### Break Prompt

After each work session completes, you'll see a prompt:

```
🎉 WORK SESSION COMPLETE! 🎉
Time for a Short Break!
Ready to start your break?
```

**Controls:**
- `ENTER` - Start the break timer
- `s` - Skip break and go to next work session
- `q` - Quit

## 🎨 Color Coding

- **Cyan**: Work sessions
- **Yellow**: Short breaks
- **Green**: Long breaks

## 🔧 Configuration

### Custom Sound

Edit the `play_sound()` function in `src/main.rs` to change the alert sound:

```rust
let song_path = "/path/to/your/sound.mp3";
```

### Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
crossterm = "0.27"
notify-rust = "4.10"
```

## 📋 How It Works

The Pomodoro Technique:

1. Work for 25 minutes (customizable)
2. Take a 5-minute short break
3. Repeat steps 1-2 three more times
4. After the 4th session, take a 15-minute long break
5. Repeat the cycle

## 🐛 Troubleshooting

### Notifications not working?

**Linux**: Make sure you have a notification daemon running (usually included in desktop environments)

**Windows**: Notifications use PowerShell and should work by default

### Sound not playing?

**Linux/macOS**: Install `mpv`:
```bash
# Ubuntu/Debian
sudo apt install mpv

# macOS
brew install mpv
```

**Windows**: The app uses PowerShell's Media.SoundPlayer. Make sure the audio file path is correct.

## 📝 License

MIT License - feel free to use and modify as you wish!

## 🤝 Contributing

Contributions are welcome! Feel free to:

- Report bugs
- Suggest new features
- Submit pull requests

## 🙏 Acknowledgments

- Built with [crossterm](https://github.com/crossterm-rs/crossterm) for terminal manipulation
- Uses [notify-rust](https://github.com/hoodie/notify-rust) for desktop notifications
- Inspired by the Pomodoro Technique by Francesco Cirillo

## 📸 Screenshots
<img width="1920" height="1080" alt="Screenshot_14-Jan_12-15-46_4512" src="https://github.com/user-attachments/assets/9a37b262-1733-4d02-bd29-e01e52bf8654" />
<img width="1920" height="1080" alt="Screenshot_14-Jan_12-17-40_7113" src="https://github.com/user-attachments/assets/b174065b-9e39-4796-b850-fa8cb2df1c57" />
<img width="1920" height="1080" alt="Screenshot_14-Jan_12-18-08_30748" src="https://github.com/user-attachments/assets/2c9c5232-ab67-4096-bca8-814c57278671" />
<img width="1920" height="1080" alt="Screenshot_14-Jan_12-16-07_8002" src="https://github.com/user-attachments/assets/8120a8bf-3383-4af2-a21d-5dbb282f66c5" />


### Setup Screen
```
                POMODORO SETUP

Work Duration (minutes):
╭────────────────────────────────────────╮
│   25                                   │
╰────────────────────────────────────────╯
...
```

### Timer Screen
```
           WORK SESSION 1/4

  ██████  ██████      ██████  ██████
  █    █  █    █      █    █  █    █
  █    █  █    █  ██  █    █  █    █
  █    █  █    █      █    █  █    █
  ██████  ██████      ██████  ██████

               RUNNING
```

---

**Happy Pomodoro-ing! 🍅✨**
