//! Persistent, host-independent character-authoring data for AniCraft 360 Studio.
//!
//! This crate owns authored character semantics only. AniCraft remains the
//! authority for document storage, symbols, bones, rendering, undo and timeline.
//! Forma Deform remains the authority for controller definitions/evaluation,
//! deformation targets and deformation algorithms.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

/// Current on-disk schema version. Changes require an explicit migration.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;
/// Tolerance used only when detecting coincident authored turn angles.
pub const ANGLE_EPSILON_DEGREES: f64 = 1.0e-6;

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn is_empty(&self) -> bool {
                self.0.trim().is_empty()
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }
    };
}

// Studio-owned identities.
id_type!(CharacterId);
id_type!(CharacterPartId);
id_type!(ViewSetId);
id_type!(ViewId);
id_type!(LandmarkId);
id_type!(SmartBoneId);
id_type!(SwitchGroupId);
id_type!(SwitchOptionId);
id_type!(AssetReferenceId);

// References to identities owned by AniCraft or Forma Deform. These are
// references only: this crate never defines or evaluates those host objects.
id_type!(HostSymbolId);
id_type!(HostBoneId);
id_type!(HostLibraryFolderId);
id_type!(HostAssetId);
id_type!(ControllerId);
id_type!(DeformationTargetId);

/// The complete persistent Studio-owned extension for one AniCraft character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterDocument {
    pub schema_version: u32,
    pub id: CharacterId,
    pub name: String,
    pub root_part_id: Option<CharacterPartId>,
    pub parts: Vec<CharacterPart>,
    pub view_sets: Vec<ViewSet>,
    pub smart_bones: Vec<SmartBoneDefinition>,
    pub switch_groups: Vec<SwitchGroup>,
    pub assets: Vec<AssetReference>,
}

impl CharacterDocument {
    pub fn new(id: impl Into<CharacterId>, name: impl Into<String>) -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            id: id.into(),
            name: name.into(),
            root_part_id: None,
            parts: Vec::new(),
            view_sets: Vec::new(),
            smart_bones: Vec::new(),
            switch_groups: Vec::new(),
            assets: Vec::new(),
        }
    }

    /// Validate all Studio-owned identities and cross-references.
    ///
    /// References into AniCraft and Forma Deform are checked by the host
    /// adapter against the active document/capabilities, not guessed here.
    pub fn validate(&self) -> Result<(), Vec<ModelIssue>> {
        let mut issues = Vec::new();
        if self.schema_version != CURRENT_SCHEMA_VERSION {
            issue(&mut issues, "schema.unsupported", "schema_version", format!(
                "Unsupported schema version {}; this build supports {}",
                self.schema_version, CURRENT_SCHEMA_VERSION
            ));
        }
        check_id(&mut issues, "character.id.empty", "id", self.id.is_empty());
        check_text(&mut issues, "character.name.empty", "name", &self.name);

        let part_ids = unique_ids(
            &mut issues,
            "part.id.duplicate",
            "parts",
            self.parts.iter().map(|part| (&part.id.0, "parts")),
        );
        let view_set_ids = unique_ids(
            &mut issues,
            "view_set.id.duplicate",
            "view_sets",
            self.view_sets.iter().map(|set| (&set.id.0, "view_sets")),
        );
        let smart_bone_ids = unique_ids(
            &mut issues,
            "smart_bone.id.duplicate",
            "smart_bones",
            self.smart_bones.iter().map(|item| (&item.id.0, "smart_bones")),
        );
        let switch_group_ids = unique_ids(
            &mut issues,
            "switch_group.id.duplicate",
            "switch_groups",
            self.switch_groups.iter().map(|item| (&item.id.0, "switch_groups")),
        );
        let asset_ids = unique_ids(
            &mut issues,
            "asset.id.duplicate",
            "assets",
            self.assets.iter().map(|item| (&item.id.0, "assets")),
        );
        let _ = (smart_bone_ids, switch_group_ids, asset_ids);

        if let Some(root) = &self.root_part_id {
            if !part_ids.contains(root.as_str()) {
                issue(&mut issues, "part.root.missing", "root_part_id", "Root part does not exist");
            }
        }

        let part_map: HashMap<&str, &CharacterPart> =
            self.parts.iter().map(|part| (part.id.as_str(), part)).collect();
        for (index, part) in self.parts.iter().enumerate() {
            let path = format!("parts[{index}]");
            check_id(&mut issues, "part.id.empty", &format!("{path}.id"), part.id.is_empty());
            check_text(&mut issues, "part.name.empty", &format!("{path}.name"), &part.name);
            if let Some(symbol_id) = &part.host_symbol_id {
                check_id(&mut issues, "part.host_symbol.empty", &format!("{path}.host_symbol_id"), symbol_id.is_empty());
            }
            if let Some(parent) = &part.parent_part_id {
                if parent == &part.id {
                    issue(&mut issues, "part.parent.self", format!("{path}.parent_part_id"), "A part cannot be its own parent");
                } else if !part_ids.contains(parent.as_str()) {
                    issue(&mut issues, "part.parent.missing", format!("{path}.parent_part_id"), "Parent part does not exist");
                }
            }
        }
        // Detect cycles in the Studio-owned character-part hierarchy.
        for part in &self.parts {
            let mut visited = HashSet::new();
            let mut cursor = Some(part.id.as_str());
            while let Some(id) = cursor {
                if !visited.insert(id) {
                    issue(&mut issues, "part.parent.cycle", "parts", format!("Parent cycle includes part '{id}'"));
                    break;
                }
                cursor = part_map.get(id).and_then(|node| node.parent_part_id.as_ref()).map(|id| id.as_str());
            }
        }

        for (set_index, set) in self.view_sets.iter().enumerate() {
            let base = format!("view_sets[{set_index}]");
            check_id(&mut issues, "view_set.id.empty", &format!("{base}.id"), set.id.is_empty());
            check_text(&mut issues, "view_set.name.empty", &format!("{base}.name"), &set.name);
            if !set.views.iter().any(|view| view.id == set.zero_angle_view_id) {
                issue(&mut issues, "view_set.zero_view.missing", format!("{base}.zero_angle_view_id"),
                    "Zero-angle reference must identify a view in this ViewSet");
            }
            let view_ids = unique_ids(
                &mut issues,
                "view.id.duplicate",
                &format!("{base}.views"),
                set.views.iter().map(|view| (&view.id.0, "views")),
            );
            let mut angles: Vec<(f64, usize)> = Vec::new();
            for (view_index, view) in set.views.iter().enumerate() {
                let path = format!("{base}.views[{view_index}]");
                check_id(&mut issues, "view.id.empty", &format!("{path}.id"), view.id.is_empty());
                check_text(&mut issues, "view.name.empty", &format!("{path}.name"), &view.name);
                if let Some(symbol_id) = &view.artwork_symbol_id {
                    check_id(&mut issues, "view.artwork_symbol.empty", &format!("{path}.artwork_symbol_id"), symbol_id.is_empty());
                }
                if !view.angle_degrees.is_finite()
                    || !(0.0..360.0).contains(&view.angle_degrees)
                {
                    issue(&mut issues, "view.angle.invalid", format!("{path}.angle_degrees"),
                        "Turn angles must be finite and in the half-open range [0, 360) degrees");
                } else {
                    angles.push((view.angle_degrees, view_index));
                }
            }
            angles.sort_by(|a, b| a.0.total_cmp(&b.0));
            for pair in angles.windows(2) {
                if (pair[1].0 - pair[0].0).abs() <= ANGLE_EPSILON_DEGREES {
                    issue(&mut issues, "view.angle.duplicate", format!("{base}.views"),
                        format!("Views {} and {} occupy the same turn angle", pair[0].1, pair[1].1));
                }
            }
            let landmark_ids = unique_ids(
                &mut issues,
                "landmark.id.duplicate",
                &format!("{base}.landmarks"),
                set.landmarks.iter().map(|item| (&item.id.0, "landmarks")),
            );
            let _ = landmark_ids;
            for (landmark_index, landmark) in set.landmarks.iter().enumerate() {
                let path = format!("{base}.landmarks[{landmark_index}]");
                check_id(&mut issues, "landmark.id.empty", &format!("{path}.id"), landmark.id.is_empty());
                check_text(&mut issues, "landmark.name.empty", &format!("{path}.name"), &landmark.name);
                for (view_id, point) in &landmark.positions {
                    if !view_ids.contains(view_id.as_str()) {
                        issue(&mut issues, "landmark.view.missing", format!("{path}.positions"),
                            format!("Landmark references unknown view '{}'", view_id.as_str()));
                    }
                    if !point.is_normalized() {
                        issue(&mut issues, "landmark.position.invalid", format!("{path}.positions"),
                            "Landmark coordinates must be finite normalized values in [0, 1]");
                    }
                }
            }
        }

        for (index, bone) in self.smart_bones.iter().enumerate() {
            let path = format!("smart_bones[{index}]");
            check_id(&mut issues, "smart_bone.id.empty", &format!("{path}.id"), bone.id.is_empty());
            check_text(&mut issues, "smart_bone.name.empty", &format!("{path}.name"), &bone.name);
            check_id(&mut issues, "smart_bone.host_bone.empty", &format!("{path}.host_bone_id"), bone.host_bone_id.is_empty());
            if let Some(controller_id) = &bone.controller_id {
                check_id(&mut issues, "smart_bone.controller.empty", &format!("{path}.controller_id"), controller_id.is_empty());
            }
            for (target_index, target_id) in bone.corrective_targets.iter().enumerate() {
                check_id(&mut issues, "smart_bone.target.empty", &format!("{path}.corrective_targets[{target_index}]"), target_id.is_empty());
            }
            if !bone.trigger_range.min_degrees.is_finite()
                || !bone.trigger_range.max_degrees.is_finite()
                || bone.trigger_range.min_degrees >= bone.trigger_range.max_degrees
            {
                issue(&mut issues, "smart_bone.range.invalid", format!("{path}.trigger_range"),
                    "Trigger range must have finite bounds with min_degrees < max_degrees");
            }
            if let Some(set_id) = &bone.view_set_id {
                if !view_set_ids.contains(set_id.as_str()) {
                    issue(&mut issues, "smart_bone.view_set.missing", format!("{path}.view_set_id"),
                        "Referenced view set does not exist");
                }
            }
            for part_id in &bone.affected_part_ids {
                if !part_ids.contains(part_id.as_str()) {
                    issue(&mut issues, "smart_bone.part.missing", format!("{path}.affected_part_ids"),
                        format!("Affected character part '{}' does not exist", part_id.as_str()));
                }
            }
        }

        for (group_index, group) in self.switch_groups.iter().enumerate() {
            let path = format!("switch_groups[{group_index}]");
            check_id(&mut issues, "switch_group.id.empty", &format!("{path}.id"), group.id.is_empty());
            check_text(&mut issues, "switch_group.name.empty", &format!("{path}.name"), &group.name);
            if let Some(folder_id) = &group.host_library_folder_id {
                check_id(&mut issues, "switch_group.library_folder.empty", &format!("{path}.host_library_folder_id"), folder_id.is_empty());
            }
            if let Some(set_id) = &group.view_set_id {
                if !view_set_ids.contains(set_id.as_str()) {
                    issue(&mut issues, "switch_group.view_set.missing", format!("{path}.view_set_id"),
                        "Referenced view set does not exist");
                }
            }
            let option_ids = unique_ids(
                &mut issues,
                "switch_option.id.duplicate",
                &format!("{path}.options"),
                group.options.iter().map(|option| (&option.id.0, "options")),
            );
            let mut symbols = BTreeSet::new();
            for (option_index, option) in group.options.iter().enumerate() {
                let option_path = format!("{path}.options[{option_index}]");
                check_id(&mut issues, "switch_option.id.empty", &format!("{option_path}.id"), option.id.is_empty());
                check_text(&mut issues, "switch_option.name.empty", &format!("{option_path}.name"), &option.name);
                check_id(&mut issues, "switch_option.symbol.empty", &format!("{option_path}.symbol_id"), option.symbol_id.is_empty());
                if !symbols.insert(option.symbol_id.as_str()) {
                    issue(&mut issues, "switch_option.symbol.duplicate", format!("{option_path}.symbol_id"),
                        "A symbol may appear only once in a Switch Group");
                }
                if let Some(view_id) = &option.view_id {
                    let view_exists = group.view_set_id.as_ref()
                        .and_then(|id| self.view_sets.iter().find(|set| &set.id == id))
                        .is_some_and(|set| set.views.iter().any(|view| &view.id == view_id));
                    if !view_exists {
                        issue(&mut issues, "switch_option.view.missing", format!("{option_path}.view_id"),
                            "Option view must exist in the Switch Group's view set");
                    }
                }
            }
            if let Some(default_id) = &group.default_option_id {
                if !option_ids.contains(default_id.as_str()) {
                    issue(&mut issues, "switch_group.default.missing", format!("{path}.default_option_id"),
                        "Default option must identify an option in this group");
                }
            } else if !group.options.is_empty() {
                issue(&mut issues, "switch_group.default.unset", format!("{path}.default_option_id"),
                    "A non-empty Switch Group must declare its default option");
            }
        }

        for (index, asset) in self.assets.iter().enumerate() {
            let path = format!("assets[{index}]");
            check_id(&mut issues, "asset.id.empty", &format!("{path}.id"), asset.id.is_empty());
            check_id(&mut issues, "asset.host_id.empty", &format!("{path}.host_asset_id"), asset.host_asset_id.is_empty());
            check_text(&mut issues, "asset.role.empty", &format!("{path}.role"), &asset.role);
        }

        if issues.is_empty() { Ok(()) } else { Err(issues) }
    }

    /// Serialize only valid, supported documents.
    pub fn to_json(&self) -> Result<String, PersistenceError> {
        self.validate().map_err(PersistenceError::Invalid)?;
        serde_json::to_string_pretty(self).map_err(|error| PersistenceError::Json(error.to_string()))
    }

    /// Read a document without silently accepting unknown schema versions or invalid references.
    pub fn from_json(json: &str) -> Result<Self, PersistenceError> {
        let document: Self = serde_json::from_str(json)
            .map_err(|error| PersistenceError::Json(error.to_string()))?;
        document.validate().map_err(PersistenceError::Invalid)?;
        Ok(document)
    }
}

fn unique_ids<'a>(
    issues: &mut Vec<ModelIssue>,
    code: &'static str,
    path: &str,
    ids: impl Iterator<Item = (&'a String, &'static str)>,
) -> BTreeSet<&'a str> {
    let mut seen = BTreeSet::new();
    for (id, _) in ids {
        if id.trim().is_empty() {
            issue(issues, "identity.empty", path, "Stable IDs cannot be empty");
        } else if !seen.insert(id.as_str()) {
            issue(issues, code, path, format!("Duplicate stable ID '{id}'"));
        }
    }
    seen
}

fn check_id(issues: &mut Vec<ModelIssue>, code: &'static str, path: &str, invalid: bool) {
    if invalid { issue(issues, code, path, "Stable ID must not be empty"); }
}

fn check_text(issues: &mut Vec<ModelIssue>, code: &'static str, path: &str, value: &str) {
    if value.trim().is_empty() { issue(issues, code, path, "Value must not be empty"); }
}

fn issue(issues: &mut Vec<ModelIssue>, code: &'static str, path: impl Into<String>, message: impl Into<String>) {
    issues.push(ModelIssue { code, path: path.into(), message: message.into() });
}

/// A machine-readable validation diagnostic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelIssue {
    pub code: &'static str,
    pub path: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PersistenceError {
    Json(String),
    Invalid(Vec<ModelIssue>),
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(message) => write!(f, "Character document JSON error: {message}"),
            Self::Invalid(issues) => write!(f, "Character document failed validation ({} issue(s))", issues.len()),
        }
    }
}

impl std::error::Error for PersistenceError {}

/// A character part is an organizational node that references host-owned artwork.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterPart {
    pub id: CharacterPartId,
    pub name: String,
    pub role: CharacterPartRole,
    pub parent_part_id: Option<CharacterPartId>,
    pub host_symbol_id: Option<HostSymbolId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharacterPartRole {
    Group,
    Artwork,
    Limb,
    Head,
    Torso,
    Accessory,
    Other,
}

/// One authored turn/view set. Studio stores discrete views, not interpolated poses.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ViewSet {
    pub id: ViewSetId,
    pub name: String,
    pub direction: TurnDirection,
    /// The authored view defining zero degrees for this set.
    pub zero_angle_view_id: ViewId,
    pub views: Vec<CharacterView>,
    pub landmarks: Vec<LandmarkTrack>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TurnDirection {
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterView {
    pub id: ViewId,
    pub name: String,
    /// Normalized authored angle in degrees: finite and in [0, 360).
    pub angle_degrees: f64,
    pub artwork_symbol_id: Option<HostSymbolId>,
    pub notes: String,
}

/// A landmark has one stable identity across all views in a ViewSet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LandmarkTrack {
    pub id: LandmarkId,
    pub name: String,
    pub role: LandmarkRole,
    /// Coordinates are normalized to the authored view bounds, origin top-left.
    pub positions: BTreeMap<ViewId, NormalizedPoint>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LandmarkRole {
    Centerline,
    Silhouette,
    Joint,
    FacialFeature,
    ProportionGuide,
    Custom,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct NormalizedPoint {
    pub x: f64,
    pub y: f64,
}

impl NormalizedPoint {
    pub fn is_normalized(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
            && (0.0..=1.0).contains(&self.x)
            && (0.0..=1.0).contains(&self.y)
    }
}

/// Character-specific Smart Bone authoring metadata. The referenced bone and
/// controller remain owned by AniCraft and Forma Deform respectively.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartBoneDefinition {
    pub id: SmartBoneId,
    pub name: String,
    pub host_bone_id: HostBoneId,
    pub controller_id: Option<ControllerId>,
    pub view_set_id: Option<ViewSetId>,
    pub trigger_range: TriggerRange,
    pub affected_part_ids: Vec<CharacterPartId>,
    pub corrective_targets: Vec<DeformationTargetId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerRange {
    pub min_degrees: f64,
    pub max_degrees: f64,
}

/// Mutually exclusive alternatives. Actual symbol swapping is performed by AniCraft.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchGroup {
    pub id: SwitchGroupId,
    pub name: String,
    pub host_library_folder_id: Option<HostLibraryFolderId>,
    pub view_set_id: Option<ViewSetId>,
    pub default_option_id: Option<SwitchOptionId>,
    pub options: Vec<SwitchOption>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchOption {
    pub id: SwitchOptionId,
    pub name: String,
    pub symbol_id: HostSymbolId,
    pub view_id: Option<ViewId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetReference {
    pub id: AssetReferenceId,
    pub host_asset_id: HostAssetId,
    pub role: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_document() -> CharacterDocument {
        let mut document = CharacterDocument::new("character-1", "Test Character");
        let root = CharacterPartId::from("part-root");
        document.root_part_id = Some(root.clone());
        document.parts.push(CharacterPart {
            id: root.clone(),
            name: "Character".into(),
            role: CharacterPartRole::Group,
            parent_part_id: None,
            host_symbol_id: None,
        });
        let front = ViewId::from("view-front");
        let side = ViewId::from("view-side");
        document.view_sets.push(ViewSet {
            id: ViewSetId::from("turn-main"),
            name: "Main turn".into(),
            direction: TurnDirection::Clockwise,
            zero_angle_view_id: front.clone(),
            views: vec![
                CharacterView { id: front.clone(), name: "Front".into(), angle_degrees: 0.0, artwork_symbol_id: None, notes: String::new() },
                CharacterView { id: side.clone(), name: "Side".into(), angle_degrees: 90.0, artwork_symbol_id: Some(HostSymbolId::from("symbol-side")), notes: String::new() },
            ],
            landmarks: vec![LandmarkTrack {
                id: LandmarkId::from("landmark-nose"),
                name: "Nose tip".into(),
                role: LandmarkRole::FacialFeature,
                positions: BTreeMap::from([
                    (front, NormalizedPoint { x: 0.5, y: 0.3 }),
                    (side, NormalizedPoint { x: 0.62, y: 0.31 }),
                ]),
            }],
        });
        document.smart_bones.push(SmartBoneDefinition {
            id: SmartBoneId::from("smartbone-head-turn"),
            name: "Head turn".into(),
            host_bone_id: HostBoneId::from("bone-head"),
            controller_id: Some(ControllerId::from("fd-controller-head")),
            view_set_id: Some(ViewSetId::from("turn-main")),
            trigger_range: TriggerRange { min_degrees: -45.0, max_degrees: 45.0 },
            affected_part_ids: vec![root],
            corrective_targets: vec![DeformationTargetId::from("fd-target-head")],
        });
        document.switch_groups.push(SwitchGroup {
            id: SwitchGroupId::from("switch-mouth"),
            name: "Mouth".into(),
            host_library_folder_id: None,
            view_set_id: Some(ViewSetId::from("turn-main")),
            default_option_id: Some(SwitchOptionId::from("mouth-closed")),
            options: vec![
                SwitchOption { id: SwitchOptionId::from("mouth-closed"), name: "Closed".into(), symbol_id: HostSymbolId::from("symbol-mouth-closed"), view_id: None },
                SwitchOption { id: SwitchOptionId::from("mouth-open"), name: "Open".into(), symbol_id: HostSymbolId::from("symbol-mouth-open"), view_id: None },
            ],
        });
        document
    }

    #[test]
    fn valid_character_document_round_trips() {
        let document = valid_document();
        let json = document.to_json().expect("serialize valid document");
        let decoded = CharacterDocument::from_json(&json).expect("deserialize valid document");
        assert_eq!(decoded, document);
    }

    #[test]
    fn rejects_unsupported_schema_versions() {
        let mut document = valid_document();
        document.schema_version = CURRENT_SCHEMA_VERSION + 1;
        let errors = document.validate().expect_err("future schema must fail closed");
        assert!(errors.iter().any(|error| error.code == "schema.unsupported"));
    }

    #[test]
    fn rejects_duplicate_or_out_of_range_view_angles() {
        let mut document = valid_document();
        document.view_sets[0].views[1].angle_degrees = 360.0;
        let errors = document.validate().expect_err("360 is outside normalized range");
        assert!(errors.iter().any(|error| error.code == "view.angle.invalid"));
        document.view_sets[0].views[1].angle_degrees = ANGLE_EPSILON_DEGREES / 2.0;
        let errors = document.validate().expect_err("coincident views must fail");
        assert!(errors.iter().any(|error| error.code == "view.angle.duplicate"));
    }

    #[test]
    fn rejects_missing_landmark_views_and_non_normalized_points() {
        let mut document = valid_document();
        let landmark = &mut document.view_sets[0].landmarks[0];
        landmark.positions.insert(ViewId::from("missing"), NormalizedPoint { x: 1.2, y: 0.5 });
        let errors = document.validate().expect_err("invalid correspondence must fail");
        assert!(errors.iter().any(|error| error.code == "landmark.view.missing"));
        assert!(errors.iter().any(|error| error.code == "landmark.position.invalid"));
    }

    #[test]
    fn rejects_parent_cycles() {
        let mut document = valid_document();
        let root = document.parts[0].id.clone();
        document.parts[0].parent_part_id = Some(root);
        let errors = document.validate().expect_err("self parent is a cycle");
        assert!(errors.iter().any(|error| error.code == "part.parent.self"));
        assert!(errors.iter().any(|error| error.code == "part.parent.cycle"));
    }

    #[test]
    fn switch_group_default_must_belong_to_group() {
        let mut document = valid_document();
        document.switch_groups[0].default_option_id = Some(SwitchOptionId::from("not-an-option"));
        let errors = document.validate().expect_err("missing default must fail");
        assert!(errors.iter().any(|error| error.code == "switch_group.default.missing"));
    }

    #[test]
    fn smart_bones_reference_external_controllers_without_defining_them() {
        let document = valid_document();
        let smart_bone = &document.smart_bones[0];
        assert_eq!(smart_bone.controller_id.as_ref().unwrap().as_str(), "fd-controller-head");
        assert_eq!(smart_bone.corrective_targets[0].as_str(), "fd-target-head");
    }
}
