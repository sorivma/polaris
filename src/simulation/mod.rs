// Simulation layer — pure game logic, no rendering dependencies.
//
// This module contains:
//   - Province graph (15K+ nodes, adjacency, pathfinding)
//   - ECS world (hecs): divisions, air wings, naval units, agents
//   - Systems: combat, supply, production, diplomacy, AI
//   - Lua scripting runtime (mlua): events, decisions, modding API
//   - Serialization (serde/rkyv): save/load, version migration
//   - Deterministic tick-based loop for multiplayer lockstep

// Stub — implementation begins in future commits.
