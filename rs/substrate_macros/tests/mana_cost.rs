use overseer_substrate_core::game::{ManaCostPip as Pip, *};
use overseer_substrate_macros::mana_cost;

#[test]
fn test_mana_cost_color() {
  const X: ManaCost = mana_cost!("{W}");
  let y = ManaCost::build().color(Color::W).seal();
  assert_eq!(X, y);
}

#[test]
fn test_mana_cost_simple() {
  const X: ManaCost = mana_cost!("{G/P}");
  assert_eq!(X.get_object_color(), ObjectColor::G);

  const Y: ManaCost = mana_cost!("{S}");
  assert_eq!(Y.get_object_color(), ObjectColor::NONE);

  assert_ne!(X, Y);
}

#[test]
fn test_mana_cost_complex() {
  const X: ManaCost = mana_cost!("{5}{C}{W}{U}{B}{R}{G}{X}{2/G}{R/W}{S}{G/P}");
  let y: ManaCost = ManaCost::from(vec![
    Pip::Generic(5),
    Pip::Colorless,
    Pip::Color(W),
    Pip::Color(U),
    Pip::Color(B),
    Pip::Color(R),
    Pip::Color(G),
    Pip::X,
    Pip::MonoHybrid(G),
    Pip::Hybrid(HybridColor::RW),
    Pip::Snow,
    Pip::Phyrexian(G),
  ]);
  assert_eq!(X, y);
  assert_eq!(X.mana_value(None), 16);
  assert_eq!(X.mana_value(Some(42)), 58);
  assert_eq!(X.get_object_color(), ObjectColor::WUBRG);
}
