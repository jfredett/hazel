use std::{fs::File, io::{Read, Write}, path::{PathBuf}};

use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng};
use rayon::prelude::*;
use tracing::{debug, error, info, instrument, trace};

use crate::wizard::arena::error::ArenaError;

use self::error::ArenaResult;

use super::*;

pub mod error;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct Arena {
    size: usize,
    generations: usize,
    path: Option<PathBuf>,
    population: Vec<Wizard>
}

// FIXME: Need to build up an ArenaError enum
impl Arena {
    #[instrument]
    pub fn empty() -> Arena {
        Arena { 
            size: 0,
            generations: 0,
            path: None,
            population: vec![]
        }
    }
    
    /// remove the weakest until arena returns to it's set size.
    pub fn cull(&mut self) {
        self.population.sort_by_key(|e| usize::MAX - e.fitness());
        while self.population.len() > self.size {
            // occasionally let more through
            if rand::random::<u8>() % 64 == 0 { break }
            self.population.remove(0);
        }
    }
    
    /// irradiate everyone in the Arena and mutate them
    pub fn mutate(&mut self) {
        self.population.par_iter_mut().for_each(|e| e.mutate());
    }
    
    fn new_wizard(&mut self) -> Wizard {
        trace!("Building new wizard");
        let weighted_population = WeightedIndex::new(
            self.population.iter().map(|e| e.fitness())
        ).unwrap();
        
        let parent1 = &self.population[weighted_population.sample(&mut thread_rng())];
        let parent2 = &self.population[weighted_population.sample(&mut thread_rng())];
        
        let child = parent1.combine(parent2);

        trace!("parents: ({}, {}) -> child {}", parent1.fitness(), parent2.fitness(), child.fitness());

        child
    }
    
    fn breed(&mut self) {
        loop {
            if rand::random::<u8>() % 32 == 0 { break }
            let new_wizard = self.new_wizard();
            
            self.population.push(new_wizard)
        }
    }

    /// Executes a combine/mutate step on the population of the arena
    #[instrument(skip(self))]
    pub fn step(&mut self) -> ArenaResult<()> {
        
        self.breed();
        self.mutate();
        self.cull();
        self.generations += 1;
        
        if self.generations % 10 == 0 {
            self.save()?;
        }

        self.report();
        
        Ok(())
    }
    
    pub fn save(&self) -> ArenaResult<()> {
        info!("Saving Arena to disk");
        let mut f = if let Ok(f) = File::create(&self.path.clone().unwrap()) { 
            debug!("Created new file for storage");
            f 
        } else {
            error!("Failed to create file at {:?}", &self.path);
            return Err(ArenaError::InvalidPath(self.path.clone().unwrap()));
        };
        let bytes = bincode::serialize(&self)?;
        f.write_all(&bytes)?;
        f.flush()?;
        Ok(())
    }

    fn report(&self) {
        info!("Generation: {}", self.generations);
        info!("Popcount: {}", self.population.len());
        info!("Best: {}", self.min_fitness());
        info!("Worst: {}", self.max_fitness());
        info!("Max Collisions: {}", self.max_collisions());
        info!("Min Collisions: {}", self.min_collisions());
    }
    
    #[instrument]
    pub fn new(size: usize, path: PathBuf) -> ArenaResult<Arena> {
        info!("Initializing new {}-wizard arena at: {:?}", size, path);
        let mut arena = Arena {
            size,
            generations: 0,
            population: vec![],
            path: Some(path),
        };
        
        info!("Adding wizards to the arena");
        for _ in 0..size {
            arena.population.push(Wizard::new())
        }

        
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
    
    pub fn min_fitness(&self) -> usize {
        self.population.par_iter().min_by_key(|&e| e.fitness()).unwrap().fitness()
    }

    pub fn max_fitness(&self) -> usize {
        self.population.par_iter().max_by_key(|&e| e.fitness()).unwrap().fitness()
    }

    pub fn min_collisions(&self) -> usize {
        self.population.par_iter().min_by_key(|&e| e.collisions).unwrap().collisions
    }

    pub fn max_collisions(&self) -> usize {
        self.population.par_iter().max_by_key(|&e| e.collisions).unwrap().collisions
    }
    
    pub fn size(&self) -> usize { self.size }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use tracing_test::traced_test;

    use super::*;
    
    #[test]
    fn cull_culls_wizards_of_least_fitness() {
        let mut arena = Arena::new(10, PathBuf::from("/tmp/hazel-arena")).unwrap();
        let fitnesses : Vec<usize> = arena.population.iter().map(|e| e.fitness()).collect();
        arena.size = 5;
        arena.cull();
        let after_fitnesses : Vec<usize> = arena.population.iter().map(|e| e.fitness()).collect();
        
        let diff = fitnesses.iter().filter(|&e| !after_fitnesses.contains(e));
        for f in diff {
            for a in &after_fitnesses {
                assert!(a < f);
            }
        }
    }

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