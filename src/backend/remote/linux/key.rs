use std::{fmt::Display, time::Duration};

use anyhow::Context;
use ashpd::desktop::{
    remote_desktop::{KeyState, RemoteDesktop},
    Session,
};
use tokio::time;

pub async fn press_key(
    proxy: &RemoteDesktop,
    session: &Session<'static, RemoteDesktop>,
    key: Key,
) -> anyhow::Result<()> {
    press_single_key(proxy, session, key, KeyState::Pressed).await?;
    time::sleep(Duration::from_millis(10)).await;
    press_single_key(proxy, session, key, KeyState::Released).await?;
    Ok(())
}

pub async fn press_key_with_modifiers(
    proxy: &RemoteDesktop,
    session: &Session<'static, RemoteDesktop>,
    key: Key,
    modifiers: &[Key],
) -> anyhow::Result<()> {
    for modifier in modifiers {
        press_single_key(proxy, session, *modifier, KeyState::Pressed).await?;
    }

    press_single_key(proxy, session, key, KeyState::Pressed).await?;
    time::sleep(Duration::from_millis(10)).await;
    press_single_key(proxy, session, key, KeyState::Released).await?;

    for modifier in modifiers.iter().rev() {
        press_single_key(proxy, session, *modifier, KeyState::Released).await?;
    }

    Ok(())
}

async fn press_single_key(
    proxy: &RemoteDesktop,
    session: &Session<'static, RemoteDesktop>,
    key: Key,
    state: KeyState,
) -> anyhow::Result<()> {
    proxy
        .notify_keyboard_keysym(session, key.symbol(), state)
        .await
        .with_context(|| format!("Failed to set key '{}' to state '{:?}'", key, state))
}

/// Keyboard keys.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[allow(dead_code)]
pub enum Key {
    // ----------------------------------------------
    // Commands:
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,

    Escape,
    Tab,
    Backspace,
    Enter,
    Space,

    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
    CapsLock,
    ShiftLock,

    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,

    // ----------------------------------------------
    // Punctuation:
    /// `:`
    Colon,

    /// `,`
    Comma,

    /// `\`
    Backslash,

    /// `/`
    Slash,

    /// `|`, a vertical bar
    Pipe,

    /// `?`
    Questionmark,

    // '!'
    Exclamationmark,

    // `[`
    OpenBracket,

    // `]`
    CloseBracket,

    // `{`
    OpenCurlyBracket,

    // `}`
    CloseCurlyBracket,

    /// Also known as "backquote" or "grave"
    Backtick,

    /// `-`
    Minus,

    /// `.`
    Period,

    /// `+`
    Plus,

    /// `=`
    Equals,

    /// `;`
    Semicolon,

    /// `'`
    Quote,

    // ----------------------------------------------
    // Digits:
    /// `0` (from main row or numpad)
    Num0,

    /// `1` (from main row or numpad)
    Num1,

    /// `2` (from main row or numpad)
    Num2,

    /// `3` (from main row or numpad)
    Num3,

    /// `4` (from main row or numpad)
    Num4,

    /// `5` (from main row or numpad)
    Num5,

    /// `6` (from main row or numpad)
    Num6,

    /// `7` (from main row or numpad)
    Num7,

    /// `8` (from main row or numpad)
    Num8,

    /// `9` (from main row or numpad)
    Num9,

    // ----------------------------------------------
    // Letters:
    A, // Used for cmd+A (select All)
    B,
    C, // |CMD COPY|
    D, // |CMD BOOKMARK|
    E, // |CMD SEARCH|
    F, // |CMD FIND firefox & chrome|
    G, // |CMD FIND chrome|
    H, // |CMD History|
    I, // italics
    J, // |CMD SEARCH firefox/DOWNLOAD chrome|
    K, // Used for ctrl+K (delete text after cursor)
    L,
    M,
    N,
    O, // |CMD OPEN|
    P, // |CMD PRINT|
    Q,
    R, // |CMD REFRESH|
    S, // |CMD SAVE|
    T, // |CMD TAB|
    U, // Used for ctrl+U (delete text before cursor)
    V, // |CMD PASTE|
    W, // Used for ctrl+W (delete previous word)
    X, // |CMD CUT|
    Y,
    Z, // |CMD UNDO|

    // ----------------------------------------------
    // Function keys:
    F1,
    F2,
    F3,
    F4,
    F5, // |CMD REFRESH|
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
}

impl Key {
    /// Human-readable English name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ArrowDown => "Down",
            Self::ArrowLeft => "Left",
            Self::ArrowRight => "Right",
            Self::ArrowUp => "Up",

            Self::Escape => "Escape",
            Self::Tab => "Tab",
            Self::Backspace => "Backspace",
            Self::Enter => "Enter",

            Self::LeftShift => "Left shift",
            Self::RightShift => "Right shift",
            Self::LeftControl => "Left control",
            Self::RightControl => "Right control",
            Self::CapsLock => "Caps lock",
            Self::ShiftLock => "Shift lock",

            Self::Insert => "Insert",
            Self::Delete => "Delete",
            Self::Home => "Home",
            Self::End => "End",
            Self::PageUp => "PageUp",
            Self::PageDown => "PageDown",

            Self::Space => "Space",
            Self::Colon => "Colon",
            Self::Comma => "Comma",
            Self::Minus => "Minus",
            Self::Period => "Period",
            Self::Plus => "Plus",
            Self::Equals => "Equals",
            Self::Semicolon => "Semicolon",
            Self::Backslash => "Backslash",
            Self::Slash => "Slash",
            Self::Pipe => "Pipe",
            Self::Questionmark => "Question mark",
            Self::Exclamationmark => "Exclamation mark",
            Self::OpenBracket => "Open bracket",
            Self::CloseBracket => "Close bracket",
            Self::OpenCurlyBracket => "Open curly bracket",
            Self::CloseCurlyBracket => "Close curly bracket",
            Self::Backtick => "Backtick",
            Self::Quote => "Quote",

            Self::Num0 => "0",
            Self::Num1 => "1",
            Self::Num2 => "2",
            Self::Num3 => "3",
            Self::Num4 => "4",
            Self::Num5 => "5",
            Self::Num6 => "6",
            Self::Num7 => "7",
            Self::Num8 => "8",
            Self::Num9 => "9",

            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
            Self::H => "H",
            Self::I => "I",
            Self::J => "J",
            Self::K => "K",
            Self::L => "L",
            Self::M => "M",
            Self::N => "N",
            Self::O => "O",
            Self::P => "P",
            Self::Q => "Q",
            Self::R => "R",
            Self::S => "S",
            Self::T => "T",
            Self::U => "U",
            Self::V => "V",
            Self::W => "W",
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
            Self::F1 => "F1",
            Self::F2 => "F2",
            Self::F3 => "F3",
            Self::F4 => "F4",
            Self::F5 => "F5",
            Self::F6 => "F6",
            Self::F7 => "F7",
            Self::F8 => "F8",
            Self::F9 => "F9",
            Self::F10 => "F10",
            Self::F11 => "F11",
            Self::F12 => "F12",
            Self::F13 => "F13",
            Self::F14 => "F14",
            Self::F15 => "F15",
            Self::F16 => "F16",
            Self::F17 => "F17",
            Self::F18 => "F18",
            Self::F19 => "F19",
            Self::F20 => "F20",
            Self::F21 => "F21",
            Self::F22 => "F22",
            Self::F23 => "F23",
            Self::F24 => "F24",
            Self::F25 => "F25",
            Self::F26 => "F26",
            Self::F27 => "F27",
            Self::F28 => "F28",
            Self::F29 => "F29",
            Self::F30 => "F30",
            Self::F31 => "F31",
            Self::F32 => "F32",
            Self::F33 => "F33",
            Self::F34 => "F34",
            Self::F35 => "F35",
        }
    }

    /// X11 keysym code.
    pub fn symbol(self) -> i32 {
        match self {
            Self::ArrowDown => 0xff54,
            Self::ArrowLeft => 0xff51,
            Self::ArrowRight => 0xff53,
            Self::ArrowUp => 0xff52,

            Self::Escape => 0xff1b,
            Self::Tab => 0xff09,
            Self::Backspace => 0xff08,
            Self::Enter => 0xff0d,

            Self::LeftShift => 0xffe1,
            Self::RightShift => 0xffe2,
            Self::LeftControl => 0xffe3,
            Self::RightControl => 0xffe4,
            Self::CapsLock => 0xffe5,
            Self::ShiftLock => 0xffe6,

            Self::Insert => 0xff63,
            Self::Delete => 0xffff,
            Self::Home => 0xff50,
            Self::End => 0xff57,
            Self::PageUp => 0xff55,
            Self::PageDown => 0xff56,

            Self::Space => 0x0020,
            Self::Colon => 0x003a,
            Self::Comma => 0x002c,
            Self::Minus => 0x002d,
            Self::Period => 0x002e,
            Self::Plus => 0x002b,
            Self::Equals => 0x003d,
            Self::Semicolon => 0x003b,
            Self::Backslash => 0x005c,
            Self::Slash => 0x002f,
            Self::Pipe => 0x007c,
            Self::Questionmark => 0x003f,
            Self::Exclamationmark => 0x0021,
            Self::OpenBracket => 0x005b,
            Self::CloseBracket => 0x005d,
            Self::OpenCurlyBracket => 0x007b,
            Self::CloseCurlyBracket => 0x007d,
            Self::Backtick => 0x0060,
            Self::Quote => 0x0022,

            Self::Num0 => 0x0030,
            Self::Num1 => 0x0031,
            Self::Num2 => 0x0032,
            Self::Num3 => 0x0033,
            Self::Num4 => 0x0034,
            Self::Num5 => 0x0035,
            Self::Num6 => 0x0036,
            Self::Num7 => 0x0037,
            Self::Num8 => 0x0038,
            Self::Num9 => 0x0039,

            Self::A => 0x0061,
            Self::B => 0x0062,
            Self::C => 0x0063,
            Self::D => 0x0064,
            Self::E => 0x0065,
            Self::F => 0x0066,
            Self::G => 0x0067,
            Self::H => 0x0068,
            Self::I => 0x0069,
            Self::J => 0x006a,
            Self::K => 0x006b,
            Self::L => 0x006c,
            Self::M => 0x006d,
            Self::N => 0x006e,
            Self::O => 0x006f,
            Self::P => 0x0070,
            Self::Q => 0x0071,
            Self::R => 0x0072,
            Self::S => 0x0073,
            Self::T => 0x0074,
            Self::U => 0x0075,
            Self::V => 0x0076,
            Self::W => 0x0077,
            Self::X => 0x0078,
            Self::Y => 0x0079,
            Self::Z => 0x007a,
            Self::F1 => 0xffbe,
            Self::F2 => 0xffbf,
            Self::F3 => 0xffc0,
            Self::F4 => 0xffc1,
            Self::F5 => 0xffc2,
            Self::F6 => 0xffc3,
            Self::F7 => 0xffc4,
            Self::F8 => 0xffc5,
            Self::F9 => 0xffc6,
            Self::F10 => 0xffc7,
            Self::F11 => 0xffc8,
            Self::F12 => 0xffc9,
            Self::F13 => 0xffca,
            Self::F14 => 0xffcb,
            Self::F15 => 0xffcc,
            Self::F16 => 0xffcd,
            Self::F17 => 0xffce,
            Self::F18 => 0xffcf,
            Self::F19 => 0xffd0,
            Self::F20 => 0xffd1,
            Self::F21 => 0xffd2,
            Self::F22 => 0xffd3,
            Self::F23 => 0xffd4,
            Self::F24 => 0xffd5,
            Self::F25 => 0xffd6,
            Self::F26 => 0xffd7,
            Self::F27 => 0xffd8,
            Self::F28 => 0xffd9,
            Self::F29 => 0xffda,
            Self::F30 => 0xffdb,
            Self::F31 => 0xffdc,
            Self::F32 => 0xffdd,
            Self::F33 => 0xffde,
            Self::F34 => 0xffdf,
            Self::F35 => 0xffe0,
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
