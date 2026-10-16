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
    let mut text: Vec<char> = Vec::new();
    let mut string: Vec<Vec<char>> = Vec::new();

    while let Ok(ref readed) = read() {
        match readed {
            x if x == &key_case(KeyCode::Char('c'), Some(KeyModifiers::CONTROL)) => {
                disable_raw_mode()?;
                exit(0);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
                state: KeyEventState::NONE,
                ..
            }) => {
                cursor.coulmen += 1;

                print!("{}", c);
                stdout().flush()?;

                text.push(*c);
            }
            x if x == &key_case(KeyCode::Left, None) => {
                if cursor.coulmen != 1 {
                    cursor.coulmen -= 1;
                    stdout().execute(MoveLeft(1))?;
                } else if cursor.row > 0
                    && cursor.coulmen == 1
                    && let Some(txt) = string.get((cursor.row - 1) as usize)
                {
                    cursor.coulmen = (txt.len() + 1) as u8;
                    cursor.row -= 1;

                    if readed.is_key_press() {
                        stdout().execute(MoveTo((cursor.coulmen - 1) as u16, cursor.row as u16))?;
                    }
                }
            }
            x if x == &key_case(KeyCode::Right, None) => {
                if cursor.coulmen
                    < (string
                        .get(cursor.row as usize)
                        .unwrap_or_else(|| &text)
                        .len()
                        + 1) as u8
                {
                    cursor.coulmen += 1;
                    stdout().execute(MoveRight(1))?;
                } else if (cursor.row as usize) < string.len() {
                    cursor.row += 1;
                    cursor.coulmen = 1;
                    stdout().execute(MoveTo(0, cursor.row as u16))?;
                }
            }
            x if x == &key_case(KeyCode::Enter, None) => {
                cursor.row += 1;

                let prev_txt = std::mem::replace(&mut text, Vec::new());
                string.push(prev_txt);

                cursor.coulmen = 1;

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
