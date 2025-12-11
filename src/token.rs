use core::fmt;

pub enum TokenType {
    Command,  // i.e grep, fzf, echo
    Argument, // flags and args
    Keyword,  // should be for |, &, >, ...
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let diplay_str = match self {
            TokenType::Command => "Command",
            TokenType::Argument => "Argument",
            TokenType::Keyword => "Keyword",
        };
        write!(f, "{}", diplay_str)
    }
}

pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) token_string: String,
}
