use std::{env, path::PathBuf, fs, io::BufReader};

use bevy::prelude::{ResMut};


use crate::lib::tools::resources::AllSettings;

pub fn load_settings(mut res: ResMut<AllSettings>) {
    let path = if let Ok(manifest_dir) = env::var("BEVY_ASSET_ROOT") {
        PathBuf::from(manifest_dir)
    } else if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(manifest_dir)
    } else {
        env::current_exe()
            .map(|path| {
                path.parent()
                    .map(|exe_parent_path| exe_parent_path.to_owned())
                    .unwrap()
            })
            .unwrap()
    };

    let path = path.join("assets/data");

    if !path.exists() {
        fs::create_dir_all(path.clone()).expect("assets/data does not exist and we can't make it for default save data");
        let path = path.join("settings.json");
        let res = fs::write(path, serde_json::to_string(res.as_ref()).expect("couldn't serialize settings into settings.json"));
        match res {
            Ok(_) => {},
            Err(err) => println!("{}",err)
        }
    } else {
        let path = path.join("settings.json");
        if !path.exists() {
            let res = fs::write(path, serde_json::to_string(res.as_ref()).expect("couldn't serialize settings into settings.json"));
            match res {
                Ok(_) => {},
                Err(err) => println!("{}",err)
            }
        } else {
            let file = fs::File::open(path).unwrap();
            let read = BufReader::new(file);
            let de: AllSettings = serde_json::from_reader(read).unwrap();
            *res = de;
        }
    }
}