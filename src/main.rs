use std::io;

fn main() -> io::Result<()> {
    let mut calories_count:Vec<i32> = Vec::new();
    let mut current_elf_calories = 0;

    for line in io::stdin().lines() {
        let current_line = line.unwrap();

        if current_line == "" {
            calories_count.push(current_elf_calories);
            current_elf_calories = 0;
            continue
        } else {
            let current_calories:i32 = current_line.parse().unwrap();
            current_elf_calories = current_elf_calories + current_calories;
        }
    }

    calories_count.sort();
    calories_count.reverse();
    let top_three = calories_count[0] + calories_count[1] + calories_count[2];
    println!("Top three calory count: {}", top_three);

    Ok(())
}
