mod draw;
pub use draw::*;
mod meta;
pub use meta::*;

use async_trait::async_trait;
use overseer_substrate_core::{game::Game, session::Session};

pub type EventList<S> = Option<Vec<Box<dyn Event<S>>>>;

#[async_trait]
pub trait Event<S: Session> {
  async fn apply(self, game: &mut Game) -> EventList<S>;
}

/*

613.1a Layer 1: Rules and effects that modify copiable values are applied.

613.1b Layer 2: Control-changing effects are applied.

613.1c Layer 3: Text-changing effects are applied. See rule 612, “Text-Changing Effects.”

613.1d Layer 4: Type-changing effects are applied. These include effects that change an object’s card type, subtype, and/or supertype.

613.1e Layer 5: Color-changing effects are applied.

613.1f Layer 6: Ability-adding effects, keyword counters, ability-removing effects, and effects that say an object can’t have an ability are applied.

613.1g Layer 7: Power- and/or toughness-changing effects are applied.

613.2. Within layer 1, apply effects in a series of sublayers in the order described below. Within each sublayer, apply effects in timestamp order (see rule 613.7). Note that dependency may alter the order in which effects are applied within a sublayer. (See rule 613.8.)

613.2a Layer 1a: Copiable effects are applied. This includes copy effects (see rule 706, “Copying Objects”) and changes to an object’s base characteristics determined by merging an object with a permanent (see rule 721, “Merging with Permanents”). “As . . . enters the battlefield” and “as . . . is turned face up” abilities generate copiable effects if they set power and toughness, even if they also define other characteristics.

613.2b Layer 1b: Face-down spells and permanents have their characteristics modified as defined in rule 707.2.

613.2c After all rules and effects in layer 1 have been applied, the object’s characteristics are its copiable values. (See rule 706.2.)

613.3. Within layers 2–6, apply effects from characteristic-defining abilities first (see rule 604.3), then all other effects in timestamp order (see rule 613.7). Note that dependency may alter the order in which effects are applied within a layer. (See rule 613.8.)

613.4. Within layer 7, apply effects in a series of sublayers in the order described below. Within each sublayer, apply effects in timestamp order. (See rule 613.7.) Note that dependency may alter the order in which effects are applied within a sublayer. (See rule 613.8.)

613.4a Layer 7a: Effects from characteristic-defining abilities that define power and/or toughness are applied. See rule 604.3.

613.4b Layer 7b: Effects that set power and/or toughness to a specific number or value are applied. Effects that refer to the base power and/or toughness of a creature apply in this layer.

613.4c Layer 7c: Effects and counters that modify power and/or toughness (but don’t set power and/or toughness to a specific number or value) are applied.

613.4d Layer 7d: Effects that switch a creature’s power and toughness are applied. Such effects take the value of power and apply it to the creature’s toughness, and take the value of toughness and apply it to the creature’s power.
Example: A 1/3 creature is given +0/+1 by an effect. Then another effect switches the creature’s power and toughness. Its new power and toughness is 4/1. A new effect gives the creature +5/+0. Its “unswitched” power and toughness would be 6/4, so its actual power and toughness is 4/6.
Example: A 1/3 creature is given +0/+1 by an effect. Then another effect switches the creature’s power and toughness. Its new power and toughness is 4/1. If the +0/+1 effect ends before the switch effect ends, the creature becomes 3/1.
Example: A 1/3 creature is given +0/+1 by an effect. Then another effect switches the creature’s power and toughness. Then another effect switches its power and toughness again. The two switches essentially cancel each other, and the creature becomes 1/4.
 */

pub enum Layer {
  OneA,
  OneB,
  Two,
  Three,
  Four,
  Five,
  Six,
  SevenA,
  SevenB,
  SevenC,
  SevenD,
}

pub struct CardHandle;

#[async_trait]
pub trait Effect<S: Session> {
  async fn get_layer(&self) -> Layer;

  async fn applies_to(&self, session: &S) -> CardHandle;

  async fn text(&self) -> String;
}

pub fn color_changing<S: Session>(text: String) -> impl Effect<S> {
  pub struct ColorChangingEffect {
    pub text: String,
  }

  #[async_trait]
  impl<S: Session> Effect<S> for ColorChangingEffect {
    async fn get_layer(&self) -> Layer {
      Layer::Two
    }

    async fn applies_to(&self, session: &S) -> CardHandle {
      todo!()
    }

    async fn text(&self) -> String {
      todo!()
    }
  }

  return ColorChangingEffect { text };
}

pub trait Ability<S: Session> {}

pub async fn foo<S: Session>(session: S, _: &Game) {
  let bar = color_changing::<S>("bar".to_string());

  let x = bar.applies_to(&session).await;

  let z: Box<dyn Effect<S>> = todo!();

  // bar.applies_to()
  todo!()
}
