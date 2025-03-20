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
struct Player_data {
    name: String,
    creatures: Vec<Creature>,
    xp: f32,
    inventory: Vec<Item>,
    pos: Pixel_Position,
}

#[derive(Clone, Debug)]
struct Pixel_Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct World_data {
    x: usize,
    y: usize,
}

fn print_keybind() {
    line(Position { x: 0, y: 0 }, "i - 💰 Inventory", "blue");
    line(Position { x: 0, y: 20 }, "m - 🏃 Move", "red");
    line(Position { x: 0, y: 32 }, "s - 🛒 Shop", "yellow");
}

fn render_world(player_position: Pixel_Position, world_data: World_data) {
    let mut current_pixel: Pixel_Position = Pixel_Position { x: 0, y: 0 };
    for row in 0..player_position.x {
        current_pixel.x = row + 1;
        for col in 0..player_position.y {
            current_pixel.y = col + 1;
            line(
                Position {
                    x: current_pixel.x,
                    y: current_pixel.y,
                },
                "X",
                "green",
            );
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
            emoji: "🧞‍♂️".to_string(),
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

    let mut save = Player_data {
        name: "".to_string(),
        creatures: vec![],
        xp: 0.0,
        inventory: vec![],
        pos: Pixel_Position { x: 0, y: 0 },
    };

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
        }

        if playing && !has_starter {
            line(Position { x: 2, y: 0 }, "Choose your starter! 🐣", "white");
            line(Position { x: 3, y: 0 }, "👞 Tikashoe", "red");
            line(Position { x: 3, y: 20 }, "Enjoy balance? ⚖️ (j)", "red");
            line(Position { x: 5, y: 0 }, "🧞‍♂️ Troy", "blue");
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
            if key_press(&app, "m") {
                render_world(save.pos.clone());
            }
        }
    }

    raw_mode(false);
}
