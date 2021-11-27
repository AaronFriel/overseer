use dyn_partial_eq::DynPartialEq;
use overseer_substrate::{
  action::*,
  game::*,
  interface::{DecisionHandle, YesNo},
};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize)]
pub struct StartGame {
  roll_for_first_player: ActionThunk<PlayerHandle, DetermineFirstPlayer>,
  accept_first_player: Option<ActionThunk<PlayerHandle, AcceptFirstPlayer>>,
  // perform_mulligans: ActionThunk<PlayerHandle, AcceptFirstPlayer>,
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize)]
enum StartGameState {
  Roll(DetermineFirstPlayer),
}

impl StartGame {
  pub fn new(game: &mut Game) -> StartGame {
    Self {
      roll_for_first_player: ActionThunk::Action(DetermineFirstPlayer::new(game)),
      accept_first_player: None,
      // perform_mulligans: todo!(),
    }
  }
}

#[typetag::serde]
impl SimpleAction for StartGame {
  fn perform(&mut self, game: &mut Game) -> ActionResult<()> {
    use ActionErr::*;

    let first_player = self.roll_for_first_player.apply(game)?;

    game.state_mut().set_active_player(first_player);

    let first_player = self
      .accept_first_player
      .get_or_insert_with(|| AcceptFirstPlayer::new(game, first_player).into())
      .apply(game)?;

    game.state_mut().set_active_player(first_player);

    // let first_player = self.accept_first_player.apply(game)?;

    // replace_with_or_abort_and_return(self, |this| match this.state {
    //   StartGameState::Roll(action) => {
    //     let player = action.perform(game);
    //     todo!()
    //   }
    //   _ => (Step, this),
    // })
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
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize)]
struct DetermineFirstPlayer {
  decision: DecisionHandle,
}

impl DetermineFirstPlayer {
  fn new(game: &mut Game) -> Self {
    Self {
      decision: game.reserve_decision(),
    }
  }
}

impl ComplexAction<PlayerHandle> for DetermineFirstPlayer {
  fn apply(&mut self, game: &mut Game) -> ActionResult<PlayerHandle> {
    use ActionErr::*;

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

impl ComplexAction<PlayerHandle> for AcceptFirstPlayer {
  fn apply(&mut self, game: &mut Game) -> ActionResult<PlayerHandle> {
    use ActionErr::*;
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
        format!(
          "Player {}, {}, you've won the die roll, do you want to go first?",
          self.current_player,
          game.state().get_player(current_player).name
        ),
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

    Ok(self.current_player)
  }
}
