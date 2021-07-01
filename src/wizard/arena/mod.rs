use std::{fs::File, io::{Read, Write}, path::{PathBuf}};

use tracing::{error, debug, info, instrument};

use crate::wizard::arena::error::ArenaError;

use self::error::ArenaResult;

use super::*;

pub mod error;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct Arena {
    size: usize,
    path: Option<PathBuf>,
    population: Vec<Wizard>
}

// FIXME: Need to build up an ArenaError enum
impl Arena {
    #[instrument]
    pub fn empty() -> Arena {
        Arena { 
            size: 0,
            path: None,
            population: vec![]
        }
    }
    
    #[instrument]
    pub fn new(size: usize, path: PathBuf) -> ArenaResult<Arena> {
        info!("Initializing new {}-wizard arena at: {:?}", size, path);
        let mut arena = Arena {
            size,
            population: vec![],
            path: Some(path.clone()),
        };
        
        info!("Adding wizards to the arena");
        for _ in 0..size {
            arena.population.push(Wizard::new())
        }

        info!("Saving Arena to disk");
        let mut f = if let Ok(f) = File::create(&path) { 
            debug!("Created new file for storage");
            f 
        } else {
            error!("Failed to create file at {:?}", &path);
            return Err(ArenaError::InvalidPath(path));
        };
        let bytes = bincode::serialize(&arena)?;
        f.write_all(&bytes)?;
        f.flush()?;
        
        info!("Arena creation complete");
        Ok(arena)
    }
    
    #[instrument]
    pub fn load(path: PathBuf) -> ArenaResult<Arena> {
        info!("Loading Wizard arena from {:?}", path);
        let mut f= File::open(path)?;
        let mut bytes: Vec<u8> = vec![];
        f.read_to_end(&mut bytes)?;

        info!("Deserializing from disk");
        let des = bincode::deserialize(&bytes)?;

        info!("Arena Loading Complete");
        Ok(des)
    }
    
    pub fn size(&self) -> usize { self.size }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn empty_arena_is_empty() {
        let arena = Arena::empty();
        assert!(arena.path.is_none());
        assert_eq!(arena.size, 0);
        assert!(arena.population.is_empty());
    }

    #[test]
    fn can_round_trip_to_disk() -> ArenaResult<()> {
        let path = PathBuf::from("/tmp/hazel-arena");
        let arena = Arena::new(100, path.clone())?;
        let loaded_arena = Arena::load(path)?;
        assert_eq!(arena, loaded_arena);
        Ok(())
    }
    
}