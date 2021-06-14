use std::{borrow::Cow, fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;
use strum::{AsRefStr, Display, EnumString, EnumVariantNames, IntoStaticStr};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde_diff(opaque)]
pub enum SubtypeKind {
  ArtifactType,
  EnchantmentType,
  LandType,
  PlaneswalkerType,
  SpellType,
  CreatureType,
  PlanarType,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde(untagged)]
#[serde_diff(opaque)]
pub enum CardSubtype {
  Predefined(PredefinedSubtype),
  Custom(CustomSubtype),
}

impl Display for CardSubtype {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CardSubtype::Predefined(v) => v.fmt(f),
      CardSubtype::Custom(v) => v.value.fmt(f),
    }
  }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde(untagged)]
#[serde_diff(opaque)]
pub enum PredefinedSubtype {
  Artifact(ArtifactType),
  Enchantment(EnchantmentType),
  Land(LandType),
  Planeswalker(PlaneswalkerType),
  Spell(SpellType),
  Creature(CreatureType),
  Planar(PlanarType),
}

impl Display for PredefinedSubtype {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      PredefinedSubtype::Artifact(v) => v.fmt(f),
      PredefinedSubtype::Enchantment(v) => v.fmt(f),
      PredefinedSubtype::Land(v) => v.fmt(f),
      PredefinedSubtype::Planeswalker(v) => v.fmt(f),
      PredefinedSubtype::Spell(v) => v.fmt(f),
      PredefinedSubtype::Creature(v) => v.fmt(f),
      PredefinedSubtype::Planar(v) => v.fmt(f),
    }
  }
}

impl FromStr for PredefinedSubtype {
  type Err = String;

  fn from_str(value: &str) -> Result<Self, Self::Err> {
    ArtifactType::from_str(value)
      .map(PredefinedSubtype::Artifact)
      .or(EnchantmentType::from_str(value).map(PredefinedSubtype::Enchantment))
      .or(LandType::from_str(value).map(PredefinedSubtype::Land))
      .or(PlaneswalkerType::from_str(value).map(PredefinedSubtype::Planeswalker))
      .or(SpellType::from_str(value).map(PredefinedSubtype::Spell))
      .or(CreatureType::from_str(value).map(PredefinedSubtype::Creature))
      .or(PlanarType::from_str(value).map(PredefinedSubtype::Planar))
      .or(Err(format!("Invalid subtype {}", value)))
  }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde_diff(opaque)]
pub struct CustomSubtype {
  pub parent_type: SubtypeKind,
  pub value: Cow<'static, str>,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[derive(Display, EnumString, AsRefStr, IntoStaticStr, EnumVariantNames)] // From strum
pub enum ArtifactType {
  Clue,
  Contraption,
  Equipment,
  Food,
  Fortification,
  Gold,
  Treasure,
  Vehicle,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[derive(Display, EnumString, AsRefStr, IntoStaticStr, EnumVariantNames)] // From strum
pub enum EnchantmentType {
  Aura,
  Cartouche,
  Curse,
  Rune,
  Saga,
  Shard,
  Shrine,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[derive(Display, EnumString, AsRefStr, IntoStaticStr, EnumVariantNames)] // From strum
pub enum LandType {
  Desert,
  Forest,
  Gate,
  Island,
  Lair,
  Locus,
  Mine,
  Mountain,
  Plains,
  #[strum(serialize = "Power-Plant")]
  PowerPlant,
  Swamp,
  Tower,
  #[strum(serialize = "Urza's")]
  Urzas,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[derive(Display, EnumString, AsRefStr, IntoStaticStr, EnumVariantNames)] // From strum
pub enum PlaneswalkerType {
  Ajani,
  Aminatou,
  Angrath,
  Arlinn,
  Ashiok,
  Basri,
  Bolas,
  Calix,
  Chandra,
  Dack,
  Daretti,
  Davriel,
  Domri,
  Dovin,
  Elspeth,
  Estrid,
  Freyalise,
  Garruk,
  Gideon,
  Huatli,
  Jace,
  Jaya,
  Jeska,
  Karn,
  Kasmina,
  Kaya,
  Kiora,
  Koth,
  Liliana,
  Lukka,
  Nahiri,
  Narset,
  Niko,
  Nissa,
  Nixilis,
  Oko,
  Ral,
  Rowan,
  Saheeli,
  Samut,
  Sarkhan,
  Serra,
  Sorin,
  Szat,
  Tamiyo,
  Teferi,
  Teyo,
  Tezzeret,
  Tibalt,
  Tyvar,
  Ugin,
  Venser,
  Vivien,
  Vraska,
  Will,
  Windgrace,
  Wrenn,
  Xenagos,
  Yanggu,
  Yanling,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[derive(Display, EnumString, AsRefStr, IntoStaticStr, EnumVariantNames)] // From strum
pub enum SpellType {
  Adventure,
  Arcane,
  Lesson,
  Trap,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[derive(Display, EnumString, AsRefStr, IntoStaticStr, EnumVariantNames)] // From strum
pub enum CreatureType {
  Advisor,
  Aetherborn,
  Ally,
  Angel,
  Antelope,
  Ape,
  Archer,
  Archon,
  Army,
  Artificer,
  Assassin,
  #[strum(serialize = "Assembly-Worker")]
  AssemblyWorker,
  Atog,
  Aurochs,
  Avatar,
  Azra,
  Badger,
  Barbarian,
  Basilisk,
  Bat,
  Bear,
  Beast,
  Beeble,
  Berserker,
  Bird,
  Blinkmoth,
  Boar,
  Bringer,
  Brushwagg,
  Camarid,
  Camel,
  Caribou,
  Carrier,
  Cat,
  Centaur,
  Cephalid,
  Chimera,
  Citizen,
  Cleric,
  Cockatrice,
  Construct,
  Coward,
  Crab,
  Crocodile,
  Cyclops,
  Dauthi,
  Demigod,
  Demon,
  Deserter,
  Devil,
  Dinosaur,
  Djinn,
  Dog,
  Dragon,
  Drake,
  Dreadnought,
  Drone,
  Druid,
  Dryad,
  Dwarf,
  Efreet,
  Egg,
  Elder,
  Eldrazi,
  Elemental,
  Elephant,
  Elf,
  Elk,
  Eye,
  Faerie,
  Ferret,
  Fish,
  Flagbearer,
  Fox,
  Fractal,
  Frog,
  Fungus,
  Gargoyle,
  Germ,
  Giant,
  Gnome,
  Goat,
  Goblin,
  God,
  Golem,
  Gorgon,
  Graveborn,
  Gremlin,
  Griffin,
  Hag,
  Harpy,
  Hellion,
  Hippo,
  Hippogriff,
  Homarid,
  Homunculus,
  Horror,
  Horse,
  Human,
  Hydra,
  Hyena,
  Illusion,
  Imp,
  Incarnation,
  Inkling,
  Insect,
  Jackal,
  Jellyfish,
  Juggernaut,
  Kavu,
  Kirin,
  Kithkin,
  Knight,
  Kobold,
  Kor,
  Kraken,
  Lamia,
  Lammasu,
  Leech,
  Leviathan,
  Lhurgoyf,
  Licid,
  Lizard,
  Manticore,
  Masticore,
  Mercenary,
  Merfolk,
  Metathran,
  Minion,
  Minotaur,
  Mole,
  Monger,
  Mongoose,
  Monk,
  Monkey,
  Moonfolk,
  Mouse,
  Mutant,
  Myr,
  Mystic,
  Naga,
  Nautilus,
  Nephilim,
  Nightmare,
  Nightstalker,
  Ninja,
  Noble,
  Noggle,
  Nomad,
  Nymph,
  Octopus,
  Ogre,
  Ooze,
  Orb,
  Orc,
  Orgg,
  Otter,
  Ouphe,
  Ox,
  Oyster,
  Pangolin,
  Peasant,
  Pegasus,
  Pentavite,
  Pest,
  Phelddagrif,
  Phoenix,
  Phyrexian,
  Pilot,
  Pincher,
  Pirate,
  Plant,
  Praetor,
  Prism,
  Processor,
  Rabbit,
  Rat,
  Rebel,
  Reflection,
  Rhino,
  Rigger,
  Rogue,
  Sable,
  Salamander,
  Samurai,
  Sand,
  Saproling,
  Satyr,
  Scarecrow,
  Scion,
  Scorpion,
  Scout,
  Sculpture,
  Serf,
  Serpent,
  Servo,
  Shade,
  Shaman,
  Shapeshifter,
  Shark,
  Sheep,
  Siren,
  Skeleton,
  Slith,
  Sliver,
  Slug,
  Snake,
  Soldier,
  Soltari,
  Spawn,
  Specter,
  Spellshaper,
  Sphinx,
  Spider,
  Spike,
  Spirit,
  Splinter,
  Sponge,
  Squid,
  Squirrel,
  Starfish,
  Surrakar,
  Survivor,
  Tentacle,
  Tetravite,
  Thalakos,
  Thopter,
  Thrull,
  Treefolk,
  Trilobite,
  Triskelavite,
  Troll,
  Turtle,
  Unicorn,
  Vampire,
  Vedalken,
  Viashino,
  Volver,
  Wall,
  Warlock,
  Warrior,
  Weird,
  Werewolf,
  Whale,
  Wizard,
  Wolf,
  Wolverine,
  Wombat,
  Worm,
  Wraith,
  Wurm,
  Yeti,
  Zombie,
  Zubera,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[derive(Display, EnumString, AsRefStr, IntoStaticStr, EnumVariantNames)] // From strum
pub enum PlanarType {
  Alara,
  Arkhos,
  Azgol,
  Belenon,
  #[strum(serialize = "Bolas's Meditation Realm")]
  BolassMeditationRealm,
  Dominaria,
  Equilor,
  Ergamon,
  Fabacin,
  Innistrad,
  Iquatana,
  Ir,
  Kaldheim,
  Kamigawa,
  Karsus,
  Kephalai,
  Kinshala,
  Kolbahan,
  Kyneth,
  Lorwyn,
  Luvion,
  Mercadia,
  Mirrodin,
  Moag,
  Mongseng,
  Muraganda,
  #[strum(serialize = "New Phyrexia")]
  NewPhyrexia,
  Phyrexia,
  Pyrulea,
  Rabiah,
  Rath,
  Ravnica,
  Regatha,
  Segovia,
  #[strum(serialize = "Serra's Realm")]
  SerrasRealm,
  Shadowmoor,
  Shandalar,
  Ulgrotha,
  Valla,
  Vryn,
  Wildfire,
  Xerex,
  Zendikar,
}
