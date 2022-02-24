pub struct GameDir {
    full_path: String,
}

impl GameDir {
    pub fn new(game_id: &str) -> Option<Self> {
        std::fs::create_dir(format!("/tmp/{}", game_id)).ok()?;
        Some(GameDir {
            full_path: format!("/tmp/{}", game_id),
        })
    }
    pub fn get_path(&self) -> &str {
        &self.full_path
    }
}
impl Drop for GameDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir(self.get_path());
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::GameDir;

    #[test]
    fn dir_creation_and_deletion_check() {
        let game_id = "030af985-f4b5-4914-94d8-e559576449e3";
        let match_dir_handle = GameDir::new(&game_id).unwrap();

        let full_path = match_dir_handle.get_path().to_owned();

        assert!(Path::new(&full_path).exists());

        drop(match_dir_handle);

        assert!(!Path::new(&full_path).exists());
    }
}
