use tracing::*;

mod ui;
mod uci;


fn main() {
    info!("Welcome to Hazel.");

    // Start the Hazel Main Thread
    //   Start the UI
    let _ = ui::run();
    //   The UI displays and manages the Grid via Race Control 

    //   Start the Grid
    //       Start the Hazel Engine in Idle, Grid managed the UCI Socket.
    //   Start the Race Control
    // 
    // The UI sends commands to the Race Control, and manages the user's desired state. The
    // Race Control manages querying the Grid, which contains multiple engine instances with their
    // output routed to a socket. The Grid manages routing UCI messages to the correct engine, it
    // also manages configuring engines; a generic "UCI Entrant" would expect all configuration to
    // be done by the user via the UI, but non-generic variants could preconfigure engines and
    // ensure they are limited in terms of resources, etc.
    //
    // This allows a couple cool things.
    //
    // 1. Dummy Engines that forward UCI messages to arbitrary transport protocols.
    // 2. Meta-engines that manage multiple instances of multiple engines/Multi-engine strategy
    // 3. Natural path to create a distributed `perft` calculator, or general chess engine.
    //
    // The Hazel Engine itself is a statemachine that manages a single game.
    //
    // The Main Thread manages the other threads and ensures cleanup is done with the UI triggers
    // an Exit. Ideally it does very little most of the time.
    //
    // I'm going to need a UCI Parser, I think I'll build one.
    // I'm going to need the Ratatui UI roughed in.
    // I'm going to need to write the Hazel Engine wrapper, It should be able to understand UCI
    // messages.
    // I'm going to need to write the Grid manager
    // I'm going to need to write an adapter that starts another engine (stockfish, probably)
    //
    // Parser is easy to start, I'll go with `nom` probably since it's a simple line format.
    // Ratatui UI could be roughed in quickly to at least a hello-world state.
    //
    //
}
