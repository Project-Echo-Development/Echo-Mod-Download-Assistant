use crate::utils::file_utils::{self, InstallRecord};
use egui::{self, Button, Color32, Context, CornerRadius, RichText};
#[derive(Clone, Copy, PartialEq)]
pub enum Mod {
    dark_roles,
    endless_host_roles,
    town_of_host,
    project_lotus,
    town_of_us_mira,
    toh_enhanced,
}
pub fn create_title_panel(ctx: &Context, fill_color: Color32) {
    egui::TopBottomPanel::top("Header")
        .frame(
            egui::Frame::default()
                .fill(fill_color)
                .corner_radius(CornerRadius {
                    nw: 12,
                    ne: 12,
                    sw: 0,
                    se: 0,
                }),
        )
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let drag_handle_rect = ui
                    .max_rect()
                    .with_max_y(30.0);
                let response = ui.interact(
                    drag_handle_rect,
                    ui.id()
                        .with("window_drag"),
                    egui::Sense::drag(),
                );
                if response.dragged() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                ui.add_space(6.0);
                ui.heading("Echo Mod Download Assistant 1.0.0");
                ui.add_space(6.0);
            });
        });
}
pub fn create_mod_selection_panel<'a>(
    ctx: &Context,
    labels: [&'static str; 6],
    button_size: [f32; 2],
    fill_color: Color32,
    selected_mod: &'a mut Option<Mod>,
) {
    let buttons = [
        ("The Dark Roles", Mod::dark_roles),
        ("Endless Host Roles", Mod::endless_host_roles),
        ("Town Of Host", Mod::town_of_host),
        ("Project Lotus", Mod::project_lotus),
        ("Town Of Us: Mira", Mod::town_of_us_mira),
        ("TOH Enhanced", Mod::toh_enhanced),
    ];
    egui::SidePanel::left("Buttons")
        .resizable(false)
        .frame(
            egui::Frame::default()
                .inner_margin(8)
                .fill(fill_color)
                .corner_radius(CornerRadius {
                    nw: 0,
                    ne: 0,
                    se: 0,
                    sw: 12,
                }),
        )
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(141.0, 0.0),
                    egui::Layout::centered_and_justified(
                        egui::Direction::LeftToRight,
                    ),
                    |ui| {
                        ui.label(
                            RichText::new("Select A Mod Below")
                                .strong()
                                .size(14.0)
                                .color(
                                    ui.visuals()
                                        .text_color(),
                                ),
                        );
                    },
                );
                ui.add_space(4.0);
                let mut iter = buttons
                    .iter()
                    .peekable();
                while let Some((label, mod_id)) = iter.next() {
                    let clicked = ui
                        .add_sized(
                            button_size,
                            egui::Button::new(*label).corner_radius(8),
                        )
                        .clicked();
                    if clicked {
                        *selected_mod = Some(*mod_id);
                    }
                    if iter
                        .peek()
                        .is_some()
                    {
                        ui.add_space(6.0);
                    }
                }
            });
        });
}
pub fn create_main_panel<'a>(
    ctx: &Context,
    fill_color: Color32,
    install_steam: &mut bool,
    install_epic: &mut bool,
    install_custom: &mut bool,
    custom_install_path: &mut String,
    selected_mod: &'a mut Option<Mod>,
    github_repo: &mut String,
    last_install: &mut Option<InstallRecord>,
) {
    egui::CentralPanel::default().frame(egui::Frame::default() .fill(fill_color) .corner_radius(CornerRadius {
        nw: 0, ne: 0, se: 12, sw: 0,
    }) .inner_margin(8), ) .show(ctx, |ui| { 
        ui.label( egui::RichText::new("Select Steam OR Epic to install to a custom directory") .size(13.0) .color(ui.visuals().text_color()), ); 
        ui.label( RichText::new("Where do you want to install?") .strong() .size(14.0) .color(ui.visuals().text_color()), ); ui.add_space(2.0); 
        
        match selected_mod { 
            Some(Mod::dark_roles) => { 
                ui.label("Selected: The Dark Roles"); 
            *github_repo = "Project-Echo-Development/The-Dark-Roles".to_string(); 
            } 
            Some(Mod::endless_host_roles) => { 
                ui.label("Selected: Endless Host Roles"); 
                *github_repo = "Gurge44/EndlessHostRoles".to_string(); 
            } 
            Some(Mod::town_of_host) => {
                ui.label("Selected: Town Of Host"); 
                *github_repo = "tukasa0001/TownOfHost".to_string(); 
            } 
            Some(Mod::project_lotus) => {
                ui.label("Selected: Project Lotus"); 
                *github_repo = "Lotus-AU/LotusContinued".to_string(); 
            }
            Some(Mod::town_of_us_mira) => { 
                ui.label("Selected: Town Of Us: Mira"); 
                *github_repo = "AU-Avengers/TOU-Mira".to_string(); 
            } 
            Some(Mod::toh_enhanced) => { 
                ui.label("Selected: TOH Enhanced"); 
                *github_repo = "EnhancedNetwork/TownofHost-Enhanced".to_string();
            } 
            None => {
                ui.label("Select a mod on the left"); 
            } 
        } 
        
        ui.checkbox(install_steam, "Steam"); 
        ui.checkbox(install_epic, "Epic Games"); 
        let custom_enabled = *install_steam || *install_epic; ui.add_enabled_ui(custom_enabled, |ui| { 
            ui.checkbox(&mut *install_custom, "Custom directory"); 
        }); 
        if *install_custom && !custom_enabled { 
            *install_custom = false; 
        } 
        if *install_custom {
            ui.horizontal(|ui| { 
                ui.add_sized( [220.0, 20.0], egui::TextEdit::singleline(custom_install_path), ); 
                if ui.button("Browse...").clicked() { 
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() { 
                        *custom_install_path = folder.display().to_string(); 
                    } 
                } 
            }); 
        } 
        
        if *install_steam || *install_epic || *install_custom { 
            ui.add_space(6.0); 
            ui.label( RichText::new("[WARNING] This will overwrite ANY installed mods!\nMake sure you backed up your Among Us folder!\n(Auto backup will be added in the future)")
            .strong()
            .size(12.0)
            .color(Color32::ORANGE), ); 
            if ui.add_sized([60.0, 21.0], Button::new("Install")).clicked() { 
                match file_utils::install_mod(github_repo, *install_steam, *install_epic, *install_custom,custom_install_path) {
                Ok(record) => {
                    *last_install = Some(record);
                }
                Err(e) => {}
                }
            } 

            if ui.add_sized([60.0, 21.0], Button::new("Clean")).clicked() { 
                if let Some(record) = &last_install {
                    match file_utils::get_install_path(*install_steam, *install_epic, *install_custom, &custom_install_path) {
                        Ok(install_dir) => {
                            if let Err(e) = file_utils::clean_install(record, &install_dir) {
                                eprintln!("Clear failed: {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to resolve install path: {}", e);
                        }
                    }
                }
            }
        } 
    });
}
pub fn add_panels_to_window<'a>(
    ctx: &Context,
    button_size: [f32; 2],
    labels: [&'static str; 6],
    fill_color: Color32,
    install_steam: &mut bool,
    install_epic: &mut bool,
    install_custom: &mut bool,
    custom_install_path: &mut String,
    selected_mod: &'a mut Option<Mod>,
    github_repo: &mut String,
    last_install: &mut Option<InstallRecord>,
) {
    create_title_panel(ctx, fill_color);
    create_mod_selection_panel(
        ctx,
        labels,
        button_size,
        fill_color,
        selected_mod,
    );
    create_main_panel(
        ctx,
        fill_color,
        install_steam,
        install_epic,
        install_custom,
        custom_install_path,
        selected_mod,
        github_repo,
        last_install,
    );
}
pub fn style_window(
    ctx: &Context,
    window_color: Color32,
    text_edit_fill: Color32,
    text_color: Color32,
) {
    ctx.style_mut(|style| {
        style
            .visuals
            .panel_fill = window_color;
        style
            .visuals
            .text_edit_bg_color = Some(text_edit_fill);
        style
            .visuals
            .widgets
            .inactive
            .weak_bg_fill = text_edit_fill;
        style
            .visuals
            .override_text_color = Some(text_color);
    });
}
