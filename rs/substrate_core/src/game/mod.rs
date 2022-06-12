mod card;
mod cardtype;
mod characteristics;
mod client_state;
mod color;
mod game;
mod mana_cost;
mod object;
mod object_color;
mod player;
mod server_state;
mod target;
mod zone;

use smallvec::SmallVec;

pub use self::{
  card::*, cardtype::*, characteristics::*, client_state::*, color::*, game::*, mana_cost::*,
  object::*, object_color::*, player::*, server_state::*, target::*, zone::*,
};

/**
 * A number of players sized small vector.
 */
pub type PlayerVec<T> = SmallVec<[T; 4]>;

/**
 * A number of cards in hand sized small vector.
 */
pub type HandVec<T> = SmallVec<[T; 10]>;
