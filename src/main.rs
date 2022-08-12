// import path system
use std::collections::HashMap;
use std::fs::{self};

static ACTUALLY_DELETE: bool = true;
static DELETE_BETAS: bool = true;
static INPUT: &str = "/mnt/d/DeckEmulationSync/roms/";
static RECURSE: bool = true;

fn main() {
    println!("Rust Rom Cleaner v0.1");
    run(INPUT);
}

fn run(folder: &str) {
    println!("Scanning {}", folder);
    let mut counts: HashMap<String, Vec<String>> = HashMap::new();

    if let Ok(entries) = fs::read_dir(folder) {
        for entry in entries {
            // check if entry is a directory
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() && RECURSE {
                    if metadata.is_dir() {
                        run(entry.path().to_str().unwrap());
                        continue;
                    }
                }

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
            }
            // we are a file... continue
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

            // println!("{:?}", flags);

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
                rom_flags.entry("beta").or_insert(false);

                set_flags(rom, &mut rom_flags);

                // rules are as follows...
                // if USA and VAL delete all other but the valid USA
                // elseif EUR and VAL delete all other but the valid EUR
                // elseif JPN and VAL delete all other but the valid JPN
                // if not VAL pick the first USA then EUR, THEN JPN

                let mut should_delete_rom = false;

                if rom_flags["beta"] && DELETE_BETAS {
                    should_delete_rom = true;
                } else if (flags["usa"] && !rom_flags["usa"])
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
                    _ = delete_rom(rom, folder);
                }
            }
        }
    }
}

fn delete_rom(rom: &str, folder: &str) -> std::io::Result<()> {
    // delete file at rom path
    let path = "".to_owned() + folder + "/" + rom;
    println!("DELETING: {}", path);
    if ACTUALLY_DELETE {
        return fs::remove_file(path);
    } else {
        return Ok(());
    }
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

    if file_name.contains("(Beta)") || file_name.contains("[Beta]") || file_name.contains("(Beta)")
    {
        countries.insert("beta", true);
        // println!("VAL YAY")
    }

    return;
}
