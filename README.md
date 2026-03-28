# Trail of Hunger

A 2D top-down game where nomads move through ecosystems, consuming all animals and depleting flora before migrating to the next biome. The player acts as the destructive force. The goal is to completely exhaust each ecosystem.

## Core Loop

1. Nomads move through the map
2. Animals wander
3. Player directs nomads
4. Nomads consume animals and resources
5. Ecosystem depletes
6. Transition to next biome
7. Repeat with increased difficulty

## Player Role

- Direct control over nomads
- Responsible for efficient destruction
- Must eliminate all life before conditions fail

## Win / Lose Conditions

### Win (per level)

- All animals eliminated
- Flora reduced

### Lose

- Nomads die (starvation or other condition)
- Ecosystem not cleared in time (optional)

## Entities

### Nomads

- Position
- Hunger meter
- Movement (basic steering or player input)
- Consume animals to survive

### Animals

- Position
- Simple wandering behavior
- Flee when nomads are near
- Die on contact or attack

### Flora (abstract or simple)

- Static resource
- Can decay over time or through interaction

## Mechanics

### Hunger System

- Nomads get hungrier over time
- Eating animals restores hunger bar
- Hunger at zero results in death

### Movement

- Simple directional movement
- No pathfinding required
- Optional: group movement behavior

### Interaction

- Distance-based (no complex collision)
- Nomads eliminate animals on contact or attack

## Biomes / Levels

- Each level represents a different ecosystem
- Minimal visual variation (color palette swap is enough)
- Increasing difficulty:
  - More animals
  - Faster animals
  - Larger map (optional)

## Systems

### Game State

- Playing
- Level Complete
- Game Over

### Spawning

- Animals spawn at start of level
- Optional: limited respawn

### Progression

- Linear level progression
- Optional scaling variables:
  - hunger drain rate
  - number of animals

## UI (Minimal)

- Hunger bar (or value)
- Animal count remaining
- Current level indicator

## Visual Scope

- Simple 8x8 pixel art
- One sprite per entity type
- Basic animations optional
- Effects:
  - hit flash
  - simple particles

## Audio

- One sound for consumption
- One ambient loop

## Technical Constraints

- Single scene
- No complex physics
- No pathfinding
- Simple distance checks
- Fixed resolution

## Out of Scope

- Inventory systems
- Crafting
- Complex AI
- Narrative systems
- Advanced UI
- Multiplayer

## Development Plan (48h)

### Day 1

- Movement system
- Nomad entity
- Animal entity
- Basic interaction (consume)
- Core loop playable

### Day 2

- Hunger system
- UI
- Level progression
- Visual feedback
- Basic balancing

## Priority Order

1. Core loop working
2. Hunger system
3. Interaction feedback
4. UI
5. Progression

## Success Criteria

- Game is playable from start to finish
- Player can understand objective immediately
- At least 2–3 levels functional
- No major bugs blocking gameplay
