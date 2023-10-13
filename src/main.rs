use ratatui::{
  prelude::{CrosstermBackend, Terminal},
  widgets::Paragraph,
};

use crossterm::event;

type Result<T> = std::result::Result<T, Err>;
pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;

#[derive(Debug)]
struct App {
    is_running: bool,
    msg: String,
}

fn startup() -> result<()> {
  // startup: enable raw mode for the terminal, giving us fine control over user input
  crossterm::terminal::enable_raw_mode()?;
  crossterm::execute!(std::io::stderr(), crossterm::terminal::enteralternatescreen)?;
  ok(())
}

fn close() -> result<()> {
  // shutdown down: reset terminal back to original state
  crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
  crossterm::terminal::disable_raw_mode()?;
  Ok(())
}

fn ui(app: &App, f: &mut Frame<'_>) {
    f.render_widget(
        Paragraph::new(format!("Counter: {}", app.msg)),
        f.size()
    );
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    event::KeyCode::Char('q') => app.is_running = false,
                    _ => (),
                }
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
  // Initialize the terminal backend using crossterm
  let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

  let mut app = App {
      is_running: true,
      msg: String::from("todo-rs"),
  };

  loop {
      terminal.draw(|f| {
          ui(&app, f);
      })?;

      update(&mut app)?;

      if !app.is_running {
          break;
      }
  }

  Ok(())
}

fn main() -> Result<()> {
  startup()?;
  let status = run();
  close()?;
  status?;

  Ok(())
}
