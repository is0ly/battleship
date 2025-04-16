use std::collections::HashSet;
use std::io;

#[derive(Debug)]
enum ShipType {
    Battleship,
    Cruiser,
    Destroyer,
    Motortorpedoboot,
}

impl ShipType  {
    fn decks(&self) -> u8 {
        match self {
            ShipType::Battleship => 4,
            ShipType::Cruiser => 3,
            ShipType::Destroyer=> 2,
            ShipType::Motortorpedoboot => 1,
        }
    }
    
    fn as_str(&self) -> &'static str {
        match self {
            ShipType::Battleship => "Battleship",
            ShipType::Cruiser => "Cruiser",
            ShipType::Destroyer=> "Destroyer",
            ShipType::Motortorpedoboot => "Motortorpedoboot",
        }
    }
}

struct Ship {
    name: String,
    decks_amount: u8,
    occupied: HashSet<(u8, u8)>,
    is_live: bool,
}

struct NavalFleet {
    ships: Vec<Ship>,
}

fn main() {
    // let mut battleship = create_ship("Battleship", 4);

    let mut player_naval_fleet = create_naval_fleet();

    println!("{}", player_naval_fleet.battleship[0].name);
    println!("{}", player_naval_fleet.battleship[0].decks_amount);
    println!("{}", player_naval_fleet.battleship[0].is_live);

    player_naval_fleet.battleship[0].is_live = false;

    print_field(player_naval_fleet);
}

fn print_field(user_naval_fleet: NavalFleet) {
    for n in (1..11).rev() {
        for o in 1..11 {
            let coords = (n, o);
            if user_naval_fleet.battleship[0].occupied.contains(&coords) {
                print!("{} ", 'x');
                continue;
            }
            if o == 10 {
                println!("");
                continue;
            }
            print!("{} ", 'o');
        }
    }
}

fn get_user_coordinates() -> (u8, u8) {
    let mut point = String::new();

    println!("Enter coordinates-> ");

    io::stdin()
        .read_line(&mut point)
        .expect("Failed to read line");

    let digits: Vec<u8> = point
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let coordinates = (digits[0], digits[1]);

    coordinates
}

fn create_ship(name: &str, decks_amount: u8) -> Ship {
    let mut battleship_occupied: HashSet<(u8, u8)> = HashSet::new();

    for _n in 0..decks_amount {
        battleship_occupied.insert(get_user_coordinates());
    }

    Ship {
        name: name.to_string(),
        decks_amount: 4,
        occupied: battleship_occupied,
        is_live: true,
    }
}

fn create_naval_fleet() -> NavalFleet {
    let battleship_count: u8 = 1;
    // let cruiser_count: u8 = 2;
    // let destroyer_count: u8 = 3;
    // let motortorpedoboot_count: u8 = 4;

    let mut battleships = vec![];
    // let mut cruisers = vec![];
    // let mut destroyers = vec![];
    // let mut motortorpedoboots = vec![];

    for _n in 0..battleship_count {
        println!("Create battleship - 1");
        battleships.push(create_ship("Battleship", 4));
    }

    // for n in 0..cruiser_count {
    //     println!("Create cruisers - 2");
    //     battleships.push(create_ship("Cruiser", 3));
    // }

    // for n in 0..destroyer_count {
    //     println!("Create destroyers - 3");
    //     battleships.push(create_ship("Destroyer", 2));
    // }

    // for n in 0.. motortorpedoboot_count {
    //     println!("Create motortorpedoboots - 4");
    //     battleships.push(create_ship("Motortorpedoboot", 1));
    // }

    NavalFleet {
        battleship: battleships,
        // cruiser: cruisers,
        // destroyer: destroyers,
        // motortorpedoboot: motortorpedoboots,
    }
}
