use std::{
    any::Any,
    error::Error,
    panic::{self, PanicHookInfo},
};

use log::{log, trace, Level};
use native_dialog::MessageLevel;
pub trait ShowDialog {
    fn show_dialog_message_format(&self, message: &str) -> (String, String);
    fn show_dialog(&self, message: &str, level: MessageLevel) {
        self.show_dialog_message_format(message).show_dialog(message, level)
    }
    fn show_dialog_with_log(&self, message: &str, level: Level) {
        self.show_dialog_message_format(message)
            .show_dialog_with_log(message, level)
    }
    fn show_failed_dialog(&self) {
        self.show_dialog_with_log("FAILED", Level::Error)
    }
}
impl ShowDialog for (String, String) {
    fn show_dialog_message_format(&self, message: &str) -> (String, String) {
        self.clone()
    }

    fn show_dialog(&self, message: &str, level: MessageLevel) {
        let (display, _) = self;
        let old_hook = panic::take_hook();
        panic::set_hook(Box::new(|info| {
            eprint!("[FAILED] {info}");
        }));
        if let Err(err) = native_dialog::DialogBuilder::message()
            .set_title(env!("CARGO_PKG_NAME"))
            .set_level(level)
            .set_text(display)
            .alert()
            .show()
        {
            eprint!("[FAILED] {err}");
        }
        panic::set_hook(old_hook);
    }

    fn show_dialog_with_log(&self, message: &str, level: Level) {
        let (display, debug) = self;
        log!(level, "{debug}");
        let old_hook = panic::take_hook();
        panic::set_hook(Box::new(|info| {
            eprint!("[FAILED] {info}");
        }));
        if let Err(err) = native_dialog::DialogBuilder::message()
            .set_title(env!("CARGO_PKG_NAME"))
            .set_level(match level {
                Level::Error => MessageLevel::Error,
                Level::Warn => MessageLevel::Warning,
                Level::Info => MessageLevel::Info,
                Level::Debug => MessageLevel::Info,
                Level::Trace => MessageLevel::Info,
            })
            .set_text(display)
            .alert()
            .show()
        {
            eprint!("[FAILED] {err}");
        }
        panic::set_hook(old_hook);
    }
}
impl ShowDialog for dyn Any {
    fn show_dialog_message_format(&self, message: &str) -> (String, String) {
        if let Some(string) = self.downcast_ref::<String>() {
            (format!("[{message}] {string}"), format!("{string:?}"))
        } else if let Some(&str) = self.downcast_ref::<&str>() {
            (format!("[{message}] {str}"), format!("{str:?}"))
        } else if let Some(err) =
            self.downcast_ref::<Box<dyn std::error::Error>>()
        {
            (format!("[{message}] {err}"), format!("{err:?}"))
        } else {
            trace!("type: {:?}", self.type_id());
            (format!("[{message}] {self:?}"), format!("{self:?}"))
        }
    }
}
impl ShowDialog for dyn Any + Send {
    fn show_dialog_message_format(&self, message: &str) -> (String, String) {
        if let Some(string) = self.downcast_ref::<String>() {
            (format!("[{message}] {string}"), format!("{string:?}"))
        } else if let Some(&str) = self.downcast_ref::<&str>() {
            (format!("[{message}] {str}"), format!("{str:?}"))
        } else if let Some(err) =
            self.downcast_ref::<Box<dyn std::error::Error>>()
        {
            (format!("[{message}] {err}"), format!("{err:?}"))
        } else {
            trace!("type: {:?}", self.type_id());
            (format!("[{message}] {self:?}"), format!("{self:?}"))
        }
    }
}
impl ShowDialog for dyn Error {
    fn show_dialog_message_format(&self, message: &str) -> (String, String) {
        (format!("[{message}] {self}"), format!("{self:?}"))
    }
}
impl ShowDialog for PanicHookInfo<'_> {
    fn show_dialog_message_format(&self, message: &str) -> (String, String) {
        (self.to_string(), self.to_string())
    }
}
