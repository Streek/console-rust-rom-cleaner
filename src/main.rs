// import path system
use std::collections::HashMap;
use std::fs;

static INPUT: &str = "/mnt/d/DeckEmulationSync/roms/nes/";

fn main() {
    // hashmap of strings and arrays of strings
    let mut counts: HashMap<String, Vec<String>> = HashMap::new();
    // read files in directory

    if let Ok(entries) = fs::read_dir(INPUT) {
        for entry in entries {
            if let Ok(entry) = entry {
                // get string file_name from entry
                let file_name: String = entry.file_name().into_string().unwrap();

                let split = &file_name
                    .split(&['(', ')', '[', ']'][..])
                    .collect::<Vec<&str>>();

                if split.len() > 1 {
                    counts
                        .entry(split[0].to_string())
                        .or_insert(vec![])
                        .push(String::from(&file_name));
                }

                continue;
            }
        }
        // loop through hashmap and print out key and value
        for (_key, value) in counts.iter() {
            if value.len() == 0 {
                // println!("No values: {:?}", key);
                continue;
            }
            if value.len() == 1 {
                // println!("One value: {:?}", key);
                continue;
            }
            if value.len() > 1 {
                // println!("Multiple values: {:?}", key);
                // clean up multiple values
            }

            let mut flags = HashMap::new();
            flags.entry("usa").or_insert(false);
            flags.entry("eur").or_insert(false);
            flags.entry("jpn").or_insert(false);
            flags.entry("val").or_insert(false);

            println!("{:?}", flags);

            // loop through each value
            for file_name in value {
                set_flags(file_name, &mut flags);
            }

            for rom in value {
                let mut rom_flags = HashMap::new();
                rom_flags.entry("usa").or_insert(false);
                rom_flags.entry("eur").or_insert(false);
                rom_flags.entry("jpn").or_insert(false);
                rom_flags.entry("val").or_insert(false);

                set_flags(rom, &mut rom_flags);

                // rules are as follows...
                // if USA and VAL delete all other but the valid USA
                // elseif EUR and VAL delete all other but the valid EUR
                // elseif JPN and VAL delete all other but the valid JPN
                // if not VAL pick the first USA then EUR, THEN JPN

                let mut should_delete_rom = false;

                if (flags["usa"] && !rom_flags["usa"])
                    || (flags["usa"] && flags["val"] && !rom_flags["val"])
                {
                    should_delete_rom = true;
                } else if (flags["eur"] && rom_flags["jpn"])
                    || (flags["eur"] && flags["val"] && !rom_flags["val"])
                {
                    should_delete_rom = true;
                } else if (flags["jpn"] && !rom_flags["usa"])
                    || (flags["jpn"] && flags["val"] && !rom_flags["val"])
                {
                    should_delete_rom = true;
                }

                if should_delete_rom {
                    // println!("FLAGS {:?}", flags);
                    // println!("ROM FLAGS {:?}", rom_flags);
                    _ = delete_rom(rom);
                }
            }
        }
    }
}

fn delete_rom(rom: &str) -> std::io::Result<()> {
    println!("Deleting: {:?}", rom);
    fs::remove_file(rom)?;
    Ok(())
}

fn set_flags(file_name: &String, countries: &mut HashMap<&str, bool>) {
    if file_name.contains("(U)") || file_name.contains("[U]") || file_name.contains("(USA)") {
        countries.insert("usa", true);
        // println!("USA YAY")
    }

    if file_name.contains("(E)") || file_name.contains("[E]") || file_name.contains("(EUROPE)") {
        countries.insert("eur", true);
        // println!("EUR YAY")
    }

    if file_name.contains("(J)") || file_name.contains("[J]") || file_name.contains("(JAPAN)") {
        countries.insert("jpn", true);
        // println!("JPN YAY")
    }
    if file_name.contains("(!)") || file_name.contains("[!]") || file_name.contains("(VALID)") {
        countries.insert("val", true);
        // println!("VAL YAY")
    }

    return;
}
