use std::{fs, path::PathBuf};

use rpackwiz::model::{IndexFile, MetaFile, PackFile};

fn main() {
    let base_dir = PathBuf::from("/tmp/pack/");
    let pack: PackFile =
        toml::from_str(&fs::read_to_string(base_dir.join("pack.toml")).unwrap()).unwrap();
    println!("{:#?}", pack);
    let index_path = base_dir.join(pack.index.file);
    let index: IndexFile = toml::from_str(&fs::read_to_string(index_path).unwrap()).unwrap();
    println!("{:#?}", index);
    for file in index.files {
        if file.metafile {
            let file_path = base_dir.join(file.file);
            let file: MetaFile = toml::from_str(&fs::read_to_string(file_path).unwrap()).unwrap();
            println!("{:?}", file.download.mode);
        }
    }
}
