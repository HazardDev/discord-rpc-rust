/*

* Figure out Rust and FFI (foreign function interface)
* Are we writing bytes to a stream? How does this deal work.
* Figure out how to bridge Rust and C++ https://github.com/discordapp/discord-rpc
* Figure out how to structure the project :P

*/

#![feature(libc)]

extern crate libc;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

static mut READY: bool = false;

pub mod bindings;
pub mod presence;

use presence::Presence;

#[allow(dead_code)]
pub struct DiscordPresenceConnection {
    ready: bool,
    presence: Presence,
    application_id: String,
    auto_register: i32,
    steam_id: String,
    handlers: bindings::DiscordEventHandlers,
}

#[allow(dead_code)]
impl DiscordPresenceConnection {
    pub fn new(
        application_id: String,
        handlers: &mut bindings::DiscordEventHandlers,
        auto_register: libc::c_int,
        steam_id: String,
    ) -> DiscordPresenceConnection {
        unsafe {
            bindings::Discord_Initialize(
                CString::new(application_id.as_str()).unwrap().as_ptr(),
                handlers,
                auto_register,
                CString::new(steam_id.as_str()).unwrap().as_ptr(),
            );
        }

        DiscordPresenceConnection {
            ready: false,
            application_id: application_id,
            auto_register: auto_register,
            steam_id: steam_id,
            handlers: handlers.to_owned(),

            //Default, empty presence
            presence: Presence::default(),
        }
    }

    pub fn update(&self, presence: &mut Presence) {
        // println!("Updating with presence: {:?}\n", presence);

        if !self.ready() {
            panic!("Update called before discord was ready.");
        }

        unsafe {
            bindings::Discord_UpdatePresence(&presence.as_c_presence());
        }
    }

    pub fn run_callbacks(&mut self) {
        unsafe {
            //Sets the struct's ready callback to whatever is in the global READY
            self.ready = READY;
            bindings::Discord_RunCallbacks();
        }
    }

    #[no_mangle]
    pub extern "C" fn handle_ready() {
        unsafe {
            READY = true;
        }
        println!("Ready called!");
    }

    #[no_mangle]
    pub extern "C" fn handle_errored(error_code: i32, error_message: *const c_char) {
        let error_message: String =
            unsafe { CStr::from_ptr(error_message).to_string_lossy().into_owned() };

        println!("Errored called: {:?} - {:?}", error_code, error_message);
    }

    #[no_mangle]
    pub extern "C" fn handle_disconnected(
        error_code: i32,
        error_message: *const c_char,
    ) {
        let error_message: &CStr = unsafe { CStr::from_ptr(error_message) };

        println!(
            "Disconnected called: {:?} - {:?}",
            error_code, error_message
        );
    }

    #[no_mangle]
    pub extern "C" fn handle_join_game(join_secret: *const c_char) {
        let join_secret: &CStr = unsafe { CStr::from_ptr(join_secret) };

        println!("Join Game called: {:?}", join_secret);
    }

    #[no_mangle]
    pub extern "C" fn handle_join_request(join_request: *const bindings::DiscordJoinRequest) {
        println!("Join Request called: {:?}", join_request);
    }

    #[no_mangle]
    pub extern "C" fn handle_spectate(spectate_secret: *const c_char) {
        let spectate_secret: &CStr = unsafe { CStr::from_ptr(spectate_secret) };

        println!("Spectate called: {:?}", spectate_secret);
    }

    pub fn ready(&self) -> bool {
        self.ready
    }
}

impl Drop for DiscordPresenceConnection {
    fn drop(&mut self) {
        println!("Dropping DiscordPresenceConnection");

        unsafe {
            READY = false;
            bindings::Discord_ClearPresence();
            bindings::Discord_Shutdown();
        }
    }
}