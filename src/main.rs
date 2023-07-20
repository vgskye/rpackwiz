use std::{fs, path::PathBuf};

use rpackwiz::model::{PackIndex, Mod, Pack};

fn main() {
    let base_dir = PathBuf::from(r"C:\Users\dennis\source\Repos\TheAlan404\lctr-modpack");
    let pack: Pack = Pack::load_from(&base_dir.join("pack.toml")).unwrap();
    println!("{:#?}", pack);
    let index: PackIndex = pack.get_index().unwrap();
    println!("{:#?}", index);
    for file in index.files {
        if file.metafile {
            let file_path = base_dir.join(file.file);
            let file: Mod = toml::from_str(&fs::read_to_string(file_path).unwrap()).unwrap();
            println!("{:#?}", file);
        }
    }
}
