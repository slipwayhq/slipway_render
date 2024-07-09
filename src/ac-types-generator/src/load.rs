use std::{fs::File, hash::Hasher, io::BufReader, path::Path};

use walkdir::WalkDir;

use crate::sanitize::sanitize_type_ident;

pub(super) struct Loaded<T> {
    pub value: T,
    pub file_name: String,
    pub type_name: String,
    pub id: String,
}

impl<T> PartialEq for Loaded<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Loaded<T> {}

impl<T> std::hash::Hash for Loaded<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub(super) fn load_json_files<P: AsRef<Path>>(
    path: P,
) -> impl Iterator<Item = Loaded<serde_json::Value>> {
    WalkDir::new(path).into_iter().filter_map(|entry| {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_file() && path.extension()?.to_str()? == "json" {
            let file = File::open(path).ok()?;
            let reader = BufReader::new(file);
            let file_name = path.file_name()?.to_str()?.to_string();
            let file_name_without_extension = path.file_stem()?.to_str()?.to_string();
            Some(Loaded::<serde_json::Value> {
                value: serde_json::from_reader(reader).ok()?,
                file_name,
                type_name: sanitize_type_ident(&file_name_without_extension),
                id: file_name_without_extension,
            })
        } else {
            None
        }
    })
}
