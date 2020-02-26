use env_logger::fmt::{Color, Style, StyledValue};
use log::{info, Level};
use std::env;
use std::fmt;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

const DEFAULT_LOG_LEVEL: &str = "info";

pub(crate) fn init() {
    setup_log_level_env();
    init_logger();
}

fn setup_log_level_env() {
    let log_level = env::vars()
        .find(|(key, _)| key.eq_ignore_ascii_case("LOG_LEVEL"))
        .map(|(_, val)| val)
        .unwrap_or_else(|| DEFAULT_LOG_LEVEL.to_owned());
    env::set_var("LOG_LEVEL", &log_level);
    info!("Using LOG_LEVEL = {}", &log_level);
}

fn init_logger() {
    let mut builder = env_logger::Builder::new();

    builder.format(|formatter, record| {
        let mut style = formatter.style();
        let time = colored_time(&mut style);

        let target = record.target();
        let max_width = max_target_width(target);

        let mut style = formatter.style();
        let level = colored_level(&mut style, record.level());

        let mut style = formatter.style();
        let target = style.set_bold(true).value(Padded {
            value: target,
            width: max_width,
        });

        writeln!(
            formatter,
            "{} [{}] ({}): {}",
            time,
            level,
            target,
            record.args()
        )
    });

    builder.parse_filters(&env::var("LOG_LEVEL").unwrap());
    builder.init();
}

fn colored_time<'a>(style: &'a mut Style) -> StyledValue<'a, String> {
    let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    style.set_color(Color::Rgb(50, 50, 50)).value(time)
}

// START of Exercept from
// [seanmonstar/pretty-env-logger](https://github.com/seanmonstar/pretty-env-logger/blob/master/src/lib.rs)
struct Padded<T> {
    value: T,
    width: usize,
}

impl<T: fmt::Display> fmt::Display for Padded<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{: <width$}", self.value, width = self.width)
    }
}

static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);

fn max_target_width(target: &str) -> usize {
    let max_width = MAX_MODULE_WIDTH.load(Ordering::Relaxed);
    if max_width < target.len() {
        MAX_MODULE_WIDTH.store(target.len(), Ordering::Relaxed);
        target.len()
    } else {
        max_width
    }
}

fn colored_level<'a>(style: &'a mut Style, level: Level) -> StyledValue<'a, &'static str> {
    match level {
        Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO "),
        Level::Warn => style.set_color(Color::Yellow).value("WARN "),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
    }
}
// END of exercept
