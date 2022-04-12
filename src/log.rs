use std::default::Default;
use std::io::Write;

pub use termcolor::{Color, ColorChoice};
use termcolor::{ColorSpec, StandardStream, WriteColor};

#[allow(dead_code)]
pub enum Stream {
    Stdout,
    Stderr,
}

#[allow(dead_code)]
pub enum Level {
    Error,
    Warn,
    Success,
    Info,
    Verbose,
}

pub struct Logger {
    pub error_color: Color,
    pub warn_color: Color,
    pub success_color: Color,
    pub info_color: Color,
    pub verbose_color: Color,

    pub error_prefix: &'static str,
    pub warn_prefix: &'static str,
    pub success_prefix: &'static str,
    pub info_prefix: &'static str,
    pub verbose_prefix: &'static str,

    out: StandardStream,
}

#[allow(dead_code)]
impl Logger {
    pub fn new(output: Stream, choice: ColorChoice) -> Self {
        Self {
            out: match output {
                Stream::Stdout => StandardStream::stdout(choice),
                Stream::Stderr => StandardStream::stderr(choice),
            },
            ..Default::default()
        }
    }

    fn set_color_fg(&mut self, color: Color) {
        self.out
            .set_color(ColorSpec::new().set_fg(Some(color)))
            .unwrap();
    }

    fn reset_color_fg(&mut self) {
        self.out
            .set_color(ColorSpec::new().set_fg(Some(Color::White)))
            .unwrap();
    }

    pub fn v<T>(&mut self, level: Level, facility: &str, data: T)
    where
        T: std::fmt::Display,
    {
        let color: Color;
        let prefix;

        match level {
            Level::Error => {
                color = self.error_color;
                prefix = self.error_prefix;
            }
            Level::Warn => {
                color = self.warn_color;
                prefix = self.warn_prefix;
            }
            Level::Success => {
                color = self.success_color;
                prefix = self.success_prefix;
            }
            Level::Info => {
                color = self.info_color;
                prefix = self.info_prefix;
            }
            Level::Verbose => {
                color = self.verbose_color;
                prefix = self.verbose_prefix;
            }
        }

        self.set_color_fg(color);
        write!(&mut self.out, "{} {}: ", prefix, facility).unwrap();

        self.reset_color_fg();
        writeln!(&mut self.out, "{}", data).unwrap();
    }
}

#[allow(dead_code)]
impl Default for Logger {
    fn default() -> Self {
        Self {
            error_color: Color::Red,
            warn_color: Color::Yellow,
            success_color: Color::Green,
            info_color: Color::Blue,
            verbose_color: Color::White,

            error_prefix: " ✗ ",
            warn_prefix: " ⚠ ",
            success_prefix: " ✔ ",
            info_prefix: " → ",
            verbose_prefix: " + ",

            out: StandardStream::stdout(ColorChoice::Auto),
        }
    }
}
