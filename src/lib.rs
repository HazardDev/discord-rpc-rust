/*

* Figure out Rust and FFI (foreign function interface)
* Are we writing bytes to a stream? How does this deal work.
* Figure out how to bridge Rust and C++ https://github.com/discordapp/discord-rpc
* Figure out how to structure the project :P

*/



#![feature(libc)]

extern crate libc;
use std::ffi::CStr;
use std::ffi::CString;

static mut READY: bool = false;

mod bindings;

#[allow(dead_code)]
pub struct DiscordConnection {
    ready: bool,
    pub presence: bindings::DiscordRichPresence,
    application_id: String,
    auto_register: i32,
    steam_id: String,
    handlers: bindings::DiscordEventHandlers,
}

#[allow(dead_code)]
impl DiscordConnection {
    fn new(
        application_id: String,
        handlers: &mut bindings::DiscordEventHandlers,
        auto_register: libc::c_int,
        steam_id: String,
    ) -> DiscordConnection {
        unsafe {
            bindings::Discord_Initialize(
                CString::new(application_id.as_str()).unwrap().as_ptr(),
                handlers,
                auto_register,
                CString::new(steam_id.as_str()).unwrap().as_ptr(),
            );
        }

        DiscordConnection {
            ready: false,
            application_id: application_id,
            auto_register: auto_register,
            steam_id: steam_id,
            handlers: handlers.to_owned(),

            //Default, empty presence
            presence: bindings::DiscordRichPresence {
                state: CString::new(String::new()).unwrap().as_ptr(),
                details: CString::new(String::new()).unwrap().as_ptr(),
                start_timestamp: 0,
                end_timestamp: 0,
                large_image_key: CString::new(String::new()).unwrap().as_ptr(),
                large_image_text: CString::new(String::new()).unwrap().as_ptr(),
                small_image_key: CString::new(String::new()).unwrap().as_ptr(),
                small_image_text: CString::new(String::new()).unwrap().as_ptr(),
                party_id: CString::new(String::new()).unwrap().as_ptr(),
                party_size: 0,
                party_max: 0,
                match_secret: CString::new(String::new()).unwrap().as_ptr(),
                join_secret: CString::new(String::new()).unwrap().as_ptr(),
                spectate_secret: CString::new(String::new()).unwrap().as_ptr(),
                instance: 0,
            },
        }
    }

    fn update(&self, presence: &bindings::DiscordRichPresence) {
        // println!("Updating with presence: {:?}\n", presence);

        unsafe {

            bindings::Discord_UpdatePresence(presence);
        }
    }

    fn run_callbacks(&mut self) {
        unsafe {
            self.ready = READY;
            bindings::Discord_RunCallbacks();
        }
    }

    #[no_mangle]
    extern "C" fn handle_ready() {
        unsafe { READY = true; }
        println!("Ready called!");
    }

    #[no_mangle]
    extern "C" fn handle_errored(error_code: i32, error_message: *const ::std::os::raw::c_char) {
        let error_message: String = unsafe { CStr::from_ptr(error_message).to_string_lossy().into_owned() };

        println!("Errored called: {:?} - {:?}", error_code, error_message);
    }

    #[no_mangle]
    extern "C" fn handle_disconnected(
        error_code: i32,
        error_message: *const ::std::os::raw::c_char,
    ) {
        let error_message: &CStr = unsafe { CStr::from_ptr(error_message) };

        println!(
            "Disconnected called: {:?} - {:?}",
            error_code, error_message
        );
    }

    #[no_mangle]
    extern "C" fn handle_join_game(join_secret: *const ::std::os::raw::c_char) {
        let join_secret: &CStr = unsafe { CStr::from_ptr(join_secret) };

        println!("Join Game called: {:?}", join_secret);
    }

    #[no_mangle]
    extern "C" fn handle_join_request(join_request: *const bindings::DiscordJoinRequest) {
        println!("Join Request called: {:?}", join_request);
    }

    #[no_mangle]
    extern "C" fn handle_spectate(spectate_secret: *const ::std::os::raw::c_char) {
        let spectate_secret: &CStr = unsafe { CStr::from_ptr(spectate_secret) };

        println!("Spectate called: {:?}", spectate_secret);
    }

    fn ready(&self) -> bool {
        self.ready
    }
}

impl Drop for DiscordConnection {
    fn drop(&mut self) {
        println!("Dropping DiscordConnection");

        unsafe {
            READY = false;
            bindings::Discord_Shutdown();
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::{thread, time};

    #[test]
    fn connect_and_listen() {
        let mut handlers = bindings::DiscordEventHandlers {
            ready: Some(DiscordConnection::handle_ready),
            errored: Some(DiscordConnection::handle_errored),
            disconnected: Some(DiscordConnection::handle_disconnected),
            join_game: None,
            join_request: None,
            spectate_game: None,
        };

        let mut conn: DiscordConnection = DiscordConnection::new(
            "421166510254587905".to_string(),
            &mut handlers,
            1,
            "".to_string(),
        );
        conn.run_callbacks();

        let state_value = CString::new("WOO DISCORD").unwrap();
        let state_pointer = state_value.as_ptr();
        println!("State Pointer: {:?}", state_pointer);

        //Initial Presence

        let detail_value = CString::new("Rust Rich Presence")
                .unwrap();
        let detail_pointer = detail_value.as_ptr();

        let image_key_value = CString::new("default").unwrap();
        let image_key_pointer = image_key_value.as_ptr();
        let presence = bindings::DiscordRichPresence {
            state: state_pointer,
            details: detail_pointer,
            large_image_key: image_key_pointer,
            ..conn.presence
        };

        println!("State Pointer: {:?}", state_pointer);
        println!("State Presence: {:?}", presence.state);
        println!("State Presence String: {:?}", unsafe {
            CStr::from_ptr(presence.state).to_string_lossy()
        });
        println!("Presence: {:?}", presence);

        conn.run_callbacks();

        let two_seconds = time::Duration::from_secs(2);

        loop {

            conn.run_callbacks();
            if conn.ready() {
                break;
            }
        }

        loop {
            let now = time::Instant::now();
            thread::sleep(two_seconds);
            assert!(now.elapsed() >= two_seconds);
            println!("State Pointer: {:?}", state_pointer);
            println!("State Presence: {:?}", presence.state);
            println!("State Presence String: {:?}", unsafe {
                CStr::from_ptr(presence.state).to_string_lossy()
            });
            println!("Presence: {:?}", presence);
            conn.run_callbacks();
            conn.update(&presence);
        }

    }
}
