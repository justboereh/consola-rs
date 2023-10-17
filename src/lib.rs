mod consola;
mod utils;

use consola::Consola;
use utils::{is_unicode_supported, TypeIcons};

pub fn create_consola() -> Consola {
    let unicode_supported = is_unicode_supported();
    let type_icons = TypeIcons::new(unicode_supported);

    Consola {
        type_icons,
        unicode_supported,
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    use crate::create_consola;

    #[test]
    fn basic_test() {
        let c = create_consola();

        c.info("Developing rust with Consola");
        c.start("Building project...");
        c.warn("A new version of Consola is available: 0.0.1");
        c.success("Project built!");
        c.error("This is an example error. Everything is fine!", None);
    }
}
