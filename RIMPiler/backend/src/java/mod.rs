use std::path::Path;
use krakatau2::lib::assemble;
use krakatau2::lib::assemble::AssemblerOptions;
use krakatau2::file_output_util::Writer;


pub fn assemble_byte_code(byte_code: String, output_folder: String) {
    std::fs::create_dir_all(&output_folder).expect("Unable to create directory");
    let output_folder = Path::new(&output_folder);
    let mut writer = Writer::new(output_folder).expect("Unable to create writer");

    let res = assemble(&byte_code, AssemblerOptions {});
    let classes = match res {
        Ok(classes) => classes,
        Err(err) => {
            err.display("byte_code", &byte_code);
            panic!("Error assembling byte code");
        }
    };
    println!("got {} classes", classes.len());

    let output_folder = Path::new(&output_folder);

    for (name, out) in classes {
        let name = name.map(|name| format!("{}.class", name));
        writer.write(name.as_deref(), &out).expect("Unable to write file");
    }

    let mut output_file = output_folder.to_path_buf();
    output_file.push("RIMPInt.class");
    std::fs::copy("data/RIMPInt.class", output_file).expect("Unable to copy file");

    let mut output_file = output_folder.to_path_buf();
    output_file.push("RIMPFloat.class");
    std::fs::copy("data/RIMPFloat.class", output_file).expect("Unable to copy file");
}