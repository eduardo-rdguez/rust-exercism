use rust_exercism::enums::semi_structured_logs::{debug, error, info, log, warn, LogLevel};

static MESSAGE: &str = "Stack overflow";

#[test]
fn emits_info() {
    let expected_message = format!("[INFO]: {}", MESSAGE);
    assert_eq!(expected_message, info(MESSAGE))
}

#[test]
fn emits_warning() {
    let expected_message = format!("[WARNING]: {}", MESSAGE);
    assert_eq!(expected_message, warn(MESSAGE))
}

#[test]
fn emits_error() {
    let expected_message = format!("[ERROR]: {}", MESSAGE);
    assert_eq!(expected_message, error(MESSAGE))
}

#[test]
fn log_emits_info() {
    let expected_message = format!("[INFO]: {}", MESSAGE);
    assert_eq!(expected_message, log(LogLevel::Info, MESSAGE))
}

#[test]
fn log_emits_warning() {
    let expected_message = format!("[WARNING]: {}", MESSAGE);
    assert_eq!(expected_message, log(LogLevel::Warning, MESSAGE))
}

#[test]
fn log_emits_error() {
    let expected_message = format!("[ERROR]: {}", MESSAGE);
    assert_eq!(expected_message, log(LogLevel::Error, MESSAGE))
}

#[test]
fn emits_variant() {
    let expected_message = format!("[DEBUG]: {}", MESSAGE);
    assert_eq!(expected_message, debug(MESSAGE))
}

#[test]
fn log_emits_variant() {
    let expected_message = format!("[DEBUG]: {}", MESSAGE);
    assert_eq!(expected_message, log(LogLevel::Debug, MESSAGE))
}
