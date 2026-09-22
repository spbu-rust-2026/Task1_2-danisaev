use std::io;

fn main() {
    let mut counter = 0;
    loop {
        let mut curr = String::new();
        let bytes = io::stdin()
            .read_line(&mut curr)
            .expect("Cho to ne poluchilos");
        if bytes == 0 {
            println!("{}", counter);
            break;
        }
        if curr.trim() == "" {
            continue;
        }
        match curr.trim().parse::<i128>() {
            Ok(n) => {
                if n == -1 {
                    println!("{}\n", counter);
                    break;
                } else if n <= 0 {
                    println!("NaN\n");
                    break;
                } else {
                    counter += n
                }
            }
            Err(_) => {
                println!("NaN\n");
                break;
            }
        }
    }
}
