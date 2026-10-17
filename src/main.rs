mod def;
use crossterm::cursor::{MoveDown, MoveUp};
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
                if cursor.coulmen != 0 {
                    cursor.coulmen -= 1;
                    stdout().execute(MoveLeft(1))?;
                } else if cursor.row > 0
                    && cursor.coulmen == 0
                    && let Some(txt) = string.get((cursor.row - 1) as usize)
                {
                    cursor.coulmen = txt.len() as u8;
                    cursor.row -= 1;

                    if readed.is_key_press() {
                        stdout().execute(MoveTo(cursor.coulmen as u16, cursor.row as u16))?;
                    }
                }
            }
            x if x == &key_case(KeyCode::Right, None) => {
                if cursor.coulmen
                    < string
                        .get(cursor.row as usize)
                        .unwrap_or_else(|| &text)
                        .len() as u8
                {
                    cursor.coulmen += 1;
                    stdout().execute(MoveRight(1))?;
                } else if (cursor.row as usize) < string.len() {
                    cursor.row += 1;
                    cursor.coulmen = 0;
                    stdout().execute(MoveTo(0, cursor.row as u16))?;
                }
            }
            x if x == &key_case(KeyCode::Enter, None) => {
                cursor.row += 1;

                let prev_txt = std::mem::replace(&mut text, Vec::new());
                string.push(prev_txt);

                cursor.coulmen = 0;

                println!();
                stdout().execute(MoveToColumn(0))?;
            }
            x if x == &key_case(KeyCode::Up, None) => {
                let before_jump = cursor.coulmen;

                if cursor.row != 0 {
                    cursor.row -= 1;

                    let len = string[cursor.row as usize].len();
                    let exists = string[cursor.row as usize].get(before_jump as usize);

                    if exists.is_some() {
                        stdout().execute(MoveUp(1))?;
                    } else {
                        cursor.coulmen = len as u8;
                        stdout().execute(MoveTo(cursor.coulmen as u16, cursor.row as u16))?;
                    }
                }
            }
            x if x == &key_case(KeyCode::Down, None) => {
                let before_jump = cursor.coulmen;

                if cursor.row < string.len() as u8 {
                    cursor.row += 1;

                    let len = string.get(cursor.row as usize).unwrap_or(&text).len();
                    let exists = string
                        .get(cursor.row as usize)
                        .unwrap_or(&text)
                        .get(before_jump as usize);

                    if exists.is_some() {
                        stdout().execute(MoveDown(1))?;
                    } else {
                        cursor.coulmen = len as u8;
                        stdout().execute(MoveTo(cursor.coulmen as u16, cursor.row as u16))?;
                    }
                }
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
