use super::*;
use tracing::*;

impl Wizard {
    /// The position of the last element in the table -- lower is better.
    pub fn space_occupied(&self) -> usize {
        debug!("Checking space occupied");
        for i in 1..=TABLE_SIZE {
            if self.table[TABLE_SIZE - i].is_some() { 
                return TABLE_SIZE - i; 
            }
        }
        debug!("Could not find any set entries");
        // If it's all empty, then it's not really a wizard, so just return a sentinel value
        TABLE_SIZE + 1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_occupied_is_not_table_size() {
        let mut wiz = Wizard::new();
        wiz.initialize();
        assert!(wiz.space_occupied() < TABLE_SIZE)
    }
}