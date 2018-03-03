/*

* Figure out Rust and FFI (foreign function interface)
* Are we writing bytes to a stream? How does this deal work.
* Figure out how to bridge Rust and C++ https://github.com/discordapp/discord-rpc
* Figure out how to structure the project :P

*/

#![feature(libc)]
extern crate libc;
use std::ffi::CString;
use libc::c_int;

pub mod bindings;

#[no_mangle]
pub unsafe extern "C" fn handle_ready() {
    println!("Ready called!")
}

#[no_mangle]
pub unsafe extern "C" fn handle_errored(error_code: i32, error_message: *const i8) {
    println!("Errored called!")
}

#[no_mangle]
pub unsafe extern "C" fn handle_disconnected(error_code: i32, error_message: *const i8) {
    println!("Disconnected called!")
}

#[no_mangle]
pub unsafe extern "C" fn handle_join_game(join_secret: *const i8) {
    println!("Disconnected called!")
}

#[no_mangle]
pub unsafe extern "C" fn handle_join_request(join_request: *const bindings::DiscordJoinRequest) {
    println!("Disconnected called!")
}

#[no_mangle]
pub unsafe extern "C" fn handle_spectate(spectate_secret: *const i8) {
    println!("Disconnected called!")
}

struct DiscordConnection {
    status: bindings::DiscordRichPresence,
}

impl DiscordConnection {
    fn new(application_id: String, auto_register: libc::c_int, steam_id: String) {
        let mut handlers = bindings::DiscordEventHandlers {
            ready: Some(handle_ready),
            errored: Some(handle_errored),
            disconnected: Some(handle_disconnected),
            joinGame: Some(handle_join_game),
            joinRequest: Some(handle_join_request),
            spectateGame: Some(handle_spectate)
        };

        // let application_id  = CString::from_vec_unchecked(application_id);
        // let steam_id 		= CString::from_vec_unchecked(steam_id);

        unsafe {
            bindings::Discord_Initialize(
                CString::new(application_id).unwrap().as_ptr(),
                &mut handlers,
                auto_register,
                CString::new(steam_id).unwrap().as_ptr(),
            );
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

#[test]
fn connect_and_listen() {
    let conn = DiscordConnection::new("419354795699339287".to_string(), 1, "".to_string());
}
