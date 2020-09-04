#![no_std]

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IntegerBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FloatBase {
    Decimal,
    Hexadecimal,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token {
    Invalid,
    Whitespace,
    Comment,
    DocComment,
    Builtin,
    Identifier,
    RawIdentifier {
        is_unterminated: bool,
        has_invalid_escape: bool,
    },
    IntegerLiteral {
        base: IntegerBase,
        is_unterminated: bool,
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    FloatLiteral {
        base: FloatBase,
        is_unterminated: bool,
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    StringLiteral {
        is_unterminated: bool,
        has_invalid_escape: bool,
    },
    CharacterLiteral {
        is_empty: bool,
        is_unterminated: bool,
        has_invalid_escape: bool,
    },
    MultilineStringLiteralLine,
    Bang,              // !
    BangEqual,         // !=
    Percent,           // %
    PercentEqual,      // %=
    And,               // &
    And2,              // &&
    AndEqual,          // &=
    LParen,            // (
    RParen,            // )
    Star,              // *
    Star2,             // **
    StarEqual,         // *=
    StarPercent,       // *%
    StarPercentEqual,  // *%=
    Plus,              // +
    Plus2,             // ++
    PlusEqual,         // +=
    PlusPercent,       // +%
    PlusPercentEqual,  // +%=
    Comma,             // ,
    Minus,             // -
    MinusEqual,        // -=
    MinusPercent,      // -%
    MinusPercentEqual, // -%=
    Dot,               // .
    Dot2,              // ..
    Dot3,              // ...
    DotStar,           // .*
    Slash,             // /
    SlashEqual,        // /=
    Colon,             // :
    Semicolon,         // ;
    LAngle,            // <
    LAngleEqual,       // <=
    LAngle2,           // <<
    LAngle2Equal,      // <<=
    Equal,             // =
    Equal2,            // ==
    EqualRAngle,       // =>
    RAngle,            // >
    RAngleEqual,       // >=
    RAngle2,           // >>
    RAngle2Equal,      // >>=
    Question,          // ?
    At,                // @
    LBracket,          // [
    RBracket,          // ]
    Caret,             // ^
    CaretEqual,        // ^=
    LBrace,            // {
    Or,                // |
    Or2,               // ||
    OrEqual,           // |=
    RBrace,            // }
}

#[derive(PartialEq)]
enum EscapeKind {
    RawIdentifier,
    StringLiteral,
    CharacterLiteral,
}

enum State {
    Start,
    Invalid,
    Whitespace,
    Bang,
    Percent,
    And,
    Star,
    StarPercent,
    Plus,
    PlusPercent,
    Minus,
    MinusPercent,
    Dot,
    Dot2,
    Slash,
    Slash2,
    LAngle,
    LAngle2,
    Equal,
    RAngle,
    RAngle2,
    At,
    Caret,
    Or,
    Comment,
    DocComment,
    MultilineStringLiteralLine,
    Identifier,
    Builtin,
    StringLiteral {
        is_empty: bool,
        escape_kind: EscapeKind,
        has_invalid_escape: bool,
    },
    StringEscape {
        is_empty: bool,
        escape_kind: EscapeKind,
        has_invalid_escape: bool,
    },
    StringEscapeHex1 {
        escape_kind: EscapeKind,
        has_invalid_escape: bool,
    },
    StringEscapeHex2 {
        escape_kind: EscapeKind,
        has_invalid_escape: bool,
    },
    StringEscapeUnicode1 {
        escape_kind: EscapeKind,
        has_invalid_escape: bool,
    },
    StringEscapeUnicode2 {
        escape_kind: EscapeKind,
        has_invalid_escape: bool,
    },
    StringEscapeUnicode3 {
        escape_kind: EscapeKind,
        has_invalid_escape: bool,
    },
    Zero,
    Number {
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberUnderscore {
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberExponent {
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberExponentSign {
        is_unterminated: bool,
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberExponentSignUnderscore {
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberBinary {
        is_unterminated: bool,
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberBinaryUnderscore {
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberOctal {
        is_unterminated: bool,
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberOctalUnderscore {
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberHex {
        is_unterminated: bool,
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberHexUnderscore {
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberHexDot {
        is_unterminated: bool,
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberHexDotUnderscore {
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberHexExponent {
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberHexExponentSign {
        is_unterminated: bool,
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberHexExponentSignUnderscore {
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberDot {
        is_unterminated: bool,
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
    NumberDotUnderscore {
        has_invalid_characters: bool,
        has_duplicate_underscore: bool,
    },
}

enum Step {
    Continue(State),
    Reprocess(State),
    Backtrack,
    Abort(Token),
    End(Token),
}

fn step(state: State, c: Option<char>) -> Step {
    match state {
        State::Start => match c {
            Some(' ') | Some('\t') | Some('\r') | Some('\n') => Step::Continue(State::Whitespace),
            Some('!') => Step::Continue(State::Bang),
            Some('"') => Step::Continue(State::StringLiteral {
                is_empty: true,
                escape_kind: EscapeKind::StringLiteral,
                has_invalid_escape: false,
            }),
            Some('%') => Step::Continue(State::Percent),
            Some('&') => Step::Continue(State::And),
            Some('\'') => Step::Continue(State::StringLiteral {
                is_empty: true,
                escape_kind: EscapeKind::CharacterLiteral,
                has_invalid_escape: false,
            }),
            Some('(') => Step::End(Token::LParen),
            Some(')') => Step::End(Token::RParen),
            Some('*') => Step::Continue(State::Star),
            Some('+') => Step::Continue(State::Plus),
            Some(',') => Step::End(Token::Comma),
            Some('-') => Step::Continue(State::Minus),
            Some('.') => Step::Continue(State::Dot),
            Some('/') => Step::Continue(State::Slash),
            Some('0') => Step::Continue(State::Zero),
            Some('1'..='9') => Step::Continue(State::Number {
                has_invalid_characters: false,
                has_duplicate_underscore: false,
            }),
            Some(':') => Step::End(Token::Colon),
            Some(';') => Step::End(Token::Semicolon),
            Some('<') => Step::Continue(State::LAngle),
            Some('=') => Step::Continue(State::Equal),
            Some('>') => Step::Continue(State::RAngle),
            Some('?') => Step::End(Token::Question),
            Some('@') => Step::Continue(State::At),
            Some('A'..='Z') | Some('a'..='z') | Some('_') => Step::Continue(State::Identifier),
            Some('[') => Step::End(Token::LBracket),
            Some('\\') => Step::Continue(State::MultilineStringLiteralLine),
            Some(']') => Step::End(Token::RBracket),
            Some('^') => Step::Continue(State::Caret),
            Some('{') => Step::End(Token::LBrace),
            Some('|') => Step::Continue(State::Or),
            Some('}') => Step::End(Token::RBrace),
            Some(_) => Step::Continue(State::Invalid),
            None => Step::Abort(Token::Invalid),
        },
        State::Invalid => match step(State::Start, c) {
            Step::Continue(State::Invalid) | Step::Abort(Token::Invalid) => match c {
                Some(_) => Step::Continue(State::Invalid),
                None => Step::Abort(Token::Invalid),
            },
            _ => Step::Abort(Token::Invalid),
        },
        State::Whitespace => match c {
            Some(' ') | Some('\t') | Some('\r') | Some('\n') => Step::Continue(State::Whitespace),
            Some(_) | None => Step::Abort(Token::Whitespace),
        },
        State::Bang => match c {
            Some('=') => Step::End(Token::BangEqual),
            Some(_) | None => Step::Abort(Token::Bang),
        },
        State::Percent => match c {
            Some('=') => Step::End(Token::PercentEqual),
            Some(_) | None => Step::Abort(Token::Percent),
        },
        State::And => match c {
            Some('&') => Step::End(Token::And2),
            Some('=') => Step::End(Token::AndEqual),
            Some(_) | None => Step::Abort(Token::And),
        },
        State::Star => match c {
            Some('*') => Step::End(Token::Star2),
            Some('=') => Step::End(Token::StarEqual),
            Some('%') => Step::Continue(State::StarPercent),
            Some(_) | None => Step::Abort(Token::Star),
        },
        State::StarPercent => match c {
            Some('=') => Step::End(Token::StarPercentEqual),
            Some(_) | None => Step::Abort(Token::StarPercent),
        },
        State::Plus => match c {
            Some('+') => Step::End(Token::Plus2),
            Some('=') => Step::End(Token::PlusEqual),
            Some('%') => Step::Continue(State::PlusPercent),
            Some(_) | None => Step::Abort(Token::Plus),
        },
        State::PlusPercent => match c {
            Some('=') => Step::End(Token::PlusPercentEqual),
            Some(_) | None => Step::Abort(Token::PlusPercent),
        },
        State::Minus => match c {
            Some('=') => Step::End(Token::MinusEqual),
            Some('%') => Step::Continue(State::MinusPercent),
            Some(_) | None => Step::Abort(Token::Minus),
        },
        State::MinusPercent => match c {
            Some('=') => Step::End(Token::MinusPercentEqual),
            Some(_) | None => Step::Abort(Token::MinusPercent),
        },
        State::Dot => match c {
            Some('.') => Step::Continue(State::Dot2),
            Some('*') => Step::End(Token::DotStar),
            Some(_) | None => Step::Abort(Token::Dot),
        },
        State::Dot2 => match c {
            Some('.') => Step::End(Token::Dot3),
            Some(_) | None => Step::Abort(Token::Dot2),
        },
        State::Slash => match c {
            Some('/') => Step::Continue(State::Slash2),
            Some('=') => Step::End(Token::SlashEqual),
            Some(_) | None => Step::Abort(Token::Slash),
        },
        State::Slash2 => match c {
            Some('\n') | None => Step::Abort(Token::Comment),
            Some('/') => Step::Continue(State::DocComment),
            Some(_) => Step::Continue(State::Comment),
        },
        State::LAngle => match c {
            Some('=') => Step::End(Token::LAngleEqual),
            Some('<') => Step::Continue(State::LAngle2),
            Some(_) | None => Step::Abort(Token::LAngle),
        },
        State::LAngle2 => match c {
            Some('=') => Step::End(Token::LAngle2Equal),
            Some(_) | None => Step::Abort(Token::LAngle2),
        },
        State::Equal => match c {
            Some('=') => Step::End(Token::Equal2),
            Some('>') => Step::End(Token::EqualRAngle),
            Some(_) | None => Step::Abort(Token::Equal),
        },
        State::RAngle => match c {
            Some('=') => Step::End(Token::RAngleEqual),
            Some('>') => Step::Continue(State::RAngle2),
            Some(_) | None => Step::Abort(Token::RAngle),
        },
        State::RAngle2 => match c {
            Some('=') => Step::End(Token::RAngle2Equal),
            Some(_) | None => Step::Abort(Token::RAngle2),
        },
        State::At => match c {
            Some('"') => Step::Continue(State::StringLiteral {
                escape_kind: EscapeKind::RawIdentifier,
                has_invalid_escape: false,
                is_empty: true,
            }),
            Some('A'..='Z') | Some('a'..='z') | Some('_') => Step::Continue(State::Builtin),
            Some(_) | None => Step::Abort(Token::At),
        },
        State::Caret => match c {
            Some('=') => Step::End(Token::CaretEqual),
            Some(_) | None => Step::Abort(Token::Caret),
        },
        State::Or => match c {
            Some('|') => Step::End(Token::Or2),
            Some('=') => Step::End(Token::OrEqual),
            Some(_) | None => Step::Abort(Token::Or),
        },
        State::Comment => match c {
            Some('\n') | None => Step::Abort(Token::Comment),
            Some(_) => Step::Continue(State::Comment),
        },
        State::DocComment => match c {
            Some('\n') | None => Step::Abort(Token::DocComment),
            Some(_) => Step::Continue(State::DocComment),
        },
        State::MultilineStringLiteralLine => match c {
            Some('\n') | None => Step::Abort(Token::MultilineStringLiteralLine),
            Some(_) => Step::Continue(State::MultilineStringLiteralLine),
        },
        State::Identifier => match c {
            Some('0'..='9') | Some('A'..='Z') | Some('a'..='z') | Some('_') => {
                Step::Continue(State::Identifier)
            }
            Some(_) | None => Step::Abort(Token::Identifier),
        },
        State::Builtin => match c {
            Some('0'..='9') | Some('A'..='Z') | Some('a'..='z') | Some('_') => {
                Step::Continue(State::Builtin)
            }
            Some(_) | None => Step::Abort(Token::Builtin),
        },
        State::StringLiteral {
            is_empty,
            escape_kind,
            has_invalid_escape,
        } => match c {
            Some('\\') => Step::Continue(State::StringEscape {
                is_empty,
                escape_kind,
                has_invalid_escape,
            }),
            Some('"') if escape_kind != EscapeKind::CharacterLiteral => match escape_kind {
                EscapeKind::RawIdentifier => Step::End(Token::RawIdentifier {
                    is_unterminated: false,
                    has_invalid_escape,
                }),
                EscapeKind::StringLiteral => Step::End(Token::StringLiteral {
                    is_unterminated: false,
                    has_invalid_escape,
                }),
                EscapeKind::CharacterLiteral => unreachable!(),
            },
            Some('\'') if escape_kind == EscapeKind::CharacterLiteral => match escape_kind {
                EscapeKind::CharacterLiteral => Step::End(Token::CharacterLiteral {
                    is_empty,
                    is_unterminated: false,
                    has_invalid_escape,
                }),
                EscapeKind::RawIdentifier | EscapeKind::StringLiteral => unreachable!(),
            },
            Some('\n') | None => match escape_kind {
                EscapeKind::RawIdentifier => Step::Abort(Token::RawIdentifier {
                    is_unterminated: true,
                    has_invalid_escape,
                }),
                EscapeKind::StringLiteral => Step::Abort(Token::StringLiteral {
                    is_unterminated: true,
                    has_invalid_escape,
                }),
                EscapeKind::CharacterLiteral => Step::Abort(Token::CharacterLiteral {
                    is_empty,
                    is_unterminated: true,
                    has_invalid_escape,
                }),
            },
            Some(_) => Step::Continue(State::StringLiteral {
                is_empty: false,
                escape_kind,
                has_invalid_escape,
            }),
        },
        State::StringEscape {
            is_empty,
            escape_kind,
            has_invalid_escape,
        } => match c {
            Some('n') | Some('r') | Some('\\') | Some('t') | Some('\'') | Some('"') => {
                Step::Continue(State::StringLiteral {
                    is_empty: false,
                    escape_kind,
                    has_invalid_escape,
                })
            }
            Some('x') => Step::Continue(State::StringEscapeHex1 {
                escape_kind,
                has_invalid_escape,
            }),
            Some('u') => Step::Continue(State::StringEscapeUnicode1 {
                escape_kind,
                has_invalid_escape,
            }),
            Some(_) | None => Step::Reprocess(State::StringLiteral {
                is_empty,
                escape_kind,
                has_invalid_escape: true,
            }),
        },
        State::StringEscapeHex1 {
            escape_kind,
            has_invalid_escape,
        } => match c {
            Some('0'..='9') | Some('A'..='F') | Some('a'..='f') => {
                Step::Continue(State::StringEscapeHex2 {
                    escape_kind,
                    has_invalid_escape,
                })
            }
            Some('G'..='Z') | Some('g'..='z') => Step::Continue(State::StringEscapeHex2 {
                escape_kind,
                has_invalid_escape: true,
            }),
            Some(_) | None => Step::Reprocess(State::StringLiteral {
                is_empty: false,
                escape_kind,
                has_invalid_escape: true,
            }),
        },
        State::StringEscapeHex2 {
            escape_kind,
            has_invalid_escape,
        } => match c {
            Some('0'..='9') | Some('A'..='F') | Some('a'..='f') => {
                Step::Continue(State::StringLiteral {
                    is_empty: false,
                    escape_kind,
                    has_invalid_escape,
                })
            }
            Some('G'..='Z') | Some('g'..='z') => Step::Continue(State::StringLiteral {
                is_empty: false,
                escape_kind,
                has_invalid_escape: true,
            }),
            Some(_) | None => Step::Reprocess(State::StringLiteral {
                is_empty: false,
                escape_kind,
                has_invalid_escape: true,
            }),
        },
        State::StringEscapeUnicode1 {
            escape_kind,
            has_invalid_escape,
        } => match c {
            Some('{') => Step::Continue(State::StringEscapeUnicode2 {
                escape_kind,
                has_invalid_escape,
            }),
            Some(_) | None => Step::Reprocess(State::StringLiteral {
                is_empty: false,
                escape_kind,
                has_invalid_escape: true,
            }),
        },
        State::StringEscapeUnicode2 {
            escape_kind,
            has_invalid_escape,
        } => match c {
            Some('0'..='9') | Some('A'..='F') | Some('a'..='f') => {
                Step::Continue(State::StringEscapeUnicode3 {
                    escape_kind,
                    has_invalid_escape,
                })
            }
            Some(_) | None => Step::Reprocess(State::StringLiteral {
                is_empty: false,
                escape_kind,
                has_invalid_escape: true,
            }),
        },
        State::StringEscapeUnicode3 {
            escape_kind,
            has_invalid_escape,
        } => match c {
            Some('0'..='9') | Some('A'..='F') | Some('a'..='f') => {
                Step::Continue(State::StringEscapeUnicode3 {
                    escape_kind,
                    has_invalid_escape,
                })
            }
            Some('}') => Step::Continue(State::StringLiteral {
                is_empty: false,
                escape_kind,
                has_invalid_escape,
            }),
            Some(_) | None => Step::Reprocess(State::StringLiteral {
                is_empty: false,
                escape_kind,
                has_invalid_escape: true,
            }),
        },
        State::Zero => match c {
            Some('b') => Step::Continue(State::NumberBinary {
                is_unterminated: true,
                has_invalid_characters: false,
                has_duplicate_underscore: false,
            }),
            Some('o') => Step::Continue(State::NumberOctal {
                is_unterminated: true,
                has_invalid_characters: false,
                has_duplicate_underscore: false,
            }),
            Some('x') => Step::Continue(State::NumberHex {
                is_unterminated: true,
                has_invalid_characters: false,
                has_duplicate_underscore: false,
            }),
            Some(_) | None => Step::Reprocess(State::Number {
                has_invalid_characters: false,
                has_duplicate_underscore: false,
            }),
        },
        State::Number {
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('0'..='9') => Step::Continue(State::Number {
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('A'..='D') | Some('a'..='d') | Some('F'..='O') | Some('f'..='o')
            | Some('Q'..='Z') | Some('q'..='z') => Step::Continue(State::Number {
                has_invalid_characters: true,
                has_duplicate_underscore,
            }),
            Some('_') => Step::Continue(State::NumberUnderscore {
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('.') => Step::Continue(State::NumberDot {
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('e') | Some('E') => Step::Continue(State::NumberExponent {
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('p') | Some('P') => Step::Continue(State::NumberHexExponent {
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some(_) | None => Step::Abort(Token::IntegerLiteral {
                base: IntegerBase::Decimal,
                is_unterminated: false,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberUnderscore {
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('0'..='9') => Step::Continue(State::Number {
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('A'..='Z') | Some('a'..='z') => Step::Continue(State::Number {
                has_invalid_characters: true,
                has_duplicate_underscore,
            }),
            Some('_') => Step::Continue(State::Number {
                has_invalid_characters,
                has_duplicate_underscore: true,
            }),
            Some(_) | None => Step::Abort(Token::IntegerLiteral {
                base: IntegerBase::Decimal,
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberExponent {
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('+') | Some('-') => Step::Continue(State::NumberExponentSign {
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some(_) | None => Step::Reprocess(State::NumberExponentSign {
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberExponentSign {
            is_unterminated,
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('0'..='9') => Step::Continue(State::NumberExponentSign {
                is_unterminated: false,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('A'..='Z') | Some('a'..='z') => Step::Continue(State::NumberExponentSign {
                is_unterminated: false,
                has_invalid_characters: true,
                has_duplicate_underscore,
            }),
            Some('_') => Step::Continue(State::NumberExponentSignUnderscore {
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some(_) | None => Step::Abort(Token::FloatLiteral {
                base: FloatBase::Decimal,
                is_unterminated,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberExponentSignUnderscore {
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('0'..='9') => Step::Continue(State::NumberExponentSign {
                is_unterminated: false,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('A'..='Z') | Some('a'..='z') => Step::Continue(State::NumberExponentSign {
                is_unterminated: false,
                has_invalid_characters: true,
                has_duplicate_underscore,
            }),
            Some('_') => Step::Continue(State::NumberExponentSign {
                is_unterminated: false,
                has_invalid_characters,
                has_duplicate_underscore: true,
            }),
            Some(_) | None => Step::Abort(Token::FloatLiteral {
                base: FloatBase::Decimal,
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberBinary {
            is_unterminated,
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('0'..='1') => Step::Continue(State::NumberBinary {
                is_unterminated: false,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('2'..='9') | Some('A'..='Z') | Some('a'..='z') => {
                Step::Continue(State::NumberBinary {
                    is_unterminated: false,
                    has_invalid_characters: true,
                    has_duplicate_underscore,
                })
            }
            Some('_') => Step::Continue(State::NumberBinaryUnderscore {
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some(_) | None => Step::Abort(Token::IntegerLiteral {
                base: IntegerBase::Binary,
                is_unterminated,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberBinaryUnderscore {
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('0'..='1') => Step::Continue(State::NumberBinary {
                is_unterminated: false,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('2'..='9') | Some('A'..='Z') | Some('a'..='z') => {
                Step::Continue(State::NumberBinary {
                    is_unterminated: false,
                    has_invalid_characters: true,
                    has_duplicate_underscore,
                })
            }
            Some('_') => Step::Continue(State::NumberBinary {
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore: true,
            }),
            Some(_) | None => Step::Abort(Token::IntegerLiteral {
                base: IntegerBase::Binary,
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberOctal {
            is_unterminated,
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('0'..='7') => Step::Continue(State::NumberOctal {
                is_unterminated: false,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('2'..='9') | Some('A'..='Z') | Some('a'..='z') => {
                Step::Continue(State::NumberOctal {
                    is_unterminated: false,
                    has_invalid_characters: true,
                    has_duplicate_underscore,
                })
            }
            Some('_') => Step::Continue(State::NumberOctalUnderscore {
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some(_) | None => Step::Abort(Token::IntegerLiteral {
                base: IntegerBase::Octal,
                is_unterminated,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberOctalUnderscore {
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('0'..='7') => Step::Continue(State::NumberOctal {
                is_unterminated: false,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('2'..='9') | Some('A'..='Z') | Some('a'..='z') => {
                Step::Continue(State::NumberOctal {
                    is_unterminated: false,
                    has_invalid_characters: true,
                    has_duplicate_underscore,
                })
            }
            Some('_') => Step::Continue(State::NumberOctal {
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore: true,
            }),
            Some(_) | None => Step::Abort(Token::IntegerLiteral {
                base: IntegerBase::Octal,
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberHex {
            is_unterminated,
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('0'..='9') | Some('A'..='F') | Some('a'..='f') => {
                Step::Continue(State::NumberHex {
                    is_unterminated: false,
                    has_invalid_characters,
                    has_duplicate_underscore,
                })
            }
            Some('G'..='O') | Some('g'..='o') | Some('Q'..='Z') | Some('q'..='z') => {
                Step::Continue(State::NumberHex {
                    is_unterminated: false,
                    has_invalid_characters: true,
                    has_duplicate_underscore,
                })
            }
            Some('_') => Step::Continue(State::NumberHexUnderscore {
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('.') => Step::Continue(State::NumberHexDot {
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('p') | Some('P') => Step::Continue(State::NumberHexExponent {
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some(_) | None => Step::Abort(Token::IntegerLiteral {
                base: IntegerBase::Hexadecimal,
                is_unterminated,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberHexUnderscore {
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('0'..='9') | Some('A'..='F') | Some('a'..='f') => {
                Step::Continue(State::NumberHex {
                    is_unterminated: false,
                    has_invalid_characters,
                    has_duplicate_underscore,
                })
            }
            Some('G'..='Z') | Some('g'..='z') => Step::Continue(State::NumberHex {
                is_unterminated: false,
                has_invalid_characters: true,
                has_duplicate_underscore,
            }),
            Some('_') => Step::Continue(State::NumberHex {
                is_unterminated: false,
                has_invalid_characters,
                has_duplicate_underscore: false,
            }),
            Some(_) | None => Step::Abort(Token::IntegerLiteral {
                base: IntegerBase::Hexadecimal,
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberHexDot {
            is_unterminated,
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('0'..='9') | Some('A'..='F') | Some('a'..='f') => {
                Step::Continue(State::NumberHexDot {
                    is_unterminated: false,
                    has_invalid_characters,
                    has_duplicate_underscore,
                })
            }
            Some('_') => Step::Continue(State::NumberHexDotUnderscore {
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('.') => Step::Backtrack,
            Some('p') | Some('P') => Step::Continue(State::NumberHexExponent {
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some(_) | None => Step::Abort(Token::FloatLiteral {
                base: FloatBase::Hexadecimal,
                is_unterminated,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberHexDotUnderscore {
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('0'..='9') | Some('A'..='F') | Some('a'..='f') => {
                Step::Continue(State::NumberHexDot {
                    is_unterminated: false,
                    has_invalid_characters,
                    has_duplicate_underscore,
                })
            }
            Some('_') => Step::Continue(State::NumberHexDot {
                is_unterminated: false,
                has_invalid_characters,
                has_duplicate_underscore: true,
            }),
            Some(_) | None => Step::Abort(Token::FloatLiteral {
                base: FloatBase::Hexadecimal,
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberHexExponent {
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('+') | Some('-') => Step::Continue(State::NumberHexExponentSign {
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some(_) | None => Step::Reprocess(State::NumberHexExponentSign {
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberHexExponentSign {
            is_unterminated,
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('0'..='9') | Some('A'..='F') | Some('a'..='f') => {
                Step::Continue(State::NumberHexExponentSign {
                    is_unterminated: false,
                    has_invalid_characters,
                    has_duplicate_underscore,
                })
            }
            Some('_') => Step::Continue(State::NumberHexExponentSignUnderscore {
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some(_) | None => Step::Abort(Token::FloatLiteral {
                base: FloatBase::Hexadecimal,
                is_unterminated,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberHexExponentSignUnderscore {
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('0'..='9') | Some('A'..='F') | Some('a'..='f') => {
                Step::Continue(State::NumberHexExponentSign {
                    is_unterminated: false,
                    has_invalid_characters,
                    has_duplicate_underscore,
                })
            }
            Some('_') => Step::Continue(State::NumberHexExponentSign {
                is_unterminated: false,
                has_invalid_characters,
                has_duplicate_underscore: true,
            }),
            Some(_) | None => Step::Abort(Token::FloatLiteral {
                base: FloatBase::Hexadecimal,
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberDot {
            is_unterminated,
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('0'..='9') => Step::Continue(State::NumberDot {
                is_unterminated: false,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('_') => Step::Continue(State::NumberDotUnderscore {
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('.') => Step::Backtrack,
            Some('e') | Some('E') => Step::Continue(State::NumberExponent {
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some(_) | None => Step::Abort(Token::FloatLiteral {
                base: FloatBase::Decimal,
                is_unterminated,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
        State::NumberDotUnderscore {
            has_invalid_characters,
            has_duplicate_underscore,
        } => match c {
            Some('0'..='9') => Step::Continue(State::NumberDot {
                is_unterminated: false,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
            Some('A'..='Z') | Some('a'..='z') => Step::Continue(State::NumberDot {
                is_unterminated: false,
                has_invalid_characters: true,
                has_duplicate_underscore,
            }),
            Some('_') => Step::Continue(State::NumberDot {
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore: true,
            }),
            Some(_) | None => Step::Abort(Token::FloatLiteral {
                base: FloatBase::Decimal,
                is_unterminated: true,
                has_invalid_characters,
                has_duplicate_underscore,
            }),
        },
    }
}

pub fn lex(s: &str) -> (Token, usize) {
    let mut state = State::Start;
    let mut iter = s.chars();
    let (mut cur_pos, mut last_pos) = (0, None);
    'outer: while let Some(c) = iter.next() {
        'inner: loop {
            match step(state, Some(c)) {
                Step::Continue(new_state) => {
                    last_pos = Some(cur_pos);
                    cur_pos += c.len_utf8();
                    state = new_state;
                    continue 'outer;
                }
                Step::Reprocess(new_state) => {
                    state = new_state;
                    continue 'inner;
                }
                Step::Backtrack => {
                    return lex(&s[..last_pos.unwrap().into()]);
                }
                Step::Abort(kind) => {
                    return (kind, cur_pos);
                }
                Step::End(kind) => {
                    cur_pos += c.len_utf8();
                    return (kind, cur_pos);
                }
            }
        }
    }
    loop {
        match step(state, None) {
            Step::Reprocess(new_state) => {
                state = new_state;
                continue;
            }
            Step::Backtrack => {
                return lex(&s[..last_pos.unwrap().into()]);
            }
            Step::Abort(kind) => {
                return (kind, cur_pos);
            }
            Step::Continue(_) | Step::End(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_invalid() {
        assert_eq!(lex(""), (Token::Invalid, 0));
        assert_eq!(lex("$"), (Token::Invalid, 1));
        assert_eq!(lex("$$"), (Token::Invalid, 2));
        assert_eq!(lex("$0"), (Token::Invalid, 1));
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(lex(" "), (Token::Whitespace, 1));
        assert_eq!(lex(" \t\r\n"), (Token::Whitespace, 4));
    }

    #[test]
    fn test_comment() {
        assert_eq!(lex("// \n"), (Token::Comment, 3));
        assert_eq!(lex("//hello\n"), (Token::Comment, 7));
        assert_eq!(lex("// hello\n"), (Token::Comment, 8));
    }

    #[test]
    fn test_doc_comment() {
        assert_eq!(lex("/// \n"), (Token::DocComment, 4));
        assert_eq!(lex("///hello\n"), (Token::DocComment, 8));
        assert_eq!(lex("/// hello\n"), (Token::DocComment, 9));
    }

    #[test]
    fn test_builtin() {
        assert_eq!(lex("@hello"), (Token::Builtin, 6));
        assert_eq!(lex("@hello "), (Token::Builtin, 6));
    }

    #[test]
    fn test_identifier() {
        fn raw(is_unterminated: bool, has_invalid_escape: bool) -> Token {
            Token::RawIdentifier {
                is_unterminated,
                has_invalid_escape,
            }
        }
        assert_eq!(lex("foo"), (Token::Identifier, 3));
        assert_eq!(lex("foo1"), (Token::Identifier, 4));
        assert_eq!(lex("_foo1"), (Token::Identifier, 5));
        assert_eq!(lex(r#"@"foo bar""#), (raw(false, false), 10));
        assert_eq!(lex(r#"@"foo bar\x00""#), (raw(false, false), 14));
        assert_eq!(lex(r#"@"foo bar\x0""#), (raw(false, true), 13));
        assert_eq!(lex(r#"@"foo bar"#), (raw(true, false), 9));
    }

    #[test]
    fn test_integer_literal() {
        fn lit(
            base: IntegerBase,
            is_unterminated: bool,
            has_invalid_characters: bool,
            has_duplicate_underscore: bool,
        ) -> Token {
            Token::IntegerLiteral {
                base,
                is_unterminated,
                has_invalid_characters,
                has_duplicate_underscore,
            }
        }
        use IntegerBase::{Binary as Bin, Decimal as Dec, Hexadecimal as Hex, Octal as Oct};
        assert_eq!(lex("100"), (lit(Dec, false, false, false), 3));
        assert_eq!(lex("001"), (lit(Dec, false, false, false), 3));
        assert_eq!(lex("100_000"), (lit(Dec, false, false, false), 7));
        assert_eq!(lex("0b"), (lit(Bin, true, false, false), 2));
        assert_eq!(lex("0b1"), (lit(Bin, false, false, false), 3));
        assert_eq!(lex("0b1_"), (lit(Bin, true, false, false), 4));
        assert_eq!(lex("0b1_0"), (lit(Bin, false, false, false), 5));
        assert_eq!(lex("0b12"), (lit(Bin, false, true, false), 4));
        assert_eq!(lex("0o"), (lit(Oct, true, false, false), 2));
        assert_eq!(lex("0x"), (lit(Hex, true, false, false), 2));
        assert_eq!(lex("1_"), (lit(Dec, true, false, false), 2));
    }

    #[test]
    fn test_float_literal() {
        fn lit(
            base: FloatBase,
            is_unterminated: bool,
            has_invalid_characters: bool,
            has_duplicate_underscore: bool,
        ) -> Token {
            Token::FloatLiteral {
                base,
                is_unterminated,
                has_invalid_characters,
                has_duplicate_underscore,
            }
        }
        use FloatBase::{Decimal as Dec, Hexadecimal as Hex};
        assert_eq!(lex("1."), (lit(Dec, true, false, false), 2));
        assert_eq!(lex("1._"), (lit(Dec, true, false, false), 3));
        assert_eq!(lex("1.__"), (lit(Dec, true, false, true), 4));
        assert_eq!(lex("1.5"), (lit(Dec, false, false, false), 3));
        assert_eq!(lex("1.5f"), (lit(Dec, false, false, false), 3));
        assert_eq!(lex("1.5_"), (lit(Dec, true, false, false), 4));
        assert_eq!(lex("0x1.f"), (lit(Hex, false, false, false), 5));
        assert_eq!(lex("0x1f.5f"), (lit(Hex, false, false, false), 7));
        assert_eq!(lex("1p1"), (lit(Hex, false, false, false), 3));
        assert_eq!(lex("1P1"), (lit(Hex, false, false, false), 3));
        assert_eq!(lex("1p+1"), (lit(Hex, false, false, false), 4));
        assert_eq!(lex("1p-1"), (lit(Hex, false, false, false), 4));
        assert_eq!(lex("1p1_2"), (lit(Hex, false, false, false), 5));
        assert_eq!(lex("1p+1_2"), (lit(Hex, false, false, false), 6));
        assert_eq!(lex("1p-1_2"), (lit(Hex, false, false, false), 6));
        assert_eq!(lex("1p1_2_"), (lit(Hex, true, false, false), 6));
        assert_eq!(lex("1p+1_2_"), (lit(Hex, true, false, false), 7));
        assert_eq!(lex("1p-1_2_"), (lit(Hex, true, false, false), 7));
        assert_eq!(lex("1E1"), (lit(Dec, false, false, false), 3));
        assert_eq!(lex("1e1"), (lit(Dec, false, false, false), 3));
        assert_eq!(lex("1e+1"), (lit(Dec, false, false, false), 4));
        assert_eq!(lex("1e-1"), (lit(Dec, false, false, false), 4));
        assert_eq!(lex("0x1p27"), (lit(Hex, false, false, false), 6));
        assert_eq!(lex("0x1.p+64"), (lit(Hex, false, false, false), 8));
        assert_eq!(lex("1e-7"), (lit(Dec, false, false, false), 4));
    }

    #[test]
    fn test_string_literal() {
        fn lit(is_unterminated: bool, has_invalid_escape: bool) -> Token {
            Token::StringLiteral {
                is_unterminated,
                has_invalid_escape,
            }
        }
        assert_eq!(lex(r#""foo bar""#), (lit(false, false), 9));
        assert_eq!(lex(r#""foo bar\x00""#), (lit(false, false), 13));
        assert_eq!(lex(r#""foo bar\x0""#), (lit(false, true), 12));
        assert_eq!(lex(r#""foo bar"#), (lit(true, false), 8));
        assert_eq!(lex(r#""foo bar\\\"\'\r\n\t""#), (lit(false, false), 21));
        assert_eq!(lex(r#""foo bar\xFF""#), (lit(false, false), 13));
        assert_eq!(lex(r#""foo bar\xFG""#), (lit(false, true), 13));
        assert_eq!(lex(r#""foo bar\xGF""#), (lit(false, true), 13));
        assert_eq!(lex(r#""foo bar\xGG""#), (lit(false, true), 13));
        assert_eq!(lex(r#""foo bar\u{""#), (lit(false, true), 12));
        assert_eq!(lex(r#""foo bar\u{1""#), (lit(false, true), 13));
        assert_eq!(lex(r#""foo bar\u{1F""#), (lit(false, true), 14));
        assert_eq!(lex(r#""foo bar\u{1G""#), (lit(false, true), 14));
        assert_eq!(lex(r#""foo bar\u{1G}""#), (lit(false, true), 15));
        assert_eq!(lex(r#""foo bar\u{1F}""#), (lit(false, false), 15));
    }

    #[test]
    fn test_character_literal() {
        fn lit(is_empty: bool, is_unterminated: bool, has_invalid_escape: bool) -> Token {
            Token::CharacterLiteral {
                is_empty,
                is_unterminated,
                has_invalid_escape,
            }
        }
        assert_eq!(lex("''"), (lit(true, false, false), 2));
        assert_eq!(lex("'a"), (lit(false, true, false), 2));
        assert_eq!(lex("'\\x0'"), (lit(false, false, true), 5));
        assert_eq!(lex("'\\xzz'"), (lit(false, false, true), 6));
        assert_eq!(lex("'a'"), (lit(false, false, false), 3));
    }

    #[test]
    fn test_multiline_string_literal() {
        assert_eq!(lex("\\\\"), (Token::MultilineStringLiteralLine, 2));
        assert_eq!(lex("\\\\hello"), (Token::MultilineStringLiteralLine, 7));
        assert_eq!(lex("\\\\ hello\n"), (Token::MultilineStringLiteralLine, 8));
    }

    #[test]
    fn test_punctuation_1() {
        assert_eq!(lex("!"), (Token::Bang, 1));
        assert_eq!(lex("%"), (Token::Percent, 1));
        assert_eq!(lex("&"), (Token::And, 1));
        assert_eq!(lex("("), (Token::LParen, 1));
        assert_eq!(lex(")"), (Token::RParen, 1));
        assert_eq!(lex("*"), (Token::Star, 1));
        assert_eq!(lex("+"), (Token::Plus, 1));
        assert_eq!(lex(","), (Token::Comma, 1));
        assert_eq!(lex("-"), (Token::Minus, 1));
        assert_eq!(lex("."), (Token::Dot, 1));
        assert_eq!(lex("/"), (Token::Slash, 1));
        assert_eq!(lex(":"), (Token::Colon, 1));
        assert_eq!(lex(";"), (Token::Semicolon, 1));
        assert_eq!(lex("<"), (Token::LAngle, 1));
        assert_eq!(lex("="), (Token::Equal, 1));
        assert_eq!(lex(">"), (Token::RAngle, 1));
        assert_eq!(lex("?"), (Token::Question, 1));
        assert_eq!(lex("@"), (Token::At, 1));
        assert_eq!(lex("["), (Token::LBracket, 1));
        assert_eq!(lex("]"), (Token::RBracket, 1));
        assert_eq!(lex("^"), (Token::Caret, 1));
        assert_eq!(lex("{"), (Token::LBrace, 1));
        assert_eq!(lex("|"), (Token::Or, 1));
        assert_eq!(lex("}"), (Token::RBrace, 1));
    }

    #[test]
    fn test_punctuation_2() {
        assert_eq!(lex("!="), (Token::BangEqual, 2));
        assert_eq!(lex("%="), (Token::PercentEqual, 2));
        assert_eq!(lex("&&"), (Token::And2, 2));
        assert_eq!(lex("&="), (Token::AndEqual, 2));
        assert_eq!(lex("**"), (Token::Star2, 2));
        assert_eq!(lex("*="), (Token::StarEqual, 2));
        assert_eq!(lex("*%"), (Token::StarPercent, 2));
        assert_eq!(lex("++"), (Token::Plus2, 2));
        assert_eq!(lex("+="), (Token::PlusEqual, 2));
        assert_eq!(lex("+%"), (Token::PlusPercent, 2));
        assert_eq!(lex("-="), (Token::MinusEqual, 2));
        assert_eq!(lex("-%"), (Token::MinusPercent, 2));
        assert_eq!(lex(".."), (Token::Dot2, 2));
        assert_eq!(lex(".*"), (Token::DotStar, 2));
        assert_eq!(lex("/="), (Token::SlashEqual, 2));
        assert_eq!(lex("<="), (Token::LAngleEqual, 2));
        assert_eq!(lex("<<"), (Token::LAngle2, 2));
        assert_eq!(lex("=="), (Token::Equal2, 2));
        assert_eq!(lex("=>"), (Token::EqualRAngle, 2));
        assert_eq!(lex(">="), (Token::RAngleEqual, 2));
        assert_eq!(lex(">>"), (Token::RAngle2, 2));
        assert_eq!(lex("^="), (Token::CaretEqual, 2));
        assert_eq!(lex("||"), (Token::Or2, 2));
        assert_eq!(lex("|="), (Token::OrEqual, 2));
    }

    #[test]
    fn test_punctuation_3() {
        assert_eq!(lex("*%="), (Token::StarPercentEqual, 3));
        assert_eq!(lex("+%="), (Token::PlusPercentEqual, 3));
        assert_eq!(lex("-%="), (Token::MinusPercentEqual, 3));
        assert_eq!(lex("..."), (Token::Dot3, 3));
        assert_eq!(lex("<<="), (Token::LAngle2Equal, 3));
        assert_eq!(lex(">>="), (Token::RAngle2Equal, 3));
    }
}
