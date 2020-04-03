//! Data type definition for a meeting session.

use uuid::Uuid;

/// A meeting's id to differentiate between different meetings.
pub type MeetingId = Uuid;

/// A presenter, identified by their `user_id`.
pub type Presenter = Uuid;

/// A listener, identified by their `user_id`.
pub type Listener = Uuid;

/// A `MeetingSession` represents an on-going meeting. A meeting session consists of one *presenter*
/// and zero or more *listeners*. Each meeting session is given a unique `Uuid` to differentiate
/// between sessions and to determine which user(s) have priviledges to change meeting session
/// settings, invite attendees, kick attendees, upload presentation slides, etc.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct MeetingSession {
	presenter: Presenter,
	listeners: Vec<Listener>,
}

impl MeetingSession {
	/// Initate a new `MeetingSession`. It is required that such a `MeetingSession` must be started
	/// by an authenticated `presenter`.
	pub fn new(presenter_id: Uuid) -> Self {
		Self {
			presenter: presenter_id,
			// `Vec::new()` does not allocate yet by default.
			listeners: Vec::new(),
		}
	}

	/// Initiate a new `MeetingSession` with `listeners`. Must also be started by an authenticated
	/// `presenter`.
	pub fn with_listeners(presenter_id: Uuid, listeners_ids: Vec<Uuid>) -> Self {
		Self {
			presenter: presenter_id,
			listeners: listeners_ids,
		}
	}

	/// Get the presenter's `user_id`.
	pub fn presenter(&self) -> &Uuid {
		&self.presenter
	}

	/// Get the listeners's `user_id`.
	pub fn listeners(&self) -> &[Uuid] {
		&self.listeners[..]
	}

	/// Get all participants of a meeting session, including the presenter and listeners.
	///
	/// Note: this method allocates a `Vec` with size `1 + listeners.len()`.
	pub fn participants(&self) -> Vec<Uuid> {
		let mut participants: Vec<Uuid> = Vec::with_capacity(1 + self.listeners.len());
		participants.clone_from(&self.listeners);
		participants.push(self.presenter.clone());
		participants
	}

	/// Add a `listener` to the meeting session. If the listener already exists as the presenter or
	/// listener, nothing is modified.
	pub fn add_listener(&mut self, listener_id: Uuid) -> &mut Self {
		if &self.presenter == &listener_id {
			return self;
		}

		if self.listeners.contains(&listener_id) {
			return self;
		}

		self.listeners.push(listener_id);
		self
	}

	/// Remove a `listener` from the meeting session.
	pub fn remove_listener(&mut self, listener_id: &Uuid) -> &mut Self {
		self.listeners.retain(|id| listener_id != id);
		self
	}
}
