
mod field;


fn read_two_integers() -> (usize, usize) {
    use std::io;
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let coords: Vec<usize> = input
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        if coords.len() == 2 {
            return (coords[0], coords[1]);
        }

        println!("Invalid input. Please enter two space-separated numbers.");
    }
}

fn read_coordinates() -> (usize, usize) {
    println!("Enter coordinates (x y): ");
    return read_two_integers();
}

fn read_field_dimensions() -> (std::num::NonZeroUsize, std::num::NonZeroUsize) {
    loop {
        println!("Enter width and height of the field (e.g., '10 5'): ");
        let (width, height) = read_two_integers();
        if let (Some(width), Some(height)) = (std::num::NonZeroUsize::new(width), std::num::NonZeroUsize::new(height)) {
            return (width, height);
        } else {
            println!("Invalid dimensions. Both width and height must be non-zero positive integers.");
        }
    }
}

fn main() {
    let (width, height) = read_field_dimensions();
    let mut field = field::Field::new(width, height);

    loop {
        field.show();
        let (x, y) = read_coordinates();
        match field.click(x, y) {
            Ok(()) => {
                if field.all_non_mine_visible() {
                    field.show();
                    println!("Congratulations! You've revealed all non-mine cells.");
                    break;
                }
            }
            Err(field::Fail::OutOfBounds) => println!("Invalid coordinates. Please enter coordinates within the field."),
            Err(field::Fail::MineExploded) => {
                field.show();
                println!("Boom! You clicked on a mine. Game over.");
                break;
            }
        }
    }
}
