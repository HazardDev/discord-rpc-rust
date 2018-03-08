/*

* Figure out Rust and FFI (foreign function interface)
* Are we writing bytes to a stream? How does this deal work.
* Figure out how to bridge Rust and C++ https://github.com/discordapp/discord-rpc
* Figure out how to structure the project :P

*/
#![feature(libc)]

extern crate libc;
use std::ffi::CString;
use std::ffi::CStr;

pub mod bindings;

#[allow(dead_code)]
struct DiscordConnection {
    presence: bindings::DiscordRichPresence,
    applicationId: String,
    auto_register: i32,
    steam_id: String,
    handlers: bindings::DiscordEventHandlers,
}

#[allow(dead_code)]
impl DiscordConnection {
    fn new(
        applicationId: String,
        handlers: &mut bindings::DiscordEventHandlers,
        auto_register: libc::c_int,
        steam_id: String,
    ) -> DiscordConnection {
        unsafe {
            bindings::Discord_Initialize(
                CString::new(applicationId.as_str()).unwrap().as_ptr(),
                handlers,
                auto_register,
                CString::new(steam_id.as_str()).unwrap().as_ptr(),
            );
        }

        DiscordConnection {
            applicationId: applicationId,
            auto_register: auto_register,
            steam_id: steam_id,
            handlers: handlers.to_owned(),
            presence: bindings::DiscordRichPresence {
                state: CString::new(String::new()).unwrap().as_ptr(),
                details: CString::new(String::new()).unwrap().as_ptr(),
                startTimestamp: 0,
                endTimestamp: 0,
                largeImageKey: CString::new(String::new()).unwrap().as_ptr(),
                largeImageText: CString::new(String::new()).unwrap().as_ptr(),
                smallImageKey: CString::new(String::new()).unwrap().as_ptr(),
                smallImageText: CString::new(String::new()).unwrap().as_ptr(),
                partyId: CString::new(String::new()).unwrap().as_ptr(),
                partySize: 0,
                partyMax: 0,
                matchSecret: CString::new(String::new()).unwrap().as_ptr(),
                joinSecret: CString::new(String::new()).unwrap().as_ptr(),
                spectateSecret: CString::new(String::new()).unwrap().as_ptr(),
                instance: 0,
            },
        }
    }

    fn update(&self, presence: &bindings::DiscordRichPresence) {

        unsafe {
            bindings::Discord_UpdatePresence(presence);
        }
    }

    fn run_callbacks(&self) {
        unsafe {
            bindings::Discord_RunCallbacks();
        }
    }
}

impl Drop for DiscordConnection {
    fn drop(&mut self) {
        println!("Dropping DiscordConnection");

        unsafe {
            bindings::Discord_Shutdown();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn handle_ready() {
    println!("Ready called!");
}

#[no_mangle]
pub unsafe extern "C" fn handle_errored(error_code: i32, error_message: *const ::std::os::raw::c_char) {

    let error_message: &CStr = CStr::from_ptr(error_message);

    println!("Errored called: {:?} - {:?}", error_code, error_message);
}

#[no_mangle]
pub unsafe extern "C" fn handle_disconnected(error_code: i32, error_message: *const ::std::os::raw::c_char) {
    println!("Disconnected called!");
}

#[no_mangle]
pub unsafe extern "C" fn handle_join_game(join_secret: *const ::std::os::raw::c_char) {
    println!("Join Game called!");
}

#[no_mangle]
pub unsafe extern "C" fn handle_join_request(join_request: *const bindings::DiscordJoinRequest) {
    println!("Join Request called!");
}
#[no_mangle]

pub unsafe extern "C" fn handle_spectate(spectate_secret: *const ::std::os::raw::c_char) {
    println!("Spectate called!");
}

#[test]
fn connect_and_listen() {
    let mut handlers = bindings::DiscordEventHandlers {
        ready: Some(handle_ready),
        errored: Some(handle_errored),
        disconnected: Some(handle_disconnected),
        joinGame: Some(handle_join_game),
        joinRequest: Some(handle_join_request),
        spectateGame: Some(handle_spectate),
    };

    let conn = DiscordConnection::new(
        "421166510254587905".to_string(),
        &mut handlers,
        1,
        "".to_string(),
    );

    let presence = bindings::DiscordRichPresence {
        state: CString::new(String::from("WOOH DISCORD")).unwrap().as_ptr(),
        details: CString::new(String::from("RICH PRESENCE"))
            .unwrap()
            .as_ptr(),
        ..conn.presence
    };

    loop {
        conn.update(&presence);
        conn.run_callbacks();
    }
}
