///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
use crossterm::{
	cursor::Show,
	execute,
	terminal::{
		Clear,
		ClearType,
	},
};
use std::io::Stdout;
use super::{
	SetForegroundColor,
	Color,
	log,
};
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Print the reset ansi sequence.
pub(crate) fn exit(out: &mut Stdout) {
	print!("\r");
	if let Err(why) = execute!(out,
		Show,
		SetForegroundColor(Color::Reset),
	) { log!(err: "reset the terminal style" => why) }
	print!("\0")
}

/// Print the clear line sequence.
pub(crate) fn clear(out: &mut Stdout) {
	print!("\r");
	if let Err(why) = execute!(out,
		Clear(ClearType::CurrentLine)
	) { log!(err: "clear the current line" => why) };
	print!("\n\n\0")
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
