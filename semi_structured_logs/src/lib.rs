pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub fn log(level: LogLevel, message : &str) -> String {
    let level_str = match level {
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR",
    };
    format!("[{}]: {}",level_str,message)
}

pub fn info(message : &str) -> String {
    format!("[INFO]: {}",message)
}

pub fn warn(message : &str) -> String {
    format!("[WARNING]: {}",message)
}

pub fn error(message : &str) -> String {
    format!("[ERROR]: {}",message)
}