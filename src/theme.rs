use lazy_static::lazy_static;
use cursive::theme::{
	PaletteColor,
	Theme,
	Color,
	BaseColor,
	BorderStyle,
};

lazy_static! {
	static ref base_theme: Theme = {
		let mut cbase_theme = Theme::terminal_default();

		cbase_theme.shadow = false;
		cbase_theme.borders = BorderStyle::Simple;
		cbase_theme.palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
		cbase_theme.palette[PaletteColor::Shadow] = Color::Dark(BaseColor::Black);
		cbase_theme.palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
		cbase_theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::White);
		cbase_theme.palette[PaletteColor::Secondary] = Color::Dark(BaseColor::White);
		cbase_theme.palette[PaletteColor::Tertiary] = Color::Light(BaseColor::White);
		cbase_theme.palette[PaletteColor::TitlePrimary] = Color::Dark(BaseColor::Yellow);
		cbase_theme.palette[PaletteColor::TitleSecondary] = Color::Dark(BaseColor::Yellow);
		cbase_theme.palette[PaletteColor::Highlight] = Color::Dark(BaseColor::White);
		cbase_theme.palette[PaletteColor::HighlightInactive] = Color::Dark(BaseColor::White);
		cbase_theme.palette[PaletteColor::HighlightText] = Color::Dark(BaseColor::Black);

		cbase_theme
	};

	// theme of the text that shows the current tty
	static ref accent_message_theme: Theme = {
		let mut caccent_message_theme = base_theme.clone();
		caccent_message_theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::Cyan);

		return caccent_message_theme;
	};

	static ref edit_view_theme: Theme = {
		let mut cedit_view_theme = base_theme.clone();
		cedit_view_theme.palette[PaletteColor::Secondary] = base_theme.palette[PaletteColor::Background];
		cedit_view_theme.palette[PaletteColor::Background] = base_theme.palette[PaletteColor::Secondary];
		cedit_view_theme.palette[PaletteColor::View] = base_theme.palette[PaletteColor::Primary];

		cedit_view_theme
	};

	static ref hostname_art_theme: Theme = {
		let mut chostname_art_theme = base_theme.clone();
		chostname_art_theme.palette[PaletteColor::Primary] = base_theme.palette[PaletteColor::TitlePrimary];

		chostname_art_theme
	};

	static ref error_message_theme: Theme = {
		let mut cerror_message_theme = base_theme.clone();
		cerror_message_theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::Red);

		cerror_message_theme
	};
}

pub fn get_base_theme_ref() -> &'static Theme { &base_theme }
pub fn get_base_theme() -> Theme { base_theme.clone() }
pub fn get_accent_message_theme() -> Theme { accent_message_theme.clone() }
pub fn get_edit_view_theme() -> Theme { edit_view_theme.clone() }
pub fn get_error_message_theme() -> Theme { error_message_theme.clone() }
pub fn get_hostname_art_theme() -> Theme { hostname_art_theme.clone() }
