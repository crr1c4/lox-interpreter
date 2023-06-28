use lox_interpreter::compiler::{*, token::TokenType};

#[test]

fn cast_enum_index() {
    assert_eq!(TokenType::RightParen as usize, usize::from(1u8));
}

