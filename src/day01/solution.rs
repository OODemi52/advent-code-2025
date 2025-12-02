use std::fs;

pub fn run() -> () {
    let input = fs::read_to_string("input/day01.txt").unwrap();
    // First thing is we need to parse out the input commands, line by line
    // First character will indicate the direction, L for left, R for right
    // The remainaing is the step (e.g. L10 mean step to the left by 10 units)
    //
    // We can loop through the list, split between first character and remaining
    // then execute on these values. L and R can correspond to +1 and -1 multipliers
    // As we go through, keep a running total and count, every iteration check if the
    // total is 0, and add to the count if so. Return the count at the end
    //
    // Key to keep in mind, the problem has a circular nature, meaning that the values can
    // only step between 0 and 99, then the begin again (e.g. 95 + 10 = 05)
    let rotations: Vec<&str> = input.lines().collect();

    let mut count: usize = 0;
    let mut total: isize = 50;

    for rotation in rotations {
        let (direction, step_str) = rotation.split_at(1);

        let multiplier: isize = match direction {
            "L" => -1,
            "R" => 1,
            _ => panic!("invalid direction"),
        };

        let step: isize = step_str.parse().unwrap();

        println!("Direction: {} Step: {}", multiplier, step);

        total = total + (step * multiplier);

        // Normalizes it, if outside the range
        // We do a while loop because if this is like a big number, we need to wittle down
        while total > 99 || total < 0 {
            if total > 99 {
                total = total - 100
            } else if total < 0 {
                total = 100 + total;
            }
        }

        // ^ this is what i came up with but this is probably not so efficient if the values grow
        // Chat says this is a better way to handle: total = (total % 100 + 100) % 100;
        // But i will leave this here since this is the solution i came up with and it worked.

        println!("Total: {}", total);

        if total == 0 {
            count += 1
        }
    }

    println!("Password: {}", count);
}
