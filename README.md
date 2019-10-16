# General Notes

## Action structure

Actions should have a target, and a series of effects:

- Target
- Casting Time
- Label
- Report
- Effect: []
  - Damage
  - Add Status (with optional duration)
  - Alter Stat (with optional duration)

## Level / Value

Instead of trying to assign a level to spells/items/creatures/characters, I could instead try to automatically rank them. If we can build a gambit system that's good enough to let characters play skillfully against each other we can just simulate many skirmishes of randomly generated parties using some sort of point buy system.

1. create two parties with `x` points to buy up creatures
2. each creature is worth `y` points, and can therefore buy `y` points worth of items and spells
3. simulate a skirmish
4. change point assignments for all tome entries, using an ELO like system
5. repeat 1000s of times
6. points should start to stabelize around what tome entires are actually worth

This should obviously be audited, a poorly build gambit system could result in widely over/underpowered tome entries.
This does also have the advantage that we can tweak the properties of entries, and have them updated against the scores of all other non-updated items
