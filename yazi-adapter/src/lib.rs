#![allow(clippy::unit_arg)]

yazi_macro::mod_flat!(
	adapter chafa dimension emulator iip image kgp kgp_old mux sixel ueberzug
);

use yazi_shared::{RoCell, env_exists, in_wsl};
pub static ADAPTOR: RoCell<Adapter> = RoCell::new();

// Tmux support
pub static TMUX: RoCell<bool> = RoCell::new();
static ESCAPE: RoCell<&'static str> = RoCell::new();
static START: RoCell<&'static str> = RoCell::new();
static CLOSE: RoCell<&'static str> = RoCell::new();

// WSL support
pub static WSL: RoCell<bool> = RoCell::new();

// Image state
static SHOWN: RoCell<arc_swap::ArcSwapOption<ratatui::layout::Rect>> = RoCell::new();

pub fn init() {
	// Tmux support
	TMUX.init(env_exists("TMUX_PANE") && env_exists("TMUX"));
	ESCAPE.init(if *TMUX { "\x1b\x1b" } else { "\x1b" });
	START.init(if *TMUX { "\x1bPtmux;\x1b\x1b" } else { "\x1b" });
	CLOSE.init(if *TMUX { "\x1b\\" } else { "" });

	if *TMUX {
		_ = std::process::Command::new("tmux")
			.args(["set", "-p", "allow-passthrough", "all"])
			.stdin(std::process::Stdio::null())
			.stdout(std::process::Stdio::null())
			.stderr(std::process::Stdio::null())
			.status();
	}

	// WSL support
	WSL.init(in_wsl());

	// Image state
	SHOWN.with(<_>::default);

	ADAPTOR.init(Adapter::matches());
	ADAPTOR.start();
}
