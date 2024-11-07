use mcrs::{Block, Connection, Coordinate};

fn main() {
    let mut mc = Connection::new().expect("Failed to connect");

    println!("{:?}", mc);

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

    let location_a = Coordinate::new(287, 67, 167);
    let location_b = Coordinate::new(288, 69, 166);

    let chunk = mc
        .get_blocks(location_a, location_b)
        .expect("Failed to get blocks");
    println!("Chunk:");
    for item in chunk.iter() {
        println!(
            "{} {} {}",
            item.position_relative(),
            item.position_absolute(),
            item.block(),
        );
    }
    println!("{:?}", chunk);

    let location_a = Coordinate::new(297, 67, 167);
    let location_b = Coordinate::new(298, 69, 166);

    mc.set_blocks(location_a, location_b, Block::GOLD_BLOCK)
        .expect("Failed to set blocks");
    println!("Set blocks.");

    let height_map = mc
        .get_heights(location_a, location_b)
        .expect("Failed to get heights");
    println!("Heights:");
    for item in height_map.iter() {
        println!(
            "{} {} {}",
            item.position_relative(),
            item.position_absolute(),
            item.height(),
        );
    }

    println!("{:?}", height_map);
    println!("{}", height_map.iter().min().unwrap().height());
    println!("{}", height_map.iter().max().unwrap().height());

    println!("{}", Coordinate::from([4, 5, 6]));
}
