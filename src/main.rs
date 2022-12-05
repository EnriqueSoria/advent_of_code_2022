use std::io;

fn main() -> io::Result<()> {
    let mut elf_max_calories = 0;
    let mut current_elf_calories = 0;

    for line in io::stdin().lines() {
        let current_line = line.unwrap();

        if current_line == "" {
            if current_elf_calories > elf_max_calories {
                elf_max_calories = current_elf_calories;
            }
            current_elf_calories = 0;
            continue
        } else {
            let current_calories:i32 = current_line.parse().unwrap();
            current_elf_calories = current_elf_calories + current_calories;
        }
    }

    println!("Max elf: {}", elf_max_calories);

    Ok(())
}
