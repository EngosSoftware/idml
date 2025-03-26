use idml::tokenize;

#[test]
fn _0001() {
  // Empty input is invalid.
  let input = "";
  assert_eq!("empty input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0002() {
  // No node name, only delimiter present.
  let input = ".";
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0003() {
  // No node name, space after delimiter.
  let input = ". ";
  assert_eq!("unexpected character: ' ' 0x20 at row 1 and column 2", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0004() {
  // No node name, line feed after delimiter.
  let input = ".\n";
  assert_eq!("unexpected character: '\n' 0xA at row 1 and column 2", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0005() {
  // No node name, carriage return after delimiter.
  let input = ".\r";
  assert_eq!("unexpected character: '\r' 0xD at row 1 and column 2", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0006() {
  // No node name, carriage return and line feed after delimiter.
  let input = ".\r\n";
  assert_eq!("unexpected character: '\r' 0xD at row 1 and column 2", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0007() {
  // Node name is not followed by a whitespace, newline or both.
  let input = ".A";
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0008() {
  // Node name is not followed by a whitespace and newline.
  let input = ".A ";
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0009() {
  // Root node name must be at the very beginning of the line, without any indentation.
  let input = " .A\n";
  assert_eq!("unexpected character: ' ' 0x20 at row 1 and column 1", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0010() {
  // Root node name must be at the very beginning of the line,
  // without any additional characters before it.
  let input = "n.A\n";
  assert_eq!("unexpected character: 'n' 0x6E at row 1 and column 1", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0011() {
  // Node name must start with a single dot.
  let input = "..A\n";
  assert_eq!("unexpected character: '.' 0x2E at row 1 and column 2", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0012() {
  // Node name must end with whitespace or newline or whitespace and newline.
  let input = ".A!\n";
  assert_eq!("unexpected character: '!' 0x21 at row 1 and column 3", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0013() {
  // After delimiter there must be a node name start.
  let input = ".A\n   .!\n";
  assert_eq!("unexpected character: '!' 0x21 at row 2 and column 5", tokenize(input).unwrap_err().to_string());
}

#[test]
fn _0014() {
  // Spaces after the last newline character.
  let input = r#".MODEL
    .NAMESPACE https://decision-toolkit.org/2_0001/
  "#;
  assert_eq!("unexpected end of input", tokenize(input).unwrap_err().to_string());
}
