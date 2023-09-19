///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
use crossterm::{
	cursor::Show,
	execute,
	terminal::{
		Clear,
		ClearType,
	},
};
use crate::{
	SetForegroundColor,
	Color,
	log,
	stdout,
};
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Print the reset ansi sequence.
///
/// Printable part: `\r{}0`
pub(crate) fn exit() {
	print!("\r");
	if let Err(why) = execute!(stdout(),
		Show,
		SetForegroundColor(Color::Reset),
	) { log!(; "resetting the terminal style" why) }
	print!("\0")
}

/// Print the clear line sequence.
///
/// Printable part: `\r{}\n\n\0`
pub(crate) fn clear() {
	print!("\r");
	if let Err(why) = execute!(stdout(),
		Clear(ClearType::CurrentLine)
	) { log!(; "clearing the current line" why) };
	print!("\0")
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
