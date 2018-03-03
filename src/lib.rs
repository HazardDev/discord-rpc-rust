/*

* Figure out Rust and FFI (foreign function interface)
* Are we writing bytes to a stream? How does this deal work.
* Figure out how to bridge Rust and C++ https://github.com/discordapp/discord-rpc
* Figure out how to structure the project :P

*/

#![feature(libc)]
extern crate libc;
use std::ffi::CString;


#[link(name = "discord-rpc", kind="static")]
extern "C" {

	fn Discord_Initialize(application_id: *const u8, handlers: DiscordEventHandlers, auto_register: u32, steam_id: *const u8);
	fn Discord_Shutdown();
	fn Discord_UpdatePresence(update: DiscordRichPresence);
	fn Discord_ClearPresence();
	fn Discord_RunCallbacks();
	fn Discord_Respond(userid: *const u8, reply: u32);

}

#[repr(C)]
struct DiscordEventHandlers {

	ready: extern fn(),
	errored: extern fn(error_code: u32, error_message: *const u8),
	disconnected: extern fn(error_code: u32, error_message: *const u8)

}

#[repr(C)]
struct DiscordRichPresence {
	state: *const u8,
	details: *const u8,
	start_timestamp: u64,
	end_timestamp: u64,
	large_image_key: *const u8,
	large_image_text: *const u8,
	small_image_key: *const u8,
	small_image_text: *const u8,
	party_id: *const u8,
	party_size: u32,
	party_max: u32
}


#[no_mangle]
pub extern fn handle_ready() {

	println!("Ready called!")

}

#[no_mangle]
pub extern fn handle_errored(error_code: u32, error_message: *const u8) {

	println!("Errored called!")


}

#[no_mangle]
pub extern fn handle_disconnected(error_code: u32, error_message: *const u8) {
	println!("Disconnected called!")

}

struct DiscordConnection {
	status: DiscordRichPresence
}

impl DiscordConnection {
	
	fn new(application_id: String, auto_register: u32, steam_id: String) {

		let handlers = DiscordEventHandlers {
			ready: handle_ready,
			errored: handle_errored,
			disconnected: handle_disconnected
		};

		// let application_id  = CString::from_vec_unchecked(application_id);
		// let steam_id 		= CString::from_vec_unchecked(steam_id); 

		unsafe { 
			Discord_Initialize(application_id.as_ptr(), handlers, auto_register, steam_id.as_ptr());
		}

	}

}

impl Drop for DiscordConnection {

	fn drop(&mut self) {

		println!("Dropping DiscordConnection");

		unsafe {
			Discord_Shutdown();
		}

	}

}


#[test]
fn connect_and_listen() {
	
	let conn = DiscordConnection::new("419354795699339287".to_string(), 1, "".to_string());

}