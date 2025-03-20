use idml::{Parser, ParserState, Token, parse, parse_tokens};

#[test]
fn _0001() {
  // Empty input.
  let input = "";
  assert_eq!("empty input", parse(&input).unwrap_err().to_string());
}

#[test]
fn _0002() {
  // Malformed indentation
  let input = r#".A
    .B
    .C
   .D
"#;
  assert_eq!("malformed indentation 3, expected multiplication of 4", parse(&input).unwrap_err().to_string());
}

#[test]
fn _0003() {
  // Malformed indentation
  let input = r#".A
        .B
    .C
    .D
"#;
  assert_eq!("malformed indentation 4, expected multiplication of 8", parse(&input).unwrap_err().to_string());
}

#[test]
fn _0004() {
  // No indentation token.
  let tokens = vec![Token::NodeName("A".to_string())];
  assert_eq!("expected indentation token", parse_tokens(tokens).unwrap_err().to_string())
}

#[test]
fn _0005() {
  // No node name token.
  let tokens = vec![Token::Indentation(0), Token::NodeContent("content".to_string())];
  assert_eq!("expected node name token", parse_tokens(tokens).unwrap_err().to_string())
}

#[test]
fn _0006() {
  // No node content token.
  let tokens = vec![Token::Indentation(0), Token::NodeName("name".to_string()), Token::Indentation(0)];
  assert_eq!("expected node content token", parse_tokens(tokens).unwrap_err().to_string())
}

#[test]
fn _0007() {
  // No previous indentation token.
  let tokens = vec![Token::NodeContent("content".to_string())];
  let mut parser = Parser::new(tokens);
  parser.state = ParserState::NodeContent;
  assert_eq!("no previous indentation token", parser.parse().unwrap_err().to_string())
}

#[test]
fn _0008() {
  // No previous node name token.
  let tokens = vec![Token::NodeContent("content".to_string())];
  let mut parser = Parser::new(tokens);
  parser.last_indent = Some(4);
  parser.state = ParserState::NodeContent;
  assert_eq!("no previous node name token", parser.parse().unwrap_err().to_string())
}
