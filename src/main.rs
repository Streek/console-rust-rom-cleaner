// import path system
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{self};

static ACTUALLY_DELETE: bool = false;
static DELETE_BETAS: bool = true;
static DELETE_HACKS: bool = true;
static KEEP_ALL_VALIDS: bool = false;
static INPUT: &str = "/mnt/d/DeckEmulationSync/roms/";
static RECURSE: bool = true;
static PRIORITY_VECTOR: &[&str] = &["usa", "glo", "eur", "jpn", "brz"];

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
        for (_key, rom_collection) in counts.iter() {
            if rom_collection.len() == 0 {
                // println!("No values: {:?}", key);
                continue;
            }
            if rom_collection.len() == 1 {
                // println!("One value: {:?}", key);
                continue;
            }

            let mut flags = HashMap::new();
            let mut val_flags = HashMap::new();

            flags.entry("usa").or_insert(false);
            flags.entry("eur").or_insert(false);
            flags.entry("jpn").or_insert(false);
            flags.entry("brz").or_insert(false);
            flags.entry("glo").or_insert(false);
            flags.entry("val").or_insert(false);
            flags.entry("beta").or_insert(false);
            flags.entry("hack").or_insert(false);
            flags.entry("").or_insert(false);

            // loop through each value
            for rom in rom_collection {
                val_flags = set_flags(rom, &mut flags);
            }

            for rom in rom_collection {
                let mut rom_flags = HashMap::new();
                rom_flags.entry("usa").or_insert(false);
                rom_flags.entry("eur").or_insert(false);
                rom_flags.entry("jpn").or_insert(false);
                rom_flags.entry("val").or_insert(false);
                rom_flags.entry("brz").or_insert(false);
                rom_flags.entry("glo").or_insert(false);
                rom_flags.entry("beta").or_insert(false);
                rom_flags.entry("hack").or_insert(false);

                _ = set_flags(rom, &mut rom_flags);

                if rom_flags["beta"] && DELETE_BETAS {
                    delete_rom(rom, folder);
                    continue;
                } else if rom_flags["hack"] && DELETE_HACKS {
                    delete_rom(rom, folder);
                    continue;
                }

                for item in PRIORITY_VECTOR {
                    if KEEP_ALL_VALIDS == true && rom_flags["val"] {
                        break;
                    }

                    if !flags[item] {
                        continue;
                    }

                    // if we have this type and the rom is NOT the same type AND our previous flag EXISTS && HAS BEEN VISITED.
                    if flags[item] && !rom_flags[item] {
                        delete_rom(rom, folder);
                        break;
                    } else {
                        if val_flags[&item.to_string()] && !rom_flags["val"] {
                            delete_rom(rom, folder);
                            break;
                        }
                    }
                }
            }
        }
    }
}
fn delete_rom(rom: &str, folder: &str) {
    let path = "".to_owned() + folder + "/" + rom;
    println!("DELETING: {}", path);
    if ACTUALLY_DELETE {
        fs::remove_file(path).unwrap();
    }
    return;
}

fn set_flags(file_name: &String, countries: &mut HashMap<&str, bool>) -> HashMap<String, bool> {
    lazy_static! {
        static ref RE_USA: Regex = Regex::new(r"(?i)[\(\[\{](u|us|usa)[\)\]\}]").unwrap();
        static ref RE_EUR: Regex = Regex::new(r"(?i)[\(\[\{][europe]*[\)\]\}]").unwrap();
        static ref RE_JPN: Regex = Regex::new(r"(?i)[\(\[\{][japan]*[\)\]\}]").unwrap();
        static ref RE_BRZ: Regex = Regex::new(r"(?i)[\(\[\{][brazil]*[\)\]\}]").unwrap();
        static ref RE_GLO: Regex = Regex::new(r"(?i)[\(\[\{](g|gl|glo|global)[\)\]\}]").unwrap();
        static ref RE_VAL: Regex = Regex::new(r"(?i)[\(\[\{](!|v|valid)[\)\]\}]").unwrap();
        static ref RE_BETA: Regex =
            Regex::new(r"(?i)[\(\[\{](a\d*|a|al|alt.*|b|be|bet|proto|prototype|beta|b\d*)[\)\]\}]")
                .unwrap();
        static ref RE_HACK: Regex =
            Regex::new(r"(?i)[\(\[\{](h|ha|hak|hac|hack|h\d*)[\)\]\}]").unwrap();
    }

    let mut val_flags: HashMap<String, bool> = HashMap::new();
    val_flags.entry("usa".to_string()).or_insert(false);
    val_flags.entry("eur".to_string()).or_insert(false);
    val_flags.entry("jpn".to_string()).or_insert(false);
    val_flags.entry("val".to_string()).or_insert(false);
    val_flags.entry("brz".to_string()).or_insert(false);
    val_flags.entry("glo".to_string()).or_insert(false);
    val_flags.entry("beta".to_string()).or_insert(false);
    val_flags.entry("hack".to_string()).or_insert(false);

    if RE_VAL.is_match(file_name) {
        countries.insert("val", true);
    }

    if RE_USA.is_match(file_name) {
        countries.insert("usa", true);
        if countries["val"] {
            val_flags.insert("usa".to_string(), true);
        }
    }
    if RE_EUR.is_match(file_name) {
        countries.insert("eur", true);
        if countries["val"] {
            val_flags.insert("eur".to_string(), true);
        }
    }
    if RE_JPN.is_match(file_name) {
        countries.insert("jpn", true);
        if countries["val"] {
            val_flags.insert("jpn".to_string(), true);
        }
    }
    if RE_BRZ.is_match(file_name) {
        countries.insert("brz", true);
        if countries["val"] {
            val_flags.insert("brz".to_string(), true);
        }
    }
    if RE_GLO.is_match(file_name) {
        countries.insert("glo", true);
        if countries["val"] {
            val_flags.insert("glo".to_string(), true);
        }
    }
    if RE_BETA.is_match(file_name) {
        countries.insert("beta", true);
        if countries["val"] {
            val_flags.insert("beta".to_string(), true);
        }
    }
    if RE_HACK.is_match(file_name) {
        countries.insert("hack", true);
        if countries["val"] {
            val_flags.insert("hack".to_string(), true);
        }
    }

    return val_flags;
}
