use std::{borrow::Cow, fmt::Display, io::stdin, str::FromStr};

use clap::{Parser, ValueEnum};
use colored_json::ToColoredJson;
use serde_json::Value;
use yansi::Paint;

enum Level<'a> {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
    Unknown(Cow<'a, str>),
}

impl<'a> From<Cow<'a, str>> for Level<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        match value.to_uppercase().as_str() {
            "ERROR" => Self::Error,
            "WARN" => Self::Warn,
            "INFO" => Self::Info,
            "DEBUG" => Self::Debug,
            "TRACE" => Self::Trace,
            _ => Self::Unknown(value),
        }
    }
}

impl<'a> FromStr for Level<'a> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(Cow::from(s.to_owned())))
    }
}

impl<'a> Display for Level<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            &match self {
                Level::Error => Paint::red("ERROR"),
                Level::Warn => Paint::yellow("WARN"),
                Level::Info => Paint::green("INFO"),
                Level::Debug => Paint::blue("DEBUG"),
                Level::Trace => Paint::new("TRACE"),
                Level::Unknown(s) => Paint::new(s.as_ref()),
            }
        ))
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Format {
    /// Similar to what `tracing_subscriber` outputs.
    Line,
    /// Coloured json, with a level heading per line.
    JSON,
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Format::Line => "line",
            Format::JSON => "json",
        })
    }
}

/// Format json structured log message lines from stdin into ANSI coloured stdout.
#[derive(Parser, Debug)]
struct Args {
    /// JSON pointer to the message string.
    #[arg(short, long, default_value = "/fields/message")]
    message_pointer: String,
    /// JSON pointer to the timestamp string.
    #[arg(short, long, default_value = "/timestamp")]
    timestamp_pointer: String,
    /// JSON pointer to the log level string.
    #[arg(short, long, default_value = "/level")]
    level_pointer: String,
    /// What output format to use.
    #[arg(short, long, default_value_t = Format::Line)]
    format: Format,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let stdin = stdin();

    for result in stdin.lines() {
        let line = result?;

        match args.format {
            Format::Line => format_line(&line, &args)?,
            Format::JSON => format_json(&line, &args)?,
        }
        print!("\n");
    }

    Ok(())
}

fn format_line(line: &str, args: &Args) -> anyhow::Result<()> {
    let log_line: serde_json::Value = serde_json::from_str(&line)?;

    if let Some(timestamp) = log_line
        .pointer(&args.timestamp_pointer)
        .and_then(Value::as_str)
    {
        print!(
            "[{}]",
            Paint::new(timestamp).fg(yansi::Color::RGB(180, 180, 180))
        );
    }

    if let Some(log_level) = log_line
        .pointer(&args.level_pointer)
        .and_then(Value::as_str)
        .map(Cow::Borrowed)
        .map(Level::from)
    {
        print!(" {log_level}");
    }

    if let Some(target) = log_line.pointer("/target").and_then(Value::as_str) {
        print!(
            " {}",
            Paint::new(target).fg(yansi::Color::RGB(128, 128, 128))
        );
    }

    let spans: Vec<String> = log_line
        .pointer("/spans")
        .and_then(Value::as_array)
        .iter()
        .flat_map(|spans| {
            spans.iter().filter_map(|span| {
                let name = span.pointer("/name").and_then(Value::as_str)?;

                let object = span.as_object()?;
                let pairs: Vec<String> = object
                    .iter()
                    .filter_map(|(key, value)| {
                        if key.as_str() == "name" && value.as_str() == Some(name) {
                            return None;
                        }

                        let pair = format!("{key}={value}");
                        Some(pair)
                    })
                    .collect();

                let pairs = pairs.join(",");

                let name_coloured = Paint::new(name).fg(yansi::Color::RGB(128, 128, 255));

                let pairs_coloured_with_braces = if !pairs.is_empty() {
                    Paint::new(format!("{{{pairs}}}"))
                        .fg(yansi::Color::RGB(200, 200, 255))
                        .to_string()
                } else {
                    String::new()
                };

                Some(format!("{name_coloured}{pairs_coloured_with_braces}"))
            })
        })
        .collect();

    print!(
        " {}",
        spans.join(
            &Paint::new("::")
                .fg(yansi::Color::RGB(128, 128, 255))
                .to_string()
        )
    );

    if let Some(message) = log_line
        .pointer(&args.message_pointer)
        .and_then(Value::as_str)
    {
        print!(": {message}");
    }

    Ok(())
}

fn format_json(line: &str, args: &Args) -> anyhow::Result<()> {
    let log_line: serde_json::Value = serde_json::from_str(&line)?;
    if let Some(log_level) = log_line
        .pointer(&args.level_pointer)
        .and_then(Value::as_str)
        .map(Cow::Borrowed)
        .map(Level::from)
    {
        print!("{log_level}:\n");
    }
    print!("{}", line.to_colored_json_auto()?);
    Ok(())
}
