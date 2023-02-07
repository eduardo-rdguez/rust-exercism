use rust_exercism::enums::semi_estructured_logs::{error, info, log, warn, LogLevel};

static MESSAGE: &str = "Stack overflow";

#[test]
fn emits_info() {
    let message_expected = format!("[INFO]: {}", MESSAGE);
    assert_eq!(message_expected, info(MESSAGE))
}

#[test]
fn emits_warning() {
    let message_expected = format!("[WARNING]: {}", MESSAGE);
    assert_eq!(message_expected, warn(MESSAGE))
}

#[test]
fn emits_error() {
    let message_expected = format!("[ERROR]: {}", MESSAGE);
    assert_eq!(message_expected, error(MESSAGE))
}

#[test]
fn log_emits_info() {
    let message_expected = format!("[INFO]: {}", MESSAGE);
    assert_eq!(message_expected, log(LogLevel::Info, MESSAGE))
}

#[test]
fn log_emits_warning() {
    let message_expected = format!("[WARNING]: {}", MESSAGE);
    assert_eq!(message_expected, log(LogLevel::Warning, MESSAGE))
}

#[test]
fn log_emits_error() {
    let message_expected = format!("[ERROR]: {}", MESSAGE);
    assert_eq!(message_expected, log(LogLevel::Error, MESSAGE))
}

#[test]
fn emmits_variant() {
    let message_expected = format!("[DEBUG]: {}", MESSAGE);
    assert_eq!(message_expected, log(LogLevel::Debug, MESSAGE))
}
