use std::str::FromStr;

pub struct TypeIcons {
    pub error: String,
    pub fatal: String,
    pub ready: String,
    pub warn: String,
    pub info: String,
    pub success: String,
    pub debug: String,
    pub trace: String,
    pub fail: String,
    pub start: String,
    pub log: String,
}

impl TypeIcons {
    pub fn new(unicode_supported: bool) -> TypeIcons {
        TypeIcons {
            error: s("✖", "×", unicode_supported),
            fatal: s("✖", "×", unicode_supported),
            ready: s("✔", "√", unicode_supported),
            warn: s("⚠", "‼", unicode_supported),
            info: s("ℹ", "i", unicode_supported),
            success: s("✔", "√", unicode_supported),
            debug: s("⚙", "D", unicode_supported),
            trace: s("→", "→", unicode_supported),
            fail: s("✖", "×", unicode_supported),
            start: s("◐", "o", unicode_supported),
            log: String::from(""),
        }
    }
}

fn s(c: &str, fallback: &str, unicode_supported: bool) -> String {
    if unicode_supported {
        return String::from_str(&c).unwrap();
    }

    String::from_str(fallback).unwrap()
}

pub fn is_unicode_supported() -> bool {
    let term = std::env::var_os("TERM");
    let ci = std::env::var_os("CI");
    let wt = std::env::var_os("WT_SESSION");
    let terminus = std::env::var_os("TERMINUS_SUBLIME");
    let cet = std::env::var_os("ConEmuTask");
    let term_pro = std::env::var_os("TERM_PROGRAM");
    let terminal_emu = std::env::var_os("TERMINAL_EMULATOR");

    if std::env::consts::OS == "windows" {
        if term.is_none() {
            return true;
        }

        return term.unwrap() != "LINUX";
    }

    if !ci.is_none() {
        return true;
    }

    if !wt.is_none() {
        return true;
    }

    if !terminus.is_none() {
        return true;
    }

    if !cet.is_none() && cet.unwrap() == "{cmd::Cmder}" {
        return true;
    }

    if !term_pro.is_none() && term_pro.as_ref().unwrap() == "Terminus-Sublime" {
        return true;
    }

    if !term_pro.is_none() && term_pro.as_ref().unwrap() == "vscode" {
        return true;
    }

    if !term.is_none() && term.as_ref().unwrap() == "'xterm-256color" {
        return true;
    }

    if !term.is_none() && term.as_ref().unwrap() == "alacritty" {
        return true;
    }

    if !terminal_emu.is_none() && terminal_emu.as_ref().unwrap() == "JetBrains-JediTerm" {
        return true;
    }

    return false;
}
