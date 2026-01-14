use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor, SetAttribute, Attribute},
    terminal::{self, ClearType},
    event::{self, Event, KeyCode, KeyEvent},
};
use std::io::{self, Write};
use std::time::{Duration, Instant};

// ASCII digits
const ASCII_DIGITS: [[&str; 5]; 11] = [
    ["██████", "█    █", "█    █", "█    █", "██████"], // 0
    ["  ██  ", "  ██  ", "  ██  ", "  ██  ", "  ██  "], // 1
    ["██████", "     █", "██████", "█     ", "██████"], // 2
    ["██████", "     █", "██████", "     █", "██████"], // 3
    ["█    █", "█    █", "██████", "     █", "     █"], // 4
    ["██████", "█     ", "██████", "     █", "██████"], // 5
    ["██████", "█     ", "██████", "█    █", "██████"], // 6
    ["██████", "     █", "     █", "     █", "     █"], // 7
    ["██████", "█    █", "██████", "█    █", "██████"], // 8
    ["██████", "█    █", "██████", "     █", "██████"], // 9
    ["      ", "  ██  ", "      ", "  ██  ", "      "], // :
];

#[derive(Clone, Copy, PartialEq, Debug)]
enum AppState {
    Setup,
    Running,
    BreakPrompt,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum TimerType {
    Work,
    ShortBreak,
    LongBreak,
}

struct InputField {
    value: String,
    placeholder: String,
    focused: bool,
}

impl InputField {
    fn new(placeholder: &str) -> Self {
        Self {
            value: String::new(),
            placeholder: placeholder.to_string(),
            focused: false,
        }
    }

    fn get_value(&self, default: u32) -> u32 {
        self.value.trim().parse().unwrap_or(default)
    }
}

struct PomodoroApp {
    state: AppState,
    timer_type: TimerType,
    paused: bool,
    
    // Input fields
    inputs: Vec<InputField>,
    focus_index: usize,
    
    // Timer durations
    work_minutes: u32,
    short_break_minutes: u32,
    long_break_minutes: u32,
    total_sessions: u32,
    current_session: u32,
    time_left: Duration,
    
    // Break prompt
    next_break_type: Option<TimerType>,
    
    // Screen size
    width: u16,
    height: u16,
}

impl PomodoroApp {
    fn new() -> Self {
        let mut inputs = Vec::new();
        inputs.push(InputField::new("25"));
        inputs.push(InputField::new("5"));
        inputs.push(InputField::new("15"));
        inputs.push(InputField::new("4"));
        inputs[0].focused = true;
        
        Self {
            state: AppState::Setup,
            timer_type: TimerType::Work,
            paused: false,
            inputs,
            focus_index: 0,
            work_minutes: 25,
            short_break_minutes: 5,
            long_break_minutes: 15,
            total_sessions: 4,
            current_session: 1,
            time_left: Duration::from_secs(25 * 60),
            next_break_type: None,
            width: 0,
            height: 0,
        }
    }

    fn start_timer(&mut self) {
        self.work_minutes = self.inputs[0].get_value(25);
        self.short_break_minutes = self.inputs[1].get_value(5);
        self.long_break_minutes = self.inputs[2].get_value(15);
        self.total_sessions = self.inputs[3].get_value(4);
        
        self.current_session = 1;
        self.state = AppState::Running;
        self.timer_type = TimerType::Work;
        self.time_left = Duration::from_secs(self.work_minutes as u64 * 60);
        self.paused = false;
    }

    fn advance_timer(&mut self) -> bool {
        match self.timer_type {
            TimerType::Work => {
                // Work session finished - show break prompt
                if self.current_session < self.total_sessions {
                    if self.current_session % 4 == 0 {
                        self.next_break_type = Some(TimerType::LongBreak);
                        send_notification("Pomodoro", "Work session finished! Time for a long break.");
                    } else {
                        self.next_break_type = Some(TimerType::ShortBreak);
                        send_notification("Pomodoro", "Work session finished! Time for a short break.");
                    }
                    self.state = AppState::BreakPrompt;
                    play_sound();
                    false  // Don't exit, show break prompt
                } else {
                    send_notification("Pomodoro", "All sessions completed! 🎉");
                    true  // Exit - all sessions done
                }
            }
            TimerType::ShortBreak | TimerType::LongBreak => {
                // Break finished - go back to work
                let msg = if self.timer_type == TimerType::ShortBreak {
                    "Short break finished! Back to work."
                } else {
                    "Long break finished! Back to work."
                };
                send_notification("Pomodoro", msg);
                
                self.current_session += 1;
                if self.current_session > self.total_sessions {
                    send_notification("Pomodoro", "All sessions completed! 🎉");
                    true  // Exit - all sessions done
                } else {
                    self.timer_type = TimerType::Work;
                    self.time_left = Duration::from_secs(self.work_minutes as u64 * 60);
                    self.paused = false;
                    self.state = AppState::Running;
                    play_sound();
                    false  // Continue to next work session
                }
            }
        }
    }

    fn start_break(&mut self) {
        if let Some(break_type) = self.next_break_type {
            self.timer_type = break_type;
            let duration = match break_type {
                TimerType::LongBreak => self.long_break_minutes,
                TimerType::ShortBreak => self.short_break_minutes,
                _ => 5,
            };
            self.time_left = Duration::from_secs(duration as u64 * 60);
            self.paused = false;
            self.state = AppState::Running;
            self.next_break_type = None;
            
            eprintln!("✓ Break started: {:?}, duration: {} minutes", break_type, duration);
        }
    }
}

fn render_big_time(seconds: u64) -> Vec<String> {
    let minutes = seconds / 60;
    let secs = seconds % 60;
    let time_str = format!("{:02}:{:02}", minutes, secs);
    
    let mut lines = vec![String::new(); 5];
    
    for ch in time_str.chars() {
        let digit_idx = if ch == ':' { 10 } else { ch.to_digit(10).unwrap_or(0) as usize };
        
        for (i, line) in ASCII_DIGITS[digit_idx].iter().enumerate() {
            lines[i].push_str(line);
            lines[i].push(' ');
        }
    }
    
    lines
}

fn draw_setup(app: &PomodoroApp) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, terminal::Clear(ClearType::All))?;
    
    let labels = [
        "Work Duration (minutes):",
        "Short Break (minutes):",
        "Long Break (minutes):",
        "Total Sessions:",
    ];
    
    let start_row = (app.height / 2).saturating_sub(10);
    
    // Title
    let title = "POMODORO SETUP";
    let title_col = (app.width / 2).saturating_sub((title.len() / 2) as u16);
    queue!(
        stdout,
        cursor::MoveTo(title_col, start_row),
        SetForegroundColor(Color::Cyan),
        SetAttribute(Attribute::Bold),
        Print(title),
        SetAttribute(Attribute::Reset)
    )?;
    
    // Input fields
    let mut current_row = start_row + 2;
    for (input, label) in app.inputs.iter().zip(labels.iter()) {
        // Label
        let label_col = (app.width / 2).saturating_sub(20);
        queue!(
            stdout,
            cursor::MoveTo(label_col, current_row),
            SetForegroundColor(Color::DarkGrey),
            Print(label)
        )?;
        current_row += 1;
        
        // Input box
        let box_col = (app.width / 2).saturating_sub(20);
        let border_color = if input.focused { Color::Cyan } else { Color::DarkGrey };
        
        // Top border
        queue!(
            stdout,
            cursor::MoveTo(box_col, current_row),
            SetForegroundColor(border_color),
            Print("╭────────────────────────────────────────╮")
        )?;
        current_row += 1;
        
        // Input content
        let display_text = if input.value.is_empty() {
            &input.placeholder
        } else {
            &input.value
        };
        
        let text_color = if input.value.is_empty() { Color::DarkGrey } else { Color::White };
        queue!(
            stdout,
            cursor::MoveTo(box_col, current_row),
            SetForegroundColor(border_color),
            Print("│   "),
            SetForegroundColor(text_color),
            Print(format!("{:<32}", display_text)),
            SetForegroundColor(border_color),
            Print("   │")
        )?;
        current_row += 1;
        
        // Bottom border
        queue!(
            stdout,
            cursor::MoveTo(box_col, current_row),
            SetForegroundColor(border_color),
            Print("╰────────────────────────────────────────╯")
        )?;
        current_row += 2;
    }
    
    // Help text
    current_row += 1;
    let help = "[TAB] Switch  •  [ENTER] Start  •  [q] Quit";
    let help_col = (app.width / 2).saturating_sub((help.len() / 2) as u16);
    queue!(
        stdout,
        cursor::MoveTo(help_col, current_row),
        SetForegroundColor(Color::DarkGrey),
        Print(help),
        ResetColor
    )?;
    
    stdout.flush()?;
    Ok(())
}

fn draw_break_prompt(app: &PomodoroApp) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, terminal::Clear(ClearType::All))?;
    
    let (color, message) = match app.next_break_type {
        Some(TimerType::LongBreak) => (Color::Green, "Time for a Long Break!"),
        Some(TimerType::ShortBreak) => (Color::Yellow, "Time for a Short Break!"),
        _ => (Color::White, "Break Time!"),
    };
    
    let start_row = (app.height / 2).saturating_sub(4);
    
    // Title
    let title = "🎉 WORK SESSION COMPLETE! 🎉";
    let title_col = (app.width / 2).saturating_sub((title.len() / 2) as u16);
    queue!(
        stdout,
        cursor::MoveTo(title_col, start_row),
        SetForegroundColor(Color::Cyan),
        SetAttribute(Attribute::Bold),
        Print(title),
        SetAttribute(Attribute::Reset)
    )?;
    
    // Break message
    let msg_col = (app.width / 2).saturating_sub((message.len() / 2) as u16);
    queue!(
        stdout,
        cursor::MoveTo(msg_col, start_row + 2),
        SetForegroundColor(color),
        SetAttribute(Attribute::Bold),
        Print(message),
        SetAttribute(Attribute::Reset)
    )?;
    
    // Prompt
    let prompt = "Ready to start your break?";
    let prompt_col = (app.width / 2).saturating_sub((prompt.len() / 2) as u16);
    queue!(
        stdout,
        cursor::MoveTo(prompt_col, start_row + 4),
        SetForegroundColor(Color::White),
        Print(prompt)
    )?;
    
    // Help text
    let help = "[ENTER] Start Break  •  [s] Skip  •  [q] Quit";
    let help_col = (app.width / 2).saturating_sub((help.len() / 2) as u16);
    queue!(
        stdout,
        cursor::MoveTo(help_col, start_row + 6),
        SetForegroundColor(Color::DarkGrey),
        Print(help),
        ResetColor
    )?;
    
    stdout.flush()?;
    Ok(())
}

fn draw_timer(app: &PomodoroApp) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, terminal::Clear(ClearType::All))?;
    
    let (color, mode_str) = match app.timer_type {
        TimerType::Work => (
            Color::Cyan,
            format!("WORK SESSION {}/{}", app.current_session, app.total_sessions)
        ),
        TimerType::ShortBreak => (Color::Yellow, "SHORT BREAK".to_string()),
        TimerType::LongBreak => (Color::Green, "LONG BREAK".to_string()),
    };
    
    let start_row = (app.height / 2).saturating_sub(6);
    
    // Title
    let title_col = (app.width / 2).saturating_sub((mode_str.len() / 2) as u16);
    queue!(
        stdout,
        cursor::MoveTo(title_col, start_row),
        SetForegroundColor(color),
        SetAttribute(Attribute::Bold),
        Print(&mode_str),
        SetAttribute(Attribute::Reset)
    )?;
    
    // Big timer
    let lines = render_big_time(app.time_left.as_secs());
    
    for (i, line) in lines.iter().enumerate() {
        let line_width = line.chars().count();
        let col = (app.width / 2).saturating_sub((line_width / 2) as u16);
        queue!(
            stdout,
            cursor::MoveTo(col, start_row + 2 + i as u16),
            SetForegroundColor(color),
            Print(line)
        )?;
    }
    
    // Status
    let status = if app.paused { "PAUSED" } else { "RUNNING" };
    let status_col = (app.width / 2).saturating_sub((status.len() / 2) as u16);
    queue!(
        stdout,
        cursor::MoveTo(status_col, start_row + 8),
        SetForegroundColor(Color::DarkGrey),
        Print(status)
    )?;
    
    // Help
    let help = "[SPACE] Pause  •  [s] Skip  •  [↑/↓] +/- 1m  •  [q] Quit";
    let help_col = (app.width / 2).saturating_sub((help.len() / 2) as u16);
    queue!(
        stdout,
        cursor::MoveTo(help_col, start_row + 10),
        SetForegroundColor(Color::DarkGrey),
        Print(help),
        ResetColor
    )?;
    
    stdout.flush()?;
    Ok(())
}

fn send_notification(title: &str, message: &str) {
    #[cfg(not(target_os = "windows"))]
    {
        let _ = notify_rust::Notification::new()
            .summary(title)
            .body(message)
            .show();
    }
    
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let _ = Command::new("powershell")
            .args(&["-Command", &format!("
                [Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] | Out-Null;
                $Template = [Windows.UI.Notifications.ToastNotificationManager]::GetTemplateContent([Windows.UI.Notifications.ToastTemplateType]::ToastText02);
                $RawXml = [xml] $Template.GetXml();
                ($RawXml.toast.visual.binding.text|where {{$_.id -eq '1'}}).AppendChild($RawXml.CreateTextNode('{}')) | Out-Null;
                ($RawXml.toast.visual.binding.text|where {{$_.id -eq '2'}}).AppendChild($RawXml.CreateTextNode('{}')) | Out-Null;
                $SerializedXml = New-Object Windows.Data.Xml.Dom.XmlDocument;
                $SerializedXml.LoadXml($RawXml.OuterXml);
                $Toast = [Windows.UI.Notifications.ToastNotification]::new($SerializedXml);
                [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier('Pomodoro').Show($Toast);
            ", title, message)])
            .output();
    }
}

fn play_sound() {
    use std::process::Command;
    
    let song_path = "/home/rohitrai/Music/music.mp3";
    
    eprintln!("🔊 Playing sound: {}", song_path);
    
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("powershell")
            .args(&["-c", &format!("(New-Object Media.SoundPlayer '{}').PlaySync()", song_path)])
            .spawn();
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        match Command::new("mpv")
            .arg("--no-video")
            .arg(song_path)
            .spawn() {
                Ok(_) => eprintln!("✓ mpv started"),
                Err(e) => eprintln!("✗ mpv failed: {}", e),
            }
    }
}

fn run_app() -> io::Result<()> {
    let mut app = PomodoroApp::new();
    let mut stdout = io::stdout();
    
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
    
    let (width, height) = terminal::size()?;
    app.width = width;
    app.height = height;
    
    let mut last_tick = Instant::now();
    
    loop {
        // Draw based on state
        match app.state {
            AppState::Setup => draw_setup(&app)?,
            AppState::Running => draw_timer(&app)?,
            AppState::BreakPrompt => draw_break_prompt(&app)?,
        }
        
        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => break,
                    
                    _ => {
                        if app.state == AppState::Setup {
                            match code {
                                KeyCode::Tab | KeyCode::Down => {
                                    app.inputs[app.focus_index].focused = false;
                                    app.focus_index = (app.focus_index + 1) % app.inputs.len();
                                    app.inputs[app.focus_index].focused = true;
                                }
                                KeyCode::Up => {
                                    app.inputs[app.focus_index].focused = false;
                                    app.focus_index = if app.focus_index == 0 {
                                        app.inputs.len() - 1
                                    } else {
                                        app.focus_index - 1
                                    };
                                    app.inputs[app.focus_index].focused = true;
                                }
                                KeyCode::Char(c) if c.is_ascii_digit() => {
                                    if app.inputs[app.focus_index].value.len() < 3 {
                                        app.inputs[app.focus_index].value.push(c);
                                    }
                                }
                                KeyCode::Backspace => {
                                    app.inputs[app.focus_index].value.pop();
                                }
                                KeyCode::Enter => {
                                    app.start_timer();
                                }
                                _ => {}
                            }
                        } else if app.state == AppState::BreakPrompt {
                            match code {
                                KeyCode::Enter => {
                                    app.start_break();
                                }
                                KeyCode::Char('s') => {
                                    // Skip break - go to next work session
                                    app.current_session += 1;
                                    if app.current_session > app.total_sessions {
                                        send_notification("Pomodoro", "All sessions completed! 🎉");
                                        break;
                                    }
                                    app.timer_type = TimerType::Work;
                                    app.time_left = Duration::from_secs(app.work_minutes as u64 * 60);
                                    app.paused = false;
                                    app.state = AppState::Running;
                                    app.next_break_type = None;
                                }
                                _ => {}
                            }
                        } else {
                            match code {
                                KeyCode::Char(' ') => app.paused = !app.paused,
                                KeyCode::Char('s') => {
                                    if app.advance_timer() {
                                        break;
                                    }
                                }
                                KeyCode::Up => app.time_left += Duration::from_secs(60),
                                KeyCode::Down => {
                                    if app.time_left > Duration::from_secs(60) {
                                        app.time_left -= Duration::from_secs(60);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        
        // Update timer
        if app.state == AppState::Running && !app.paused && last_tick.elapsed() >= Duration::from_secs(1) {
            last_tick = Instant::now();
            
            if app.time_left > Duration::from_secs(0) {
                app.time_left = app.time_left.saturating_sub(Duration::from_secs(1));
            }
            
            if app.time_left == Duration::from_secs(0) {
                eprintln!("⏰ Timer hit zero! Current state: {:?}, Type: {:?}", app.state, app.timer_type);
                let should_exit = app.advance_timer();
                eprintln!("   After advance: state={:?}, should_exit={}", app.state, should_exit);
                if should_exit {
                    break;
                }
            }
        }
    }
    
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    
    println!("\n✓ Pomodoro session completed!\n");
    Ok(())
}

fn main() -> io::Result<()> {
    run_app()
}
