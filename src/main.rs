use mcrs::Connection;

fn main() {
    let mut mc = Connection::new().expect("Failed to connect");

    mc.post_to_chat("Hel)\nlo").expect("Failed to send");
    println!("Sent.");

    let mut player = mc
        .get_player_position()
        .expect("Failed to get player position");
    println!("Player position: {:?}", player);

    player.x += 1;

    mc.set_player_position(player)
        .expect("Failed to set player position");
    println!("Set position.");
}
