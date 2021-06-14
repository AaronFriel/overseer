use overseer_substrate_core::game::{
  ArtifactType, CardSubtype, CardSupertype, CardType, CreatureType, EnchantmentType, LandType,
  PlanarType, PlaneswalkerType, PredefinedSubtype, PredefinedSupertype, PredefinedType, SpellType,
  TypeLine, TypeLineValue,
};
use overseer_substrate_macros::type_line;

#[test]
fn test_empty_type_line() {
  const X: TypeLine = type_line!("");

  assert_eq!(X, TypeLine::from_static(&[]));
}

#[test]
fn test_supertypes() {
  use CardSupertype::*;
  use PredefinedSupertype::*;
  use TypeLineValue::*;

  const X: TypeLine = type_line!("Basic Legendary Ongoing Snow World");

  let y = TypeLine::from_static(&[
    Supertype(Predefined(Basic)),
    Supertype(Predefined(Legendary)),
    Supertype(Predefined(Ongoing)),
    Supertype(Predefined(Snow)),
    Supertype(Predefined(World)),
  ]);

  assert_eq!(X, y);
}

#[test]
fn test_types() {
  use CardType::*;
  use PredefinedType::*;
  use TypeLineValue::*;

  const X: TypeLine = type_line!(
    "Artifact Conspiracy Creature Enchantment Instant Land Phenomenon Plane Planeswalker Scheme \
     Sorcery Tribal Vanguard"
  );

  let y = TypeLine::from_static(&[
    JustAType(Predefined(Artifact)),
    JustAType(Predefined(Conspiracy)),
    JustAType(Predefined(Creature)),
    JustAType(Predefined(Enchantment)),
    JustAType(Predefined(Instant)),
    JustAType(Predefined(Land)),
    JustAType(Predefined(Phenomenon)),
    JustAType(Predefined(Plane)),
    JustAType(Predefined(Planeswalker)),
    JustAType(Predefined(Scheme)),
    JustAType(Predefined(Sorcery)),
    JustAType(Predefined(Tribal)),
    JustAType(Predefined(Vanguard)),
  ]);

  assert_eq!(X, y);
}

#[test]
fn test_artifact_types() {
  use ArtifactType::*;
  use CardSubtype::*;
  use PredefinedSubtype::*;
  use TypeLineValue::*;

  const X: TypeLine =
    type_line!("Clue Contraption Equipment Food Fortification Gold Treasure Vehicle");

  let y = TypeLine::from_static(&[
    Subtype(Predefined(Artifact(Clue))),
    Subtype(Predefined(Artifact(Contraption))),
    Subtype(Predefined(Artifact(Equipment))),
    Subtype(Predefined(Artifact(Food))),
    Subtype(Predefined(Artifact(Fortification))),
    Subtype(Predefined(Artifact(Gold))),
    Subtype(Predefined(Artifact(Treasure))),
    Subtype(Predefined(Artifact(Vehicle))),
  ]);

  assert_eq!(X, y);
}

#[test]
fn test_enchantment_types() {
  use CardSubtype::*;
  use EnchantmentType::*;
  use PredefinedSubtype::*;
  use TypeLineValue::*;

  const X: TypeLine = type_line!("Aura Cartouche Curse Rune Saga Shard Shrine");

  let y = TypeLine::from_static(&[
    Subtype(Predefined(Enchantment(Aura))),
    Subtype(Predefined(Enchantment(Cartouche))),
    Subtype(Predefined(Enchantment(Curse))),
    Subtype(Predefined(Enchantment(Rune))),
    Subtype(Predefined(Enchantment(Saga))),
    Subtype(Predefined(Enchantment(Shard))),
    Subtype(Predefined(Enchantment(Shrine))),
  ]);

  assert_eq!(X, y);
}

#[test]
fn test_land_types() {
  use CardSubtype::*;
  use LandType::*;
  use PredefinedSubtype::*;
  use TypeLineValue::*;

  const X: TypeLine = type_line!(
    "Desert Forest Gate Island Lair Locus Mine Mountain Plains Power-Plant Swamp Tower Urza's"
  );

  let y = TypeLine::from_static(&[
    Subtype(Predefined(Land(Desert))),
    Subtype(Predefined(Land(Forest))),
    Subtype(Predefined(Land(Gate))),
    Subtype(Predefined(Land(Island))),
    Subtype(Predefined(Land(Lair))),
    Subtype(Predefined(Land(Locus))),
    Subtype(Predefined(Land(Mine))),
    Subtype(Predefined(Land(Mountain))),
    Subtype(Predefined(Land(Plains))),
    Subtype(Predefined(Land(PowerPlant))),
    Subtype(Predefined(Land(Swamp))),
    Subtype(Predefined(Land(Tower))),
    Subtype(Predefined(Land(Urzas))),
  ]);

  assert_eq!(X, y);
}

#[test]
fn test_planeswalker_types() {
  use CardSubtype::*;
  use PlaneswalkerType::*;
  use PredefinedSubtype::*;
  use TypeLineValue::*;

  const X: TypeLine = type_line!(
    "Ajani Aminatou Angrath Arlinn Ashiok Basri Bolas Calix Chandra Dack Daretti Davriel Domri \
     Dovin Elspeth Estrid Freyalise Garruk Gideon Huatli Jace Jaya Jeska Karn Kasmina Kaya Kiora \
     Koth Liliana Lukka Nahiri Narset Niko Nissa Nixilis Oko Ral Rowan Saheeli Samut Sarkhan \
     Serra Sorin Szat Tamiyo Teferi Teyo Tezzeret Tibalt Tyvar Ugin Venser Vivien Vraska Will \
     Windgrace Wrenn Xenagos Yanggu Yanling"
  );

  let y = TypeLine::from_static(&[
    Subtype(Predefined(Planeswalker(Ajani))),
    Subtype(Predefined(Planeswalker(Aminatou))),
    Subtype(Predefined(Planeswalker(Angrath))),
    Subtype(Predefined(Planeswalker(Arlinn))),
    Subtype(Predefined(Planeswalker(Ashiok))),
    Subtype(Predefined(Planeswalker(Basri))),
    Subtype(Predefined(Planeswalker(Bolas))),
    Subtype(Predefined(Planeswalker(Calix))),
    Subtype(Predefined(Planeswalker(Chandra))),
    Subtype(Predefined(Planeswalker(Dack))),
    Subtype(Predefined(Planeswalker(Daretti))),
    Subtype(Predefined(Planeswalker(Davriel))),
    Subtype(Predefined(Planeswalker(Domri))),
    Subtype(Predefined(Planeswalker(Dovin))),
    Subtype(Predefined(Planeswalker(Elspeth))),
    Subtype(Predefined(Planeswalker(Estrid))),
    Subtype(Predefined(Planeswalker(Freyalise))),
    Subtype(Predefined(Planeswalker(Garruk))),
    Subtype(Predefined(Planeswalker(Gideon))),
    Subtype(Predefined(Planeswalker(Huatli))),
    Subtype(Predefined(Planeswalker(Jace))),
    Subtype(Predefined(Planeswalker(Jaya))),
    Subtype(Predefined(Planeswalker(Jeska))),
    Subtype(Predefined(Planeswalker(Karn))),
    Subtype(Predefined(Planeswalker(Kasmina))),
    Subtype(Predefined(Planeswalker(Kaya))),
    Subtype(Predefined(Planeswalker(Kiora))),
    Subtype(Predefined(Planeswalker(Koth))),
    Subtype(Predefined(Planeswalker(Liliana))),
    Subtype(Predefined(Planeswalker(Lukka))),
    Subtype(Predefined(Planeswalker(Nahiri))),
    Subtype(Predefined(Planeswalker(Narset))),
    Subtype(Predefined(Planeswalker(Niko))),
    Subtype(Predefined(Planeswalker(Nissa))),
    Subtype(Predefined(Planeswalker(Nixilis))),
    Subtype(Predefined(Planeswalker(Oko))),
    Subtype(Predefined(Planeswalker(Ral))),
    Subtype(Predefined(Planeswalker(Rowan))),
    Subtype(Predefined(Planeswalker(Saheeli))),
    Subtype(Predefined(Planeswalker(Samut))),
    Subtype(Predefined(Planeswalker(Sarkhan))),
    Subtype(Predefined(Planeswalker(Serra))),
    Subtype(Predefined(Planeswalker(Sorin))),
    Subtype(Predefined(Planeswalker(Szat))),
    Subtype(Predefined(Planeswalker(Tamiyo))),
    Subtype(Predefined(Planeswalker(Teferi))),
    Subtype(Predefined(Planeswalker(Teyo))),
    Subtype(Predefined(Planeswalker(Tezzeret))),
    Subtype(Predefined(Planeswalker(Tibalt))),
    Subtype(Predefined(Planeswalker(Tyvar))),
    Subtype(Predefined(Planeswalker(Ugin))),
    Subtype(Predefined(Planeswalker(Venser))),
    Subtype(Predefined(Planeswalker(Vivien))),
    Subtype(Predefined(Planeswalker(Vraska))),
    Subtype(Predefined(Planeswalker(Will))),
    Subtype(Predefined(Planeswalker(Windgrace))),
    Subtype(Predefined(Planeswalker(Wrenn))),
    Subtype(Predefined(Planeswalker(Xenagos))),
    Subtype(Predefined(Planeswalker(Yanggu))),
    Subtype(Predefined(Planeswalker(Yanling))),
  ]);

  assert_eq!(X, y);
}

#[test]
fn test_spell_types() {
  use CardSubtype::*;
  use PredefinedSubtype::*;
  use SpellType::*;
  use TypeLineValue::*;

  const X: TypeLine = type_line!("Adventure Arcane Lesson Trap");

  let y = TypeLine::from_static(&[
    Subtype(Predefined(Spell(Adventure))),
    Subtype(Predefined(Spell(Arcane))),
    Subtype(Predefined(Spell(Lesson))),
    Subtype(Predefined(Spell(Trap))),
  ]);

  assert_eq!(X, y);
}

#[test]
fn test_creature_types() {
  use CardSubtype::*;
  use CreatureType::*;
  use PredefinedSubtype::*;
  use TypeLineValue::*;

  const X: TypeLine = type_line!(
    "Advisor Aetherborn Ally Angel Antelope Ape Archer Archon Army Artificer Assassin \
     Assembly-Worker Atog Aurochs Avatar Azra Badger Barbarian Basilisk Bat Bear Beast Beeble \
     Berserker Bird Blinkmoth Boar Bringer Brushwagg Camarid Camel Caribou Carrier Cat Centaur \
     Cephalid Chimera Citizen Cleric Cockatrice Construct Coward Crab Crocodile Cyclops Dauthi \
     Demigod Demon Deserter Devil Dinosaur Djinn Dog Dragon Drake Dreadnought Drone Druid Dryad \
     Dwarf Efreet Egg Elder Eldrazi Elemental Elephant Elf Elk Eye Faerie Ferret Fish Flagbearer \
     Fox Fractal Frog Fungus Gargoyle Germ Giant Gnome Goat Goblin God Golem Gorgon Graveborn \
     Gremlin Griffin Hag Harpy Hellion Hippo Hippogriff Homarid Homunculus Horror Horse Human \
     Hydra Hyena Illusion Imp Incarnation Inkling Insect Jackal Jellyfish Juggernaut Kavu Kirin \
     Kithkin Knight Kobold Kor Kraken Lamia Lammasu Leech Leviathan Lhurgoyf Licid Lizard \
     Manticore Masticore Mercenary Merfolk Metathran Minion Minotaur Mole Monger Mongoose Monk \
     Monkey Moonfolk Mouse Mutant Myr Mystic Naga Nautilus Nephilim Nightmare Nightstalker Ninja \
     Noble Noggle Nomad Nymph Octopus Ogre Ooze Orb Orc Orgg Otter Ouphe Ox Oyster Pangolin \
     Peasant Pegasus Pentavite Pest Phelddagrif Phoenix Phyrexian Pilot Pincher Pirate Plant \
     Praetor Prism Processor Rabbit Rat Rebel Reflection Rhino Rigger Rogue Sable Salamander \
     Samurai Sand Saproling Satyr Scarecrow Scion Scorpion Scout Sculpture Serf Serpent Servo \
     Shade Shaman Shapeshifter Shark Sheep Siren Skeleton Slith Sliver Slug Snake Soldier Soltari \
     Spawn Specter Spellshaper Sphinx Spider Spike Spirit Splinter Sponge Squid Squirrel Starfish \
     Surrakar Survivor Tentacle Tetravite Thalakos Thopter Thrull Treefolk Trilobite Triskelavite \
     Troll Turtle Unicorn Vampire Vedalken Viashino Volver Wall Warlock Warrior Weird Werewolf \
     Whale Wizard Wolf Wolverine Wombat Worm Wraith Wurm Yeti Zombie Zubera"
  );

  let y = TypeLine::from_static(&[
    Subtype(Predefined(Creature(Advisor))),
    Subtype(Predefined(Creature(Aetherborn))),
    Subtype(Predefined(Creature(Ally))),
    Subtype(Predefined(Creature(Angel))),
    Subtype(Predefined(Creature(Antelope))),
    Subtype(Predefined(Creature(Ape))),
    Subtype(Predefined(Creature(Archer))),
    Subtype(Predefined(Creature(Archon))),
    Subtype(Predefined(Creature(Army))),
    Subtype(Predefined(Creature(Artificer))),
    Subtype(Predefined(Creature(Assassin))),
    Subtype(Predefined(Creature(AssemblyWorker))),
    Subtype(Predefined(Creature(Atog))),
    Subtype(Predefined(Creature(Aurochs))),
    Subtype(Predefined(Creature(Avatar))),
    Subtype(Predefined(Creature(Azra))),
    Subtype(Predefined(Creature(Badger))),
    Subtype(Predefined(Creature(Barbarian))),
    Subtype(Predefined(Creature(Basilisk))),
    Subtype(Predefined(Creature(Bat))),
    Subtype(Predefined(Creature(Bear))),
    Subtype(Predefined(Creature(Beast))),
    Subtype(Predefined(Creature(Beeble))),
    Subtype(Predefined(Creature(Berserker))),
    Subtype(Predefined(Creature(Bird))),
    Subtype(Predefined(Creature(Blinkmoth))),
    Subtype(Predefined(Creature(Boar))),
    Subtype(Predefined(Creature(Bringer))),
    Subtype(Predefined(Creature(Brushwagg))),
    Subtype(Predefined(Creature(Camarid))),
    Subtype(Predefined(Creature(Camel))),
    Subtype(Predefined(Creature(Caribou))),
    Subtype(Predefined(Creature(Carrier))),
    Subtype(Predefined(Creature(Cat))),
    Subtype(Predefined(Creature(Centaur))),
    Subtype(Predefined(Creature(Cephalid))),
    Subtype(Predefined(Creature(Chimera))),
    Subtype(Predefined(Creature(Citizen))),
    Subtype(Predefined(Creature(Cleric))),
    Subtype(Predefined(Creature(Cockatrice))),
    Subtype(Predefined(Creature(Construct))),
    Subtype(Predefined(Creature(Coward))),
    Subtype(Predefined(Creature(Crab))),
    Subtype(Predefined(Creature(Crocodile))),
    Subtype(Predefined(Creature(Cyclops))),
    Subtype(Predefined(Creature(Dauthi))),
    Subtype(Predefined(Creature(Demigod))),
    Subtype(Predefined(Creature(Demon))),
    Subtype(Predefined(Creature(Deserter))),
    Subtype(Predefined(Creature(Devil))),
    Subtype(Predefined(Creature(Dinosaur))),
    Subtype(Predefined(Creature(Djinn))),
    Subtype(Predefined(Creature(Dog))),
    Subtype(Predefined(Creature(Dragon))),
    Subtype(Predefined(Creature(Drake))),
    Subtype(Predefined(Creature(Dreadnought))),
    Subtype(Predefined(Creature(Drone))),
    Subtype(Predefined(Creature(Druid))),
    Subtype(Predefined(Creature(Dryad))),
    Subtype(Predefined(Creature(Dwarf))),
    Subtype(Predefined(Creature(Efreet))),
    Subtype(Predefined(Creature(Egg))),
    Subtype(Predefined(Creature(Elder))),
    Subtype(Predefined(Creature(Eldrazi))),
    Subtype(Predefined(Creature(Elemental))),
    Subtype(Predefined(Creature(Elephant))),
    Subtype(Predefined(Creature(Elf))),
    Subtype(Predefined(Creature(Elk))),
    Subtype(Predefined(Creature(Eye))),
    Subtype(Predefined(Creature(Faerie))),
    Subtype(Predefined(Creature(Ferret))),
    Subtype(Predefined(Creature(Fish))),
    Subtype(Predefined(Creature(Flagbearer))),
    Subtype(Predefined(Creature(Fox))),
    Subtype(Predefined(Creature(Fractal))),
    Subtype(Predefined(Creature(Frog))),
    Subtype(Predefined(Creature(Fungus))),
    Subtype(Predefined(Creature(Gargoyle))),
    Subtype(Predefined(Creature(Germ))),
    Subtype(Predefined(Creature(Giant))),
    Subtype(Predefined(Creature(Gnome))),
    Subtype(Predefined(Creature(Goat))),
    Subtype(Predefined(Creature(Goblin))),
    Subtype(Predefined(Creature(God))),
    Subtype(Predefined(Creature(Golem))),
    Subtype(Predefined(Creature(Gorgon))),
    Subtype(Predefined(Creature(Graveborn))),
    Subtype(Predefined(Creature(Gremlin))),
    Subtype(Predefined(Creature(Griffin))),
    Subtype(Predefined(Creature(Hag))),
    Subtype(Predefined(Creature(Harpy))),
    Subtype(Predefined(Creature(Hellion))),
    Subtype(Predefined(Creature(Hippo))),
    Subtype(Predefined(Creature(Hippogriff))),
    Subtype(Predefined(Creature(Homarid))),
    Subtype(Predefined(Creature(Homunculus))),
    Subtype(Predefined(Creature(Horror))),
    Subtype(Predefined(Creature(Horse))),
    Subtype(Predefined(Creature(Human))),
    Subtype(Predefined(Creature(Hydra))),
    Subtype(Predefined(Creature(Hyena))),
    Subtype(Predefined(Creature(Illusion))),
    Subtype(Predefined(Creature(Imp))),
    Subtype(Predefined(Creature(Incarnation))),
    Subtype(Predefined(Creature(Inkling))),
    Subtype(Predefined(Creature(Insect))),
    Subtype(Predefined(Creature(Jackal))),
    Subtype(Predefined(Creature(Jellyfish))),
    Subtype(Predefined(Creature(Juggernaut))),
    Subtype(Predefined(Creature(Kavu))),
    Subtype(Predefined(Creature(Kirin))),
    Subtype(Predefined(Creature(Kithkin))),
    Subtype(Predefined(Creature(Knight))),
    Subtype(Predefined(Creature(Kobold))),
    Subtype(Predefined(Creature(Kor))),
    Subtype(Predefined(Creature(Kraken))),
    Subtype(Predefined(Creature(Lamia))),
    Subtype(Predefined(Creature(Lammasu))),
    Subtype(Predefined(Creature(Leech))),
    Subtype(Predefined(Creature(Leviathan))),
    Subtype(Predefined(Creature(Lhurgoyf))),
    Subtype(Predefined(Creature(Licid))),
    Subtype(Predefined(Creature(Lizard))),
    Subtype(Predefined(Creature(Manticore))),
    Subtype(Predefined(Creature(Masticore))),
    Subtype(Predefined(Creature(Mercenary))),
    Subtype(Predefined(Creature(Merfolk))),
    Subtype(Predefined(Creature(Metathran))),
    Subtype(Predefined(Creature(Minion))),
    Subtype(Predefined(Creature(Minotaur))),
    Subtype(Predefined(Creature(Mole))),
    Subtype(Predefined(Creature(Monger))),
    Subtype(Predefined(Creature(Mongoose))),
    Subtype(Predefined(Creature(Monk))),
    Subtype(Predefined(Creature(Monkey))),
    Subtype(Predefined(Creature(Moonfolk))),
    Subtype(Predefined(Creature(Mouse))),
    Subtype(Predefined(Creature(Mutant))),
    Subtype(Predefined(Creature(Myr))),
    Subtype(Predefined(Creature(Mystic))),
    Subtype(Predefined(Creature(Naga))),
    Subtype(Predefined(Creature(Nautilus))),
    Subtype(Predefined(Creature(Nephilim))),
    Subtype(Predefined(Creature(Nightmare))),
    Subtype(Predefined(Creature(Nightstalker))),
    Subtype(Predefined(Creature(Ninja))),
    Subtype(Predefined(Creature(Noble))),
    Subtype(Predefined(Creature(Noggle))),
    Subtype(Predefined(Creature(Nomad))),
    Subtype(Predefined(Creature(Nymph))),
    Subtype(Predefined(Creature(Octopus))),
    Subtype(Predefined(Creature(Ogre))),
    Subtype(Predefined(Creature(Ooze))),
    Subtype(Predefined(Creature(Orb))),
    Subtype(Predefined(Creature(Orc))),
    Subtype(Predefined(Creature(Orgg))),
    Subtype(Predefined(Creature(Otter))),
    Subtype(Predefined(Creature(Ouphe))),
    Subtype(Predefined(Creature(Ox))),
    Subtype(Predefined(Creature(Oyster))),
    Subtype(Predefined(Creature(Pangolin))),
    Subtype(Predefined(Creature(Peasant))),
    Subtype(Predefined(Creature(Pegasus))),
    Subtype(Predefined(Creature(Pentavite))),
    Subtype(Predefined(Creature(Pest))),
    Subtype(Predefined(Creature(Phelddagrif))),
    Subtype(Predefined(Creature(Phoenix))),
    Subtype(Predefined(Creature(Phyrexian))),
    Subtype(Predefined(Creature(Pilot))),
    Subtype(Predefined(Creature(Pincher))),
    Subtype(Predefined(Creature(Pirate))),
    Subtype(Predefined(Creature(Plant))),
    Subtype(Predefined(Creature(Praetor))),
    Subtype(Predefined(Creature(Prism))),
    Subtype(Predefined(Creature(Processor))),
    Subtype(Predefined(Creature(Rabbit))),
    Subtype(Predefined(Creature(Rat))),
    Subtype(Predefined(Creature(Rebel))),
    Subtype(Predefined(Creature(Reflection))),
    Subtype(Predefined(Creature(Rhino))),
    Subtype(Predefined(Creature(Rigger))),
    Subtype(Predefined(Creature(Rogue))),
    Subtype(Predefined(Creature(Sable))),
    Subtype(Predefined(Creature(Salamander))),
    Subtype(Predefined(Creature(Samurai))),
    Subtype(Predefined(Creature(Sand))),
    Subtype(Predefined(Creature(Saproling))),
    Subtype(Predefined(Creature(Satyr))),
    Subtype(Predefined(Creature(Scarecrow))),
    Subtype(Predefined(Creature(Scion))),
    Subtype(Predefined(Creature(Scorpion))),
    Subtype(Predefined(Creature(Scout))),
    Subtype(Predefined(Creature(Sculpture))),
    Subtype(Predefined(Creature(Serf))),
    Subtype(Predefined(Creature(Serpent))),
    Subtype(Predefined(Creature(Servo))),
    Subtype(Predefined(Creature(Shade))),
    Subtype(Predefined(Creature(Shaman))),
    Subtype(Predefined(Creature(Shapeshifter))),
    Subtype(Predefined(Creature(Shark))),
    Subtype(Predefined(Creature(Sheep))),
    Subtype(Predefined(Creature(Siren))),
    Subtype(Predefined(Creature(Skeleton))),
    Subtype(Predefined(Creature(Slith))),
    Subtype(Predefined(Creature(Sliver))),
    Subtype(Predefined(Creature(Slug))),
    Subtype(Predefined(Creature(Snake))),
    Subtype(Predefined(Creature(Soldier))),
    Subtype(Predefined(Creature(Soltari))),
    Subtype(Predefined(Creature(Spawn))),
    Subtype(Predefined(Creature(Specter))),
    Subtype(Predefined(Creature(Spellshaper))),
    Subtype(Predefined(Creature(Sphinx))),
    Subtype(Predefined(Creature(Spider))),
    Subtype(Predefined(Creature(Spike))),
    Subtype(Predefined(Creature(Spirit))),
    Subtype(Predefined(Creature(Splinter))),
    Subtype(Predefined(Creature(Sponge))),
    Subtype(Predefined(Creature(Squid))),
    Subtype(Predefined(Creature(Squirrel))),
    Subtype(Predefined(Creature(Starfish))),
    Subtype(Predefined(Creature(Surrakar))),
    Subtype(Predefined(Creature(Survivor))),
    Subtype(Predefined(Creature(Tentacle))),
    Subtype(Predefined(Creature(Tetravite))),
    Subtype(Predefined(Creature(Thalakos))),
    Subtype(Predefined(Creature(Thopter))),
    Subtype(Predefined(Creature(Thrull))),
    Subtype(Predefined(Creature(Treefolk))),
    Subtype(Predefined(Creature(Trilobite))),
    Subtype(Predefined(Creature(Triskelavite))),
    Subtype(Predefined(Creature(Troll))),
    Subtype(Predefined(Creature(Turtle))),
    Subtype(Predefined(Creature(Unicorn))),
    Subtype(Predefined(Creature(Vampire))),
    Subtype(Predefined(Creature(Vedalken))),
    Subtype(Predefined(Creature(Viashino))),
    Subtype(Predefined(Creature(Volver))),
    Subtype(Predefined(Creature(Wall))),
    Subtype(Predefined(Creature(Warlock))),
    Subtype(Predefined(Creature(Warrior))),
    Subtype(Predefined(Creature(Weird))),
    Subtype(Predefined(Creature(Werewolf))),
    Subtype(Predefined(Creature(Whale))),
    Subtype(Predefined(Creature(Wizard))),
    Subtype(Predefined(Creature(Wolf))),
    Subtype(Predefined(Creature(Wolverine))),
    Subtype(Predefined(Creature(Wombat))),
    Subtype(Predefined(Creature(Worm))),
    Subtype(Predefined(Creature(Wraith))),
    Subtype(Predefined(Creature(Wurm))),
    Subtype(Predefined(Creature(Yeti))),
    Subtype(Predefined(Creature(Zombie))),
    Subtype(Predefined(Creature(Zubera))),
  ]);

  assert_eq!(X, y);
}

#[test]
fn test_planar_types_one_word() {
  use CardSubtype::*;
  use PlanarType::*;
  use PredefinedSubtype::*;
  use TypeLineValue::*;

  const X: TypeLine = type_line!(
    "Alara Arkhos Azgol Belenon Dominaria Equilor Ergamon Fabacin Innistrad Iquatana Ir Kaldheim \
     Kamigawa Karsus Kephalai Kinshala Kolbahan Kyneth Lorwyn Luvion Mercadia Mirrodin Moag \
     Mongseng Muraganda Phyrexia Pyrulea Rabiah Rath Ravnica Regatha Segovia Shadowmoor Shandalar \
     Ulgrotha Valla Vryn Wildfire Xerex Zendikar"
  );

  let y = TypeLine::from_static(&[
    Subtype(Predefined(Planar(Alara))),
    Subtype(Predefined(Planar(Arkhos))),
    Subtype(Predefined(Planar(Azgol))),
    Subtype(Predefined(Planar(Belenon))),
    Subtype(Predefined(Planar(Dominaria))),
    Subtype(Predefined(Planar(Equilor))),
    Subtype(Predefined(Planar(Ergamon))),
    Subtype(Predefined(Planar(Fabacin))),
    Subtype(Predefined(Planar(Innistrad))),
    Subtype(Predefined(Planar(Iquatana))),
    Subtype(Predefined(Planar(Ir))),
    Subtype(Predefined(Planar(Kaldheim))),
    Subtype(Predefined(Planar(Kamigawa))),
    Subtype(Predefined(Planar(Karsus))),
    Subtype(Predefined(Planar(Kephalai))),
    Subtype(Predefined(Planar(Kinshala))),
    Subtype(Predefined(Planar(Kolbahan))),
    Subtype(Predefined(Planar(Kyneth))),
    Subtype(Predefined(Planar(Lorwyn))),
    Subtype(Predefined(Planar(Luvion))),
    Subtype(Predefined(Planar(Mercadia))),
    Subtype(Predefined(Planar(Mirrodin))),
    Subtype(Predefined(Planar(Moag))),
    Subtype(Predefined(Planar(Mongseng))),
    Subtype(Predefined(Planar(Muraganda))),
    Subtype(Predefined(Planar(Phyrexia))),
    Subtype(Predefined(Planar(Pyrulea))),
    Subtype(Predefined(Planar(Rabiah))),
    Subtype(Predefined(Planar(Rath))),
    Subtype(Predefined(Planar(Ravnica))),
    Subtype(Predefined(Planar(Regatha))),
    Subtype(Predefined(Planar(Segovia))),
    Subtype(Predefined(Planar(Shadowmoor))),
    Subtype(Predefined(Planar(Shandalar))),
    Subtype(Predefined(Planar(Ulgrotha))),
    Subtype(Predefined(Planar(Valla))),
    Subtype(Predefined(Planar(Vryn))),
    Subtype(Predefined(Planar(Wildfire))),
    Subtype(Predefined(Planar(Xerex))),
    Subtype(Predefined(Planar(Zendikar))),
  ]);

  assert_eq!(X, y);
}

#[test]
fn test_planar_type_bolas_meditation_realm() {
  use CardSubtype::*;
  use PlanarType::*;
  use PredefinedSubtype::*;
  use PredefinedType::*;
  use TypeLineValue::*;

  const X: TypeLine = type_line!("Plane - Bolas's Meditation Realm");

  let y = TypeLine::from_static(&[
    JustAType(CardType::Predefined(Plane)),
    Subtype(Predefined(Planar(BolassMeditationRealm))),
  ]);

  assert_eq!(X, y);
}

#[test]
fn test_planar_type_new_phyrexia() {
  use CardSubtype::*;
  use PlanarType::*;
  use PredefinedSubtype::*;
  use PredefinedType::*;
  use TypeLineValue::*;

  const X: TypeLine = type_line!("Plane - New Phyrexia");

  let y = TypeLine::from_static(&[
    JustAType(CardType::Predefined(Plane)),
    Subtype(Predefined(Planar(NewPhyrexia))),
  ]);

  assert_eq!(X, y);
}

#[test]
fn test_planar_type_serras_realm() {
  use CardSubtype::*;
  use PlanarType::*;
  use PredefinedSubtype::*;
  use PredefinedType::*;
  use TypeLineValue::*;

  const X: TypeLine = type_line!("Plane - Serra's Realm");

  let y = TypeLine::from_static(&[
    JustAType(CardType::Predefined(Plane)),
    Subtype(Predefined(Planar(SerrasRealm))),
  ]);

  assert_eq!(X, y);
}

#[test]
fn test_sample_types_one() {
  use CreatureType::*;
  use PredefinedSubtype::*;
  use PredefinedSupertype::*;
  use TypeLineValue::*;

  const X: TypeLine = type_line!("Legendary Creature - Nightmare Beast Elemental");

  let y = TypeLine::from_static(&[
    Supertype(CardSupertype::Predefined(Legendary)),
    JustAType(CardType::Predefined(PredefinedType::Creature)),
    Subtype(CardSubtype::Predefined(Creature(Nightmare))),
    Subtype(CardSubtype::Predefined(Creature(Beast))),
    Subtype(CardSubtype::Predefined(Creature(Elemental))),
  ]);

  assert_eq!(X, y);
}
