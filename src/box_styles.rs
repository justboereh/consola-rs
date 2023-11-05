pub struct BoxStyle {
    pub tl: String,
    pub tr: String,
    pub bl: String,
    pub br: String,
    pub h: String,
    pub v: String,
}

pub struct BoxStyles {
    pub solid: BoxStyle,
    pub double: BoxStyle,
    pub double_single: BoxStyle,
    pub double_single_rounded: BoxStyle,
    pub single_thick: BoxStyle,
    pub single_double: BoxStyle,
    pub single_double_rounded: BoxStyle,
    pub rounded: BoxStyle,
}

pub fn create_styles() -> BoxStyles {
    BoxStyles {
        solid: BoxStyle {
            tl: String::from("┌"),
            tr: String::from("┐"),
            bl: String::from("└"),
            br: String::from("┘"),
            h: String::from("─"),
            v: String::from("│"),
        },
        double: BoxStyle {
            tl: String::from("╔"),
            tr: String::from("╗"),
            bl: String::from("╚"),
            br: String::from("╝"),
            h: String::from("═"),
            v: String::from("║"),
        },
        double_single: BoxStyle {
            tl: String::from("╓"),
            tr: String::from("╖"),
            bl: String::from("╙"),
            br: String::from("╜"),
            h: String::from("─"),
            v: String::from("║"),
        },
        double_single_rounded: BoxStyle {
            tl: String::from("╭"),
            tr: String::from("╮"),
            bl: String::from("╰"),
            br: String::from("╯"),
            h: String::from("─"),
            v: String::from("║"),
        },
        single_thick: BoxStyle {
            tl: String::from("┏"),
            tr: String::from("┓"),
            bl: String::from("┗"),
            br: String::from("┛"),
            h: String::from("━"),
            v: String::from("┃"),
        },
        single_double: BoxStyle {
            tl: String::from("╒"),
            tr: String::from("╕"),
            bl: String::from("╘"),
            br: String::from("╛"),
            h: String::from("═"),
            v: String::from("│"),
        },
        single_double_rounded: BoxStyle {
            tl: String::from("╭"),
            tr: String::from("╮"),
            bl: String::from("╰"),
            br: String::from("╯"),
            h: String::from("═"),
            v: String::from("│"),
        },
        rounded: BoxStyle {
            tl: String::from("╭"),
            tr: String::from("╮"),
            bl: String::from("╰"),
            br: String::from("╯"),
            h: String::from("─"),
            v: String::from("│"),
        },
    }
}
