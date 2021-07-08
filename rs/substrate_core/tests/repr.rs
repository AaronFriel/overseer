use std::{borrow::Cow, convert::TryFrom};

use insta::{assert_display_snapshot, assert_snapshot, assert_yaml_snapshot};
use overseer_substrate_core::game::{
  Card, CardSubtype, CreatureType, CustomSubtype, Game, ManaCost, ManaCostPip, ObjectColor, Player,
  PredefinedSubtype, RegisteredCard, TypeLine, Zone,
};

#[test]
fn zone_repr() {
  assert_yaml_snapshot!(Zone::new_battlefield(), @r###"
  ---
  cards: []
  count: 0
  "###);
}

fn make_player() -> Player<'static> {
  Player::new("Overseer", vec![], vec![])
}

#[test]
fn player_repr() {
  assert_yaml_snapshot!(make_player(), @r###"
  ---
  name: Overseer
  handle: ~
  controller: ~
  deck: []
  sideboard: []
  library:
    cards: []
    count: 0
  hand:
    cards: []
    count: 0
  graveyard:
    cards: []
    count: 0
  life: 20
  has_left_game: false
  has_lost_game: false
  "###);
}

#[test]
fn player_handle_repr() {
  let game = Game::new(vec![], vec![make_player()]);

  assert_yaml_snapshot!(game.active_player, @r###"
  ---
  0
  "###)
}

#[test]
fn game_repr() {
  let game: Game<'static> = Game::new(vec![], vec![make_player()]);

  assert_yaml_snapshot!(game, @r###"
  ---
  cards: []
  players:
    - name: Overseer
      handle: ~
      controller: ~
      deck: []
      sideboard: []
      library:
        cards: []
        count: 0
      hand:
        cards: []
        count: 0
      graveyard:
        cards: []
        count: 0
      life: 20
      has_left_game: false
      has_lost_game: false
  active_player: 0
  log: []
  current_decision: 0
  decisions: []
  "###)
}
/*
#[test]
fn game_diff_repr() {
  let game: Game<'static> = Game::new(vec![], vec![make_player(), make_player()]);

  let mut game_two: Game<'static> = game.clone();
  game_two.set_active_player(game_two.get_players().last().unwrap());
  game_two.get_player_mut(game_two.active_player).life = 30;

  let diff = serde_diff::Diff::serializable(&game, &&game_two);

  assert_yaml_snapshot!(diff, @r###"
  ---
  - Enter:
      Field: players
  - Enter:
      CollectionIndex: 1
  - Enter:
      Field: life
  - Value: 30
  - Exit
  - Exit
  - Enter:
      Field: active_player
  - Value: 1
  "###)
}
 */
#[test]
fn color_repr() {
  let color = ObjectColor::WU;

  assert_display_snapshot!(color, @"WU")
}

#[test]
fn color_roundtrip() {
  let colors = vec![
    ObjectColor::NONE,
    ObjectColor::W,
    ObjectColor::U,
    ObjectColor::B,
    ObjectColor::R,
    ObjectColor::G,
    ObjectColor::WU,
    ObjectColor::UB,
    ObjectColor::BR,
    ObjectColor::RG,
    ObjectColor::GW,
    ObjectColor::WB,
    ObjectColor::UR,
    ObjectColor::BG,
    ObjectColor::RW,
    ObjectColor::GU,
    ObjectColor::WUB,
    ObjectColor::UBR,
    ObjectColor::BRG,
    ObjectColor::RGW,
    ObjectColor::GWU,
    ObjectColor::WBG,
    ObjectColor::URW,
    ObjectColor::BGU,
    ObjectColor::RWB,
    ObjectColor::GUR,
    ObjectColor::WUBR,
    ObjectColor::UBRG,
    ObjectColor::BRGW,
    ObjectColor::RGWU,
    ObjectColor::GWUB,
    ObjectColor::WUBRG,
  ];

  let before_list = colors
    .iter()
    .map(|x| x.as_str())
    .collect::<Vec<&str>>()
    .join(", ");

  assert_snapshot!(before_list, @"C, W, U, B, R, G, WU, UB, BR, RG, GW, WB, UR, BG, RW, GU, WUB, UBR, BRG, RGW, GWU, WBG, URW, BGU, RWB, GUR, WUBR, UBRG, BRGW, RGWU, GWUB, WUBRG");

  let serialized = serde_json::to_string(&colors).unwrap();
  let deserialized: Vec<ObjectColor> = serde_json::from_str(&serialized).unwrap();

  let after_list = deserialized
    .iter()
    .map(|x| x.as_str())
    .collect::<Vec<&str>>()
    .join(", ");

  assert_snapshot!(&after_list, @"C, W, U, B, R, G, WU, UB, BR, RG, GW, WB, UR, BG, RW, GU, WUB, UBR, BRG, RGW, GWU, WBG, URW, BGU, RWB, GUR, WUBR, UBRG, BRGW, RGWU, GWUB, WUBRG");

  assert_eq!(before_list, after_list);

  assert_yaml_snapshot!(colors, @r###"
  ---
  - C
  - W
  - U
  - B
  - R
  - G
  - WU
  - UB
  - BR
  - RG
  - GW
  - WB
  - UR
  - BG
  - RW
  - GU
  - WUB
  - UBR
  - BRG
  - RGW
  - GWU
  - WBG
  - URW
  - BGU
  - RWB
  - GUR
  - WUBR
  - UBRG
  - BRGW
  - RGWU
  - GWUB
  - WUBRG
  "###);
}

#[test]
fn mana_cost_pip_roundtrip() {
  let colors = vec![
    "{C}", "{W}", "{U}", "{B}", "{R}",
    "{G}",
    /* ObjectColor::WU,
     * ObjectColor::UB,
     * ObjectColor::BR,
     * ObjectColor::RG,
     * ObjectColor::GW,
     * ObjectColor::WB,
     * ObjectColor::UR,
     * ObjectColor::BG,
     * ObjectColor::RW,
     * ObjectColor::GU,
     * ObjectColor::WUB,
     * ObjectColor::UBR,
     * ObjectColor::BRG,
     * ObjectColor::RGW,
     * ObjectColor::GWU,
     * ObjectColor::WBG,
     * ObjectColor::URW,
     * ObjectColor::BGU,
     * ObjectColor::RWB,
     * ObjectColor::GUR,
     * ObjectColor::WUBR,
     * ObjectColor::UBRG,
     * ObjectColor::BRGW,
     * ObjectColor::RGWU,
     * ObjectColor::GWUB,
     * ObjectColor::WUBRG, */
  ];

  let (before_list, errors): (Vec<_>, Vec<_>) = colors
    .iter()
    .map(|x| ManaCostPip::try_from(*x))
    .partition(Result::is_ok);
  assert_eq!(errors, Vec::new());
  let before_list: Vec<_> = before_list.into_iter().map(Result::unwrap).collect();

  assert_yaml_snapshot!(before_list, @r###"
  ---
  - "{C}"
  - "{W}"
  - "{U}"
  - "{B}"
  - "{R}"
  - "{G}"
  "###)
}

#[test]
fn mana_cost_roundtrip() {
  let colors = vec![
    "{C}",
    "{C}{W}",
    "{W/U}{R}",
    "{120}{C}{W}{S}{R/P}{R/G}{2/G}",
    "{42}",
    "{C}",
    "{G}{R}{B}{U}{W}",
    "{W/U}{W/B}{U/B}{U/R}{B/R}{B/G}{R/G}{R/W}{G/W}{G/U}",
    "{2/W}{2/U}{2/B}{2/R}{2/G}",
    "{W/P}{U/P}{B/P}{R/P}{G/P}",
    "{S}",
    "{X}",
  ];

  let (before_list, errors): (Vec<_>, Vec<_>) = colors
    .iter()
    .map(|x| ManaCost::try_from(*x))
    .partition(Result::is_ok);
  assert_eq!(errors, Vec::new());
  let before_list: Vec<_> = before_list.into_iter().map(Result::unwrap).collect();

  assert_yaml_snapshot!(before_list, @r###"
  ---
  - "{C}"
  - "{C}{W}"
  - "{W/U}{R}"
  - "{120}{C}{W}{S}{R/P}{R/G}{2/G}"
  - "{42}"
  - "{C}"
  - "{G}{R}{B}{U}{W}"
  - "{W/U}{W/B}{U/B}{U/R}{B/R}{B/G}{R/G}{R/W}{G/W}{G/U}"
  - "{2/W}{2/U}{2/B}{2/R}{2/G}"
  - "{W/P}{U/P}{B/P}{R/P}{G/P}"
  - "{S}"
  - "{X}"
  "###);

  let serialized = serde_json::to_string(&before_list).unwrap();
  let deserialized: Vec<ManaCost> = serde_json::from_str(&serialized).unwrap();

  assert_yaml_snapshot!(deserialized, @r###"
  ---
  - "{C}"
  - "{C}{W}"
  - "{W/U}{R}"
  - "{120}{C}{W}{S}{R/P}{R/G}{2/G}"
  - "{42}"
  - "{C}"
  - "{G}{R}{B}{U}{W}"
  - "{W/U}{W/B}{U/B}{U/R}{B/R}{B/G}{R/G}{R/W}{G/W}{G/U}"
  - "{2/W}{2/U}{2/B}{2/R}{2/G}"
  - "{W/P}{U/P}{B/P}{R/P}{G/P}"
  - "{S}"
  - "{X}"
  "###);
}

#[test]
fn subtype_repr() {
  let subtype = CardSubtype::Predefined(PredefinedSubtype::Creature(CreatureType::Ape));

  assert_yaml_snapshot!(subtype, @r###"
  ---
  Ape
  "###);
}

#[test]
fn subtype_repr_custom() {
  let subtype = CardSubtype::Custom(CustomSubtype {
    parent_type: overseer_substrate_core::game::SubtypeKind::ArtifactType,
    value: Cow::Borrowed("foo"),
  });

  assert_yaml_snapshot!(subtype, @r###"
  ---
  parent_type: ArtifactType
  value: foo
  "###);
}

#[test]
fn registered_card_repr() {
  const METALLIC_SLIVER: &'static Card = &Card {
    name: Cow::Borrowed("Metallic Sliver"),
    mana_cost: ManaCost::NONE,
    type_line: TypeLine::const_default(),
    power: 1,
    toughness: 1,
    loyalty: 0,
    color_indicator: ObjectColor::NONE,
    rules_text: Cow::Borrowed(""),
    #[cfg(feature = "vanguard")]
    hand_modifier: 0,
    #[cfg(feature = "vanguard")]
    life_modifier: 0,
  };

  let metallic_sliver = RegisteredCard::register(METALLIC_SLIVER);

  assert_yaml_snapshot!(metallic_sliver, @r###"
  ---
  Registered: Metallic Sliver
  "###);
}
