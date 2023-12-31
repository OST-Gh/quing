///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//! [I hate myself, for making documentation.]
//!
//! ### How Quing works.
//! Quing works around 2 central structures:
//! - A [`Track`]
//! - A [`Playlist`] (grouping of [`Tracks`], with additional data)
//!
//! [`Track`]: playback::Track
//! [`Tracks`]: playback::Track
//! [`Playlist`]: playback::Playlist
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
use thiserror::Error;
use std::{
	io::Error as IOError,
	env::VarError,
};
use rodio::{
	PlayError,
	decoder::DecoderError,
	StreamError,
};
use toml::de::Error as TOMLError;
use crossbeam_channel::{
	RecvError,
	RecvTimeoutError,
	TryRecvError,
};
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// A module for handling and interacting with external devices.
pub mod in_out;

/// SerDe, specifically: TOML, based structure representations of the structures of [`playback`].
pub mod serde;

/// The module responsible for handling the playing of [sources]
///
/// [sources]: rodio::Source
pub mod playback;

/// Implementation utilities.
mod utilities;
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Error, Debug)]
/// Errors encountered when
#[doc = env!("CARGO_PKG_NAME")]
/// interacts with [`Vec`]-esque structures.
#[cfg_attr(any(debug_assertions, feature = "traits"), derive(PartialEq, Eq, PartialOrd, Ord), derive(Hash))]
pub enum VectorError {
	#[error("Index out of bounds")]
	/// Overflowing an index, because under-flowing an [unsigned integer] based index is impossible.
	///
	/// [unsigned integer]: usize
	OutOfBounds,

	#[error("Empty vector encountered.")]
	/// As the name says.
	Empty,
}

#[derive(Error, Debug)]
#[cfg_attr(any(debug_assertions, feature = "traits"), derive(PartialEq, Eq, PartialOrd, Ord), derive(Hash))]
pub enum ChannelError {
	#[error("A Channel-Timeout occurred.")]
	Timeout,
	#[error("A Channel is empty.")]
	Empty,
	#[error("A Channel disconnected.")]
	Disconnect,
}

#[derive(Error, Debug)]
pub enum Error {
	#[error("IO: {0}")]
	Io(#[from] IOError),

	#[error("Rodio-Decode: {0}")]
	Decode(#[from] DecoderError),
	#[error("Rodio-Play: {0}")]
	Play(#[from] PlayError),
	#[error("Rodio-Stream: {0}")]
	Stream(#[from] StreamError),

	#[error("TOML: {0}")]
	Deserialise(#[from] TOMLError),

	#[error("Variable: {0}")]
	Variable(#[from] VarError),

	#[error("Vector: {0}")]
	Vector(#[from] VectorError),

	#[error("Channel: {0}")]
	Channel(#[from] ChannelError),
}

impl From<RecvTimeoutError> for ChannelError {
	fn from(error: RecvTimeoutError) -> Self {
		match error {
			RecvTimeoutError::Timeout => Self::Timeout,
			RecvTimeoutError::Disconnected => Self::Disconnect,
		}
	}
}

impl From<()> for ChannelError {
	#[inline(always)]
	fn from(_: ()) -> Self { Self::Disconnect }
}

impl From<RecvError> for ChannelError {
	#[inline(always)]
	fn from(_: RecvError) -> Self { ().into() }
}

impl From<TryRecvError> for ChannelError {
	fn from(error: TryRecvError) -> Self {
		match error {
			TryRecvError::Empty => Self::Empty,
			TryRecvError::Disconnected => Self::Disconnect,
		}
	}
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
