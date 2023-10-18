use crate::utils::TypeIcons;
use colored::*;
use std::path::Path;

pub struct Consola {
    pub unicode_supported: bool,
    pub type_icons: TypeIcons,
}

impl Consola {
    pub fn info(&self, message: &str) {
        println!(
            "{} {}",
            self.type_icons.info.as_str().green(),
            message.white()
        );
    }

    pub fn start(&self, message: &str) {
        println!(
            "{} {}",
            self.type_icons.start.as_str().purple(),
            message.white()
        );
    }

    pub fn warn(&self, message: &str) {
        println!("\n{} {}\n", " WARN ".on_yellow(), message.white());
    }

    pub fn success(&self, message: &str) {
        println!(
            "{} {}",
            self.type_icons.success.as_str().green(),
            message.white()
        );
    }

    pub fn error(&self, message: &str, len: Option<i8>) {
        println!("\n{} {}\n", " ERROR ".on_red(), message.white());

        let mut frames_left: i8 = if len.is_none() { 125 } else { len.unwrap() };

        backtrace::trace(|frame| {
            backtrace::resolve_frame(frame, |symbol| {
                if symbol.filename().is_none() {
                    frames_left = 0;

                    return;
                }

                let name = symbol.name().unwrap().to_string();

                let mut iter = Path::new(symbol.filename().unwrap().to_str().unwrap())
                    .iter()
                    .rev();
                let mut filename = String::new();
                let file = iter.next().unwrap().to_str().unwrap();
                let dir = iter.next().unwrap().to_str().unwrap();

                filename.push_str(dir);
                filename.push_str("/");
                filename.push_str(file);

                println!(
                    "    {} {} ({})",
                    "at".truecolor(100, 100, 100),
                    name,
                    filename.as_str().green()
                );
            });

            if frames_left == 0 {
                return false;
            }

            frames_left = frames_left - 1;

            return true;
        });

        println!();
    }
}
