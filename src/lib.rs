/*

* Figure out Rust and FFI (foreign function interface)
* Are we writing bytes to a stream? How does this deal work.
* Figure out how to bridge Rust and C++ https://github.com/discordapp/discord-rpc
* Figure out how to structure the project :P

*/

pub mod bindings;
use bindings::*;

use std::error::Error;
use std::ffi::{CStr, CString};
use std::fmt;
use std::thread;

impl fmt::Display for DiscordConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl Error for DiscordConnectionError {
    fn description(&self) -> &str {
        match self {
            ConnectionFailed => "Connection failed.",
            BadUserId => "Invalid identification provided.",
            BadSteamId => "Invalid steam id was not none.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

#[derive(Debug)]
enum DiscordConnectionError {
    ConnectionFailed,
    BadUserId,
    BadSteamId,
}

#[allow(dead_code)]
pub struct DiscordPresenceConnection {
    event_thread: std::thread::JoinHandle<()>,
}

#[allow(dead_code)]
impl DiscordPresenceConnection {
    fn new(
        id: &str,
        auto_register: i32,
        steam_id: Option<&str>,
    ) -> Result<Self, DiscordConnectionError> {

        let id = match CString::new(id) {
            Ok(id) => id,
            Err(_) => return Err(DiscordConnectionError::BadUserId),
        };

        let steam_id = if let Some(steam_id) = steam_id {
            Some(match CString::new(steam_id) {
                Ok(steam_id) => steam_id,
                _ => return Err(DiscordConnectionError::BadSteamId),
            })
        } else { None };

        let mut handlers = DiscordEventHandlers {

        };

        unsafe {
            Discord_Initialize(
                id.as_ptr(),
                &mut handlers as *mut DiscordEventHandlers,
                auto_register,
                steam_id.unwrap_or(CString::new("0").unwrap()).as_ptr(),
            );
        }

        let handler_thread = thread::spawn(move || {});

        DiscordPresenceConnection {
            event_thread: handler_thread,
        }
    }
}

impl Drop for DiscordPresenceConnection {
    fn drop(&mut self) {}
}
