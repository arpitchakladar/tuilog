use cursive::theme::{
	PaletteColor,
	Theme,
	Color,
	BaseColor,
	BorderStyle,
};
use cursive::Cursive;

pub fn get_edit_view_theme(siv: &Cursive) -> Theme {
	let mut edit_view_theme = siv.current_theme().clone();
	let secondary_color = edit_view_theme.palette[PaletteColor::Secondary];
	edit_view_theme.palette[PaletteColor::Secondary] = edit_view_theme.palette[PaletteColor::Background];
	edit_view_theme.palette[PaletteColor::Background] = secondary_color;
	edit_view_theme.palette[PaletteColor::View] = edit_view_theme.palette[PaletteColor::Primary];
	edit_view_theme
}

pub fn get_theme() -> Theme {
	let mut theme = Theme::terminal_default();

	theme.shadow = false;
	theme.borders = BorderStyle::Simple;
	theme.palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
	theme.palette[PaletteColor::Shadow] = Color::Dark(BaseColor::Black);
	theme.palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
	theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::White);
	theme.palette[PaletteColor::Secondary] = Color::Dark(BaseColor::White);
	theme.palette[PaletteColor::Tertiary] = Color::Light(BaseColor::White);
	theme.palette[PaletteColor::TitlePrimary] = Color::Dark(BaseColor::Yellow);
	theme.palette[PaletteColor::TitleSecondary] = Color::Dark(BaseColor::Yellow);
	theme.palette[PaletteColor::Highlight] = Color::Dark(BaseColor::White);
	theme.palette[PaletteColor::HighlightInactive] = Color::Dark(BaseColor::White);
	theme.palette[PaletteColor::HighlightText] = Color::Dark(BaseColor::Black);

	theme
}

pub fn get_error_message_theme(siv: &Cursive) -> Theme {
	let mut error_theme = siv.current_theme().clone();
	error_theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::Red);
	error_theme
}
