/// ASCII制御文字(0-31)
pub struct Ctl(char);

/// one of SEPARATORS
pub struct Separators(char);
pub const SEPARATORS: [char; 19] = ['(', ')', '<', '>', '@', ',', ';', ':', '\\', '"', '/', '[', ']', '?', '=', '{', '}', ' ', '\t'];

/// not: Ctl, Separators
pub struct Token(char);

/// not including: Ctl; may including: Lws;
pub struct Text(String);

/// 16進数文字; one of HEX
pub struct Hex(char);
pub const HEX: [char; 23] = ['A', 'B', 'C', 'D', 'E', 'F', 'a', 'b', 'c', 'd', 'e', 'f', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

pub const CRLF: &'static str = "\r\n";
pub struct Lws {
    /// CRLF
    crlf: &'static str,
    /// one of ' ', '\t'
    space_or_tab: char,
}