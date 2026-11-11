//! Windows registry read helpers for Forklift 4.

pub const APP_KEY: &str = "Software\\Forklift\\Forklift 4";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Forklift 4";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Forklift 4") },
        RegValue { name: "Version",     value: String::from("v4.2.14") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Forklift 4") },
        RegValue { name: "Publisher",       value: String::from("Forklift") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Forklift 4\\uninstall.exe\"") },
    ]
}
