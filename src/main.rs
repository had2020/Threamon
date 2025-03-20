use std::f64::MIN_POSITIVE;

use rand::Rng;
use TerimalRtdm::*;

#[derive(Clone, Debug)]
struct Creature {
    name: String,
    emoji: String,
    health: f32,
    xp: f32,
    attack: f32,
    special: f32,
    power: f32,
}

#[derive(Clone, Debug)]
struct Item {
    name: String,
    health_benefit: f32,
    cost: f32,
}

#[derive(Clone, Debug)]
struct PlayerData {
    name: String,
    creatures: Vec<Creature>,
    xp: f32,
    inventory: Vec<Item>,
    pos: Pixel_Position,
    world_data: WorldData,
}

#[derive(Clone, Debug)]
struct Pixel_Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct Pixel {
    id: usize,
}

#[derive(Clone, Debug)]
struct WorldData {
    chunk: Vec<Vec<Pixel>>,
}

fn print_keybind() {
    line(Position { x: 0, y: 0 }, "i - 💰 Inventory", "blue");
    line(Position { x: 0, y: 20 }, "WASD - 🏃 Move", "red");
    line(Position { x: 0, y: 36 }, "s - 🛒 Shop", "yellow");
}

fn render_world(player_position: Pixel_Position, world_data: WorldData) {
    let mut current_pixel: Pixel_Position = Pixel_Position { x: 0, y: 0 };
    for row in 0..world_data.chunk.len() {
        //println!("{:?}", current_pixel);
        current_pixel.x = row + 1;
        for col in 0..world_data.chunk[1].len() {
            current_pixel.y = col;
            let color: &str = match world_data.chunk[row][col].id {
                1 => "green",
                2 => "yellow",
                3 => "red",
                _ => "", // white if error
            };
            line(
                Position {
                    x: current_pixel.x + 1, // for top menu offeset
                    y: current_pixel.y,
                },
                "π",
                color,
            );
            if player_position.x == current_pixel.x && player_position.y == current_pixel.y {
                line(
                    Position {
                        x: current_pixel.x + 1,
                        y: current_pixel.y,
                    },
                    "8",
                    "magenta",
                );
            }
        }
    }
}

fn world_gen(player_data: &mut PlayerData) {
    let world_size = Position { x: 10, y: 50 };
    for row in 0..world_size.x {
        player_data.world_data.chunk.push(vec![]);
        for col in 0..world_size.y {
            let mut rng = rand::rng();
            let value = rng.random_range(1..4);
            player_data.world_data.chunk[row].push(Pixel { id: value });
        }
    }
}

fn main() {
    let catalog: Vec<Creature> = vec![
        Creature {
            name: "Tikashoe".to_string(),
            emoji: "👞".to_string(),
            health: 10.0,
            xp: 0.0,
            attack: 1.0,
            special: 2.0,
            power: 20.0,
        },
        Creature {
            name: "Troy".to_string(),
            emoji: "🧙‍♂️".to_string(),
            health: 5.0,
            xp: 0.0,
            attack: 3.0,
            special: 8.0,
            power: 20.0,
        },
        Creature {
            name: "Dalius".to_string(),
            emoji: "🐮".to_string(),
            health: 15.0,
            xp: 0.0,
            attack: 2.0,
            special: 4.0,
            power: 10.0,
        },
    ];

    let mut playing: bool = false;
    let mut has_starter: bool = false;

    let mut save = PlayerData {
        name: "".to_string(),
        creatures: vec![],
        xp: 0.0,
        inventory: vec![],
        pos: Pixel_Position { x: 1, y: 1 },
        world_data: WorldData { chunk: vec![] },
    };

    let mut player_in_battle: bool = false;

    clear();
    let mut app = App::new();

    raw_line("________________________________________________");
    raw_line("Welcome to Threamon, the next 80s creature game!");
    raw_line("________________________________________________");
    raw_line(" ");
    raw_line("q 👈 to quit 🏃");
    raw_line(" ");
    raw_line("p 👈 to play 🎲");

    raw_mode(true);

    // app loop
    loop {
        clear();
        collect_presses(&mut app);

        if key_press(&app, "q") {
            clear();
            break;
        }

        if key_press(&app, "p") {
            playing = true;
            world_gen(&mut save);
        }

        if playing && !has_starter {
            line(Position { x: 0, y: 0 }, "Choose your starter! 🐣", "");
            line(Position { x: 3, y: 0 }, "👞 Tikashoe", "red");
            line(Position { x: 3, y: 20 }, "Enjoy balance? ⚖️ (j)", "red");
            line(Position { x: 5, y: 0 }, "🧙‍♂️ Troy", "blue");
            line(
                Position { x: 5, y: 20 },
                "Do you prefer war? 🔫 (k)",
                "blue",
            );
            line(Position { x: 7, y: 0 }, "🐮 Dalius", "yellow");
            line(
                Position { x: 7, y: 20 },
                "Are you a coward? 😬(l)",
                "yellow",
            );

            // option check
            if key_press(&app, "j") {
                has_starter = true;
                clear();
                save.creatures.push(catalog[0].clone());
                line(
                    Position { x: 2, y: 0 },
                    "👞 Tikashoe added to bag.",
                    "green",
                );
                line(
                    Position { x: 3, y: 0 },
                    "Press m to start your journey! 🌇",
                    "yellow",
                );
                if key_press(&app, "m") {
                    has_starter = true;
                }
            }
            if key_press(&app, "k") {
                has_starter = true;
                clear();
                save.creatures.push(catalog[1].clone());
                line(Position { x: 2, y: 0 }, "🧞‍♂️ Troy added to bag.", "green");
                line(
                    Position { x: 3, y: 0 },
                    "Press m to start your journey! 🌇",
                    "yellow",
                );
                if key_press(&app, "m") {
                    has_starter = true;
                }
            }
            if key_press(&app, "l") {
                has_starter = true;
                clear();
                save.creatures.push(catalog[2].clone());
                line(Position { x: 2, y: 0 }, "🐮 Dalius added to bag.", "green");
                line(
                    Position { x: 3, y: 0 },
                    "Press m to start your journey! 🌇",
                    "yellow",
                );
                if key_press(&app, "m") {
                    has_starter = true;
                }
            }
        }

        if playing && has_starter {
            clear();
            print_keybind();
            render_world(save.pos.clone(), save.world_data.clone());

            let mut moved_flag: bool = false;
            if key_press(&app, "w") && !player_in_battle {
                save.pos.x -= 1;
                moved_flag = true;
            }
            if key_press(&app, "a") && !player_in_battle {
                save.pos.y -= 1;
                moved_flag = true;
            }
            if key_press(&app, "s") && !player_in_battle {
                save.pos.x += 1;
                moved_flag = true;
            }
            if key_press(&app, "d") && !player_in_battle {
                save.pos.y += 1;
                moved_flag = true;
            }

            if moved_flag && !player_in_battle {
                let mut rng = rand::rng();
                if save.world_data.chunk[save.pos.x.clone()][save.pos.y.clone()].id == 1 {
                    if rng.random_range(1..20) == 3 {
                        player_in_battle = true;
                    }
                }
                if save.world_data.chunk[save.pos.x.clone()][save.pos.y.clone()].id == 2 {
                    if rng.random_range(1..10) == 2 {
                        player_in_battle = true;
                    }
                }
                if save.world_data.chunk[save.pos.x.clone()][save.pos.y.clone()].id == 3 {
                    if rng.random_range(1..5) == 1 {
                        player_in_battle = true;
                    }
                }
            }

            if player_in_battle {
                clear();

                let mut rng = rand::rng();
                let creature_index = rng.random_range(0..catalog.len());
                let name = format!(
                    "You encontered {} {}",
                    catalog[creature_index].emoji, catalog[creature_index].name,
                );
                let health = format!("Hp: {}", catalog[creature_index].health,);
                let xp = format!("Xp: {}", catalog[creature_index].xp,);
                let attack = format!("Attack: {}", catalog[creature_index].attack,);
                let special = format!("Special: {}", catalog[creature_index].special,);
                let power = format!("Power: {}", catalog[creature_index].power,);

                line(Position { x: 0, y: 0 }, &name, "");
                line(Position { x: 2, y: 0 }, &health, "green");
                line(Position { x: 3, y: 0 }, &xp, "blue");
                line(Position { x: 4, y: 0 }, &attack, "red");
                line(Position { x: 5, y: 0 }, &special, "red");
                line(Position { x: 6, y: 0 }, &power, "yellow");

                line(Position { x: 7, y: 0 }, "🏃Run (r), 🥊Fight (f)", "");
            }
        }
    }

    raw_mode(false);
}
