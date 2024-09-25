This was pulled from a commend in the code, it's probably 'wrong' in the sense that it's an early
design, but I didn't want to lose track of this until I was sure.

    /*
     *
     * | External GUI |
     *       |
     *       |
     *  STDIN/STDOUT
     *       |
     *       |
     * | UCI Socket |
     *       |
     *       |
     * | Race Control | --> | UI |
     *       |
     *       \-------> | Grid | 1-*> | Engine | --> STDIN/STDOUT --> External Engine over UCI
     *                     |      *> | Engine | --> STDIN/STDOUT --> Another UCI-speaking client
     *                     1      *> | Engine | --> Hazel::Driver
     *                     |      *> | Engine | --> TCP/UDP/etc --> Race Control on another machine
     *                     |      *> | Engine | --> protobuf API --> other speaker of bespoke protocol
     *                     |
     *                     \------*> | Track |
     *
     * Engines do not know about each other, programs run on the 'Grid', which is managed via the
     * UI through Race control (UI = View, Race Control = Controller, Grid = Model). The Grid
     * provides a scripting API for setting up games between all engines on the grid by providing
     * metacommands around routing UCI connections between engines and mediating games.
     *
     * The UI is a simple terminal interface that allows the user to start games, view the grid,
     * and configure engines. It also allows the user to provide 'Tracks', which are scripts that 
     * initiate and run games between engines. For instance, a 'Swiss' track would run a Swiss
     * Tournament between all engines on the grid, a 'Round Robin' track would run a round-robin,
     * etc. Tracks may also compare non-game metrics, such as 'perft' tracks or other
     * cross-reference style tests.
     *
     *
     *
     *
     */
