use mcrs::{Block, Connection, Coordinate};

#[test]
fn main() {
    let mut mc = Connection::new().expect("Failed to connect");
    // println!("{:?}", mc);

    let location_a = Coordinate::new(600, 100, 400);
    let location_b = Coordinate::new(601, 101, 401);

    // println!("{}", mc.get_player_position().unwrap_err());

    let chunk = mc
        .get_blocks(location_a, location_b)
        .expect("Failed to get blocks");
    println!("Chunk:");
    for item in chunk.into_iter() {
        println!(
            "{} {} {}",
            item.position_offset(),
            item.position_worldspace(),
            item.block(),
        );
    }
    println!("{:?}", chunk);

    let mut chunk = mc
        .get_blocks_stream(location_a, location_b)
        .expect("Failed to get blocks");
    println!("Chunk:");
    while let Some(item) = chunk.next().expect("Failed to read chunk") {
        println!(
            "{} {} {}",
            item.position_offset(),
            item.position_worldspace(),
            item.block(),
        );
    }

    let chunk = mc
        .get_blocks_stream(location_a, location_b)
        .expect("Failed to get blocks");
    println!("Chunk:");
    let chunk = chunk.collect().expect("Failed to read chunk");
    for item in chunk.into_iter() {
        println!(
            "{} {} {}",
            item.position_offset(),
            item.position_worldspace(),
            item.block(),
        );
    }
    println!("{:?}", chunk);

    println!("{}", Block::new(1, 8));
    println!("{}", Block::new(36, 0));

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
        .get_height((player.x, player.z))
        .expect("Failed to get height");
    println!("Height: {:?}", height);

    let location_a = Coordinate::new(287, 67, 167);
    let location_b = Coordinate::new(288, 69, 166);

    let chunk = mc
        .get_blocks(location_a, location_b)
        .expect("Failed to get blocks");
    println!("Chunk:");
    for item in chunk.into_iter() {
        println!(
            "{} {} {}",
            item.position_offset(),
            item.position_worldspace(),
            item.block(),
        );
    }
    println!("{:?}", chunk);

    let location_a = Coordinate::new(297, 67, 167);
    let location_b = Coordinate::new(298, 69, 166);

    mc.set_blocks(location_a, location_b, Block::GOLD_BLOCK)
        .expect("Failed to set blocks");
    println!("Set blocks.");

    let heights = mc
        .get_heights(location_a, location_b)
        .expect("Failed to get heights");
    println!("Heights:");
    for item in heights.into_iter() {
        println!(
            "{} {} {}",
            item.position_offset(),
            item.position_worldspace(),
            item.height(),
        );
    }

    let mut heights2 = mc
        .get_heights_stream(location_a, location_b)
        .expect("Failed to get heights");
    println!("Heights:");
    while let Some(item) = heights2.next().expect("Failed to get height") {
        println!(
            "{} {} {}",
            item.position_offset(),
            item.position_worldspace(),
            item.height(),
        );
    }

    println!("{:?}", heights);
    println!("{}", heights.into_iter().min().unwrap().height());
    println!("{}", heights.into_iter().max().unwrap().height());

    println!("{}", Coordinate::from([4, 5, 6]));

    println!("{}", Coordinate::from([4, 5, 6]) + (7, 8, 9));
    println!("{}", Coordinate::from((4, 5, 6)) - [7, 8, 9]);
}
