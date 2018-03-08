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
    pub startTimestamp: i64,
    pub endTimestamp: i64,
    pub largeImageKey: *const ::std::os::raw::c_char,
    pub largeImageText: *const ::std::os::raw::c_char,
    pub smallImageKey: *const ::std::os::raw::c_char,
    pub smallImageText: *const ::std::os::raw::c_char,
    pub partyId: *const ::std::os::raw::c_char,
    pub partySize: ::std::os::raw::c_int,
    pub partyMax: ::std::os::raw::c_int,
    pub matchSecret: *const ::std::os::raw::c_char,
    pub joinSecret: *const ::std::os::raw::c_char,
    pub spectateSecret: *const ::std::os::raw::c_char,
    pub instance: i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[link(name="discord-rpc", kind="static")]
pub struct DiscordJoinRequest {
    pub userId: *const ::std::os::raw::c_char,
    pub username: *const ::std::os::raw::c_char,
    pub discriminator: *const ::std::os::raw::c_char,
    pub avatar: *const ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[link(name="discord-rpc", kind="static")]
pub struct DiscordEventHandlers {
    pub ready: ::std::option::Option<unsafe extern "C" fn()>,
    pub disconnected: ::std::option::Option<unsafe extern "C" fn(errorCode: ::std::os::raw::c_int, message: *const ::std::os::raw::c_char)>,
    pub errored: ::std::option::Option<unsafe extern "C" fn(errorCode: ::std::os::raw::c_int, message: *const ::std::os::raw::c_char)>,
    pub joinGame: ::std::option::Option<unsafe extern "C" fn(joinSecret: *const ::std::os::raw::c_char)>,
    pub spectateGame: ::std::option::Option<unsafe extern "C" fn(spectateSecret: *const ::std::os::raw::c_char)>,
    pub joinRequest: ::std::option::Option<unsafe extern "C" fn(request: *const DiscordJoinRequest)>,
}


#[link(name="discord-rpc", kind="static")]
extern "C" {
    pub fn Discord_Initialize(
        applicationId: *const ::std::os::raw::c_char,
        handlers: *mut DiscordEventHandlers,
        autoRegister: ::std::os::raw::c_int,
        optionalSteamId: *const ::std::os::raw::c_char,
    );
    pub fn Discord_Shutdown();
    pub fn Discord_RunCallbacks();
    pub fn Discord_UpdatePresence(presence: *const DiscordRichPresence);
    pub fn Discord_ClearPresence();
    pub fn Discord_Respond(userid: *const ::std::os::raw::c_char, reply: ::std::os::raw::c_int);
}