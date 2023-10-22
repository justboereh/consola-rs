mod box_styles;
mod consola;
mod reporter;
mod utils;

use box_styles::{BoxStyle, STYLES};
use consola::Consola;
use reporter::Reporter;
use utils::{is_unicode_supported, TypeIcons};

pub struct ConsolaOptions<'a> {
    pub box_style: Option<BoxStyle<'a>>,
}

pub fn create_consola(option: Option<ConsolaOptions>) -> Consola<'_> {
    let unicode_supported = is_unicode_supported();
    let type_icons = TypeIcons::new(unicode_supported);

    let box_style = option
        .unwrap_or(ConsolaOptions { box_style: None })
        .box_style
        .unwrap_or(STYLES.solid);

    Consola {
        type_icons,
        unicode_supported,
        box_style,
        printable: true,
    }
}

pub fn create_reporter(name: String) -> Reporter {
    return Reporter { name };
}

#[cfg(test)]
mod tests {

    use crate::{consola::BoxiOptions, create_consola};

    #[test]
    fn basic_test() {
        let c = create_consola(None);

        c.info("Developing rust with Consola");
        let stop = c.start("Building project...");

        std::thread::sleep(std::time::Duration::from_secs(5));

        stop();

        c.warn("A new version of Consola is available: 0.0.1");
        c.success("Project built!");
        c.error("This is an example error. Everything is fine!", None);
        c.boxi("Very simple box!", None);
        c.boxi(
            "A terminal filled box!",
            Some(BoxiOptions {
                fill: true,
                center: false,
                padding: 4,
            }),
        );
        c.boxi(
            "A centered terminal filled box with a lorem ispum paragraph that isn't really not because i do not want to search online for the lorem ispum, and even though vscode has a feature for that!",
            Some(BoxiOptions {
                fill: true,
                center: false,
                padding: 4,
            }),
        );
    }
}
