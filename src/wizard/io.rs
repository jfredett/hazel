use serde::{de::{SeqAccess, Visitor}, ser::SerializeTuple};

use super::*;

impl Serialize for Wizard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let mut seq = serializer.serialize_tuple(Wizard::SERIALIZED_SIZE)?;

        for rook in self.rooks.iter() {
            seq.serialize_element(rook)?;
        }
        
        for bishop in self.bishops.iter() {
            seq.serialize_element(bishop)?;
        }
        
        // FIXME: Make this sparse -- serializing a list of pairs of the non-NONE entries.
        for attack in self.table.iter() {
            seq.serialize_element(attack)?;
        }
        
        seq.serialize_element(&self.collisions)?;

        seq.end()
    }
}

struct WizardVisitor {
    wizard: Wizard
}

impl WizardVisitor {
    pub fn new() -> WizardVisitor {
        WizardVisitor { wizard: Wizard::empty() }
    }
}

impl<'de> Visitor<'de> for WizardVisitor {
    type Value = Wizard;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "A string of {} bytes", Wizard::SERIALIZED_SIZE)
    }
    
    fn visit_seq<A>(mut self, mut seq: A) -> Result<Self::Value, A::Error>
    where A: SeqAccess<'de>, {
        for i in 0..BOARD_SIZE {
            self.wizard.rooks[i] = seq.next_element()?.unwrap();
        }

        for i in 0..BOARD_SIZE {
            self.wizard.bishops[i] = seq.next_element()?.unwrap();
        }

        for i in 0..TABLE_SIZE {
            self.wizard.table[i] = seq.next_element()?.unwrap();
        }
        
        self.wizard.collisions = seq.next_element()?.unwrap();
        
        Ok(self.wizard)
    }
    
}

impl<'de> Deserialize<'de> for Wizard {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        deserializer.deserialize_tuple(Wizard::SERIALIZED_SIZE, WizardVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn serializing_to_bincode_round_trips() {
        let expected = Wizard::new();
        let serialized = bincode::serialize(&expected).unwrap();
        
        let deserialized : Wizard = bincode::deserialize(&serialized).unwrap();

        assert_eq!(deserialized, expected);
    }

}