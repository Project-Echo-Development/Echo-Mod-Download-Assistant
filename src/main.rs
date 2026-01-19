#![windows_subsystem = "windows"]
use egui::{Color32, Context};

use crate::utils::{file_utils::InstallRecord, panels::Mod};
pub mod utils;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size([500.0, 290.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_mouse_passthrough(false),
        ..Default::default()
    };

    eframe::run_native(
        "Echo Mod Download Assistant",
        options,
        Box::new(|_cc| Ok(Box::<ModInstaller>::default())),
    )
}

struct ModInstaller {
    text_edit_bg_color: Color32,
    text_override_color: Color32,
    window_bg_color: Color32,
    button_size: [f32; 2],
    button_labels: [&'static str; 6],
    selected_mod: Option<Mod>,
    install_steam: bool,
    install_epic: bool,
    install_custom: bool,
    custom_install_path: String,
    github_repo: String,
    pub last_install: Option<InstallRecord>,
}

impl Default for ModInstaller {
    fn default() -> Self {
        Self {
            text_edit_bg_color: Color32::from_rgb(35, 35, 50),
            text_override_color: Color32::from_rgb(200, 185, 200),
            window_bg_color: Color32::from_rgb(10, 5, 10),
            button_size: [140.0, 28.0],
            button_labels: [
                "The Dark Roles",
                "Endless Host Roles",
                "Town Of Host",
                "Project Lotus",
                "Town of Us: Mira",
                "TOH Enhanced",
            ],
            selected_mod: Some(Mod::dark_roles),
            install_steam: false,
            install_epic: false,
            install_custom: false,
            custom_install_path: "Select a directory...".to_string(),
            github_repo: "Project-Echo-Development/The-Dark-Roles".to_string(),
            last_install: None,
        }
    }
}
impl eframe::App for ModInstaller {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        utils::panels::style_window(
            ctx,
            self.window_bg_color,
            self.text_edit_bg_color,
            self.text_override_color,
        );
        utils::panels::add_panels_to_window(
            ctx,
            self.button_size,
            self.button_labels,
            self.window_bg_color,
            &mut self.install_steam,
            &mut self.install_epic,
            &mut self.install_custom,
            &mut self.custom_install_path,
            &mut self.selected_mod,
            &mut self.github_repo,
            &mut self.last_install,
        );
    }
}
