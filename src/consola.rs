use crate::box_styles::BoxStyle;
use crate::utils::TypeIcons;
use colored::*;
use std::collections::HashSet;
use std::path::Path;

pub struct Consola {
    pub unicode_supported: bool,
    pub type_icons: TypeIcons,
    pub box_style: BoxStyle,
}

#[derive(Clone)]
pub struct BoxiOptions {
    pub padding: usize,
    pub fill: bool,
    pub center: bool,
}

impl Default for BoxiOptions {
    fn default() -> Self {
        BoxiOptions {
            padding: 4,
            fill: false,
            center: false,
        }
    }
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
        println!("{} {}", self.type_icons.start.purple(), message.white())
    }

    pub fn warn(&self, message: &str) {
        println!("\n{}  {}\n", self.type_icons.warn.yellow(), message.white());
    }

    pub fn success(&self, message: &str) {
        println!(
            "{}  {}\n",
            self.type_icons.success.as_str().green(),
            message.white()
        );
    }

    pub fn error(&self, message: &str, len: Option<i8>) {
        print!("{}  {}\n", self.type_icons.error.red(), message.white());

        let mut frames_left: i8 = if len.is_none() { 125 } else { len.unwrap() };
        let mut skip_frames: i8 = 4;

        backtrace::trace(|frame| {
            backtrace::resolve_frame(frame, |symbol| {
                if skip_frames > 0 {
                    skip_frames = skip_frames - 1;

                    return;
                }

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
                    self.type_icons.trace.truecolor(100, 100, 100),
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

    pub fn boxi(&self, message: &str, options: Option<BoxiOptions>) {
        let padding = options.clone().unwrap_or_default().padding;
        let fill = options.clone().unwrap_or_default().fill;
        let center = options.clone().unwrap_or_default().center;
        let padding_y = padding / 4;

        let term_size: usize = termsize::get().map(|size| size.cols).unwrap().into();
        let mut messages: Vec<String> = vec![];
        let max_size = term_size - padding / 2 - 2;

        for msg in String::from(message).split("\n") {
            if msg.len() >= max_size {
                let mut words = String::new();

                for word in msg.split(" ") {
                    if &words.len() + word.len() + 1 >= max_size {
                        messages.push(words.clone());

                        message.clear();
                    }

                    if &words.len() < &1 {
                        let _ = &words.insert_str(words.len(), &word);

                        continue;
                    }

                    let _ = &words.insert_str(words.len(), (" ".to_owned() + &word).as_str());
                }

                continue;
            }
        }

        // let horizontals = if fill.clone() {
        //     size - 2
        // } else {
        //     message.len() + padding
        // };

        // if padding + message.len() >= size {
        //     let words = message.split(" ").into_iter();
        //     let mut line = String::new();

        //     for msg in words {
        //         if line.len() + 1 + padding >= size {
        //             messages.insert(line.clone());

        //             line = String::new()
        //         }

        //         if line.len() > 0 {
        //             line.insert_str(line.len(), " ");
        //         }

        //         line.insert_str(line.len(), &msg);
        //     }
        // } else {
        //     messages.insert(String::from(message));
        // }

        // // top box
        // print!("{}", self.box_style.tl);

        // for _ in 0..horizontals {
        //     print!("{}", self.box_style.h);
        // }

        // print!("{}\n", self.box_style.tr);

        // for _ in 0..padding_y {
        //     print!("{}", self.box_style.v);

        //     for _ in 0..horizontals {
        //         print!(" ");
        //     }

        //     print!("{}\n", self.box_style.v);
        // }

        // for msg in messages {
        //     print!("{}", self.box_style.v);

        //     for _ in 0..(padding / 2) {
        //         print!(" ");
        //     }

        //     print!("{}", msg);

        //     for _ in 0..(padding / 2) {
        //         print!(" ");
        //     }

        //     print!("{}\n", self.box_style.v);
        // }

        // // bottom box
        // for _ in 0..padding_y {
        //     print!("{}", self.box_style.v);

        //     for _ in 0..horizontals {
        //         print!(" ");
        //     }

        //     print!("{}", self.box_style.v);
        // }

        // print!("\n{}", self.box_style.bl);

        // for _ in 0..horizontals {
        //     print!("{}", self.box_style.h);
        // }

        // print!("{}\n\n", self.box_style.br);
    }
}
