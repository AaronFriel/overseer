use dyn_partial_eq::DynPartialEq;
use overseer_substrate::{
  action::*,
  game::*,
  interface::{DecisionHandle, YesNo},
};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use super::{Draw, MulliganDiscard, SetupLibrary, ShuffleHandIntoLibrary};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize)]
pub struct StartGame {
  setup_libraries: Option<PlayerVec<ActionThunk<SetupLibrary>>>,
  roll_for_first_player: Option<ActionThunk<RollForFirstPlayer>>,
  accept_first_player: Option<ActionThunk<AcceptFirstPlayer>>,
  draw_hands: Option<PlayerVec<ActionThunk<SetupLibrary>>>,
  mulligans: Option<ActionThunk<TakeMulligans>>,
  accepted_hands: PlayerVec<PlayerHandle>,
  discard_to: usize,
  // perform_mulligans: ActionThunk<PlayerHandle, AcceptFirstPlayer>,
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize)]
enum StartGameState {
  Roll(RollForFirstPlayer),
}

impl StartGame {
  pub fn new(game: &mut Game) -> StartGame {
    Self {
      setup_libraries: None,
      roll_for_first_player: None,
      accept_first_player: None,
      draw_hands: None,
      mulligans: None,
      accepted_hands: SmallVec::new(),
      discard_to: 7,
      // perform_mulligans: todo!(),
    }
  }
}

#[typetag::serde]
impl SimpleAction for StartGame {
  fn perform(&mut self, game: &mut Game) -> ActionResult<()> {
    perform(self, game)
  }
}

fn perform(this: &mut StartGame, game: &mut Game) -> ActionResult<()> {
  use ActionError::*;

  let setup_library = this.setup_libraries.get_or_insert_with(|| {
    game
      .client
      .get_players()
      .map(|player| ActionThunk::Action(SetupLibrary::new(player, game)))
      .collect()
  });

  for setup in setup_library {
    setup.perform(game)?;
  }

  let first_player = this
    .roll_for_first_player
    .get_or_insert_with(|| ActionThunk::Action(RollForFirstPlayer::new(game)))
    .apply(game)?;

  let first_player = this
    .accept_first_player
    .get_or_insert_with(|| ActionThunk::Action(AcceptFirstPlayer::new(game, first_player)))
    .apply(game)?;

  let draw_hands = this.draw_hands.get_or_insert_with(|| {
    game
      .client
      .get_players_from(first_player)
      .map(|player| ActionThunk::Action(SetupLibrary::new(player, game)))
      .collect()
  });

  for draw in draw_hands {
    draw.perform(game)?;
  }

  if let Some(ref mut mulligans) = this.mulligans {
    let result = mulligans.apply_borrowed(game)?;
    this.accepted_hands.extend(result.accepted_hands.clone());
  }

  let eligible_mulligans: PlayerVec<_> = game
    .client
    .get_players_from(first_player)
    .filter(|player| !this.accepted_hands.contains(player))
    .collect();

  if eligible_mulligans.len() > 0 {
    this.mulligans = Some(TakeMulligans::new(game, eligible_mulligans, this.discard_to).into());

    return Err(Step);
  }

  Ok(())
  // match &mut *self {
  //   (DeclarePlayers(ref mut players), _) => {
  //     println!("Adding players");
  //     game.players.append(players);

  //     let player_handles = game.get_player_handles();

  //     let shuffle_actions: ActionList = player_handles
  //       .map(|player| (ShuffleDeckIntoLibrary { player }).into())
  //       .collect();

  //     *self = DiceRoll;
  //     todo!()
  //     // DoMulti(shuffle_actions)
  //   }
  //   (DiceRoll, PromptResult::None) => {
  //     // let choices = game
  //     //   .get_player_handles()
  //     //   .map(|x| ChoiceOption::Player(x))
  //     //   .collect();

  //     todo!("Refactor prompt as complex action")
  //     // Prompt(ChoicePrompt {
  //     //   player: None,
  //     //   kind: PromptKind::Shuffle,
  //     //   visibility: Visibility::Players(Vec::new()),
  //     //   prompt: "Which player will go first?".into(),
  //     //   choices,
  //     // })
  //   }
  //   (DiceRoll, PromptResult::Ordered(rolls)) => {
  //     let choices: Vec<_> = game
  //       .get_player_handles()
  //       .map(|x| ChoiceOption::Player(x))
  //       .collect();

  //     let choice = rolls.get(0).and_then(|x| choices.get(*x));

  //     if let Some(ChoiceOption::Player(player_handle)) = choice {
  //       game.set_active_player(*player_handle);
  //       *self = DrawHands {
  //         discard_to: 6,
  //         players_drawing: game.get_player_handles().collect(),
  //       };
  //       Step
  //     } else {
  //       todo!()
  //     }
  //   }
  //   (
  //     DrawHands {
  //       ref discard_to,
  //       ref mut players_drawing,
  //     },
  //     PromptResult::None,
  //   ) => {
  //     if let Some(player) = players_drawing.first() {
  //       let draw_actions: ActionList = players_drawing
  //         .iter()
  //         .map(|player| {
  //           Draw::new(*player, 7).into()
  //         })
  //         .collect();

  //       *self = DeclareMulligans {
  //         discard_to: *discard_to,
  //         current_player: *player,
  //         players_declaring_mulligans: std::mem::take(players_drawing),
  //       };
  //       todo!()
  //       // DoMulti(draw_actions)
  //     } else {
  //       Resolved(())
  //     }
  //   }
  //   // (DeclareMulligans { current_player, .. }, PromptResult::None) =>
  // Prompt(ChoicePrompt {   //   player Some(*current_player),
  //   //   kind: PromptKind::Select,
  //   //   visibility: Visibility::Players(Vec::new()),
  //   //   prompt: "Would you like to take a mulligan?".into(),
  //   //   choices: CHOICE_YES_NO.into(),
  //   // }),
  //   (
  //     DeclareMulligans {
  //       current_player,
  //       discard_to,
  //       players_declaring_mulligans,
  //     },
  //     PromptResult::Selected(choice),
  //   ) => {
  //     let next_player = players_declaring_mulligans
  //       .iter()
  //       .skip_while(|x| **x != *current_player)
  //       .skip(1)
  //       .cloned()
  //       .next();

  //     if choice == 1 {
  //       players_declaring_mulligans.retain(|x| x != current_player);
  //     }

  //     if let Some(next_player) = next_player {
  //       *self = DeclareMulligans {
  //         discard_to: *discard_to,
  //         current_player: next_player,
  //         players_declaring_mulligans:
  // std::mem::take(players_declaring_mulligans),       };

  //       Step
  //     } else if players_declaring_mulligans.len() == 0 || *discard_to == 0
  // {       Resolved(())
  //     } else if let Some(next_current_player) =
  // players_declaring_mulligans.first() {       let take_mulligan_actions
  // = players_declaring_mulligans         .iter()
  //         .map(|player| {
  //           vec![
  //             MulliganDiscard {
  //               discard_to: *discard_to,
  //               player: *player,
  //             }
  //             .into(),
  //             Draw::new(*player, 7)
  //             .into(),
  //             ShuffleHandIntoLibrary { player: *player }.into(),
  //           ]
  //         })
  //         .flatten()
  //         .collect();

  //       *self = DeclareMulligans {
  //         current_player: *next_current_player,
  //         discard_to: *discard_to - 1,
  //         players_declaring_mulligans:
  // std::mem::take(players_declaring_mulligans),       };

  //       todo!()
  //       // DoMulti(take_mulligan_actions)
  //     } else {
  //       Resolved(())
  //     }
  //   }
  //   uhoh => {
  //     unimplemented!("Error handling. Oops: {:?}", uhoh);
  //   }
  // }
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize)]
struct RollForFirstPlayer {
  decision: DecisionHandle,
}

impl RollForFirstPlayer {
  fn new(game: &mut Game) -> Self {
    Self {
      decision: game.reserve_decision(),
    }
  }
}

impl ComplexAction for RollForFirstPlayer {
  type Result = PlayerHandle;

  fn apply(&mut self, game: &mut Game) -> ActionResult<Self::Result> {
    use ActionError::*;

    if let Ok(x) = game.wrap_decision_public(
      self.decision,
      "Who wins the die roll?",
      |state, _| perform_determine_first(state),
      |state, decision| state.set_active_player(decision.copied()),
    ) {
      Ok(x.copied())
    } else {
      Err(Waiting)
    }
  }
}

#[cfg(feature = "server")]
fn perform_determine_first(state: &ClientState) -> PlayerHandle {
  use rand::{prelude::SliceRandom, rngs::OsRng};

  let mut players: Vec<_> = state.get_players().collect();

  players.shuffle(&mut OsRng);
  players.pop().unwrap()
}

#[cfg(not(feature = "server"))]
fn perform_determine_first(state: &ClientState) -> Option<PlayerHandle> {
  unimplemented!()
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize)]
struct AcceptFirstPlayer {
  first_player: PlayerHandle,
  current_player: PlayerHandle,
  decision: DecisionHandle,
}

impl AcceptFirstPlayer {
  fn new(game: &mut Game, first_player: PlayerHandle) -> Self {
    Self {
      first_player,
      current_player: first_player,
      decision: game.reserve_decision(),
    }
  }
}

impl ComplexAction for AcceptFirstPlayer {
  type Result = PlayerHandle;

  fn apply(&mut self, game: &mut Game) -> ActionResult<Self::Result> {
    use ActionError::*;
    use YesNo::*;

    let first_player = self.first_player;
    let current_player = self.current_player;
    let next_player = game.state().get_next_player_after(current_player);

    if next_player == first_player {
      return Ok(self.current_player);
    }

    let accepted = game
      .prompt_yes_no(
        self.decision,
        "You've won the die roll, do you want to go first?",
        current_player,
        |_, _| {},
      )
      .map_err(|_| Waiting)?
      .copied()
      == Yes;

    if !accepted {
      self.current_player = next_player;
      self.decision = game.reserve_decision();

      return Err(Step);
    }

    game.client.set_active_player(self.current_player);

    Ok(self.current_player)
  }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize)]
struct DeclareMulligans {
  pub take_mulligan_prompts: PlayerVec<(PlayerHandle, DecisionHandle)>,
  pub accepted_hands: PlayerVec<PlayerHandle>,
  pub declared_mulligans: PlayerVec<PlayerHandle>,
}

impl DeclareMulligans {
  fn new(game: &mut Game, players: impl IntoIterator<Item = PlayerHandle>) -> Self {
    Self {
      take_mulligan_prompts: players
        .into_iter()
        .map(|player_handle| (player_handle, game.reserve_decision()))
        .collect(),
      accepted_hands: SmallVec::new(),
      declared_mulligans: SmallVec::new(),
    }
  }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize)]
struct DeclareMulliganResult {
  pub accepted_hands: PlayerVec<PlayerHandle>,
  pub declared_mulligans: PlayerVec<PlayerHandle>,
}

impl ComplexAction for DeclareMulligans {
  type Result = DeclareMulliganResult;

  fn apply(&mut self, game: &mut Game) -> ActionResult<Self::Result> {
    use YesNo::*;

    for (player, decision) in self.take_mulligan_prompts.iter() {
      let decision = game
        .prompt_yes_no(
          *decision,
          "Would you like to take a mulligan?",
          *player,
          |_, _| {},
        )
        .map_err(|_| ActionError::Waiting)?;

      match decision.copied() {
        Yes => self.declared_mulligans.push(*player),
        No => self.accepted_hands.push(*player),
      }
    }

    Ok(DeclareMulliganResult {
      accepted_hands: self.accepted_hands.clone(),
      declared_mulligans: self.declared_mulligans.clone(),
    })
  }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize)]
struct TakeMulligans {
  pub eligible_players: PlayerVec<PlayerHandle>,
  pub discard_to: usize,
  pub declared_mulligans: ActionThunk<DeclareMulligans>,
  pub shuffles: Option<Vec<ActionThunk<ShuffleHandIntoLibrary>>>,
  pub draws: Option<Vec<ActionThunk<Draw>>>,
  pub discards: Option<Vec<ActionThunk<MulliganDiscard>>>,
}

impl TakeMulligans {
  fn new(game: &mut Game, players: PlayerVec<PlayerHandle>, discard_to: usize) -> Self {
    Self {
      eligible_players: players.clone(),
      discard_to,
      declared_mulligans: ActionThunk::Action(DeclareMulligans::new(game, players)),
      shuffles: None,
      draws: None,
      discards: None,
    }
  }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize)]
struct TakeMulliganResult {
  pub accepted_hands: PlayerVec<PlayerHandle>,
}

impl ComplexAction for TakeMulligans {
  type Result = TakeMulliganResult;

  fn apply(&mut self, game: &mut Game) -> ActionResult<Self::Result> {
    let DeclareMulliganResult {
      accepted_hands,
      declared_mulligans,
    } = self.declared_mulligans.apply_borrowed(game)?;

    let shuffles = self.shuffles.get_or_insert_with(|| {
      declared_mulligans
        .iter()
        .map(|player| ActionThunk::Action(ShuffleHandIntoLibrary::new(game, *player)))
        .collect()
    });

    for shuffle in shuffles {
      shuffle.perform(game)?;
    }

    let draws = self.draws.get_or_insert_with(|| {
      declared_mulligans
        .iter()
        .map(|player| ActionThunk::Action(Draw::new(game, *player, 7)))
        .collect()
    });

    for draw in draws {
      draw.perform(game)?;
    }

    let discard_to = self.discard_to;
    let discards = self.discards.get_or_insert_with(|| {
      declared_mulligans
        .iter()
        .map(|player| ActionThunk::Action(MulliganDiscard::new(game, *player, discard_to)))
        .collect()
    });

    for discard in discards {
      discard.perform(game)?;
    }

    Ok(TakeMulliganResult {
      accepted_hands: accepted_hands.clone(),
    })
  }
}
