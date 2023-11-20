use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Mutex, Arc};
use std::{collections::HashMap, io::Write};
use sysinfo::*;
use rayon::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Cache {
    cache: HashMap<String, Vec<String>>,
    list_of_remembered_paths: HashMap<String, u8>,
    changed: bool,
}

impl Cache {
    fn new() -> Self {
        Cache {
            cache: HashMap::new(),
            list_of_remembered_paths: HashMap::new(),
            changed: false,
        }
    }
}

fn main() {
    let username = std::env::var("USERNAME").unwrap();

    let mut cache = Cache::new();

    match Path::new(format!("C:\\Users\\{}\\tfe\\cache.json", username).as_str()).exists() {
        true => {
            load_cache();
        }
        false => {
            start_cache(&mut cache);
            main_prog(&mut cache);
        }
    }
}

fn start_cache(cache: &mut Cache) {
    let sys = System::new_all();

    println!("[+] Caching! This may take some time!");

    for disk in sys.disks() {
        let path = disk.mount_point();

        cache_all(Arc::new(Mutex::new(cache)), path);
    }

    let username = std::env::var("USERNAME").unwrap();

    let serialized = serde_json::to_string(cache).unwrap();

    let _ = std::fs::create_dir(format!("C:\\Users\\{}\\tfe", username).as_str());

    let mut cache_file =
        std::fs::File::create(format!("C:\\Users\\{}\\tfe\\cache.json", username)).unwrap();

    cache_file.write_all(serialized.as_bytes()).unwrap();

    println!("[+] Finished Caching!");
}

fn cache_all(cache: Arc<Mutex<&mut Cache>>, path: &Path) {
    let path_dir = match path.read_dir() {
        Ok(dir) => dir,
        Err(_) => return,
    };

    path_dir.par_bridge().for_each(|i| {
        match i {
            Ok(i) => {
                let file_name = i.file_name();
                let file_type = match i.file_type() {
                    Ok(file) => file,
                    Err(_) => return,
                };
                println!("{}", file_name.to_str().unwrap());

                let file_path = i.path();

                if file_path == Path::new("C:\\$Recycle.Bin\\") {
                    return;
                }

                match cache.lock().unwrap()
                    .list_of_remembered_paths
                    .get(file_path.to_str().unwrap())
                {
                    Some(_) => {
                        if file_type.is_file() {
                            return;
                        }
                    }
                    None => {
                        cache.lock().unwrap()
                            .list_of_remembered_paths
                            .insert(file_path.to_str().unwrap().to_owned(), 0);
                    }
                }

                match cache.lock().unwrap().cache.get(file_name.to_str().unwrap()) {
                    Some(content) => {
                        let mut new = content.clone();

                        new.push(file_path.to_str().unwrap().to_owned());

                        cache.lock().unwrap()
                            .cache
                            .insert(file_name.to_str().unwrap().to_owned(), new);
                    }
                    None => {
                        cache.lock().unwrap().cache.insert(
                            file_name.to_str().unwrap().to_owned(),
                            vec![file_path.to_str().unwrap().to_owned()],
                        );
                    }
                }

                if file_type.is_file() {
                    return;
                }

                cache_all(cache.clone(), &file_path);
            }
            Err(_) => {
                return;
            }
        }
    });
}

fn load_cache() {
    use std::io::Read;

    let username = std::env::var("USERNAME").unwrap();

    let mut cache_file =
        std::fs::File::open(format!("C:\\Users\\{}\\tfe\\cache.json", username)).unwrap();

    let mut serialized = String::new();

    cache_file.read_to_string(&mut serialized).unwrap();

    let mut cache: Cache = serde_json::from_str(serialized.as_str()).unwrap();

    main_prog(&mut cache);
}

#[allow(unused_assignments)]
fn main_prog(cache: &mut Cache) {
    let mut cwd = String::from("none");

    let sys = System::new_all();

    let mut disks: Vec<&Disk> = Vec::new();

    let mut input = String::new();

    let mut tokens = Vec::new();

    println!("Please select the disk: ");

    for (count, disk) in sys.disks().iter().enumerate() {
        println!("Disk {}: {}", count, disk.mount_point().to_str().unwrap());
        disks.push(disk);
    }

    loop {
        println!("{} >>", cwd);

        input.clear();

        std::io::stdin().read_line(&mut input).unwrap();

        tokens = input.trim().split(' ').collect::<Vec<&str>>();

        match tokens[0] {
            "h" => {
                println!(
                    "Commands:\n
                [*] rc - Recache - reads the whole disk again and caches it\n
                [*] cd - Change directory\n
                [*] ts - Searches through the disk (no cache)\n
                [*] cs - Searches through the cache\n
                [*] ls - Lists current directory\n
                [*] of - Open file\n
                [*] op - Open path\n
                [*] nd - New directory\n
                [*] nf - New file\n
                [*] rd - Remove directory\n
                [*] rf - Remove file\n
                [*] q - Quit"
                );
            }
            "rc" => {
                println!("[+] Caching! This may take some time!");

                for disk in sys.disks() {
                    let path = disk.mount_point();

                    cache_all(Arc::new(Mutex::new(cache)), path);
                }

                // save_cache(cache);

                // cache.changed = false;

                println!("[+] Finished Caching!");
            }
            "cd" => {
                let mut search = String::new();

                let mut tmp = String::new();

                for i in 1..tokens.len() {
                    search.push_str(tokens[i]);
                    search.push_str(" ");
                }

                search = search.trim().to_owned();

                if search == ".." {
                    let tmp = cwd.split('\\').collect::<Vec<&str>>();

                    cwd = tmp[0..tmp.len() - 2].join("\\");

                    cwd.push('\\');

                    continue;
                }

                if search.chars().last().unwrap() != '\\' {
                    search.push('\\');
                }

                tmp = cwd.clone().to_string();

                tmp.push_str(&search);

                if !Path::new(tmp.as_str()).exists() {
                    tmp = search;

                    if !Path::new(tmp.as_str()).exists() {
                        println!("Path does not exist!");

                        continue;
                    }

                    cwd = tmp.clone();

                    continue;
                }

                cwd = tmp.clone();

                let path = Path::new(cwd.as_str());

                let read_dir = match path.read_dir() {
                    Ok(read_dir) => read_dir,
                    Err(err) => {
                        println!("Cannot read directory!");
                        println!("{}", err.to_string());
                        continue;
                    }
                };

                for item in read_dir {
                    match item {
                        Ok(item) => {
                            let file_name = item.file_name();
                            let file_path = item.path();

                            match cache
                                .list_of_remembered_paths
                                .get(file_path.to_str().unwrap())
                            {
                                Some(_) => (),
                                None => {
                                    let mut new = match cache.cache.get(file_name.to_str().unwrap())
                                    {
                                        Some(vector) => vector.clone(),
                                        None => Vec::new(),
                                    };

                                    new.push(file_path.to_str().unwrap().to_owned());

                                    cache
                                        .cache
                                        .insert(file_name.to_str().unwrap().to_owned(), new);

                                    cache.changed = true;
                                }
                            }
                        }
                        Err(err) => {
                            println!("{}", err.to_string());
                        }
                    }
                }
            }
            "ts" => {}
            "cs" => {
                let mut search = String::new();

                for i in 1..tokens.len() {
                    search.push_str(tokens[i]);
                    search.push_str(" ");
                }

                search = search.trim().to_owned();

                if let None = tokens.get(1) {
                    println!("Please specify a file to look for!");
                    continue;
                }

                let files = match cache.cache.get(&search) {
                    Some(files) => files,
                    None => {
                        println!("Nothing found! Consider running rc (to recache) or ts (for a true search)");
                        continue;
                    }
                };

                for i in files {
                    let path = Path::new(i);

                    if path.exists() {
                        println!("{}", i);
                    } else {
                        let mut new = files.clone();

                        // TODO
                    }
                }
            }
            "ls" => {
                let path = Path::new(cwd.as_str());

                let read_dir = match path.read_dir() {
                    Ok(read_dir) => read_dir,
                    Err(err) => {
                        println!("Cannot read directory!");
                        println!("{}", err.to_string());
                        continue;
                    }
                };

                for item in read_dir {
                    match item {
                        Ok(item) => {
                            let file_name = item.file_name();

                            println!("{}", file_name.to_str().unwrap());
                        }
                        Err(err) => {
                            println!("{}", err.to_string());
                        }
                    }
                }
            }
            "of" => {}
            "nd" => {}
            "nf" => {}
            "rd" => {}
            "rf" => {}
            "q" => {
                break;
            }
            _ => {
                println!("Command not found!");
            }
        }
    }

    if cache.changed {
        save_cache(cache);
    }
}

fn save_cache(cache: &mut Cache) {
    let username = std::env::var("USERNAME").unwrap();

    let serialized = serde_json::to_string(cache).unwrap();

    let mut cache_file =
        std::fs::File::create(format!("C:\\Users\\{}\\tfe\\cache.json", username)).unwrap();

    cache_file.write_all(serialized.as_bytes()).unwrap();
}
