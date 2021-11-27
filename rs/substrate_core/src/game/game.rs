use std::cell::{Ref, RefCell};

use serde::{Deserialize, Serialize};

use crate::{
  game::{ClientState, PlayerHandle, ServerState},
  interface::{Decision, DecisionHandle, DecisionList, InterfaceResult, UserInterface, YesNo},
};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Mode {
  Server,
  Client(PlayerHandle),
  Local,
}

pub struct Game {
  pub client: ClientState,
  pub server: ServerState,
  pub mode: Mode,
  pub interface: Box<dyn UserInterface>,
}

pub enum ServerResult<T> {
  Public(T),
  Private(T, Vec<T>),
}

impl Game {
  pub fn new<I>(client: ClientState, server: ServerState, mode: Mode, interface: I) -> Self
  where
    I: UserInterface + 'static,
  {
    Self {
      client,
      server,
      mode,
      interface: Box::new(interface),
    }
  }

  /// Get a reference to the game state's game.
  pub fn state(&self) -> &ClientState {
    &self.client
  }

  /// Get a mutable reference to the game's state.
  pub fn state_mut(&mut self) -> &mut ClientState {
    &mut self.client
  }

  /// Get a reference to the game state's decisions.
  pub fn decisions(&self) -> &DecisionList {
    &self.server.decisions
  }

  pub fn reserve_decision(&mut self) -> DecisionHandle {
    self.server.decisions.reserve()
  }

  pub fn is_server(&self) -> bool {
    match self.mode {
      Mode::Server => true,
      Mode::Client(_) => false,
      Mode::Local => true,
    }
  }

  pub fn is_player(&self, player: PlayerHandle) -> bool {
    match self.mode {
      Mode::Server => false,
      Mode::Client(client_player) => client_player == player,
      Mode::Local => true,
    }
  }

  pub fn server_apply<T>(
    &mut self,
    decision_handle: DecisionHandle,
    question: impl ToString,
    mut perform: impl FnMut(&mut ClientState, &mut ServerState) -> ServerResult<T>,
  ) -> InterfaceResult<DecisionEntry<T>>
  where
    T: Serialize,
    for<'de> T: Deserialize<'de>,
  {
    if self.server.decisions.contains(decision_handle) {
      if let Some(decision) = self.server.decisions.get_mut(decision_handle) {
        let value = get_decision_value(decision, self.mode);
        return Ok(DecisionEntry::new(decision, self.mode, value));
      } else {
        unreachable!()
      }
    } else {
      cfg_if::cfg_if! {
      if #[cfg(feature="server")] {
        let result = perform(&mut self.client, &mut self.server);
        let decision = match result {
          ServerResult::Private(server_value, player_values) => self.server.decisions.set_decision(
            decision_handle,
            question,
            &server_value,
            &player_values,
          ),
          ServerResult::Public(value) => {
            self
              .server
              .decisions
              .set_decision_public(decision_handle, question, &value)
          }
        };
        let value = get_decision_value(decision, self.mode);
        Ok(DecisionEntry::new(decision, self.mode, value))
        } else {
          use crate::interface::InterfaceError::*;
          Err(Waiting)
        }
      }
    }
  }

  pub fn wrap_decision<T>(
    &mut self,
    decision_handle: DecisionHandle,
    question: impl ToString,
    mut perform: impl FnMut(&ClientState, &mut ServerState) -> (T, Vec<T>),
    mut apply: impl FnMut(&mut ClientState, &mut DecisionEntry<T>),
  ) -> InterfaceResult<DecisionEntry<T>>
  where
    T: Serialize,
    for<'de> T: Deserialize<'de>,
  {
    if self.server.decisions.contains(decision_handle) {
      if let Some(decision) = self.server.decisions.get_mut(decision_handle) {
        let value = get_decision_value(decision, self.mode);
        let mut decision_entry = DecisionEntry::new(decision, self.mode, value);
        if !decision_entry.applied() {
          apply(&mut self.client, &mut decision_entry);
          decision_entry.set_applied()
        }
        Ok(decision_entry)
      } else {
        unreachable!()
      }
    } else {
      cfg_if::cfg_if! {
        if #[cfg(feature="server")] {
          let (server_value, player_values) = perform(&self.client, &mut self.server);
          let decision = self
            .server
            .decisions
            .set_decision(decision_handle, question, &server_value, &player_values);
          let value = get_decision_value(decision, self.mode);
          let mut decision_entry = DecisionEntry::new(decision, self.mode, value);
          apply(&mut self.client, &mut decision_entry);
          decision_entry.set_applied();
          Ok(decision_entry)
        } else {
          use crate::interface::InterfaceError::*;
          Err(Waiting)
        }
      }
    }
  }

  pub fn wrap_decision_public<T>(
    &mut self,
    decision_handle: DecisionHandle,
    question: impl ToString,
    prepare: impl Fn(&ClientState, &mut ServerState) -> T,
    apply: impl Fn(&mut ClientState, &mut DecisionEntry<T>),
  ) -> InterfaceResult<DecisionEntry<T>>
  where
    T: Serialize,
    for<'de> T: Deserialize<'de>,
  {
    if self.server.decisions.contains(decision_handle) {
      if let Some(decision) = self.server.decisions.get_mut(decision_handle) {
        let value = decision.get_server_decision().unwrap();
        let mut decision_entry = DecisionEntry::new(decision, self.mode, value);
        if !decision_entry.applied() {
          apply(&mut self.client, &mut decision_entry);
          decision_entry.set_applied()
        }
        Ok(decision_entry)
      } else {
        unreachable!()
      }
    } else {
      cfg_if::cfg_if! {
        if #[cfg(feature="server")] {
          let value = prepare(&self.client, &mut self.server);
          let decision = self.server.decisions.set_decision_public(decision_handle, question, &value);
          let mut decision_entry = DecisionEntry::new(decision, self.mode, value);
          apply(&mut self.client, &mut decision_entry);
          decision_entry.set_applied();
          Ok(decision_entry)
        } else {
          use crate::interface::InterfaceError::*;
          Err(Waiting)
        }
      }
    }
  }

  pub fn prompt_yes_no(
    &mut self,
    decision_handle: DecisionHandle,
    question: impl ToString,
    player: PlayerHandle,
    apply: impl Fn(&mut ClientState, &mut DecisionEntry<YesNo>),
  ) -> InterfaceResult<DecisionEntry<YesNo>> {
    let prompt = question.to_string();

    if self.server.decisions.contains(decision_handle) {
      if let Some(decision) = self.server.decisions.get_mut(decision_handle) {
        let value = decision.get_server_decision().unwrap();
        let mut decision_entry = DecisionEntry::new(decision, self.mode, value);
        if !decision_entry.applied() {
          apply(&mut self.client, &mut decision_entry);
          decision_entry.set_applied()
        }
        Ok(decision_entry)
      } else {
        unreachable!()
      }
    } else {
      match self.interface.prompt_yes_no(&self.client, player, &prompt) {
        Ok(value) => {
          let decision = self
            .server
            .decisions
            .set_decision_public(decision_handle, prompt, &value);
          let mut decision_entry = DecisionEntry::new(decision, self.mode, value);
          apply(&mut self.client, &mut decision_entry);
          decision_entry.set_applied();
          Ok(decision_entry)
        }
        Err(e) => Err(e),
      }
    }
  }

  pub fn wrap_prompt_public<T>(
    &mut self,
    decision_handle: DecisionHandle,
    question: impl ToString,
    prepare: impl Fn(&ClientState, &mut dyn UserInterface) -> InterfaceResult<T>,
    apply: impl Fn(&mut ClientState, &mut DecisionEntry<T>),
  ) -> InterfaceResult<DecisionEntry<T>>
  where
    T: Serialize,
    for<'de> T: Deserialize<'de>,
  {
    if self.server.decisions.contains(decision_handle) {
      if let Some(decision) = self.server.decisions.get_mut(decision_handle) {
        let value = decision.get_server_decision().unwrap();
        let mut decision_entry = DecisionEntry::new(decision, self.mode, value);
        if !decision_entry.applied() {
          apply(&mut self.client, &mut decision_entry);
          decision_entry.set_applied()
        }
        Ok(decision_entry)
      } else {
        unreachable!()
      }
    } else {
      match prepare(&mut self.client, &mut *self.interface) {
        Ok(value) => {
          let decision =
            self
              .server
              .decisions
              .set_decision_public(decision_handle, question, &value);
          let mut decision_entry = DecisionEntry::new(decision, self.mode, value);
          apply(&mut self.client, &mut decision_entry);
          decision_entry.set_applied();
          Ok(decision_entry)
        }
        Err(e) => Err(e),
      }
    }
  }
}

fn get_decision_value<T>(decision: &Decision, mode: Mode) -> T
where
  T: Serialize,
  for<'de> T: Deserialize<'de>,
{
  let value = match mode {
    Mode::Client(player_handle) => decision.get_player_decision(&player_handle),
    Mode::Server | Mode::Local => decision.get_server_decision(),
  };
  let value: T = value.unwrap();
  value
}

pub struct DecisionEntry<'a, T> {
  pub(crate) decision: &'a mut Decision,
  mode: Mode,
  value: RefCell<Option<T>>,
}

impl<'a, T> DecisionEntry<'a, T>
where
  T: Serialize,
  for<'de> T: Deserialize<'de>,
{
  pub(crate) fn new(decision: &'a mut Decision, mode: Mode, value: T) -> Self {
    Self {
      decision,
      mode,
      value: RefCell::new(Some(value)),
    }
  }

  pub fn decision(&self) -> &Decision {
    return self.decision;
  }

  pub(crate) fn applied(&mut self) -> bool {
    self.decision.applied()
  }

  pub(crate) fn set_applied(&mut self) {
    self.decision.set_applied()
  }

  pub fn take(&self) -> T {
    match self.value.replace(None) {
      Some(value) => value,
      None => get_decision_value(self.decision, self.mode),
    }
  }

  pub fn borrow(&self) -> Ref<T> {
    let borrowed = self.value.borrow();

    if borrowed.is_none() {
      std::mem::drop(borrowed);

      self
        .value
        .replace(get_decision_value(self.decision, self.mode));

      Ref::map(self.value.borrow(), |opt| opt.as_ref().unwrap())
    } else {
      Ref::map(borrowed, |opt| opt.as_ref().unwrap())
    }
  }

  pub fn apply_borrowed(&mut self, f: impl Fn(Ref<T>) -> ()) {
    f(self.borrow());
    self.set_applied()
  }

  pub fn apply_copy(&mut self, f: impl Fn(T) -> ())
  where
    T: Copy,
  {
    f(*self.borrow());
    self.set_applied()
  }

  pub fn apply_cloned(&mut self, f: impl Fn(T) -> ())
  where
    T: Clone,
  {
    f(self.cloned());
    self.set_applied()
  }

  pub fn copied(&self) -> T
  where
    T: Copy,
  {
    *self.borrow()
  }

  pub fn cloned(&self) -> T
  where
    T: Clone,
  {
    self.borrow().clone()
  }
}
