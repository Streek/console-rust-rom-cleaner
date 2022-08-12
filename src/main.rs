// import path system
use std::collections::HashMap;
use std::fs;

fn main() {
    // hashmap of strings and arrays of strings
    let mut counts: HashMap<String, Vec<String>> = HashMap::new();
    // read files in directory

    if let Ok(entries) = fs::read_dir("./") {
        for entry in entries {
            if let Ok(entry) = entry {
                // get string file_name from entry
                let file_name: String = entry.file_name().into_string().unwrap();

                // print!("{:?} ", file_name);

                let split = &file_name
                    .split(&['(', ')', '[', ']'][..])
                    .collect::<Vec<&str>>();

                // print!("{:?} ", split);

                // // add split(key) and file_name(value) to hashmap
                if split.len() > 1 {
                    counts
                        .entry(split[0].to_string())
                        .or_insert(vec![])
                        .push(String::from(&file_name));
                }

                println!("{:?}", counts);
                continue;
            }
        }
        // loop through hashmap and print out key and value
        for (key, value) in counts.iter() {
            let mut USA = false;
            let mut EUR = false;
            let mut JAP = false;
            let mut VALID = false;

            // if value is empty, print key
            // if value has a length of 1 continue
            // if vlaue has a length > 1 print key and value
            if value.len() == 0 {
                println!("{:?}", key);
            } else if value.len() == 1 {
                continue;
            } else {
                // loop through each value
                for file_name in value {
                    // if (U), [U] or (USA) exist in file_name mark USA as true
                    if file_name.contains("(U)")
                        || file_name.contains("[U]")
                        || file_name.contains("(USA)")
                    {
                        USA = true;
                        print!("USA YAY")
                    }
                    if file_name.contains("(E)")
                        || file_name.contains("[E]")
                        || file_name.contains("(Europe)")
                    {
                        EUR = true;
                        print!("EUR YAY")
                    }
                    if file_name.contains("(J)")
                        || file_name.contains("[J]")
                        || file_name.contains("(JAPAN)")
                    {
                        JAP = true;
                        print!("JAPAN YAY")
                    }
                }
                for rom in value {
                    if USA {
                        if rom.contains("(U)") || rom.contains("[U]") || rom.contains("(USA)") {
                            if (VALID && rom.contains("[!]")) || !VALID {
                                continue;
                            }
                        }
                    } else if EUR {
                        if rom.contains("(E)") || rom.contains("[E]") || rom.contains("(Europe)") {
                            if (VALID && rom.contains("[!]")) || !VALID {
                                continue;
                            }
                        }
                    } else if JAP {
                        if VALID && rom.contains("[!]") {
                            continue;
                        }
                    }
                    // delete rom
                }
            }

            println!("{:?}", key);
            println!("{:?}", value);
        }
    }
}
