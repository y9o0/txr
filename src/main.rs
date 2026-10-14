mod def;
use def::*;

fn main() -> anyhow::Result<()> {
    handle_keys()?;
    Ok(())
}

fn handle_keys() -> anyhow::Result<()> {
    enable_raw_mode()?;
    start()?;

    let mut cursor = Cursor::new();

    while let Ok(ref readed) = read() {
        match readed {
            x if x == &key_case(KeyCode::Char('c'), Some(KeyModifiers::CONTROL)) => {
                disable_raw_mode()?;
                exit(0);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => {
                cursor.current += 1;

                print!("{}", c);
                stdout().flush()?;

                cursor.last_position = cursor.current;
            }
            x if x == &key_case(KeyCode::Left, None) => {
                if cursor.current != 0 {
                    cursor.current -= 1;
                    stdout().execute(MoveLeft(1))?;
                } else if cursor.row != 0 && cursor.current == 0 {
                    cursor.row -= 1;

                    if readed.is_key_press() {
                        stdout().execute(MoveUp(1))?;
                    }
                }
            }
            x if x == &key_case(KeyCode::Right, None) => {
                if cursor.current < cursor.last_position {
                    cursor.current += 1;
                    stdout().execute(MoveRight(1))?;
                }
            }
            x if x == &key_case(KeyCode::Enter, None) => {
                cursor.row += 1;
                cursor.current = 0;

                println!();
                stdout().execute(MoveToColumn(0))?;
            }
            _ => {}
        }
    }

    Ok(())
}

fn key_case(key: KeyCode, modifier: Option<KeyModifiers>) -> Event {
    Event::Key(KeyEvent {
        code: key,
        modifiers: modifier.unwrap_or(KeyModifiers::NONE),
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    })
}

fn start() -> anyhow::Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(Clear(crossterm::terminal::ClearType::FromCursorUp))?;
    stdout().execute(MoveTo(0, 0))?;

    Ok(())
}
