use std::collections::HashSet;
use std::io;

struct Ship {
    name: String,
    decks_amount: u8,
    occupied: HashSet<(u8, u8)>,
    is_live: bool,
}

struct NavalFleet {
    battleship: Vec<u8>,
    cruiser: Vec<u8>,
    destroyer: Vec<u8>,
    motortorpedoboot: Vec<u8>,
}

fn main() {
    let mut battleship = create_ship("Battleship", 4);
    
    println!("{}", battleship.name);
    println!("{}", battleship.decks_amount);
    println!("{}", battleship.is_live);
    
    battleship.is_live = false;
    
    print_field(battleship);

}

fn print_field(ship: Ship) {
    for n in 0..10 {
        for o in 0..10 {
            let coords = (n, o);
            if ship.occupied.contains(&coords) {
                print!("{} ", 'x');
                continue;
            }
            if o == 9 {
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

fn create naval_fleet() -> NavalFleet {
    
    NavalFleet {
        
    }
}

