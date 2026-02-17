# Fantasy World Generator

## Purpose

Generate world-scale content inspired by TownGeneratorOS-style procedural workflows:
- Continents
- Countries
- Capitals
- Town link data per country

## Data Types

Defined in `creator/src/game_logic/mod.rs`:
- `FantasyMapSettings`
- `ContinentData`
- `CountryData`
- `CountryBorder`
- `FantasyWorldMapData`

## Inputs

From editor settings panel:
- World name
- Seed
- Continent count
- Countries per continent
- Towns per country
- Islands toggle

## Output

Serialized into project config:
- `[fantasy_world_generator]`
- Summary fields (`seed`, `world_name`, counts)
- JSON payload in `data`

## Generation Notes

- Continents are seeded with radial placement and climate assignment.
- Countries are distributed around continent centers.
- Each country receives a generated capital town using town generation logic.
- Border graph is built using nearest-country adjacency with optional inter-continent routes.
