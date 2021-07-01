use std::{fs::File, io::{Read, Write}, path::{self, PathBuf}};

use tracing::{error, debug, info, instrument};

use super::*;

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
    pub fn new(size: usize, path: PathBuf) -> Arena {
        info!("Initializing new {}-wizard arena at: {:?}", size, path.clone());
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
        let mut f = if let Ok(f) = File::create(path.clone()) { 
            debug!("Created new file for storage");
            f 
        } else {
            error!("Failed to create file at {:?}", path.clone());
            panic!("Failed to access file");
        };
        let bytes = bincode::serialize(&arena).unwrap();
        f.write_all(&bytes).unwrap();
        f.flush().unwrap();
        
        info!("Arena creation complete");
        arena
    }
    
    #[instrument]
    pub fn load(path: PathBuf) -> Arena {
        info!("Loading Wizard arena from {:?}", path.clone());
        let mut f= File::open(path.clone()).unwrap();
        let mut bytes: Vec<u8> = vec![];
        f.read_to_end(&mut bytes).unwrap();
        info!("Deserializing from disk");
        let des = bincode::deserialize(&bytes);
        info!("Arena Loading Complete");
        des.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use tracing_test::traced_test;

    use super::*;

    #[test]
    fn empty_arena_is_empty() {
        let arena = Arena::empty();
        assert!(arena.path.is_none());
        assert_eq!(arena.size, 0);
        assert!(arena.population.is_empty());
    }

    #[test]
    #[traced_test]
    fn can_round_trip_to_disk() {
        let path = PathBuf::from("/tmp/hazel-arena");
        let arena = Arena::new(100, path.clone());
        let loaded_arena = Arena::load(path.clone());
        assert_eq!(arena, loaded_arena);
    }
    
}