use std::collections::HashSet;
use std::io;
use std::io::{Write};

#[derive(Debug)]
enum ShipType {
    Battleship,
    Cruiser,
}

#[derive(Debug)]
struct Ship {
    name: String,
    ship_type: ShipType,
    decks_amount: u8,
    occupied: HashSet<(u8, u8)>,
    is_live: bool,
}

fn main () {
    let mut my_ship = Ship {
        name: String::from("Cruiser-1"),
        ship_type: ShipType::Cruiser,
        decks_amount: 3,
        occupied: HashSet::new(),
        is_live: true,
    };
    
    for n in 0..my_ship.decks_amount {
        clear_console();
        my_ship.occupied.insert(get_user_coordinates());
        print_field(&my_ship);
    }
}

fn print_field(ship: &Ship) {
    for n in (0..10).rev() {
        for o in 0..10 {
            let coords = (n, o);
            if ship.occupied.contains(&coords) {
                print!("■ ");
                continue;
            }
            print!("· ");
            if o == 9 {
                println!();
                continue;
            }
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

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}
