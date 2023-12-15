use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let mut food = (0, 0);
    let mut snake = vec![(1, 2), (1, 3)];
    let mut direction = (0, 1);
    print_board(&snake, &food);
    loop {
        let command: String = io::stdin().lock().read_line().unwrap();
        match command.as_str() {
            "\n" | "" => {}
            "w" | "W" => direction = (0, -1),
            "a" | "A" => direction = (-1, 0),
            "s" | "S" => direction = (0, 1),
            "d" | "D" => direction = (1, 0),
            _ => println!("Invalid command!"),
        }
        let new_head = get_new_position(&snake.last().unwrap(), &direction);
        if snake.contains(&new_head)
            || new_head.0 < 0
            || new_head.0 >= 20
            || new_head.1 < 0
            || new_head.1 >= 20
        {
            println!("Game over!");
            break;
        }
        snake.push(new_head);
        if new_head == food {
            food = get_new_food();
        } else {
            snake.remove(0);
        }
        print_board(&snake, &food);
        thread::sleep(Duration::from_millis(50));
    }
}

fn get_new_position(head: &(i32, i32), direction: &(i32, i32)) -> (i32, i32) {
    let x = head.0 + direction.0;
    let y = head.1 + direction.1;
    (x, y)
}

fn get_new_food() -> (i32, i32) {
    (rand::random::<i32>() % 20, rand::random::<i32>() % 20)
}

fn print_board(snake: &Vec<(i32, i32)>, food: &(i32, i32)) {
    // Clear screen
    print!("\x1b[H\x1b[J");
    for y in 0..20 {
        for x in 0..20 {
            if snake.contains(&(x, y)) {
                print!("█");
            } else if (x, y) == *food {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
