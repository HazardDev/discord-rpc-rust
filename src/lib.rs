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
                CString::new(application_id.as_str()).unwrap().into_raw(),
                handlers,
                auto_register,
                CString::new(steam_id.as_str()).unwrap().into_raw(),
            );
        }

        DiscordConnection {
            application_id: application_id,
            auto_register: auto_register,
            steam_id: steam_id,
            handlers: handlers.to_owned(),
            presence: bindings::DiscordRichPresence {
                state: CString::new(String::new()).unwrap().into_raw(),
                details: CString::new(String::new()).unwrap().into_raw(),
                start_timestamp: 0,
                end_timestamp: 0,
                large_image_key: CString::new(String::new()).unwrap().into_raw(),
                large_image_text: CString::new(String::new()).unwrap().into_raw(),
                small_image_key: CString::new(String::new()).unwrap().into_raw(),
                small_image_text: CString::new(String::new()).unwrap().into_raw(),
                party_id: CString::new(String::new()).unwrap().into_raw(),
                party_size: 0,
                party_max: 0,
                match_secret: CString::new(String::new()).unwrap().into_raw(),
                join_secret: CString::new(String::new()).unwrap().into_raw(),
                spectate_secret: CString::new(String::new()).unwrap().into_raw(),
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
pub unsafe extern "C" fn handle_errored(
    error_code: i32,
    error_message: *const ::std::os::raw::c_char,
) {
    let error_message: &CStr = CStr::from_ptr(error_message);

    println!("Errored called: {:?} - {:?}", error_code, error_message);
}

#[no_mangle]
pub unsafe extern "C" fn handle_disconnected(
    error_code: i32,
    error_message: *const ::std::os::raw::c_char,
) {
    let error_message: &CStr = CStr::from_ptr(error_message);

    println!(
        "Disconnected called: {:?} - {:?}",
        error_code, error_message
    );
}

#[no_mangle]
pub unsafe extern "C" fn handle_join_game(join_secret: *const ::std::os::raw::c_char) {
    let join_secret: &CStr = CStr::from_ptr(join_secret);

    println!("Join Game called: {:?}", join_secret);
}

#[no_mangle]
pub unsafe extern "C" fn handle_join_request(join_request: *const bindings::DiscordJoinRequest) {
    println!("Join Request called: {:?}", join_request);
}

#[no_mangle]
pub unsafe extern "C" fn handle_spectate(spectate_secret: *const ::std::os::raw::c_char) {
    let spectate_secret: &CStr = CStr::from_ptr(spectate_secret);
    
    println!("Spectate called: {:?}", spectate_secret);
}

#[test]
fn connect_and_listen() {
    let mut handlers = bindings::DiscordEventHandlers {
        ready: Some(handle_ready),
        errored: Some(handle_errored),
        disconnected: Some(handle_disconnected),
        join_game: Some(handle_join_game),
        join_request: Some(handle_join_request),
        spectate_game: Some(handle_spectate),
    };

    let conn = DiscordConnection::new(
        "421166510254587905".to_string(),
        &mut handlers,
        1,
        "".to_string(),
    );
    conn.run_callbacks();

    let presence = bindings::DiscordRichPresence {
        state: CString::new(String::from("WOOH DISCORD"))
            .unwrap()
            .into_raw(),
        details: CString::new(String::from("RICH PRESENCE"))
            .unwrap()
            .into_raw(),
        ..conn.presence
    };

    println!("State: {:?}", unsafe { CString::new(String::from("WOOH DISCORD")).unwrap().into_raw() });
    println!("State: {:?}", unsafe { CStr::from_ptr(presence.state) });
    println!("Presence: {:?}", presence);

    conn.update(&presence);
    loop {
        conn.run_callbacks();
    }
}
