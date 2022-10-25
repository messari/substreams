use std::fmt::format;
use std::io::Read;
use dialoguer::{Input, Select};
/// Place for setting all related style for the command line tool. Aim is to make sure style is kept consistent across all commands
use dialoguer::theme::ColorfulTheme;
use lazy_static::lazy_static;
use strum::{EnumString, EnumIter, IntoEnumIterator, EnumVariantNames, VariantNames};

lazy_static! {
    static ref DIALOGUE_COLOR_THEME : ColorfulTheme = ColorfulTheme::default();
}

#[cfg(not(target_os = "windows"))]
mod constants {
    pub(super) const GREEN_TICK: &'static str = "\x1b[32m✔\x1b[0m";
    pub(super) const RED_CROSS: &'static str = "\x1b[31m✘\x1b[0m";
}
#[cfg(target_os = "windows")]
mod constants {
    pub(super) const GREEN_TICK: &'static str = "\x1b[32m√\x1b[0m";
    pub(super) const RED_CROSS: &'static str = "\x1b[31m×\x1b[0m";
}

#[inline(always)]
fn get_dialogue_color_theme() -> &'static ColorfulTheme {
    &DIALOGUE_COLOR_THEME as &ColorfulTheme
}

pub(crate) fn get_input<P: Into<String>>(prompt: P, completion_message: Option<&str>, allow_empty: bool) -> String {
    // if let Some(completion_message) = completion_message {
    //     Input::with_theme(get_dialogue_color_theme())
    //         .with_prompt(prompt)
    //         .with_post_completion_text(completion_message)
    //         .interact_text()
    //         .unwrap()
    // } else {
    //     Input::with_theme(get_dialogue_color_theme())
    //         .with_prompt(prompt)
    //         .interact_text()
    //         .unwrap()
    // }

    let mut input = Input::with_theme(get_dialogue_color_theme()).with_prompt(prompt);
    if let Some(completion_message) = completion_message {
        input = input.with_post_completion_text(completion_message);
    }
    if allow_empty {
        input = input.allow_empty(true);
    }
    input.interact_text().unwrap()
}

pub(crate) fn select_from_values<P: Into<String>, T: ToString+Clone>(prompt: P, values: &[T], default_option: Option<usize>) -> T {
    let selection  = if let Some(default) = default_option {
        Select::with_theme(get_dialogue_color_theme())
        .with_prompt(prompt)
        .default(default)
        .items(values)
        .interact()
        .unwrap()
    } else {
        Select::with_theme(get_dialogue_color_theme())
        .with_prompt(prompt)
        .items(values)
        .interact()
        .unwrap()
    };

    values[selection].clone()
}

pub(crate) fn select_from_enum<P: Into<String>, T: IntoEnumIterator+VariantNames+Clone>(prompt: P, default_option: Option<usize>) -> T {
    let values = T::VARIANTS;

    let selection  = if let Some(default) = default_option {
        Select::with_theme(get_dialogue_color_theme())
            .with_prompt(prompt)
            .default(default)
            .items(&values)
            .interact()
            .unwrap()
    } else {
        Select::with_theme(get_dialogue_color_theme())
            .with_prompt(prompt)
            .items(&values)
            .interact()
            .unwrap()
    };

    T::iter().skip(selection).next().unwrap().clone()
}

pub(crate) struct Spinner{
    spinner: spinners::Spinner
}

impl Spinner {
    pub(crate) fn new(message: String) -> Self {
        Spinner{
            spinner: spinners::Spinner::new(spinners::Spinners::Line, message)
        }
    }

    pub(crate) fn end_with_success_message(mut self, message: String) {
        self.spinner.stop_with_message(get_success_message(message));
    }

    pub(crate) fn end_with_error_message(mut self, message: String) {
        self.spinner.stop_with_message(get_error_message(message));
    }
}


#[inline(always)]
pub(crate) fn get_success_message<T: AsRef<str>>(message: T) -> String {
    #[cfg(not(target_os = "windows"))]
    const SUCCESS: &str = "\x1b[32m✔\x1b[0m";
    #[cfg(target_os = "windows")]
    const SUCCESS: &str = "\x1b[32m√\x1b[0m";

    format!("{} {}", SUCCESS, message.as_ref())
}

#[inline(always)]
pub(crate) fn get_error_message<T: AsRef<str>>(message: T) -> String {
    #[cfg(not(target_os = "windows"))]
    const ERROR: &str = "\x1b[31m✘\x1b[0m";
    #[cfg(target_os = "windows")]
    const ERROR: &str = "\x1b[31m×\x1b[0m";

    format!("{} {}\n", ERROR, message.as_ref())
}