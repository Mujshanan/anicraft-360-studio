//! AniCraft 360 Studio's native egui workspace.
//!
//! The crate deliberately depends on egui rather than AniCraft internals. The host supplies a
//! snapshot, receives typed authoring commands, and may render its own Stage inside the central
//! workspace region. This keeps the Studio UI independently testable and suitable for eventual
//! in-process integration without inventing a second document, renderer, or timeline.

#![forbid(unsafe_code)]

use egui::{Align, Color32, Context, Layout, RichText, ScrollArea, SidePanel, TopBottomPanel, Ui};

/// Version of the host-facing snapshot/command contract implemented by this crate.
pub const STUDIO_HOST_API_MAJOR: u16 = 1;

/// Status of an authored character view.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ViewStatus {
    #[default]
    Authored,
    Incomplete,
    Warning,
    Missing,
    PreviewOnly,
}

/// A view rail item supplied by the host/document adapter.
#[derive(Clone, Debug, PartialEq)]
pub struct ViewSummary {
    pub id: String,
    pub name: String,
    pub angle_degrees: f32,
    pub status: ViewStatus,
}

/// A row in the character hierarchy. Rows are already ordered by the host's visual hierarchy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HierarchyItem {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub depth: u8,
    pub selected: bool,
}

/// Contextual properties for the current Studio selection.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectionDetails {
    pub title: String,
    pub kind: String,
    pub description: String,
    pub properties: Vec<(String, String)>,
}

/// Optional capabilities. The UI hides or disables actions the host has not implemented.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HostCapabilities {
    pub create_view_set: bool,
    pub register_view: bool,
    pub create_smart_bone: bool,
    pub create_switch_group: bool,
    pub open_asset_library: bool,
    pub stage_renderer: bool,
}

/// Read-only view of host/document state needed to draw the workspace.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StudioSnapshot {
    pub schema_version: u16,
    pub document_name: String,
    pub character_name: String,
    pub hierarchy: Vec<HierarchyItem>,
    pub views: Vec<ViewSummary>,
    pub active_view_id: Option<String>,
    pub inspection_angle_degrees: f32,
    pub selection: Option<SelectionDetails>,
    pub capabilities: HostCapabilities,
    pub status_message: Option<String>,
}

/// Typed requests from Studio to its host adapter.
///
/// Persistent operations must be implemented by the host using its command/undo and document
/// persistence systems. These variants are requests, not local mutations of host state.
#[derive(Clone, Debug, PartialEq)]
pub enum StudioCommand {
    SelectHierarchyItem { id: String },
    SelectView { id: String },
    SetInspectionAngle { degrees: f32 },
    CreateViewSet,
    RegisterView,
    CreateSmartBone,
    CreateSwitchGroup,
    OpenAssetLibrary,
}

/// Adapter implemented by AniCraft (or by a test/mock host during independent UI development).
pub trait StudioHost {
    /// Return a consistent snapshot for the current UI frame.
    fn snapshot(&self) -> StudioSnapshot;

    /// Execute a typed request through host-owned commands and persistence.
    fn dispatch(&mut self, command: StudioCommand) -> Result<(), String>;

    /// Draw AniCraft's real Stage in the supplied region when integrated. The default is an
    /// explicit placeholder; it never pretends to render document artwork.
    fn show_stage(&mut self, ui: &mut Ui) {
        let (rect, _) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
        ui.painter().rect_filled(rect, 4.0, Color32::from_rgb(37, 39, 43));
        ui.painter().text(
            rect.center() - egui::vec2(0.0, 12.0),
            egui::Align2::CENTER_CENTER,
            "AniCraft Stage",
            egui::FontId::proportional(18.0),
            Color32::from_gray(220),
        );
        ui.painter().text(
            rect.center() + egui::vec2(0.0, 12.0),
            egui::Align2::CENTER_CENTER,
            "Host renderer connection pending",
            egui::FontId::proportional(13.0),
            Color32::from_gray(150),
        );
    }
}

/// Local UI state only. Authored state always comes from and returns to the host.
#[derive(Default)]
pub struct StudioUi {
    hierarchy_filter: String,
    show_diagnostics: bool,
    last_error: Option<String>,
}

impl StudioUi {
    pub fn new() -> Self {
        Self::default()
    }

    /// Draw the Studio workspace using AniCraft's egui context and a host adapter.
    pub fn show(&mut self, ctx: &Context, host: &mut impl StudioHost) {
        let snapshot = host.snapshot();
        let mut pending = None;

        TopBottomPanel::top("anicraft_360_top_bar")
            .exact_height(34.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.label(RichText::new("360°").strong().color(Color32::from_rgb(115, 170, 225)));
                    ui.separator();
                    ui.label(RichText::new(if snapshot.character_name.is_empty() { "Character Studio" } else { &snapshot.character_name }).strong());
                    ui.separator();
                    ui.menu_button("Character", |ui| {
                        if ui.add_enabled(snapshot.capabilities.create_view_set, egui::Button::new("New View Set")).clicked() {
                            pending = Some(StudioCommand::CreateViewSet);
                            ui.close();
                        }
                        if ui.add_enabled(snapshot.capabilities.open_asset_library, egui::Button::new("Open Asset Library")).clicked() {
                            pending = Some(StudioCommand::OpenAssetLibrary);
                            ui.close();
                        }
                    });
                    ui.menu_button("Views", |ui| {
                        if ui.add_enabled(snapshot.capabilities.register_view, egui::Button::new("Register View…")).clicked() {
                            pending = Some(StudioCommand::RegisterView);
                            ui.close();
                        }
                    });
                    ui.menu_button("Rigging", |ui| {
                        if ui.add_enabled(snapshot.capabilities.create_smart_bone, egui::Button::new("New Smart Bone")).clicked() {
                            pending = Some(StudioCommand::CreateSmartBone);
                            ui.close();
                        }
                        if ui.add_enabled(snapshot.capabilities.create_switch_group, egui::Button::new("New Switch Group")).clicked() {
                            pending = Some(StudioCommand::CreateSwitchGroup);
                            ui.close();
                        }
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let label = if self.show_diagnostics { "Hide Diagnostics" } else { "Diagnostics" };
                        if ui.small_button(label).clicked() {
                            self.show_diagnostics = !self.show_diagnostics;
                        }
                        ui.label(RichText::new("AniCraft workspace").small().weak());
                    });
                });
            });

        TopBottomPanel::bottom("anicraft_360_view_rail")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(3.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("TURN").small().strong().weak());
                    let mut angle = snapshot.inspection_angle_degrees;
                    let slider = egui::Slider::new(&mut angle, -180.0..=180.0).suffix("°").show_value(true);
                    if ui.add_sized([260.0, 20.0], slider).changed() {
                        pending = Some(StudioCommand::SetInspectionAngle { degrees: angle });
                    }
                    ui.separator();
                    if ui.small_button("‹").on_hover_text("Previous registered view").clicked() {
                        if let Some(view) = adjacent_view(&snapshot, false) {
                            pending = Some(StudioCommand::SelectView { id: view.id.clone() });
                        }
                    }
                    if ui.small_button("›").on_hover_text("Next registered view").clicked() {
                        if let Some(view) = adjacent_view(&snapshot, true) {
                            pending = Some(StudioCommand::SelectView { id: view.id.clone() });
                        }
                    }
                    ui.label(RichText::new(format!("{} views", snapshot.views.len())).small().weak());
                });
                ui.add_space(3.0);
                ScrollArea::horizontal().id_salt("anicraft_360_view_items").show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if snapshot.views.is_empty() {
                            ui.label(RichText::new("No views registered").weak());
                            if ui.add_enabled(snapshot.capabilities.create_view_set, egui::Button::new("Create View Set")).clicked() {
                                pending = Some(StudioCommand::CreateViewSet);
                            }
                        }
                        for view in &snapshot.views {
                            let active = snapshot.active_view_id.as_deref() == Some(view.id.as_str());
                            let status = view_status_label(view.status);
                            let title = format!("{}  {}°  ·  {}", view.name, format_angle(view.angle_degrees), status);
                            if ui.selectable_label(active, title).clicked() {
                                pending = Some(StudioCommand::SelectView { id: view.id.clone() });
                            }
                        }
                    });
                });
                ui.add_space(3.0);
            });

        SidePanel::left("anicraft_360_hierarchy")
            .default_width(224.0)
            .min_width(180.0)
            .resizable(true)
            .show(ctx, |ui| {
                panel_heading(ui, "Character");
                ui.add_space(4.0);
                ui.add(egui::TextEdit::singleline(&mut self.hierarchy_filter).hint_text("Filter hierarchy…").desired_width(f32::INFINITY));
                ui.add_space(4.0);
                ScrollArea::vertical().id_salt("anicraft_360_hierarchy_scroll").show(ui, |ui| {
                    if snapshot.hierarchy.is_empty() {
                        empty_panel(ui, "No character structure", "Select a document or create a character to begin.");
                    }
                    for item in snapshot.hierarchy.iter().filter(|item| self.hierarchy_filter.is_empty() || item.label.to_lowercase().contains(&self.hierarchy_filter.to_lowercase())) {
                        ui.horizontal(|ui| {
                            ui.add_space(item.depth as f32 * 12.0);
                            ui.label(RichText::new(kind_glyph(&item.kind)).weak());
                            if ui.selectable_label(item.selected, &item.label).clicked() {
                                pending = Some(StudioCommand::SelectHierarchyItem { id: item.id.clone() });
                            }
                        });
                    }
                });
            });

        SidePanel::right("anicraft_360_inspector")
            .default_width(276.0)
            .min_width(220.0)
            .resizable(true)
            .show(ctx, |ui| {
                panel_heading(ui, "Properties");
                ui.add_space(6.0);
                if let Some(selection) = &snapshot.selection {
                    ui.label(RichText::new(&selection.title).strong());
                    ui.label(RichText::new(&selection.kind).small().weak());
                    if !selection.description.is_empty() {
                        ui.add_space(4.0);
                        ui.label(RichText::new(&selection.description).small());
                    }
                    ui.separator();
                    for (name, value) in &selection.properties {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(name).small().weak());
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(RichText::new(value).small());
                            });
                        });
                    }
                } else {
                    empty_panel(ui, "Nothing selected", "Select a character part, view, Smart Bone, or Switch Group to inspect its properties.");
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("STAGE").small().strong().weak());
                ui.separator();
                if let Some(view) = snapshot.views.iter().find(|v| Some(v.id.as_str()) == snapshot.active_view_id.as_deref()) {
                    ui.label(RichText::new(format!("{}  ·  {}°", view.name, format_angle(view.angle_degrees))).strong());
                    ui.label(RichText::new(view_status_label(view.status)).small().weak());
                } else {
                    ui.label(RichText::new("No active authored view").weak());
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.small_button("Fit").on_hover_text("Fit artwork in the AniCraft Stage").clicked() {
                        // View fitting is a host viewport command; added to the contract when
                        // AniCraft exposes a stable viewport command for embedded tools.
                    }
                    ui.label(RichText::new("Select  ·  Pan  ·  Zoom").small().weak());
                });
            });
            ui.separator();
            host.show_stage(ui);
        });

        if self.show_diagnostics {
            egui::Window::new("360 Studio Diagnostics")
                .id(egui::Id::new("anicraft_360_diagnostics"))
                .default_width(360.0)
                .default_height(180.0)
                .show(ctx, |ui| {
                    if let Some(message) = &snapshot.status_message {
                        ui.label(message);
                    } else {
                        empty_panel(ui, "No diagnostics reported", "Only host-provided diagnostics are shown here.");
                    }
                    if let Some(error) = &self.last_error {
                        ui.colored_label(Color32::from_rgb(220, 100, 90), error);
                    }
                });
        }

        if let Some(command) = pending {
            self.last_error = host.dispatch(command).err();
        }
    }
}

fn panel_heading(ui: &mut Ui, title: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(title).strong());
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(RichText::new("⋯").weak());
        });
    });
    ui.separator();
}

fn empty_panel(ui: &mut Ui, title: &str, detail: &str) {
    ui.add_space(10.0);
    ui.vertical_centered(|ui| {
        ui.label(RichText::new(title).strong());
        ui.add_space(3.0);
        ui.label(RichText::new(detail).small().weak());
    });
}

fn adjacent_view(snapshot: &StudioSnapshot, next: bool) -> Option<&ViewSummary> {
    if snapshot.views.is_empty() {
        return None;
    }
    let index = snapshot.views.iter().position(|view| Some(view.id.as_str()) == snapshot.active_view_id.as_deref()).unwrap_or(0);
    let target = if next { (index + 1) % snapshot.views.len() } else { (index + snapshot.views.len() - 1) % snapshot.views.len() };
    snapshot.views.get(target)
}

fn view_status_label(status: ViewStatus) -> &'static str {
    match status {
        ViewStatus::Authored => "Authored",
        ViewStatus::Incomplete => "Incomplete",
        ViewStatus::Warning => "Warning",
        ViewStatus::Missing => "Missing",
        ViewStatus::PreviewOnly => "Preview only",
    }
}

fn kind_glyph(kind: &str) -> &'static str {
    match kind {
        "group" => "▾",
        "symbol" => "◇",
        "bone" => "⌁",
        "smart_bone" => "◆",
        "switch_group" => "▦",
        _ => "•",
    }
}

fn format_angle(angle: f32) -> String {
    if angle.fract().abs() < f32::EPSILON {
        format!("{angle:.0}")
    } else {
        format!("{angle:.1}")
    }
}

/// Tiny reusable mock for examples and downstream integration tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MockHost {
        snapshot: StudioSnapshot,
        commands: Vec<StudioCommand>,
    }

    impl StudioHost for MockHost {
        fn snapshot(&self) -> StudioSnapshot {
            self.snapshot.clone()
        }
        fn dispatch(&mut self, command: StudioCommand) -> Result<(), String> {
            self.commands.push(command);
            Ok(())
        }
    }

    #[test]
    fn host_contract_is_versioned_and_starts_with_empty_state() {
        let snapshot = StudioSnapshot::default();
        assert_eq!(STUDIO_HOST_API_MAJOR, 1);
        assert_eq!(snapshot.schema_version, 0);
        assert!(snapshot.views.is_empty());
        assert!(snapshot.selection.is_none());
    }

    #[test]
    fn angle_formatting_is_stable_for_integer_and_fractional_values() {
        assert_eq!(format_angle(90.0), "90");
        assert_eq!(format_angle(-22.5), "-22.5");
    }

    #[test]
    fn adjacent_views_wrap_in_both_directions() {
        let snapshot = StudioSnapshot {
            views: vec![
                ViewSummary { id: "front".into(), name: "Front".into(), angle_degrees: 0.0, status: ViewStatus::Authored },
                ViewSummary { id: "side".into(), name: "Side".into(), angle_degrees: 90.0, status: ViewStatus::Authored },
            ],
            active_view_id: Some("front".into()),
            ..StudioSnapshot::default()
        };
        assert_eq!(adjacent_view(&snapshot, true).map(|v| v.id.as_str()), Some("side"));
        assert_eq!(adjacent_view(&snapshot, false).map(|v| v.id.as_str()), Some("side"));
    }
}
