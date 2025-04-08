use idml::{parse, Parser, Token, WS};

#[test]
fn _0001() {
  // Empty input.
  let input = "";
  assert_eq!("empty input", parse(input).unwrap_err().to_string());
}

#[test]
fn _0002() {
  // Input beginning with space(s) is invalid.
  let input = " ";
  assert_eq!("unexpected character: ' ' 0x20 at row 1 and column 1", parse(input).unwrap_err().to_string());
}

#[test]
fn _0003() {
  // Input beginning with tab(s) is invalid.
  let input = "\t";
  assert_eq!("unexpected character: '\t' 0x09 at row 1 and column 1", parse(input).unwrap_err().to_string());
}

#[test]
fn _0004() {
  // Only delimiter present.
  let input = ".";
  assert_eq!("unexpected end of input", parse(input).unwrap_err().to_string());
}

#[test]
fn _0005() {
  // No newline after empty name.
  let input = ". ";
  assert_eq!("unexpected end of input", parse(input).unwrap_err().to_string());
}

#[test]
fn _0006() {
  for ch in '\u{0001}'..='\u{0020}' {
    println!("{}", ch as u8);
    let input = format!("{}", ch);
    let expected = format!("unexpected character: '{}' 0x{:02X} at row 1 and column 1", ch, ch as u8);
    assert_eq!(expected, parse(&input).unwrap_err().to_string());
  }
  assert_eq!("unexpected character: '\r' 0x0D at row 1 and column 1", parse("\r\n").unwrap_err().to_string());
}

#[test]
fn _0007() {
  for ch in '\u{0001}'..='\u{0019}' {
    if !matches!(ch, '\n' | '\r' | '\t') {
      let input = format!(".{}", ch);
      let expected = format!("unexpected character: '{}' 0x{:02X} at row 1 and column 2", ch, ch as u8);
      assert_eq!(expected, parse(&input).unwrap_err().to_string());
    }
  }
}

#[test]
fn _0008() {
  // Node name is not followed by a whitespace, newline or both.
  let input = ".A";
  assert_eq!("unexpected end of input", parse(input).unwrap_err().to_string());
}

#[test]
fn _0009() {
  // Node name is not followed by a whitespace and newline.
  let input = ".A ";
  assert_eq!("unexpected end of input", parse(input).unwrap_err().to_string());
}

#[test]
fn _0010() {
  // Root node name must be at the very beginning of the line, without any indentation before like space.
  let input = " .A\n";
  assert_eq!("unexpected character: ' ' 0x20 at row 1 and column 1", parse(input).unwrap_err().to_string());
}

#[test]
fn _0011() {
  // Root node name must be at the very beginning of the line, without any indentation before like horizontal tab.
  let input = "\t.A\n";
  assert_eq!("unexpected character: '\t' 0x09 at row 1 and column 1", parse(input).unwrap_err().to_string());
}

#[test]
fn _0012() {
  // Spaces after the last newline character.
  let input = r#".MODEL
    .NAMESPACE https://decision-toolkit.org/2_0001/
  "#;
  assert_eq!("unexpected end of input", parse(input).unwrap_err().to_string());
}

#[test]
fn _0013() {
  // Malformed indentation
  let input = r#".A
    .B
    .C
   .D
"#;
  assert_eq!("malformed indentation 3, expected multiplication of 4", parse(input).unwrap_err().to_string());
}

#[test]
fn _0014() {
  // Malformed indentation
  let input = r#".A
        .B
    .C
    .D
"#;
  assert_eq!("malformed indentation 4, expected multiplication of 8", parse(input).unwrap_err().to_string());
}

#[test]
fn _0015() {
  // Inconsistent indentation
  let input = ".A\n  .B\n\t\t.C\n  .D\n";
  assert_eq!("inconsistent indentation, mixed spaces and tabs", parse(input).unwrap_err().to_string());
}

#[test]
fn _0016() {
  // Inconsistent indentation
  let input = ".A\n  .B\n \t.C\n  .D\n";
  assert_eq!("inconsistent indentation, mixed spaces and tabs", parse(input).unwrap_err().to_string());
}

#[test]
fn _0017() {
  // No indentation token.
  let tokens = vec![Token::NodeName("A".to_string(), '.')];
  assert_eq!("expected indentation token", Parser::new(tokens).parse().unwrap_err().to_string())
}

#[test]
fn _0018() {
  // No node name token.
  let tokens = vec![Token::Indentation(0, WS), Token::NodeContent("content".to_string())];
  assert_eq!("expected node name token", Parser::new(tokens).parse().unwrap_err().to_string())
}

#[test]
fn _0019() {
  // No node content token.
  let tokens = vec![Token::Indentation(0, WS), Token::NodeName("name".to_string(), '.'), Token::Indentation(0, WS)];
  assert_eq!("expected node content token", Parser::new(tokens).parse().unwrap_err().to_string())
}
