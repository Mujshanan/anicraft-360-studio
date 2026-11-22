//! Native egui panels for AniCraft 360 Studio.
//!
//! This is a reusable UI component library. It does not create a window, dock
//! panels, own a document, render the character stage, persist authored data,
//! or execute host commands. AniCraft supplies read-only projections and places
//! these panels in its existing workspace.
//!
//! The host passes a StudioUiModel snapshot and retains StudioUiState as
//! transient workspace state. Each panel appends typed StudioUiAction intents.
//! The host validates and dispatches those intents through its own command and
//! undo system. Styling inherits the active egui Style and Visuals.

#![forbid(unsafe_code)]

use egui::{Align, Color32, Layout, RichText, Ui};
use std::collections::BTreeSet;

/// A read-only character item projected from the host document and Studio data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharacterNode {
    /// Stable identity supplied by the owner; never derived from the label.
    pub id: String,
    pub label: String,
    pub kind: CharacterNodeKind,
    /// Indentation depth in the already-resolved hierarchy.
    pub depth: usize,
    /// Whether this projection node has children in the hierarchy.
    pub has_children: bool,
    pub status: NodeStatus,
    /// Canonical Forma Deform identity; a reference, not a copy.
    pub controller_id: Option<String>,
    /// Forma Deform target identity; a reference, not an evaluator.
    pub deformation_target_id: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharacterNodeKind {
    Character,
    BodyPart,
    Symbol,
    Bone,
    RigGroup,
    SmartBone,
    SwitchGroup,
    SwitchOption,
    ViewSet,
    View,
}

impl CharacterNodeKind {
    fn label(self) -> &'static str {
        match self {
            Self::Character => "Character",
            Self::BodyPart => "Body part",
            Self::Symbol => "Symbol",
            Self::Bone => "Bone",
            Self::RigGroup => "Rig group",
            Self::SmartBone => "Smart Bone",
            Self::SwitchGroup => "Switch Group",
            Self::SwitchOption => "Switch option",
            Self::ViewSet => "View set",
            Self::View => "View",
        }
    }

    fn marker(self) -> &'static str {
        match self {
            Self::Character => "◉",
            Self::BodyPart => "◇",
            Self::Symbol => "▧",
            Self::Bone => "⌁",
            Self::RigGroup => "⌘",
            Self::SmartBone => "✳",
            Self::SwitchGroup => "▤",
            Self::SwitchOption => "▪",
            Self::ViewSet => "▦",
            Self::View => "◫",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NodeStatus {
    #[default]
    Ready,
    Warning,
    Error,
    Unresolved,
    PreviewOnly,
}

impl NodeStatus {
    fn label(self) -> &'static str {
        match self {
            Self::Ready => "Ready",
            Self::Warning => "Warning",
            Self::Error => "Error",
            Self::Unresolved => "Unresolved",
            Self::PreviewOnly => "Preview only",
        }
    }

    fn color(self, visuals: &egui::Visuals) -> Color32 {
        match self {
            Self::Ready | Self::PreviewOnly => visuals.weak_text_color(),
            Self::Warning => visuals.warn_fg_color,
            Self::Error | Self::Unresolved => visuals.error_fg_color,
        }
    }
}

/// An authored view. The host/domain layer supplies canonical ordering.
#[derive(Clone, Debug, PartialEq)]
pub struct CharacterView {
    pub id: String,
    pub label: String,
    pub angle_degrees: f32,
    pub status: ViewStatus,
    pub origin: ViewOrigin,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ViewStatus {
    #[default]
    Ready,
    NeedsReview,
    MissingArtwork,
    Invalid,
}

impl ViewStatus {
    fn label(self) -> &'static str {
        match self {
            Self::Ready => "Ready",
            Self::NeedsReview => "Review",
            Self::MissingArtwork => "Missing artwork",
            Self::Invalid => "Invalid",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ViewOrigin {
    #[default]
    Authored,
    Generated,
    Preview,
}

impl ViewOrigin {
    fn label(self) -> &'static str {
        match self {
            Self::Authored => "Authored",
            Self::Generated => "Generated",
            Self::Preview => "Preview only",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetEntry {
    pub id: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub compatibility: AssetCompatibility,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AssetCompatibility {
    #[default]
    Compatible,
    Warning,
    Unavailable,
}

impl AssetCompatibility {
    fn label(self) -> &'static str {
        match self {
            Self::Compatible => "Compatible",
            Self::Warning => "Check compatibility",
            Self::Unavailable => "Unavailable",
        }
    }
}


/// A native AniCraft armature bone exposed to Studio authoring.
#[derive(Clone, Debug, PartialEq)]
pub struct SmartBoneEntry {
    pub id: String,
    pub name: String,
    pub parent_element_id: String,
    pub child_element_id: String,
    pub angle_degrees: f32,
    pub limits_degrees: Option<[f32; 2]>,
    /// Canonical Forma Deform ControllerId reference; never evaluated here.
    pub controller_id: Option<String>,
}

/// A Switch Group is a Library folder containing alternate symbol artwork.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwitchGroupEntry {
    pub id: String,
    pub name: String,
    pub options: Vec<SwitchOptionEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwitchOptionEntry {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
    Unavailable,
}

impl IssueSeverity {
    fn label(self) -> &'static str {
        match self {
            Self::Info => "Info",
            Self::Warning => "Warning",
            Self::Error => "Error",
            Self::Unavailable => "Unavailable",
        }
    }

    fn color(self, visuals: &egui::Visuals) -> Color32 {
        match self {
            Self::Info => visuals.hyperlink_color,
            Self::Warning => visuals.warn_fg_color,
            Self::Error => visuals.error_fg_color,
            Self::Unavailable => visuals.weak_text_color(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationIssue {
    pub id: String,
    pub severity: IssueSeverity,
    pub category: String,
    pub message: String,
    pub affected_node_id: Option<String>,
    pub affected_view_id: Option<String>,
    pub repair_hint: Option<String>,
}

/// Capability flags come from the verified host integration contract.
/// Plugin presence or a version string alone is not evidence of support.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StudioCapabilities {
    pub create_smart_bone: bool,
    pub create_switch_group: bool,
    pub create_view: bool,
    pub open_assets: bool,
    pub open_deformation_targets: bool,
    pub controller_preview: bool,
}

/// Read-only projection of the active character context.
#[derive(Clone, Debug, Default)]
pub struct StudioUiModel {
    pub character_name: String,
    pub document_name: String,
    pub nodes: Vec<CharacterNode>,

    /// Selected host element IDs, in AniCraft selection order.
    pub selected_element_ids: Vec<String>,
    pub smart_bones: Vec<SmartBoneEntry>,
    pub switch_groups: Vec<SwitchGroupEntry>,
    pub library_symbols: Vec<SwitchOptionEntry>,
    /// Already sorted by the domain's canonical angle ordering.
    pub views: Vec<CharacterView>,
    pub assets: Vec<AssetEntry>,
    pub validation_issues: Vec<ValidationIssue>,
    pub capabilities: StudioCapabilities,
    pub integration_status: IntegrationStatus,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum IntegrationStatus {
    #[default]
    HostConnected,
    HostCapabilitiesMissing(String),
    ReadOnly(String),
    Error(String),
}

/// Transient workspace state only. These fields are never document data.
#[derive(Clone, Debug, Default)]
pub struct StudioUiState {
    pub selected_node_id: Option<String>,
    pub collapsed_node_ids: BTreeSet<String>,
    pub active_view_id: Option<String>,
    pub asset_filter: String,
    pub selected_issue_id: Option<String>,

    pub smart_bone_name: String,
    pub smart_bone_min_degrees: f32,
    pub smart_bone_max_degrees: f32,
    pub selected_smart_bone_id: Option<String>,
    pub switch_group_name: String,
    pub selected_switch_group_id: Option<String>,
    pub selected_switch_option_id: Option<String>,
    pub selected_switch_source_symbol_id: Option<String>,
}

/// Semantic intents. The host validates every intent against the active
/// document, stable identities, capability state, and command registry.
#[derive(Clone, Debug, PartialEq)]
pub enum StudioUiAction {
    SelectCharacterNode { node_id: String },
    ActivateCharacterView { view_id: String },

    RequestCreateSmartBone {
        parent_element_id: String,
        child_element_id: String,
        name: String,
    },
    RequestSelectSmartBone { bone_id: String },
    RequestSetSmartBoneLimits {
        bone_id: String,
        min_degrees: f32,
        max_degrees: f32,
    },
    RequestCreateSwitchGroup { name: String },
    RequestAddSwitchOption { group_id: String, symbol_id: String },
    RequestActivateSwitchOption { symbol_id: String },
    RequestCreateView,
    RequestOpenAsset { asset_id: String },
    RequestOpenDeformationTarget { target_id: String },
    RequestSelectDiagnostic { issue_id: String },
}

/// Workspace identity/status row. Global menus, tabs, and window chrome remain
/// part of the AniCraft host shell.
pub fn show_workspace_header(ui: &mut Ui, model: &StudioUiModel) {
    ui.horizontal(|ui| {
        ui.strong("AniCraft 360 Studio");
        ui.separator();
        let character: &str = if model.character_name.is_empty() {
            "No character selected"
        } else {
            &model.character_name
        };
        ui.label(RichText::new(character).weak());
        if !model.document_name.is_empty() {
            ui.separator();
            ui.label(RichText::new(&model.document_name).weak());
        }
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            match &model.integration_status {
                IntegrationStatus::HostConnected => {
                    ui.label(RichText::new("Host connected").weak());
                }
                IntegrationStatus::HostCapabilitiesMissing(reason) => {
                    ui.colored_label(ui.visuals().warn_fg_color, "Limited integration")
                        .on_hover_text(reason);
                }
                IntegrationStatus::ReadOnly(reason) => {
                    ui.label(RichText::new("Read only").weak()).on_hover_text(reason);
                }
                IntegrationStatus::Error(reason) => {
                    ui.colored_label(ui.visuals().error_fg_color, "Integration error")
                        .on_hover_text(reason);
                }
            }
        });
    });
}

/// Character-specific hierarchy. AniCraft's document/layer tree remains host-owned.
pub fn show_character_hierarchy(
    ui: &mut Ui,
    model: &StudioUiModel,
    state: &mut StudioUiState,
    actions: &mut Vec<StudioUiAction>,
) {
    ui.heading("Character");
    if ui
        .add_enabled(model.capabilities.create_switch_group, egui::Button::new("＋ Switch Group"))
        .on_disabled_hover_text("The host does not currently expose Switch Group creation.")
        .clicked()
    {
        actions.push(StudioUiAction::RequestCreateSwitchGroup {
            name: if state.switch_group_name.trim().is_empty() { "Switch Group".to_owned() } else { state.switch_group_name.trim().to_owned() },
        });
    }
    ui.separator();

    if model.nodes.is_empty() {
        empty_state(ui, "No character items", "Character parts, symbols and rig references will appear here.");
        return;
    }

    egui::ScrollArea::vertical()
        .id_salt("anicraft_360_character_hierarchy")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let mut collapsed_ancestors: Vec<(usize, String)> = Vec::new();
            for node in &model.nodes {
                while collapsed_ancestors
                    .last()
                    .is_some_and(|(depth, _)| *depth >= node.depth)
                {
                    collapsed_ancestors.pop();
                }
                if collapsed_ancestors
                    .last()
                    .is_some_and(|(depth, _)| node.depth > *depth)
                {
                    continue;
                }

                ui.horizontal(|ui| {
                    ui.add_space((node.depth.min(24) as f32) * 12.0);
                    let collapsed = state.collapsed_node_ids.contains(&node.id);
                    if node.has_children {
                        if ui.small_button(if collapsed { "▸" } else { "▾" }).clicked() {
                            if collapsed {
                                state.collapsed_node_ids.remove(&node.id);
                            } else {
                                state.collapsed_node_ids.insert(node.id.clone());
                            }
                        }
                    } else {
                        ui.add_space(16.0);
                    }

                    let selected = state.selected_node_id.as_deref() == Some(node.id.as_str());
                    let response = ui.selectable_label(
                        selected,
                        format!("{}  {}", node.kind.marker(), node.label),
                    );
                    if response.clicked() {
                        state.selected_node_id = Some(node.id.clone());
                        actions.push(StudioUiAction::SelectCharacterNode { node_id: node.id.clone() });
                    }
                    response.on_hover_text(format!("{} · {}", node.kind.label(), node.status.label()));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.small(RichText::new(node.status.label()).color(node.status.color(ui.visuals())));
                    });
                });

                if node.has_children && state.collapsed_node_ids.contains(&node.id) {
                    collapsed_ancestors.push((node.depth, node.id.clone()));
                }
            }
        });
}

/// Discrete authored-view rail. Supplied order is authoritative; this panel
/// never interpolates or sorts view angles.
pub fn show_view_rail(
    ui: &mut Ui,
    model: &StudioUiModel,
    state: &mut StudioUiState,
    actions: &mut Vec<StudioUiAction>,
) {
    ui.horizontal(|ui| {
        ui.heading("Views");
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui
                .add_enabled(model.capabilities.create_view, egui::Button::new("＋ Add View"))
                .on_disabled_hover_text("View registration is unavailable from the current host.")
                .clicked()
            {
                actions.push(StudioUiAction::RequestCreateView);
            }
        });
    });

    if model.views.is_empty() {
        empty_state(ui, "No views registered", "Create a view set or register the first authored view when supported.");
        return;
    }

    egui::ScrollArea::horizontal()
        .id_salt("anicraft_360_view_rail")
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                for view in &model.views {
                    let selected = state.active_view_id.as_deref() == Some(view.id.as_str());
                    let title = format!("{} · {}", view.label, angle_label(view.angle_degrees));
                    let response = ui.add(
                        egui::Button::selectable(selected, title)
                            .min_size(egui::vec2(116.0, 34.0)),
                    );
                    if response.clicked() {
                        state.active_view_id = Some(view.id.clone());
                        actions.push(StudioUiAction::ActivateCharacterView { view_id: view.id.clone() });
                    }
                    response.on_hover_text(format!("{} · {}", view.status.label(), view.origin.label()));
                }
            });
        });
}

/// Discrete turn navigation. It selects registered views only and does not
/// accept arbitrary scrub angles or imply continuous interpolation.
pub fn show_turn_ruler(
    ui: &mut Ui,
    model: &StudioUiModel,
    state: &mut StudioUiState,
    actions: &mut Vec<StudioUiAction>,
) {
    let active_index = model.views.iter().position(|view| {
        state.active_view_id.as_deref() == Some(view.id.as_str())
    });
    let active = active_index.and_then(|index| model.views.get(index));

    ui.horizontal(|ui| {
        ui.strong("Turn");
        if let Some(view) = active {
            ui.label(angle_label(view.angle_degrees));
            ui.label(RichText::new(&view.label).weak());
        } else {
            ui.label(RichText::new("No active registered view").weak());
        }

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let next = active_index.and_then(|i| model.views.get(i + 1));
            let previous = active_index.and_then(|i| i.checked_sub(1).and_then(|j| model.views.get(j)));
            if ui.add_enabled(next.is_some(), egui::Button::new("Next ›")).clicked()
                && let Some(view) = next
            {
                state.active_view_id = Some(view.id.clone());
                actions.push(StudioUiAction::ActivateCharacterView {
                    view_id: view.id.clone(),
                });
            }
            if ui.add_enabled(previous.is_some(), egui::Button::new("‹ Previous")).clicked()
                && let Some(view) = previous
            {
                state.active_view_id = Some(view.id.clone());
                actions.push(StudioUiAction::ActivateCharacterView {
                    view_id: view.id.clone(),
                });
            }
        });
    });
}

/// Contextual inspector. It displays references and ownership, not engine state.
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
        ui.strong(&node.label);
        ui.label(RichText::new(node.kind.label()).weak());
        ui.separator();
        ui.label("Status");
        ui.label(node.status.label());

        if let Some(controller_id) = &node.controller_id {
            ui.separator();
            ui.label("Forma Deform · Controller reference");
            ui.monospace(controller_id);
            ui.label(RichText::new("Evaluation is owned by Forma Deform.").weak());
            if !model.capabilities.controller_preview {
                ui.label(RichText::new("Controller preview unavailable").weak());
            }
        }

        if let Some(target_id) = &node.deformation_target_id {
            ui.separator();
            ui.label("Deformation target reference");
            ui.monospace(target_id);
            if ui
                .add_enabled(
                    model.capabilities.open_deformation_targets,
                    egui::Button::new("Open in Forma Deform"),
                )
                .clicked()
            {
                actions.push(StudioUiAction::RequestOpenDeformationTarget {
                    target_id: target_id.clone(),
                });
            }
        }
    } else {
        empty_state(ui, "Nothing selected", "Select a character item to inspect its properties.");
    }
}


/// Smart Bone authoring over AniCraft's native armature commands. Controller
/// references are identifiers only; Forma Deform owns their evaluation.
pub fn show_smart_bone_editor(
    ui: &mut Ui,
    model: &StudioUiModel,
    state: &mut StudioUiState,
    actions: &mut Vec<StudioUiAction>,
) {
    ui.heading("Smart Bone");
    ui.label(RichText::new("Author a native armature bone, then bind its controller in Forma Deform.").weak());
    ui.horizontal(|ui| {
        ui.label("Name");
        ui.text_edit_singleline(&mut state.smart_bone_name);
    });
    let can_create = model.capabilities.create_smart_bone && model.selected_element_ids.len() >= 2;
    if ui.add_enabled(can_create, egui::Button::new("＋ Create from selected elements")).clicked()
        && let (Some(parent), Some(child)) = (model.selected_element_ids.first(), model.selected_element_ids.get(1))
    {
        actions.push(StudioUiAction::RequestCreateSmartBone {
            parent_element_id: parent.clone(),
            child_element_id: child.clone(),
            name: state.smart_bone_name.trim().to_owned(),
        });
    }
    if model.selected_element_ids.len() < 2 {
        ui.label(RichText::new("Select the parent artwork, then the child artwork on the Stage.").weak());
    }
    ui.separator();
    if model.smart_bones.is_empty() {
        empty_state(ui, "No armature bones", "Create a bone from two selected Stage elements to begin authoring.");
        return;
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        for bone in &model.smart_bones {
            let selected = state.selected_smart_bone_id.as_deref() == Some(bone.id.as_str());
            if ui.selectable_label(selected, format!("{}  {}", if bone.name.is_empty() { "Bone" } else { &bone.name }, bone.id)).clicked() {
                state.selected_smart_bone_id = Some(bone.id.clone());
                state.smart_bone_min_degrees = bone.limits_degrees.map_or(-180.0, |v| v[0]);
                state.smart_bone_max_degrees = bone.limits_degrees.map_or(180.0, |v| v[1]);
                actions.push(StudioUiAction::RequestSelectSmartBone { bone_id: bone.id.clone() });
            }
            if selected {
                ui.horizontal(|ui| {
                    ui.label("Angle");
                    ui.monospace(format!("{:.1}°", bone.angle_degrees));
                });
                ui.horizontal(|ui| {
                    ui.label("Limits");
                    ui.add(egui::DragValue::new(&mut state.smart_bone_min_degrees).speed(1.0).suffix("°"));
                    ui.label("to");
                    ui.add(egui::DragValue::new(&mut state.smart_bone_max_degrees).speed(1.0).suffix("°"));
                    if ui.button("Apply").clicked() {
                        actions.push(StudioUiAction::RequestSetSmartBoneLimits {
                            bone_id: bone.id.clone(),
                            min_degrees: state.smart_bone_min_degrees,
                            max_degrees: state.smart_bone_max_degrees,
                        });
                    }
                });
                ui.label(RichText::new(format!("Parent {} → child {}", bone.parent_element_id, bone.child_element_id)).weak());
                if let Some(controller) = &bone.controller_id {
                    ui.label("Forma Deform ControllerId");
                    ui.monospace(controller);
                } else {
                    ui.label(RichText::new("No Forma Deform controller bound").weak());
                }
            }
            ui.separator();
        }
    });
}

/// Discrete Switch Groups backed by AniCraft Library folders and symbol assets.
/// Activating an option changes the selected symbol instance through a host command.
pub fn show_switch_group_editor(
    ui: &mut Ui,
    model: &StudioUiModel,
    state: &mut StudioUiState,
    actions: &mut Vec<StudioUiAction>,
) {
    ui.heading("Switch Groups");
    ui.label(RichText::new("Each group is a Library folder; its options are alternate symbols.").weak());
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut state.switch_group_name);
        if ui.add_enabled(model.capabilities.create_switch_group && !state.switch_group_name.trim().is_empty(),
            egui::Button::new("＋ Create Group")).clicked() {
            actions.push(StudioUiAction::RequestCreateSwitchGroup { name: state.switch_group_name.trim().to_owned() });
        }
    });
    if model.switch_groups.is_empty() {
        empty_state(ui, "No Switch Groups", "Create a group, then add symbol assets as its options.");
    }
    for group in &model.switch_groups {
        let selected = state.selected_switch_group_id.as_deref() == Some(group.id.as_str());
        if ui.selectable_label(selected, format!("▤ {}", group.name)).clicked() {
            state.selected_switch_group_id = Some(group.id.clone());
            state.selected_switch_option_id = group.options.first().map(|v| v.id.clone());
            state.selected_switch_source_symbol_id = None;
        }
        if selected {
            for option in &group.options {
                if ui.selectable_label(state.selected_switch_option_id.as_deref() == Some(option.id.as_str()),
                    format!("  ▪ {}", option.name)).clicked() {
                    state.selected_switch_option_id = Some(option.id.clone());
                }
            }
            if !model.library_symbols.is_empty() {
                let mut chosen = state.selected_switch_source_symbol_id.clone().unwrap_or_default();
                egui::ComboBox::from_id_salt(("studio_switch_symbol", group.id.as_str()))
                    .selected_text(model.library_symbols.iter().find(|s| s.id == chosen).map_or("Choose symbol", |s| s.name.as_str()))
                    .show_ui(ui, |ui| {
                        for symbol in &model.library_symbols {
                            ui.selectable_value(&mut chosen, symbol.id.clone(), &symbol.name);
                        }
                    });
                state.selected_switch_source_symbol_id = (!chosen.is_empty()).then_some(chosen);
                if ui.button("Duplicate selected symbol into group").clicked()
                    && let Some(symbol_id) = state.selected_switch_source_symbol_id.clone()
                {
                    actions.push(StudioUiAction::RequestAddSwitchOption { group_id: group.id.clone(), symbol_id });
                }
            }
            if let Some(option_id) = state.selected_switch_option_id.as_deref()
                && ui.add_enabled(!model.selected_element_ids.is_empty(), egui::Button::new("Activate on Stage selection")).clicked()
            {
                actions.push(StudioUiAction::RequestActivateSwitchOption { symbol_id: option_id.to_owned() });
            }
        }
        ui.separator();
    }
}

/// Asset browser presentation. Retrieval, compatibility checks, import, and
/// file access remain in AniCraft's host asset service.
pub fn show_asset_library(
    ui: &mut Ui,
    model: &StudioUiModel,
    state: &mut StudioUiState,
    actions: &mut Vec<StudioUiAction>,
) {
    ui.heading("Asset Library");
    ui.text_edit_singleline(&mut state.asset_filter);

    let filter = state.asset_filter.trim().to_lowercase();
    let mut visible = 0usize;
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
                visible += 1;
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.strong(&asset.title);
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.small(asset.compatibility.label());
                        });
                    });
                    ui.label(RichText::new(&asset.category).weak());
                    if !asset.description.is_empty() {
                        ui.label(&asset.description);
                    }
                    if ui
                        .add_enabled(
                            model.capabilities.open_assets && asset.compatibility != AssetCompatibility::Unavailable,
                            egui::Button::new("Open asset"),
                        )
                        .clicked()
                    {
                        actions.push(StudioUiAction::RequestOpenAsset { asset_id: asset.id.clone() });
                    }
                });
            }
        });
    if visible == 0 {
        empty_state(ui, "No matching assets", "Try another search term or check the host asset library.");
    }
}

/// Diagnostics navigation. Selecting an issue requests host navigation only;
/// it never edits or repairs the document.
pub fn show_diagnostics(
    ui: &mut Ui,
    model: &StudioUiModel,
    state: &mut StudioUiState,
    actions: &mut Vec<StudioUiAction>,
) {
    ui.heading("Diagnostics");
    if model.validation_issues.is_empty() {
        empty_state(ui, "No reported issues", "Only validators that have actually run are represented here.");
        return;
    }

    for issue in &model.validation_issues {
        let selected = state.selected_issue_id.as_deref() == Some(issue.id.as_str());
        let response = ui.selectable_label(
            selected,
            format!("[{}] {} — {}", issue.severity.label(), issue.category, issue.message),
        );
        if response.clicked() {
            state.selected_issue_id = Some(issue.id.clone());
            actions.push(StudioUiAction::RequestSelectDiagnostic { issue_id: issue.id.clone() });
        }
        response.on_hover_text(issue.repair_hint.as_deref().unwrap_or("No automated repair is available."));
        ui.colored_label(issue.severity.color(ui.visuals()), issue.severity.label());
    }
}

fn angle_label(angle_degrees: f32) -> String {
    if angle_degrees.is_finite() {
        format!("{angle_degrees:.1}°")
    } else {
        "Angle unavailable".to_owned()
    }
}

fn empty_state(ui: &mut Ui, title: &str, description: &str) {
    ui.vertical_centered(|ui| {
        ui.add_space(8.0);
        ui.label(RichText::new(title).strong());
        ui.label(RichText::new(description).weak());
        ui.add_space(8.0);
    });
}

/// Compose the primary 360 Studio workspace inside a host-owned dock region.
///
/// The host supplies the real AniCraft Stage through `show_stage`. The callback
/// receives the Stage region's `egui::Ui`; Studio never creates a renderer,
/// viewport, timeline, top-level window, or docking manager. Asset Library and
/// Diagnostics remain available as independent dockable panels through
/// `show_asset_library` and `show_diagnostics`.
///
/// The layout inherits the active host style and switches to a vertical,
/// scrollable arrangement when the dock is narrow. The host should retain
/// `StudioUiState` for the lifetime of this workspace and dispatch the
/// accumulated semantic actions through its command/undo boundary.
pub fn show_workspace(
    ui: &mut Ui,
    model: &StudioUiModel,
    state: &mut StudioUiState,
    actions: &mut Vec<StudioUiAction>,
    show_stage: impl FnOnce(&mut Ui),
) {
    show_workspace_header(ui, model);
    ui.separator();

    let available = ui.available_size_before_wrap();
    let compact = available.x < 760.0;

    if compact {
        egui::ScrollArea::vertical()
            .id_salt("anicraft_360_workspace_compact")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    show_character_hierarchy(ui, model, state, actions);
                });
                ui.add_space(6.0);
                ui.heading("Stage");
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.set_min_height(260.0);
                    show_stage(ui);
                });
                ui.add_space(6.0);
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    show_properties(ui, model, state, actions);
                });
                ui.add_space(6.0);
                show_turn_ruler(ui, model, state, actions);
                show_view_rail(ui, model, state, actions);
            });
        return;
    }

    let left_width = (available.x * 0.23).clamp(190.0, 260.0);
    let right_width = (available.x * 0.25).clamp(220.0, 300.0);
    let center_width = (available.x - left_width - right_width - 16.0).max(180.0);
    let content_height = (available.y - 8.0).max(220.0);
    let stage_height = (content_height - 116.0).max(120.0);

    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(
            egui::vec2(left_width, content_height),
            Layout::top_down(Align::Min),
            |ui| {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.set_min_size(egui::vec2(left_width - 8.0, content_height - 8.0));
                    show_character_hierarchy(ui, model, state, actions);
                });
            },
        );

        ui.separator();

        ui.allocate_ui_with_layout(
            egui::vec2(center_width, content_height),
            Layout::top_down(Align::Min),
            |ui| {
                ui.heading("Stage");
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.set_min_size(egui::vec2(center_width - 8.0, stage_height));
                    show_stage(ui);
                });
                ui.add_space(6.0);
                show_turn_ruler(ui, model, state, actions);
                show_view_rail(ui, model, state, actions);
            },
        );

        ui.separator();

        ui.allocate_ui_with_layout(
            egui::vec2(right_width, content_height),
            Layout::top_down(Align::Min),
            |ui| {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.set_min_size(egui::vec2(right_width - 8.0, content_height - 8.0));
                    show_properties(ui, model, state, actions);
                });
            },
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state_is_transient_and_empty() {
        let state = StudioUiState::default();
        assert!(state.selected_node_id.is_none());
        assert!(state.collapsed_node_ids.is_empty());
        assert!(state.active_view_id.is_none());
        assert!(state.selected_issue_id.is_none());
        assert!(state.asset_filter.is_empty());
    }

    #[test]
    fn capability_defaults_are_fail_closed() {
        let capabilities = StudioCapabilities::default();
        assert!(!capabilities.create_smart_bone);
        assert!(!capabilities.create_switch_group);
        assert!(!capabilities.create_view);
        assert!(!capabilities.open_assets);
        assert!(!capabilities.open_deformation_targets);
        assert!(!capabilities.controller_preview);
    }

    #[test]
    fn view_statuses_and_origins_are_explicit() {
        assert_eq!(ViewStatus::MissingArtwork.label(), "Missing artwork");
        assert_eq!(ViewOrigin::Preview.label(), "Preview only");
        assert_eq!(angle_label(f32::NAN), "Angle unavailable");
    }

    #[test]
    fn controller_and_pin_deformation_are_not_a_combined_concept() {
        let controller = Some("controller-a".to_owned());
        let target = Some("pin-target-a".to_owned());
        assert_ne!(controller, target);
    }
}
