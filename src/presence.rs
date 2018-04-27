use bindings::DiscordRichPresence;
use std::ffi::CString;

#[derive(Debug)]
pub struct Presence {
	c_presence: CPresence,

	pub state: String,
	pub details: String,
	///In Seconds
	pub start_timestamp: i64,
	///In Seconds
	pub end_timestamp: i64,
	pub large_image_key: String,
	pub large_image_text: String,
	pub small_image_key: String,
	pub small_image_text: String,
	pub party_id: String,
	pub party_size: i32,
	pub party_max: i32,
	pub match_secret: String,
	pub join_secret: String,
	pub spectate_secret: String,
	pub instance: i8,
}

impl Presence {
	pub fn default() -> Presence {
		Presence {
			c_presence: CPresence::default(),
			state: String::new(),
			details: String::new(),
			start_timestamp: 0,
			end_timestamp: 0,
			large_image_key: String::new(),
			large_image_text: String::new(),
			small_image_key: String::new(),
			small_image_text: String::new(),
			party_id: String::new(),
			party_size: 0,
			party_max: 0,
			match_secret: String::new(),
			join_secret: String::new(),
			spectate_secret: String::new(),
			instance: 0,
		}
	}

	pub fn as_c_presence(&mut self) -> DiscordRichPresence {
		self.c_presence = CPresence::from_presence(self);

		DiscordRichPresence {
			state: self.c_presence.state.as_ptr(),
			details: self.c_presence.details.as_ptr(),
			start_timestamp: self.start_timestamp,
			end_timestamp: self.end_timestamp,
			large_image_key: self.c_presence.large_image_key.as_ptr(),
			large_image_text: self.c_presence.large_image_text.as_ptr(),
			small_image_key: self.c_presence.small_image_key.as_ptr(),
			small_image_text: self.c_presence.small_image_text.as_ptr(),
			party_id: self.c_presence.party_id.as_ptr(),
			party_size: self.party_size,
			party_max: self.party_max,
			match_secret: self.c_presence.match_secret.as_ptr(),
			join_secret: self.c_presence.join_secret.as_ptr(),
			spectate_secret: self.c_presence.spectate_secret.as_ptr(),
			instance: self.instance,
		}
	}
}

#[derive(Debug)]
struct CPresence {
	pub state: CString,
	pub details: CString,
	pub start_timestamp: i64,
	pub end_timestamp: i64,
	pub large_image_key: CString,
	pub large_image_text: CString,
	pub small_image_key: CString,
	pub small_image_text: CString,
	pub party_id: CString,
	pub party_size: i32,
	pub party_max: i32,
	pub match_secret: CString,
	pub join_secret: CString,
	pub spectate_secret: CString,
	pub instance: i8,
}

impl CPresence {
	fn default() -> CPresence {
		CPresence {
			state: CString::new("").unwrap(),
			details: CString::new("").unwrap(),
			start_timestamp: 0,
			end_timestamp: 0,
			large_image_key: CString::new("").unwrap(),
			large_image_text: CString::new("").unwrap(),
			small_image_key: CString::new("").unwrap(),
			small_image_text: CString::new("").unwrap(),
			party_id: CString::new("").unwrap(),
			party_size: 0,
			party_max: 0,
			match_secret: CString::new("").unwrap(),
			join_secret: CString::new("").unwrap(),
			spectate_secret: CString::new("").unwrap(),
			instance: 0,
		}
	}

	fn from_presence(presence: &Presence) -> CPresence {
		CPresence {
			state: CString::new(presence.state.clone()).unwrap(),
			details: CString::new(presence.details.clone()).unwrap(),
			start_timestamp: presence.start_timestamp,
			end_timestamp: presence.end_timestamp,
			large_image_key: CString::new(presence.large_image_key.clone()).unwrap(),
			large_image_text: CString::new(presence.large_image_text.clone()).unwrap(),
			small_image_key: CString::new(presence.small_image_key.clone()).unwrap(),
			small_image_text: CString::new(presence.small_image_text.clone()).unwrap(),
			party_id: CString::new(presence.party_id.clone()).unwrap(),
			party_size: presence.party_size,
			party_max: presence.party_max,
			match_secret: CString::new(presence.match_secret.clone()).unwrap(),
			join_secret: CString::new(presence.join_secret.clone()).unwrap(),
			spectate_secret: CString::new(presence.spectate_secret.clone()).unwrap(),
			instance: presence.instance,
		}
	}
}
