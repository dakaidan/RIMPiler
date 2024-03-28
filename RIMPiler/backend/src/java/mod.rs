use std::path::Path;

fn find_kratatau2() -> Option<String> {
    let mut path = std::env::current_exe().unwrap();
    // should be at ./krak2
    path.pop();
    path.push("krak2");
    if path.exists() {
        Some(path.to_str().unwrap().to_string())
    } else {
        println!("krak2 not found at {:?}", path);
        // check if its on system path
        let path = std::env::var("PATH").unwrap();
        for p in path.split(':') {
            if p == "krak2" {
                return Some("krak2".to_string());
            }
        }

        println!("krak2 not found on system path");
        None
    }
}

pub fn assemble_byte_code(byte_code: String, output_folder: String) {
    std::fs::create_dir_all(&output_folder).expect("Unable to create directory");
    let output_folder = Path::new(&output_folder);
    // write byte code to temp file in output folder
    let mut temp_file = output_folder.to_path_buf();
    temp_file.push("Main.j");
    std::fs::write(&temp_file, &byte_code).expect("Unable to write file");

    // get path to krak2
    let krak2 = find_kratatau2().expect("Unable to find krak2");
    // run krak2
    // krak2 asm --out <output_folder> <temp_file>
    let output_file = output_folder.join("Main.class");
    let output = std::process::Command::new(krak2)
        .arg("asm")
        .arg("--out")
        .arg(output_file)
        .arg(temp_file)
        .output()
        .expect("Unable to run krak2");

    if !output.status.success() {
        panic!("krak2 failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    // copy RIMPInt.class and RIMPFloat.class
    let mut output_file = output_folder.to_path_buf();
    output_file.push("RIMPInt.class");
    std::fs::copy("data/RIMPInt.class", output_file).expect("Unable to copy file");

    let mut output_file = output_folder.to_path_buf();
    output_file.push("RIMPFloat.class");
    std::fs::copy("data/RIMPFloat.class", output_file).expect("Unable to copy file");
}
//     std::fs::create_dir_all(&output_folder).expect("Unable to create directory");
//     let output_folder = Path::new(&output_folder);
//     let mut writer = Writer::new(output_folder).expect("Unable to create writer");
//
//     let res = assemble(&byte_code, AssemblerOptions {});
//     let classes = match res {
//         Ok(classes) => classes,
//         Err(err) => {
//             err.display("byte_code", &byte_code);
//             panic!("Error assembling byte code");
//         }
//     };
//     println!("got {} classes", classes.len());
//
//     let output_folder = Path::new(&output_folder);
//
//     for (name, out) in classes {
//         let name = name.map(|name| format!("{}.class", name));
//         writer.write(name.as_deref(), &out).expect("Unable to write file");
//     }
//
//     let mut output_file = output_folder.to_path_buf();
//     output_file.push("RIMPInt.class");
//     std::fs::copy("data/RIMPInt.class", output_file).expect("Unable to copy file");
//
//     let mut output_file = output_folder.to_path_buf();
//     output_file.push("RIMPFloat.class");
//     std::fs::copy("data/RIMPFloat.class", output_file).expect("Unable to copy file");
//
