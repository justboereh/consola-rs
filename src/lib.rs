mod box_styles;
mod consola;
mod reporter;
mod utils;

use box_styles::{create_styles, BoxStyle};
use consola::Consola;
use reporter::Reporter;
use utils::{is_unicode_supported, TypeIcons};

pub struct ConsolaOptions {
    pub box_style: Option<BoxStyle>,
}

pub fn create_consola(option: Option<ConsolaOptions>) -> Consola {
    let unicode_supported = is_unicode_supported();
    let type_icons = TypeIcons::new(unicode_supported);
    let styles = create_styles();

    let box_style = option
        .unwrap_or(ConsolaOptions { box_style: None })
        .box_style
        .unwrap_or(styles.solid);

    Consola {
        type_icons,
        unicode_supported,
        box_style,
    }
}

pub fn create_reporter(name: String) -> Reporter {
    return Reporter { name };
}

#[cfg(test)]
mod tests {

    use crate::{consola::BoxiOptions, create_consola};

    #[test]
    fn test() {
        let c = create_consola(None);

        c.info("Developing rust with Consola");
        c.start("Building project...");

        std::thread::sleep(std::time::Duration::from_secs(5));

        c.warn("A new version of Consola is available: 0.0.1");
        c.success("Project built!");
        c.error("This is an example error. Everything is fine!", None);
        c.boxi("Very simple box!", None);
        c.boxi(
            "A filled centered box!",
            Some(BoxiOptions {
                fill: true,
                center: true,
                padding: 4,
            }),
        );
        c.boxi(
            "Another box with a lorem ispum paragraph that isn't really not because i do not want to search online for the lorem ispum, and even though vscode has a feature for that! and I just don't know how to use anything in life. iykyk. lmao",
            Some(BoxiOptions {
                fill: false,
                center: false,
                padding: 4,
            }),
        );
    }
}
