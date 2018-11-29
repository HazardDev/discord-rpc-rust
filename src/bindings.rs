pub const DISCORD_REPLY_NO: u32 = 0;
pub const DISCORD_REPLY_YES: u32 = 1;
pub const DISCORD_REPLY_IGNORE: u32 = 2;
// pub type int_least8_t = ::std::os::raw::c_schar;
// pub type int_least16_t = ::std::os::raw::c_short;
// pub type int_least32_t = ::std::os::raw::c_int;
// pub type int_least64_t = ::std::os::raw::c_longlong;
// pub type uint_least8_t = ::std::os::raw::c_uchar;
// pub type uint_least16_t = ::std::os::raw::c_ushort;
// pub type uint_least32_t = ::std::os::raw::c_uint;
// pub type uint_least64_t = ::std::os::raw::c_ulonglong;
// pub type int_fast8_t = ::std::os::raw::c_schar;
// pub type int_fast16_t = ::std::os::raw::c_int;
// pub type int_fast32_t = ::std::os::raw::c_int;
// pub type int_fast64_t = ::std::os::raw::c_longlong;
// pub type uint_fast8_t = ::std::os::raw::c_uchar;
// pub type uint_fast16_t = ::std::os::raw::c_uint;
// pub type uint_fast32_t = ::std::os::raw::c_uint;
// pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
// pub type intmax_t = ::std::os::raw::c_longlong;
// pub type uintmax_t = ::std::os::raw::c_ulonglong;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[link(name="discord-rpc", kind="static")]
pub struct DiscordRichPresence {
    pub state: *const ::std::os::raw::c_char,
    pub details: *const ::std::os::raw::c_char,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub large_image_key: *const ::std::os::raw::c_char,
    pub large_image_text: *const ::std::os::raw::c_char,
    pub small_image_key: *const ::std::os::raw::c_char,
    pub small_image_text: *const ::std::os::raw::c_char,
    pub party_id: *const ::std::os::raw::c_char,
    pub party_size: ::std::os::raw::c_int,
    pub party_max: ::std::os::raw::c_int,
    pub match_secret: *const ::std::os::raw::c_char,
    pub join_secret: *const ::std::os::raw::c_char,
    pub spectate_secret: *const ::std::os::raw::c_char,
    pub instance: i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[link(name="discord-rpc", kind="static")]
pub struct DiscordJoinRequest {
    pub user_id: *const ::std::os::raw::c_char,
    pub username: *const ::std::os::raw::c_char,
    pub discriminator: *const ::std::os::raw::c_char,
    pub avatar: *const ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[link(name="discord-rpc", kind="static")]
pub struct DiscordEventHandlers {
    pub ready: ::std::option::Option<unsafe extern "C" fn()>,
    pub disconnected: ::std::option::Option<unsafe extern "C" fn(error_code: ::std::os::raw::c_int, message: *const ::std::os::raw::c_char)>,
    pub errored: ::std::option::Option<unsafe extern "C" fn(error_code: ::std::os::raw::c_int, message: *const ::std::os::raw::c_char)>,
    pub join_game: ::std::option::Option<unsafe extern "C" fn(join_secret: *const ::std::os::raw::c_char)>,
    pub spectate_game: ::std::option::Option<unsafe extern "C" fn(spectate_secret: *const ::std::os::raw::c_char)>,
    pub join_request: ::std::option::Option<unsafe extern "C" fn(request: *const DiscordJoinRequest)>,
}


#[link(name="discord-rpc", kind="static")]
extern "C" {
    pub fn Discord_Initialize(
        application_id: *const ::std::os::raw::c_char,
        handlers: *mut DiscordEventHandlers,
        auto_register: ::std::os::raw::c_int,
        optional_steam_id: *const ::std::os::raw::c_char,
    );
    pub fn Discord_Shutdown();
    pub fn Discord_RunCallbacks();
    pub fn Discord_UpdatePresence(presence: *const DiscordRichPresence);
    pub fn Discord_ClearPresence();
    pub fn Discord_Respond(user_id: *const ::std::os::raw::c_char, reply: ::std::os::raw::c_int);
}
