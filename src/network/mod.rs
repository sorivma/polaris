// Networking layer — multiplayer via deterministic lockstep.
//
// Architecture:
//   - Command-only sync (like Paradox games): only player orders are transmitted
//   - All clients simulate the same state independently
//   - Desync detection via state hash comparison
//   - Hot-join via full state transfer
//   - Headless server mode
//
// Transport: laminar (UDP with reliability layers)

// Stub — implementation begins in future commits.
