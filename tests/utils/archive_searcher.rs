use std::{collections::HashMap, path::PathBuf};

use boflink::libsearch::{FoundLibrary, LibraryFind, LibsearchError};

pub struct MemoryArchiveSearcher {
    files: HashMap<String, Vec<u8>>,
}

impl MemoryArchiveSearcher {
    pub fn new() -> MemoryArchiveSearcher {
        Self {
            files: HashMap::new(),
        }
    }
}

impl LibraryFind for MemoryArchiveSearcher {
    fn find_library(&self, name: impl AsRef<str>) -> Result<FoundLibrary, LibsearchError> {
        self.files
            .get(name.as_ref())
            .map(|data| FoundLibrary::new(PathBuf::from(name.as_ref()), data.clone()))
            .ok_or(boflink::libsearch::LibsearchError::NotFound(
                name.as_ref().to_string(),
            ))
    }
}
