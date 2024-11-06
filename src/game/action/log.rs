use std::ops::Range;

use crate::{board::Alteration, notation::fen::PositionMetadata};

use super::chess::{ChessAction, Delim};

/// data container for caching compilation while traversing a gametree.
/// it doesn't own any operation, it just holds things, it is a very good halfply.
#[derive(Debug, Clone, PartialEq)]
pub struct ActionCache {
    /// The Action that was taken
    /// FIXME: Remove the visibility
    pub(crate) action: ChessAction,
    /// If there was a metadata change for this move, it will be cached here.
    metadata: Option<Option<PositionMetadata>>,
    /// None => No Cached "Compiled" set of alterations.
    /// Some(vec![data]) => Cached Alterations from prior visit.
    /// TODO: Vec<Alteration> should probably be it's own type.
    alterations: Option<Vec<Alteration>>,
}

impl From<ChessAction> for ActionCache {
    fn from(action: ChessAction) -> Self {
        Self {
            metadata: None,
            action,
            alterations: None,
        }
    }
}

impl ActionCache {
    /// We calculated the alterations, here's where we keep them.
    pub(crate) fn cache_alterations(&mut self, alters: Vec<Alteration>) {
        self.alterations = Some(alters);
    }

    /// We calculated the metadata, it's right here.
    pub(crate) fn cache_metadata(&mut self, metadata: PositionMetadata) {
        self.metadata = Some(Some(metadata));
    }

    /// We've checked, no metadata changes with this move.
    pub(crate) fn metadata_skip(&mut self) {
        self.metadata = Some(None);
    }

    /// We don't want this metadata anymore.
    pub(crate) fn clear_metadata(&mut self) {
        self.metadata = None;
    }

    /// These alterations can go.
    pub(crate) fn clear_alterations(&mut self) {
        self.alterations = None;
    }

    /// Take it all
    pub(crate) fn clear(&mut self) {
        self.clear_metadata();
        self.clear_alterations();
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ActionLog {
    /// Log
    log: Vec<ActionCache>,
    /// Write Head
    write_head: usize
}

impl ActionLog {
    fn empty() -> Self {
        Self {
            log: vec![],
            write_head: 0,
        }
    }

    pub fn record(&mut self, action: ChessAction) -> &mut ActionCache {
        self.log.insert(self.write_head, ActionCache::from(action));

        let ret = &mut self.log[self.write_head];

        self.write_head += 1;

        ret
    }

    // TODO: This should return a ref to some object that requires the `close` to be called
    // correctly. VariationBuffer or something.
    pub fn open_variation(&mut self) -> &mut Self {
        self.record(ChessAction::Variation(Delim::Start));
        self
    }

    pub fn close_variation(&mut self) -> &mut Self {
        self.record(ChessAction::Variation(Delim::End));
        self
    }

    pub fn prev(&mut self) -> Option<ActionCache> {
        if self.write_head > 0 {
            self.write_head -= 1;
            Some(self.log[self.write_head].clone())
        } else {
            None
        }
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut ActionCache> {
        self.log.iter_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ActionCache> {
        self.log.iter()
    }

    // TODO: This needs some tools to move the cursor around, and eventually to add variations 'in
    // place', that means probably replicating something like the other line/game struct I had, but
    // with these as half-ply instead.
    //
    // NOTE: Variations can be identified by a list of halfplies and variation choice (e.g., 19.2 is the
    // second variation after the mainline for black on fullmove 9). and these can be nested
    // (5.2-10.6 is the second variation for black on turn 2 followed by the sixth variation for
    // white on turn 5, all other moves are on the 'mainline' for intermediate variations)

}


// This is a cursor type that can read the log and apply the actions in order to get a
// fully 'compiled' version of the boardstate.
struct ActionCursor<'a> {
    // A reference to whatever log we want to traverse
    log: &'a ActionLog,
    // The stack of all the actions that we've applied in our current state
    action_stack: Vec<ActionCache>,
    // Where our current position is in the parent log
    cursor: usize,
}


impl<'a> ActionCursor<'a> {
    fn new(log: &'a ActionLog) -> Self {
        Self {
            log,
            action_stack: vec![],
            cursor: 0,
        }
    }

    fn log_bounds(&self) -> Range<usize> {
        0..self.log.log.len()
    }

    /*
    fn clamp_by_bounds(&self) -> usize {

    }

    fn seek(&mut self, pos: isize) {
        let cursor_pos = self.cursor as isize;
        let requested_pos = cursor_pos + pos;

        let range = self.log_bounds();

        if self.log_bounds().contains(&requested_pos as usize) {
            self.cursor = requested_pos as usize;
        } else if requested_pos < 0 {
            self.cursor = 0;
        } else {
            self.cursor = self.log.log.len();
        }
    }
    */
}



