use std::env;
use std::io::{Write, stderr};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        writeln!(&mut stderr(), "figure-1: too few arguments").unwrap();
        std::process::exit(1);
    } else if args.len() > 3 {
        writeln!(&mut stderr(), "figure-1: too much arguments").unwrap();
        std::process::exit(1);
    }

    let x: Result<i32, _> = args[1].parse();
    let y: Result<i32, _> = args[2].parse();

    match (x, y) {
        (Ok(x_coord), Ok(y_coord)) => {
            if x_coord < -100 || x_coord > 100 || y_coord < -100 || y_coord > 100 {
                writeln!(&mut stderr(), "figure-1: out of range").unwrap();
                std::process::exit(1);
            }

            // Check if the point is inside, outside, or on the border of the figure
            // You can add your own logic here
            let h = x_coord.pow(2) + y_coord.pow(2);

            // первая четверть
            if (x_coord > 10 || y_coord > 10) && h < 400 {
                println!("inside");
                std::process::exit(0);
            } else if (x_coord == 10 || y_coord == 10) || h == 400 {
                println!("border");
                std::process::exit(0);
            } else if x_coord > 0 && y_coord > 0 && h > 400 {
                println!("outside");
                std::process::exit(0);
            }

            // вторая четверть
            if (h > 100) && ((x_coord > -20 && x_coord < 0) && (y_coord < 20 && y_coord > 0)) {
                println!("inside");
                std::process::exit(0);
            } else if (x_coord == -20 || y_coord == 20) || h == 100 {
                println!("border");
                std::process::exit(0);
            } else if x_coord < -20 || y_coord > 20 {
                println!("outside");
                std::process::exit(0);
            }

            // третья четверть
            if (x_coord < -10 || y_coord < -10) && x_coord > -20 && y_coord > -20 {
                println!("inside");
                std::process::exit(0);
            } else if (x_coord == -10 || y_coord == -10) || (x_coord == -20 || y_coord == -20) {
                println!("border");
                std::process::exit(0);
            } else if x_coord < -20 || y_coord < -20 {
                println!("outside");
                std::process::exit(0);
            }

            // четвертая четверть
            if h > 100 && h < 400 {
                println!("inside");
                std::process::exit(0);
            } else if h == 100 || h == 400 {
                println!("border");
                std::process::exit(0);
            } else if x_coord > 0 && y_coord < 0 && h > 400 {
                println!("outside");
                std::process::exit(0);
            }

            if ((x_coord > -10 && x_coord <= 0) && (y_coord < 10 && y_coord >= 0) && h < 100)
                || ((x_coord < 10 && x_coord >= 0) && (y_coord > -10 && y_coord <= 0) && h < 100)
                || ((x_coord < 10 && x_coord >= 0) && (y_coord < 10 && y_coord >= 0))
                || ((x_coord > -10 && x_coord <= 0) && (y_coord > -10 && y_coord <= 0))
            {
                println!("outside");
                std::process::exit(0);
            }
            
        }
        _ => {
            writeln!(&mut stderr(), "figure-1: bad format").unwrap();
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn figure() {
        let args = vec!["main".to_string(), "10".to_string(), "20".to_string()];
        let x: Result<i32, _> = args[1].parse();
        let y: Result<i32, _> = args[2].parse();

        assert_eq!(x, Ok(10));
        assert_eq!(y, Ok(20));
    }

    #[test]
    fn figure1() {
        let args = vec!["main".to_string(), "5".to_string(), "5".to_string()];
        let x: Result<i32, _> = args[1].parse();
        let y: Result<i32, _> = args[2].parse();

        assert_eq!(x, Ok(5));
        assert_eq!(y, Ok(5));
    }

    #[test]
    fn figure2() {
        let args = vec!["main".to_string(), "15".to_string(), "10".to_string()];
        let x: Result<i32, _> = args[1].parse();
        let y: Result<i32, _> = args[2].parse();

        assert_eq!(x, Ok(15));
        assert_eq!(y, Ok(10));
    }
}
