extern crate discord_rpc;

use std::{thread, time};
use discord_rpc::DiscordConnection;
use discord_rpc::bindings::DiscordEventHandlers;
use discord_rpc::presence::Presence;

#[test]
fn connect_and_listen() {
    let mut handlers = DiscordEventHandlers {
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


    //Initial Presence
    let mut presence = Presence::default();

    presence.state = String::from("WOO DISCORD");
    presence.details = String::from("Rust Rich Presence");
    presence.large_image_key = String::from("default");
    presence.start_timestamp = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("Time flows backwards.")
        .as_secs() as i64;

    loop {
        conn.run_callbacks();
        if conn.ready() {
            break;
        }
    }

    let two_seconds = time::Duration::from_secs(2);
    for i in 0..10 {

        let now = time::Instant::now();

        thread::sleep(two_seconds);
        assert!(now.elapsed() >= two_seconds);

        presence.large_image_text = String::from("What is this amazing thing!");
        presence.party_max = 10;
        presence.party_size = i + 1;

        conn.run_callbacks();
        conn.update(&mut presence);
        println!("Updating Presence with {:?}", presence);
    }

    for i in 0..10 {

        let now = time::Instant::now();

        thread::sleep(two_seconds);
        assert!(now.elapsed() >= two_seconds);

        presence.state = String::from("Other Presence");

        conn.run_callbacks();
        conn.update(&mut presence);
        println!("Updating Presence with {:?}", presence);
    }
}
