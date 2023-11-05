use regex::Regex;
use std::{
    fmt::format,
    fs,
    io::{self, Read, Write},
    ops::RangeBounds,
    path::Path,
};

fn main() -> io::Result<()> {
    let regex = Regex::new(r"(.*)\.(png|jpg)$").unwrap();
    let regex_gltf = Regex::new(r"(.*)\.(gltf)").unwrap();
    let regex_uri_p = Regex::new(r####""uri":"(.*)\.(jpg|png)""####).unwrap();
    let folder = std::env::args().nth(1).expect("No folder given");
    for a in fs::read_dir(folder.clone()).expect("Folder is not found.") {
        let n = a.as_ref().unwrap().file_name();
        let bbbb = a.unwrap();
        let cccc = bbbb.path();
        let path = cccc.to_str().unwrap();
        if regex_gltf.find(n.to_str().unwrap()).is_some() {
            let mut s = fs::read_to_string(path).expect("Couldn't read the file");
            let mut start_index = 0;
            'b: loop {
                let (name, ftype, range) = {
                    let Some(capture) = regex_uri_p.captures(&s[start_index..]) else {
                        break 'b;
                    };
                    // println!("{}", start_index);
                    let a = capture.get(1).unwrap();
                    (
                        a.as_str(),
                        capture.get(2).unwrap().as_str(),
                        capture.get(0).unwrap().range(),
                    )
                };
                let path = format!("{}\\{}.png", folder, name);
                // println!("{}", path);

                if Path::new(&path).exists() {
                    
                    println!("{}", name.clone());
                    // fs::remove_file(format!("{}\\{}.{}", folder, name, ftype));
                    let outp = std::process::Command::new("magick").args([
                        "convert", Path::new(&path).as_os_str().to_str().unwrap(), "-depth", "8", Path::new(&path).as_os_str().to_str().unwrap()
                    ]).spawn().unwrap();
                    let outp = outp.wait_with_output().unwrap();
                    
                    println!("{:?}", outp.status);
                    println!("{:?}", outp.stdout);
                    println!("{:?}", outp.stderr);
                }
                start_index += range.clone().max().unwrap();
            }
        } else {
            // let file_name = regex.captures(n.to_str().unwrap());
            // // println!
            // //     "{} - {:?}",
            // //     n.to_str().unwrap(),
            // //     file_name.iter().collect::<Vec<_>>()
            // // );
            // let file_name = file_name.and_then(|p| p.get(1));
            //
            // if file_name.is_none() {
            //     continue;
            // }
            // let file_name = file_name.unwrap();
            //
            // let file_name = file_name.as_str();
            //
            // let file_name = format!("{}\\{}.ktx2", folder.clone(), file_name);
            // // println!("{}", fs::read(a.unwrap().path()).unwrap().len());
            // if Path::new(&file_name).exists() {
            //     println!("File {} exists. Skipping...", file_name);
            //     continue;
            // }
            // let res = std::process::Command::new("cmd")
            //     .args(&[
            //         "/C",
            //         "toktx",
            //         "--t2",
            //         "--zcmp",
            //         "17",
            //         file_name.as_str(),
            //         path,
            //     ])
            //     .output()
            //     .expect("failed to call toktx");
            // println!("{}", res.status);
            // io::stdout().write_all(&res.stdout);
            // io::stderr().write_all(&res.stderr);
        }
    }
    Ok(())
}
