use mcrs::{Block, Connection};

fn main() {
    let mut mc = Connection::new().expect("Failed to connect");

    mc.do_command("help\n").expect("Failed to send command");

    mc.post_to_chat("Hel)\nlo").expect("Failed to send");
    println!("Sent.");

    let mut player = mc
        .get_player_position()
        .expect("Failed to get player position");
    println!("Player position: {:?}", player);

    let block = Block::new(1, 0);

    mc.set_block(player, block)
        .expect("Failed to set player position");
    println!("Set block.");

    player.x += 1;

    mc.set_player_position(player)
        .expect("Failed to set player position");
    println!("Set position.");

    player.y -= 1;

    let block = mc.get_block(player).expect("Failed to get block");
    println!("Block: {:?}", block);

    let height = mc
        .get_height(player.x, player.z)
        .expect("Failed to get height");
    println!("Height: {:?}", height);
}
