/*

* Figure out Rust and FFI (foreign function interface)
* Are we writing bytes to a stream? How does this deal work.
* Figure out how to bridge Rust and C++ https://github.com/discordapp/discord-rpc
* Figure out how to structure the project :P

*/
#![feature(libc)]

extern crate libc;
use std::ffi::CString;

pub mod bindings;

#[allow(dead_code)]
struct DiscordConnection<'d> {
    status: bindings::DiscordRichPresence,
    application_id: String,
    auto_register: i32,
    steam_id: String,
    handlers: &'d mut bindings::DiscordEventHandlers
}

#[allow(dead_code)]
impl<'d> DiscordConnection<'d> {

    fn new(application_id: String, auto_register: libc::c_int, steam_id: String, handlers: &'d mut bindings::DiscordEventHandlers) -> DiscordConnection<'d> {
        unsafe {
            bindings::Discord_Initialize(
                CString::new(application_id.as_str()).unwrap().as_ptr(),
                handlers,
                auto_register,
                CString::new(steam_id.as_str()).unwrap().as_ptr(),
            );
        }

        DiscordConnection {
            application_id: application_id,
            auto_register: auto_register,
            steam_id: steam_id,
            handlers: handlers,
            status: bindings::DiscordRichPresence {
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
                instance: 0
            }
        }

    }

    // fn connect(&mut self) {
    //     unsafe {
    //         bindings::Discord_Initialize(
    //             CString::new(self.application_id).unwrap().as_ptr(),
    //             self.handlers,
    //             self.auto_register,
    //             CString::new(self.steam_id).unwrap().as_ptr(),
    //         )
    //     }
    // }
}

impl<'d> Drop for DiscordConnection<'d> {
    fn drop(&mut self) {
        println!("Dropping DiscordConnection");

        unsafe {
            bindings::Discord_Shutdown();
        }
    }
}

pub unsafe extern "C" fn handle_ready() {
    println!("Ready called!")
}

pub unsafe extern "C" fn handle_errored(error_code: i32, error_message: *const i8) {
    println!("Errored called!")
}

pub unsafe extern "C" fn handle_disconnected(error_code: i32, error_message: *const i8) {
    println!("Disconnected called!")
}

pub unsafe extern "C" fn handle_join_game(join_secret: *const i8) {
    println!("Disconnected called!")
}

pub unsafe extern "C" fn handle_join_request(join_request: *const bindings::DiscordJoinRequest) {
    println!("Disconnected called!")
}

pub unsafe extern "C" fn handle_spectate(spectate_secret: *const i8) {
    println!("Disconnected called!")
}

#[test]
fn connect_and_listen() {

    let mut handlers = bindings::DiscordEventHandlers {
            ready:        Some(handle_ready),
            errored:      Some(handle_errored),
            disconnected: Some(handle_disconnected),
            joinGame:     Some(handle_join_game),
            joinRequest:  Some(handle_join_request),
            spectateGame: Some(handle_spectate)
    };

    let conn = DiscordConnection::new("YOUR_CLIENT_ID".to_string(), 1, "".to_string(), &mut handlers);
}
