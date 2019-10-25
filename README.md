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

This is a good idea, but it leads to problems of where `acuracy` should go, does there need to be some concept of missing completly? could armour class just reduce damage? do we need acuracy at all?

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

Come to think of it, as the gambit system is data driven as well, we could perhaps also automatically generate and rank a set of gambits.

We also need to consider some measure of synergy, so that given one tome entry, we can pick others that synergise well. It makes no sense to value a warhammer highly, if you're a wizard

## Mana

Spell slots are probably not that good of an idea, it's pretty cool to have spells do variable things when used at different levels, but that adds a lot of complexity. There's probably a reason that literally no videogames use a spell slot system. There might some mileage in a dishonored like mana system where you regenerate 10 mana each turn _if_ you used a spell last turn.

This essentially makes weak spells free, as you'll regenerate the mana completely by next turn, but more powerful spells consume more than will re-generate, so if you keep using them you'll have to take a mana potion. You could also rest for a turn to regain some mana

- How would physically based classes interact with mana?
- Do they need to at all?
- Is the tradeoff that you can't cause as much damage as a spell caster, but you don't have a resource to manage?
