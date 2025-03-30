use std::fmt;

/// A Minecraft block, including `id` and `modifier`
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Block {
    /// Block identifier. Eg. 'Andesite' has id `1` (`1:5`)
    pub id: u32,
    /// Block modifier. Eg. 'Andesite' has modifier `5` (`1:5`)
    pub modifier: u32,
}

impl Block {
    /// Create a new `Block`
    pub const fn new(id: u32, modifier: u32) -> Self {
        Self { id, modifier }
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}:{})", self.id, self.modifier)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.get_name() {
            Some(name) => write!(f, "{}", name)?,
            None => write!(f, "[UNKNOWN]")?,
        }
        write!(f, " ({}:{})", self.id, self.modifier)?;
        Ok(())
    }
}

macro_rules! blocks {
    ( $( $name:ident = ($id:expr, $modifier:expr); )* ) => {
        impl Block {
            /// Get the non-standard name for the block.
            ///
            /// Corresponds to names of block constants, like `Block::ANDESITE`
            pub fn get_name(&self) -> Option<&'static str> {
                match (self.id, self.modifier) {
                    $( ($id, $modifier) => Some(stringify!($name)), )*
                    _ => None,
                }
            }

            $(
                #[doc = concat!("Minecraft `", stringify!($name), "` block")]
                pub const $name: Self = Self::new($id, $modifier);
            )*
        }
    };
}

blocks! {
    AIR = (0, 0);
    STONE = (1, 0);
    GRANITE = (1, 1);
    POLISHED_GRANITE = (1, 2);
    DIORITE = (1, 3);
    POLISHED_DIORITE = (1, 4);
    ANDESITE = (1, 5);
    POLISHED_ANDESITE = (1, 6);
    GRASS = (2, 0);
    DIRT = (3, 0);
    COARSE_DIRT = (3, 1);
    PODZOL = (3, 2);
    COBBLESTONE = (4, 0);
    OAK_WOOD_PLANK = (5, 0);
    SPRUCE_WOOD_PLANK = (5, 1);
    BIRCH_WOOD_PLANK = (5, 2);
    JUNGLE_WOOD_PLANK = (5, 3);
    ACACIA_WOOD_PLANK = (5, 4);
    DARK_OAK_WOOD_PLANK = (5, 5);
    OAK_SAPLING = (6, 0);
    SPRUCE_SAPLING = (6, 1);
    BIRCH_SAPLING = (6, 2);
    JUNGLE_SAPLING = (6, 3);
    ACACIA_SAPLING = (6, 4);
    DARK_OAK_SAPLING = (6, 5);
    BEDROCK = (7, 0);
    FLOWING_WATER = (8, 0);
    STILL_WATER = (9, 0);
    FLOWING_LAVA = (10, 0);
    STILL_LAVA = (11, 0);
    SAND = (12, 0);
    RED_SAND = (12, 1);
    GRAVEL = (13, 0);
    GOLD_ORE = (14, 0);
    IRON_ORE = (15, 0);
    COAL_ORE = (16, 0);
    OAK_WOOD = (17, 0);
    SPRUCE_WOOD = (17, 1);
    BIRCH_WOOD = (17, 2);
    JUNGLE_WOOD = (17, 3);
    OAK_LEAVES = (18, 0);
    SPRUCE_LEAVES = (18, 1);
    BIRCH_LEAVES = (18, 2);
    JUNGLE_LEAVES = (18, 3);
    SPONGE = (19, 0);
    WET_SPONGE = (19, 1);
    GLASS = (20, 0);
    LAPIS_LAZULI_ORE = (21, 0);
    LAPIS_LAZULI_BLOCK = (22, 0);
    DISPENSER = (23, 0);
    SANDSTONE = (24, 0);
    CHISELED_SANDSTONE = (24, 1);
    SMOOTH_SANDSTONE = (24, 2);
    NOTE_BLOCK = (25, 0);
    BED = (26, 0);
    POWERED_RAIL = (27, 0);
    DETECTOR_RAIL = (28, 0);
    STICKY_PISTON = (29, 0);
    COBWEB = (30, 0);
    DEAD_SHRUB = (31, 0);
    TALL_GRASS = (31, 1);
    FERN = (31, 2);
    DEAD_BUSH = (32, 0);
    PISTON = (33, 0);
    PISTON_HEAD = (34, 0);
    WHITE_WOOL = (35, 0);
    ORANGE_WOOL = (35, 1);
    MAGENTA_WOOL = (35, 2);
    LIGHT_BLUE_WOOL = (35, 3);
    YELLOW_WOOL = (35, 4);
    LIME_WOOL = (35, 5);
    PINK_WOOL = (35, 6);
    GRAY_WOOL = (35, 7);
    LIGHT_GRAY_WOOL = (35, 8);
    CYAN_WOOL = (35, 9);
    PURPLE_WOOL = (35, 10);
    BLUE_WOOL = (35, 11);
    BROWN_WOOL = (35, 12);
    GREEN_WOOL = (35, 13);
    RED_WOOL = (35, 14);
    BLACK_WOOL = (35, 15);
    DANDELION = (37, 0);
    POPPY = (38, 0);
    BLUE_ORCHID = (38, 1);
    ALLIUM = (38, 2);
    AZURE_BLUET = (38, 3);
    RED_TULIP = (38, 4);
    ORANGE_TULIP = (38, 5);
    WHITE_TULIP = (38, 6);
    PINK_TULIP = (38, 7);
    OXEYE_DAISY = (38, 8);
    BROWN_MUSHROOM = (39, 0);
    RED_MUSHROOM = (40, 0);
    GOLD_BLOCK = (41, 0);
    IRON_BLOCK = (42, 0);
    DOUBLE_STONE_SLAB = (43, 0);
    DOUBLE_SANDSTONE_SLAB = (43, 1);
    DOUBLE_WOODEN_SLAB = (43, 2);
    DOUBLE_COBBLESTONE_SLAB = (43, 3);
    DOUBLE_BRICK_SLAB = (43, 4);
    DOUBLE_STONE_BRICK_SLAB = (43, 5);
    DOUBLE_NETHER_BRICK_SLAB = (43, 6);
    DOUBLE_QUARTZ_SLAB = (43, 7);
    STONE_SLAB = (44, 0);
    SANDSTONE_SLAB = (44, 1);
    WOODEN_SLAB = (44, 2);
    COBBLESTONE_SLAB = (44, 3);
    BRICK_SLAB = (44, 4);
    STONE_BRICK_SLAB = (44, 5);
    NETHER_BRICK_SLAB = (44, 6);
    QUARTZ_SLAB = (44, 7);
    BRICKS = (45, 0);
    TNT = (46, 0);
    BOOKSHELF = (47, 0);
    MOSS_STONE = (48, 0);
    OBSIDIAN = (49, 0);
    TORCH = (50, 0);
    FIRE = (51, 0);
    MONSTER_SPAWNER = (52, 0);
    OAK_WOOD_STAIRS = (53, 0);
    CHEST = (54, 0);
    REDSTONE_WIRE = (55, 0);
    DIAMOND_ORE = (56, 0);
    DIAMOND_BLOCK = (57, 0);
    CRAFTING_TABLE = (58, 0);
    WHEAT_CROPS = (59, 0);
    FARMLAND = (60, 0);
    FURNACE = (61, 0);
    BURNING_FURNACE = (62, 0);
    STANDING_SIGN_BLOCK = (63, 0);
    OAK_DOOR_BLOCK = (64, 0);
    LADDER = (65, 0);
    RAIL = (66, 0);
    COBBLESTONE_STAIRS = (67, 0);
    WALLMOUNTED_SIGN_BLOCK = (68, 0);
    LEVER = (69, 0);
    STONE_PRESSURE_PLATE = (70, 0);
    IRON_DOOR_BLOCK = (71, 0);
    WOODEN_PRESSURE_PLATE = (72, 0);
    REDSTONE_ORE = (73, 0);
    GLOWING_REDSTONE_ORE = (74, 0);
    REDSTONE_TORCH_OFF = (75, 0);
    REDSTONE_TORCH_ON = (76, 0);
    STONE_BUTTON = (77, 0);
    SNOW = (78, 0);
    ICE = (79, 0);
    SNOW_BLOCK = (80, 0);
    CACTUS = (81, 0);
    CLAY = (82, 0);
    SUGAR_CANES = (83, 0);
    JUKEBOX = (84, 0);
    OAK_FENCE = (85, 0);
    PUMPKIN = (86, 0);
    NETHERRACK = (87, 0);
    SOUL_SAND = (88, 0);
    GLOWSTONE = (89, 0);
    NETHER_PORTAL = (90, 0);
    JACK_OLANTERN = (91, 0);
    CAKE_BLOCK = (92, 0);
    REDSTONE_REPEATER_BLOCK_OFF = (93, 0);
    REDSTONE_REPEATER_BLOCK_ON = (94, 0);
    WHITE_STAINED_GLASS = (95, 0);
    ORANGE_STAINED_GLASS = (95, 1);
    MAGENTA_STAINED_GLASS = (95, 2);
    LIGHT_BLUE_STAINED_GLASS = (95, 3);
    YELLOW_STAINED_GLASS = (95, 4);
    LIME_STAINED_GLASS = (95, 5);
    PINK_STAINED_GLASS = (95, 6);
    GRAY_STAINED_GLASS = (95, 7);
    LIGHT_GRAY_STAINED_GLASS = (95, 8);
    CYAN_STAINED_GLASS = (95, 9);
    PURPLE_STAINED_GLASS = (95, 10);
    BLUE_STAINED_GLASS = (95, 11);
    BROWN_STAINED_GLASS = (95, 12);
    GREEN_STAINED_GLASS = (95, 13);
    RED_STAINED_GLASS = (95, 14);
    BLACK_STAINED_GLASS = (95, 15);
    WOODEN_TRAPDOOR = (96, 0);
    STONE_MONSTER_EGG = (97, 0);
    COBBLESTONE_MONSTER_EGG = (97, 1);
    STONE_BRICK_MONSTER_EGG = (97, 2);
    MOSSY_STONE_BRICK_MONSTER_EGG = (97, 3);
    CRACKED_STONE_BRICK_MONSTER_EGG = (97, 4);
    CHISELED_STONE_BRICK_MONSTER_EGG = (97, 5);
    STONE_BRICKS = (98, 0);
    MOSSY_STONE_BRICKS = (98, 1);
    CRACKED_STONE_BRICKS = (98, 2);
    CHISELED_STONE_BRICKS = (98, 3);
    BROWN_MUSHROOM_BLOCK = (99, 0);
    RED_MUSHROOM_BLOCK = (100, 0);
    IRON_BARS = (101, 0);
    GLASS_PANE = (102, 0);
    MELON_BLOCK = (103, 0);
    PUMPKIN_STEM = (104, 0);
    MELON_STEM = (105, 0);
    VINES = (106, 0);
    OAK_FENCE_GATE = (107, 0);
    BRICK_STAIRS = (108, 0);
    STONE_BRICK_STAIRS = (109, 0);
    MYCELIUM = (110, 0);
    LILY_PAD = (111, 0);
    NETHER_BRICK = (112, 0);
    NETHER_BRICK_FENCE = (113, 0);
    NETHER_BRICK_STAIRS = (114, 0);
    NETHER_WART = (115, 0);
    ENCHANTMENT_TABLE = (116, 0);
    BREWING_STAND = (117, 0);
    CAULDRON = (118, 0);
    END_PORTAL = (119, 0);
    END_PORTAL_FRAME = (120, 0);
    END_STONE = (121, 0);
    DRAGON_EGG = (122, 0);
    REDSTONE_LAMP_INACTIVE = (123, 0);
    REDSTONE_LAMP_ACTIVE = (124, 0);
    DOUBLE_OAK_WOOD_SLAB = (125, 0);
    DOUBLE_SPRUCE_WOOD_SLAB = (125, 1);
    DOUBLE_BIRCH_WOOD_SLAB = (125, 2);
    DOUBLE_JUNGLE_WOOD_SLAB = (125, 3);
    DOUBLE_ACACIA_WOOD_SLAB = (125, 4);
    DOUBLE_DARK_OAK_WOOD_SLAB = (125, 5);
    OAK_WOOD_SLAB = (126, 0);
    SPRUCE_WOOD_SLAB = (126, 1);
    BIRCH_WOOD_SLAB = (126, 2);
    JUNGLE_WOOD_SLAB = (126, 3);
    ACACIA_WOOD_SLAB = (126, 4);
    DARK_OAK_WOOD_SLAB = (126, 5);
    COCOA = (127, 0);
    SANDSTONE_STAIRS = (128, 0);
    EMERALD_ORE = (129, 0);
    ENDER_CHEST = (130, 0);
    TRIPWIRE_HOOK = (131, 0);
    TRIPWIRE = (132, 0);
    EMERALD_BLOCK = (133, 0);
    SPRUCE_WOOD_STAIRS = (134, 0);
    BIRCH_WOOD_STAIRS = (135, 0);
    JUNGLE_WOOD_STAIRS = (136, 0);
    COMMAND_BLOCK = (137, 0);
    BEACON = (138, 0);
    COBBLESTONE_WALL = (139, 0);
    MOSSY_COBBLESTONE_WALL = (139, 1);
    FLOWER_POT = (140, 0);
    CARROTS = (141, 0);
    POTATOES = (142, 0);
    WOODEN_BUTTON = (143, 0);
    MOB_HEAD = (144, 0);
    ANVIL = (145, 0);
    TRAPPED_CHEST = (146, 0);
    WEIGHTED_PRESSURE_PLATE_LIGHT = (147, 0);
    WEIGHTED_PRESSURE_PLATE_HEAVY = (148, 0);
    REDSTONE_COMPARATOR_INACTIVE = (149, 0);
    REDSTONE_COMPARATOR_ACTIVE = (150, 0);
    DAYLIGHT_SENSOR = (151, 0);
    REDSTONE_BLOCK = (152, 0);
    NETHER_QUARTZ_ORE = (153, 0);
    HOPPER = (154, 0);
    QUARTZ_BLOCK = (155, 0);
    CHISELED_QUARTZ_BLOCK = (155, 1);
    PILLAR_QUARTZ_BLOCK = (155, 2);
    QUARTZ_STAIRS = (156, 0);
    ACTIVATOR_RAIL = (157, 0);
    DROPPER = (158, 0);
    WHITE_HARDENED_CLAY = (159, 0);
    ORANGE_HARDENED_CLAY = (159, 1);
    MAGENTA_HARDENED_CLAY = (159, 2);
    LIGHT_BLUE_HARDENED_CLAY = (159, 3);
    YELLOW_HARDENED_CLAY = (159, 4);
    LIME_HARDENED_CLAY = (159, 5);
    PINK_HARDENED_CLAY = (159, 6);
    GRAY_HARDENED_CLAY = (159, 7);
    LIGHT_GRAY_HARDENED_CLAY = (159, 8);
    CYAN_HARDENED_CLAY = (159, 9);
    PURPLE_HARDENED_CLAY = (159, 10);
    BLUE_HARDENED_CLAY = (159, 11);
    BROWN_HARDENED_CLAY = (159, 12);
    GREEN_HARDENED_CLAY = (159, 13);
    RED_HARDENED_CLAY = (159, 14);
    BLACK_HARDENED_CLAY = (159, 15);
    WHITE_STAINED_GLASS_PANE = (160, 0);
    ORANGE_STAINED_GLASS_PANE = (160, 1);
    MAGENTA_STAINED_GLASS_PANE = (160, 2);
    LIGHT_BLUE_STAINED_GLASS_PANE = (160, 3);
    YELLOW_STAINED_GLASS_PANE = (160, 4);
    LIME_STAINED_GLASS_PANE = (160, 5);
    PINK_STAINED_GLASS_PANE = (160, 6);
    GRAY_STAINED_GLASS_PANE = (160, 7);
    LIGHT_GRAY_STAINED_GLASS_PANE = (160, 8);
    CYAN_STAINED_GLASS_PANE = (160, 9);
    PURPLE_STAINED_GLASS_PANE = (160, 10);
    BLUE_STAINED_GLASS_PANE = (160, 11);
    BROWN_STAINED_GLASS_PANE = (160, 12);
    GREEN_STAINED_GLASS_PANE = (160, 13);
    RED_STAINED_GLASS_PANE = (160, 14);
    BLACK_STAINED_GLASS_PANE = (160, 15);
    ACACIA_LEAVES = (161, 0);
    DARK_OAK_LEAVES = (161, 1);
    ACACIA_WOOD = (162, 0);
    DARK_OAK_WOOD = (162, 1);
    ACACIA_WOOD_STAIRS = (163, 0);
    DARK_OAK_WOOD_STAIRS = (164, 0);
    SLIME_BLOCK = (165, 0);
    BARRIER = (166, 0);
    IRON_TRAPDOOR = (167, 0);
    PRISMARINE = (168, 0);
    PRISMARINE_BRICKS = (168, 1);
    DARK_PRISMARINE = (168, 2);
    SEA_LANTERN = (169, 0);
    HAY_BALE = (170, 0);
    WHITE_CARPET = (171, 0);
    ORANGE_CARPET = (171, 1);
    MAGENTA_CARPET = (171, 2);
    LIGHT_BLUE_CARPET = (171, 3);
    YELLOW_CARPET = (171, 4);
    LIME_CARPET = (171, 5);
    PINK_CARPET = (171, 6);
    GRAY_CARPET = (171, 7);
    LIGHT_GRAY_CARPET = (171, 8);
    CYAN_CARPET = (171, 9);
    PURPLE_CARPET = (171, 10);
    BLUE_CARPET = (171, 11);
    BROWN_CARPET = (171, 12);
    GREEN_CARPET = (171, 13);
    RED_CARPET = (171, 14);
    BLACK_CARPET = (171, 15);
    HARDENED_CLAY = (172, 0);
    BLOCK_OF_COAL = (173, 0);
    PACKED_ICE = (174, 0);
    SUNFLOWER = (175, 0);
    LILAC = (175, 1);
    DOUBLE_TALLGRASS = (175, 2);
    LARGE_FERN = (175, 3);
    ROSE_BUSH = (175, 4);
    PEONY = (175, 5);
    FREESTANDING_BANNER = (176, 0);
    WALLMOUNTED_BANNER = (177, 0);
    INVERTED_DAYLIGHT_SENSOR = (178, 0);
    RED_SANDSTONE = (179, 0);
    CHISELED_RED_SANDSTONE = (179, 1);
    SMOOTH_RED_SANDSTONE = (179, 2);
    RED_SANDSTONE_STAIRS = (180, 0);
    DOUBLE_RED_SANDSTONE_SLAB = (181, 0);
    RED_SANDSTONE_SLAB = (182, 0);
    SPRUCE_FENCE_GATE = (183, 0);
    BIRCH_FENCE_GATE = (184, 0);
    JUNGLE_FENCE_GATE = (185, 0);
    DARK_OAK_FENCE_GATE = (186, 0);
    ACACIA_FENCE_GATE = (187, 0);
    SPRUCE_FENCE = (188, 0);
    BIRCH_FENCE = (189, 0);
    JUNGLE_FENCE = (190, 0);
    DARK_OAK_FENCE = (191, 0);
    ACACIA_FENCE = (192, 0);
    SPRUCE_DOOR_BLOCK = (193, 0);
    BIRCH_DOOR_BLOCK = (194, 0);
    JUNGLE_DOOR_BLOCK = (195, 0);
    ACACIA_DOOR_BLOCK = (196, 0);
    DARK_OAK_DOOR_BLOCK = (197, 0);
    END_ROD = (198, 0);
    CHORUS_PLANT = (199, 0);
    CHORUS_FLOWER = (200, 0);
    PURPUR_BLOCK = (201, 0);
    PURPUR_PILLAR = (202, 0);
    PURPUR_STAIRS = (203, 0);
    PURPUR_DOUBLE_SLAB = (204, 0);
    PURPUR_SLAB = (205, 0);
    END_STONE_BRICKS = (206, 0);
    BEETROOT_BLOCK = (207, 0);
    GRASS_PATH = (208, 0);
    END_GATEWAY = (209, 0);
    REPEATING_COMMAND_BLOCK = (210, 0);
    CHAIN_COMMAND_BLOCK = (211, 0);
    FROSTED_ICE = (212, 0);
    MAGMA_BLOCK = (213, 0);
    NETHER_WART_BLOCK = (214, 0);
    RED_NETHER_BRICK = (215, 0);
    BONE_BLOCK = (216, 0);
    STRUCTURE_VOID = (217, 0);
    OBSERVER = (218, 0);
    WHITE_SHULKER_BOX = (219, 0);
    ORANGE_SHULKER_BOX = (220, 0);
    MAGENTA_SHULKER_BOX = (221, 0);
    LIGHT_BLUE_SHULKER_BOX = (222, 0);
    YELLOW_SHULKER_BOX = (223, 0);
    LIME_SHULKER_BOX = (224, 0);
    PINK_SHULKER_BOX = (225, 0);
    GRAY_SHULKER_BOX = (226, 0);
    LIGHT_GRAY_SHULKER_BOX = (227, 0);
    CYAN_SHULKER_BOX = (228, 0);
    PURPLE_SHULKER_BOX = (229, 0);
    BLUE_SHULKER_BOX = (230, 0);
    BROWN_SHULKER_BOX = (231, 0);
    GREEN_SHULKER_BOX = (232, 0);
    RED_SHULKER_BOX = (233, 0);
    BLACK_SHULKER_BOX = (234, 0);
    WHITE_GLAZED_TERRACOTTA = (235, 0);
    ORANGE_GLAZED_TERRACOTTA = (236, 0);
    MAGENTA_GLAZED_TERRACOTTA = (237, 0);
    LIGHT_BLUE_GLAZED_TERRACOTTA = (238, 0);
    YELLOW_GLAZED_TERRACOTTA = (239, 0);
    LIME_GLAZED_TERRACOTTA = (240, 0);
    PINK_GLAZED_TERRACOTTA = (241, 0);
    GRAY_GLAZED_TERRACOTTA = (242, 0);
    LIGHT_GRAY_GLAZED_TERRACOTTA = (243, 0);
    CYAN_GLAZED_TERRACOTTA = (244, 0);
    PURPLE_GLAZED_TERRACOTTA = (245, 0);
    BLUE_GLAZED_TERRACOTTA = (246, 0);
    BROWN_GLAZED_TERRACOTTA = (247, 0);
    GREEN_GLAZED_TERRACOTTA = (248, 0);
    RED_GLAZED_TERRACOTTA = (249, 0);
    BLACK_GLAZED_TERRACOTTA = (250, 0);
    WHITE_CONCRETE = (251, 0);
    ORANGE_CONCRETE = (251, 1);
    MAGENTA_CONCRETE = (251, 2);
    LIGHT_BLUE_CONCRETE = (251, 3);
    YELLOW_CONCRETE = (251, 4);
    LIME_CONCRETE = (251, 5);
    PINK_CONCRETE = (251, 6);
    GRAY_CONCRETE = (251, 7);
    LIGHT_GRAY_CONCRETE = (251, 8);
    CYAN_CONCRETE = (251, 9);
    PURPLE_CONCRETE = (251, 10);
    BLUE_CONCRETE = (251, 11);
    BROWN_CONCRETE = (251, 12);
    GREEN_CONCRETE = (251, 13);
    RED_CONCRETE = (251, 14);
    BLACK_CONCRETE = (251, 15);
    WHITE_CONCRETE_POWDER = (252, 0);
    ORANGE_CONCRETE_POWDER = (252, 1);
    MAGENTA_CONCRETE_POWDER = (252, 2);
    LIGHT_BLUE_CONCRETE_POWDER = (252, 3);
    YELLOW_CONCRETE_POWDER = (252, 4);
    LIME_CONCRETE_POWDER = (252, 5);
    PINK_CONCRETE_POWDER = (252, 6);
    GRAY_CONCRETE_POWDER = (252, 7);
    LIGHT_GRAY_CONCRETE_POWDER = (252, 8);
    CYAN_CONCRETE_POWDER = (252, 9);
    PURPLE_CONCRETE_POWDER = (252, 10);
    BLUE_CONCRETE_POWDER = (252, 11);
    BROWN_CONCRETE_POWDER = (252, 12);
    GREEN_CONCRETE_POWDER = (252, 13);
    RED_CONCRETE_POWDER = (252, 14);
    BLACK_CONCRETE_POWDER = (252, 15);
    STRUCTURE_BLOCK = (255, 0);
}
