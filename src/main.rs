use std::io::{stdout, Write};
use anyhow::{self, Ok};

use crossterm::{cursor, event::{self, read, Event}, terminal, ExecutableCommand, QueueableCommand};

enum Actions {
    Quit,

    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

enum Mode {
    Normal,
    Insert,
}

fn handle_event(mode: &Mode, ev: event::Event) -> anyhow::Result<Option<Actions>>{
    if matches!(mode, Mode::Normal) {
        handle_normal_event(ev)
    } else {
        handle_insert_event(ev)
    }
}


fn handle_normal_event(ev: event::Event) -> anyhow::Result<Option<Actions>> {
    match ev {
        event::Event::Key(event) => match event.code {
            event::KeyCode::Char('q') => Ok(Some(Actions::Quit)),
            event::KeyCode::Up | event::KeyCode::Char('k') => Ok(Some(Actions::MoveUp)),
            event::KeyCode::Down | event::KeyCode::Char('j') => Ok(Some(Actions::MoveDown)),
            event::KeyCode::Left | event::KeyCode::Char('h') => Ok(Some(Actions::MoveLeft)),
            event::KeyCode::Right | event::KeyCode::Char('l') => Ok(Some(Actions::MoveRight)),
            _ => Ok(None),
        },
        _ => Ok(None),
    }
}

fn handle_insert_event(ev: event::Event) -> anyhow::Result<Option<Actions>> {
    unimplemented!("insert event {ev:?}");
}

fn main() -> anyhow::Result<()> {
    let mut stdout = stdout();
    let mut mode = Mode::Normal;

    let mut cx = 0;
    let mut cy = 0;

    terminal::enable_raw_mode();

    stdout.execute(terminal::EnterAlternateScreen);

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    loop {
        stdout.queue(cursor::MoveTo(cx, cy))?;
        stdout.flush()?;
        if let Some(action) = handle_event(&mode, read()?)? {
            match action {
                Actions::Quit => break,
                Actions::MoveUp => if cy!=0 {cy-=1},
                Actions::MoveDown => cy+=1,
                Actions::MoveLeft => if cx!=0 {cx-=1},
                Actions::MoveRight => cx+=1,
            }
        }
        
    }

    stdout.execute(terminal::LeaveAlternateScreen)?;

    terminal::disable_raw_mode()?;

    Ok(())
}
