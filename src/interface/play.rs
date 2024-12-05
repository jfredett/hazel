pub trait Play where Self: Clone {
    type Rule: Clone;
    type Metadata: Clone;

    fn apply(&self, rule: &Self::Rule) -> Self;

    fn metadata(&self) -> Self::Metadata;

    fn apply_mut(&mut self, rule: &Self::Rule) -> &mut Self {
        *self = self.apply(rule);
        self
    }
}

pub trait Unplay where Self: Clone + Play {
    fn unapply(&self, rule: &Self::Rule) -> Self;

    fn unapply_mut(&mut self, rule: &Self::Rule) -> &mut Self {
        *self = self.unapply(rule);
        self
    }
}


// Example of Play for Nim
//
//
// enum NimAction {
//    Take(heap: usize, amount: usize)
// }
//
// struct Nim {
//    heaps: Vec<usize>
//    side_to_move: bool
// }
//
// impl Play for Nim {
//   type Rule = NimAction;
//   type Metadata = Self;
//
//   fn metadata(&self) -> Nim {
//      self.clone()
//   }
//
//   fn apply(&self, rule: NimAction) -> Nim {
//      let mut new_heaps = self.heaps.clone();
//      match rule {
//        NimAction::Take(heap, amount) => {
//          new_heaps[heap] -= amount;
//        }
//        _ => { /* nim is trivial in this context, which is sort of the point, only one action to take */}
//      }
//      Nim { heaps: new_heaps, side_to_move: !self.side_to_move }
//   }
//
//   fn unwind(&self, rule: NimAction) -> Nim {
//      let mut new_heaps = self.heaps.clone();
//      match rule {
//        NimAction::Take(heap, amount) => {
//          new_heaps[heap] += amount;
//        }
//        _ => { /* nim is trivial in this context, which is sort of the point, only one action to unwind */}
//      }
//      Nim { heaps: new_heaps, side_to_move: !self.side_to_move }
//   }
// }
//
//
// This *should* result in a system which can progressively apply/unapply actions, and can then be
// paired with a Familiar like this:
//
// let l : Log<NimAction> = Log::new();
// // some code to add a bunch of nim moves here
// let f = Familiar::new(l.raw_cursor());
//
// f.metadata().heaps // should be the initial state of the game
// f.seek(/* some position */);
// f.metadata().heaps // should be the state of the game at that position
//
//
// This should then allow the Evaluator and MoveGen to be relatively abstract, e.g., a minimax can
// be ignorant of the contents of the tree it is minimaxing, which means I could ostensibly have
// hazel support other games / fairy pieces, etc.
