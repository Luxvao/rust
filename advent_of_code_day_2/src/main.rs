use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("input.txt").unwrap();
    
    let mut input_buffer = String::new();

    file.read_to_string(&mut input_buffer).unwrap();

    let lines = input_buffer.lines();

    let lines_tuples = lines.map(|i| {
        let tmp = i.split(" ").collect::<Vec<&str>>();

        (tmp[0], tmp[1])
    }).collect::<Vec<(&str, &str)>>();

    part_1(&lines_tuples);

    part_2(&lines_tuples);
}

fn part_1(lines_tuples: &Vec<(&str, &str)>) {
    let mut total_points = 0;

    lines_tuples.iter().for_each(|(oponent, me)| {
        // Rock
        if me.trim() == "X" {
            total_points += 1;

            if oponent.trim() == "A" {
                total_points += 3;
            }
            else if oponent.trim() == "C" {
                total_points += 6;
            }
        }
        // Paper
        else if me.trim() == "Y" {
            total_points += 2;

            if oponent.trim() == "A" {
                total_points += 6;
            }
            else if oponent.trim() == "B" {
                total_points += 3;
            }
        }
        // Scissors
        else if me.trim() == "Z" {
            total_points += 3;

            if oponent.trim() == "B" {
                total_points += 6;
            }
            else if oponent.trim() == "C" {
                total_points += 3;
            }
        }
    });

    println!("Part One: Total Points: {}", total_points);
}

fn part_2(lines_tuples: &Vec<(&str, &str)>) {
   let mut total_points = 0;

    lines_tuples.iter().for_each(|(oponent, me)| {
        // Lose
        if me.trim() == "X" {
            if oponent.trim() == "A" {
                total_points += 3;
            }
            else if oponent.trim() == "B" {
                total_points += 1;
            }
            else if oponent.trim() == "C" {
                total_points += 2;
            }
        }
        // Draw
        else if me.trim() == "Y" {
            total_points += 3;

            if oponent.trim() == "A" {
                total_points += 1;
            }
            else if oponent.trim() == "B" {
                total_points += 2;
            }
            else if oponent.trim() == "C" {
                total_points += 3;
            }
        }
        // Win
        else if me.trim() == "Z" {
            total_points += 6;

            if oponent.trim() == "A" {
                total_points += 2;
            }
            else if oponent.trim() == "B" {
                total_points += 3;
            }
            else if oponent.trim() == "C" {
                total_points += 1;
            }
        }
    });

    println!("Part Two: Total Points {}", total_points);
}




