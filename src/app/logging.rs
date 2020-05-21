use sloggers::terminal::{Destination, TerminalLoggerBuilder};
use sloggers::types::Severity;
use sloggers::Build;

pub fn create_logger(verbosity: u8) -> anyhow::Result<slog::Logger> {
    let mut logger_builder = TerminalLoggerBuilder::new();
    logger_builder.level(verbosity_to_severity(verbosity));
    logger_builder.destination(Destination::Stderr);
    let logger = logger_builder.build()?;

    Ok(logger)
}

fn verbosity_to_severity(verbosity: u8) -> Severity {
    match verbosity {
        std::u8::MIN..=0 => Severity::Error,
        1 => Severity::Warning,
        2 => Severity::Info,
        3 => Severity::Debug,
        4..=std::u8::MAX => Severity::Trace,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn common_severities_converted() {
        assert_eq!(Severity::Error, verbosity_to_severity(0));
        assert_eq!(Severity::Warning, verbosity_to_severity(1));
        assert_eq!(Severity::Info, verbosity_to_severity(2));
        assert_eq!(Severity::Debug, verbosity_to_severity(3));
        assert_eq!(Severity::Trace, verbosity_to_severity(4));
    }
}
