//! Host-embedded UI panels for AniCraft 360 Studio.
//!
//! This crate deliberately does not create an application window, own a dock
//! manager, render the character stage, or persist document data. AniCraft
//! supplies read-only view models and places these panels in its existing
//! workspace. User interactions are returned as typed actions for the host to
//! validate and dispatch through its command system.

#![forbid(unsafe_code)]

use egui::{Color32, RichText, Ui};

/// Read-only character hierarchy item projected by the AniCraft host.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharacterNode {
    pub id: String,
    pub label: String,
    pub kind: CharacterNodeKind,
    /// Visual nesting depth supplied by the host's character projection.
    pub depth: usize,
    pub controller_id: Option<String>,
    pub deformation_target_id: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharacterNodeKind {
    Symbol,
    BodyPart,
    RigGroup,
    SmartBone,
    SwitchGroup,
    SwitchOption,
}

/// A registered character view. Angle semantics and ordering are owned by
/// the 360 Studio domain model; this is only its UI projection.
#[derive(Clone, Debug, PartialEq)]
pub struct CharacterView {
    pub id: String,
    pub label: String,
    pub angle_degrees: f32,
    pub status: ViewStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewStatus {
    Ready,
    NeedsReview,
    MissingArtwork,
}

/// An asset-library item supplied by the AniCraft host asset service.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetEntry {
    pub id: String,
    pub title: String,
    pub category: String,
    pub description: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationIssue {
    pub severity: IssueSeverity,
    pub message: String,
}

/// Read-only snapshot passed from the host into the panels.
///
/// The host remains authoritative for document identity, persistence, VFS,
/// symbols, bones, timeline, undo, rendering, and evaluation.
#[derive(Clone, Debug, Default)]
pub struct StudioUiModel {
    pub character_name: String,
    pub nodes: Vec<CharacterNode>,
    pub views: Vec<CharacterView>,
    pub assets: Vec<AssetEntry>,
    pub validation_issues: Vec<ValidationIssue>,
}

/// Transient UI state only. None of these fields are document data.
#[derive(Clone, Debug, Default)]
pub struct StudioUiState {
    pub selected_node_id: Option<String>,
    pub active_view_id: Option<String>,
    pub asset_filter: String,
}

/// Semantic UI intents. The host maps these to validated studio.*
/// CommandSpec operations; this crate does not dispatch host commands itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StudioUiAction {
    SelectCharacterNode { node_id: String },
    ActivateCharacterView { view_id: String },
    RequestCreateSwitchGroup,
    RequestAddCharacterView,
    RequestOpenAsset { asset_id: String },
    RequestOpenDeformationTarget { target_id: String },
}

impl StudioUiAction {
    /// Stable command-family identifier for host adapter routing.
    pub const fn command_name(&self) -> &'static str {
        match self {
            Self::SelectCharacterNode { .. } => "studio.symbol.select",
            Self::ActivateCharacterView { .. } => "studio.view.activate",
            Self::RequestCreateSwitchGroup => "studio.switch_group.create",
            Self::RequestAddCharacterView => "studio.view.create",
            Self::RequestOpenAsset { .. } => "studio.asset.open",
            Self::RequestOpenDeformationTarget { .. } => "studio.deformation.open_target",
        }
    }
}

/// Character hierarchy panel. It organizes character-specific concepts but
/// does not replace AniCraft's document/layer hierarchy.
pub fn show_character_hierarchy(
    ui: &mut Ui,
    model: &StudioUiModel,
    state: &mut StudioUiState,
    actions: &mut Vec<StudioUiAction>,
) {
    ui.heading("Character");
    ui.label(RichText::new(&model.character_name).weak());

    if ui.button("＋ Switch Group").clicked() {
        actions.push(StudioUiAction::RequestCreateSwitchGroup);
    }
    ui.separator();

    egui::ScrollArea::vertical()
        .id_salt("anicraft_360_character_hierarchy")
        .show(ui, |ui| {
            for node in &model.nodes {
                ui.horizontal(|ui| {
                    ui.add_space((node.depth as f32) * 12.0);
                    let marker = node_marker(node.kind);
                    let selected = state.selected_node_id.as_deref() == Some(&node.id);
                    if ui
                        .selectable_label(selected, format!("{marker} {}", node.label))
                        .clicked()
                    {
                        state.selected_node_id = Some(node.id.clone());
                        actions.push(StudioUiAction::SelectCharacterNode {
                            node_id: node.id.clone(),
                        });
                    }
                });
            }
        });
}

/// Horizontal authored-view selector. It selects registered views; it does
/// not imply that intermediate-view interpolation has been implemented.
pub fn show_view_rail(
    ui: &mut Ui,
    model: &StudioUiModel,
    state: &mut StudioUiState,
    actions: &mut Vec<StudioUiAction>,
) {
    ui.horizontal(|ui| {
        ui.heading("Character Views");
        if ui.button("＋ Add View").clicked() {
            actions.push(StudioUiAction::RequestAddCharacterView);
        }
    });

    egui::ScrollArea::horizontal()
        .id_salt("anicraft_360_view_rail")
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                for view in &model.views {
                    let selected = state.active_view_id.as_deref() == Some(&view.id);
                    let status = view_status_marker(view.status);
                    let label = format!("{}\n{:.0}°  {status}", view.label, view.angle_degrees);
                    if ui
                        .add_sized([104.0, 54.0], egui::SelectableLabel::new(selected, label))
                        .clicked()
                    {
                        state.active_view_id = Some(view.id.clone());
                        actions.push(StudioUiAction::ActivateCharacterView {
                            view_id: view.id.clone(),
                        });
                    }
                }
            });
        });
}

/// Contextual inspector for the selected character item. Deformation work is
/// handed off to Forma Deform by target identity; this panel does not evaluate
/// Smart Controllers or pin deformation.
pub fn show_properties(
    ui: &mut Ui,
    model: &StudioUiModel,
    state: &StudioUiState,
    actions: &mut Vec<StudioUiAction>,
) {
    ui.heading("Properties");

    let selected = state
        .selected_node_id
        .as_deref()
        .and_then(|id| model.nodes.iter().find(|node| node.id == id));

    if let Some(node) = selected {
        ui.label(RichText::new(&node.label).strong());
        ui.label(format!("Type: {}", node_kind_label(node.kind)));

        if let Some(controller_id) = &node.controller_id {
            ui.separator();
            ui.label("Forma Deform controller reference");
            ui.monospace(controller_id);
        }

        if let Some(target_id) = &node.deformation_target_id {
            ui.separator();
            ui.label("Deformation target");
            ui.monospace(target_id);
            if ui.button("Open in Forma Deform").clicked() {
                actions.push(StudioUiAction::RequestOpenDeformationTarget {
                    target_id: target_id.clone(),
                });
            }
        }
    } else {
        ui.label(RichText::new("Select a character item to inspect it.").weak());
    }

    ui.separator();
    ui.heading("Validation");
    if model.validation_issues.is_empty() {
        ui.colored_label(Color32::from_rgb(90, 170, 120), "No reported issues");
    } else {
        for issue in &model.validation_issues {
            let (marker, color) = match issue.severity {
                IssueSeverity::Info => ("ℹ", Color32::LIGHT_BLUE),
                IssueSeverity::Warning => ("⚠", Color32::YELLOW),
                IssueSeverity::Error => ("●", Color32::LIGHT_RED),
            };
            ui.colored_label(color, format!("{marker} {}", issue.message));
        }
    }
}

/// Asset browser panel. Asset retrieval and all file access remain in the
/// host's VFS/asset service.
pub fn show_asset_library(
    ui: &mut Ui,
    model: &StudioUiModel,
    state: &mut StudioUiState,
    actions: &mut Vec<StudioUiAction>,
) {
    ui.heading("Asset Library");
    ui.text_edit_singleline(&mut state.asset_filter);

    let filter = state.asset_filter.trim().to_lowercase();
    egui::ScrollArea::vertical()
        .id_salt("anicraft_360_asset_library")
        .show(ui, |ui| {
            for asset in &model.assets {
                let matches = filter.is_empty()
                    || asset.title.to_lowercase().contains(&filter)
                    || asset.category.to_lowercase().contains(&filter)
                    || asset.description.to_lowercase().contains(&filter);
                if !matches {
                    continue;
                }

                ui.group(|ui| {
                    ui.label(RichText::new(&asset.title).strong());
                    ui.label(RichText::new(&asset.category).weak());
                    ui.label(&asset.description);
                    if ui.button("Open asset").clicked() {
                        actions.push(StudioUiAction::RequestOpenAsset {
                            asset_id: asset.id.clone(),
                        });
                    }
                });
            }
        });
}

fn node_marker(kind: CharacterNodeKind) -> &'static str {
    match kind {
        CharacterNodeKind::Symbol => "◇",
        CharacterNodeKind::BodyPart => "◈",
        CharacterNodeKind::RigGroup => "⌘",
        CharacterNodeKind::SmartBone => "⌁",
        CharacterNodeKind::SwitchGroup => "▤",
        CharacterNodeKind::SwitchOption => "▪",
    }
}

fn node_kind_label(kind: CharacterNodeKind) -> &'static str {
    match kind {
        CharacterNodeKind::Symbol => "Symbol",
        CharacterNodeKind::BodyPart => "Body part",
        CharacterNodeKind::RigGroup => "Rig group",
        CharacterNodeKind::SmartBone => "Smart Bone",
        CharacterNodeKind::SwitchGroup => "Switch Group",
        CharacterNodeKind::SwitchOption => "Switch option",
    }
}

fn view_status_marker(status: ViewStatus) -> &'static str {
    match status {
        ViewStatus::Ready => "Ready",
        ViewStatus::NeedsReview => "Review",
        ViewStatus::MissingArtwork => "Missing",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actions_route_through_the_expected_command_families() {
        let cases = [
            (StudioUiAction::SelectCharacterNode { node_id: "symbol-1".into() }, "studio.symbol.select"),
            (StudioUiAction::ActivateCharacterView { view_id: "view-front".into() }, "studio.view.activate"),
            (StudioUiAction::RequestCreateSwitchGroup, "studio.switch_group.create"),
            (StudioUiAction::RequestAddCharacterView, "studio.view.create"),
            (StudioUiAction::RequestOpenAsset { asset_id: "asset-1".into() }, "studio.asset.open"),
            (StudioUiAction::RequestOpenDeformationTarget { target_id: "target-1".into() }, "studio.deformation.open_target"),
        ];

        for (action, expected) in cases {
            assert_eq!(action.command_name(), expected);
        }
    }

    #[test]
    fn default_ui_state_contains_no_document_data() {
        let state = StudioUiState::default();
        assert!(state.selected_node_id.is_none());
        assert!(state.active_view_id.is_none());
        assert!(state.asset_filter.is_empty());
    }
}
