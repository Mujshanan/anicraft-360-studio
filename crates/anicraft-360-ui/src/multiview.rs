//! Multi-view character authoring and artwork coverage feedback.
//!
//! The host supplies the canonical authored views and coverage counts. This
//! module does not generate artwork or interpolate between views; angle-based
//! selection chooses the nearest registered authored view only.

use egui::{Align, Layout, RichText, Ui};
use std::collections::BTreeMap;

use crate::CharacterView;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MultiViewKind {
    Front,
    ThreeQuarter,
    Side,
    Back,
}

impl MultiViewKind {
    pub const ALL: [Self; 4] = [Self::Front, Self::ThreeQuarter, Self::Side, Self::Back];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Front => "Front",
            Self::ThreeQuarter => "Three-quarter",
            Self::Side => "Side",
            Self::Back => "Back",
        }
    }

    pub const fn target_angle(self) -> f32 {
        match self {
            Self::Front => 0.0,
            Self::ThreeQuarter => 45.0,
            Self::Side => 90.0,
            Self::Back => 180.0,
        }
    }
}

/// Host-computed coverage for one authored view. Counts should represent
/// distinct required artwork elements, not pixels or inferred visibility.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewArtworkCoverage {
    pub view_id: String,
    pub required_artwork: usize,
    pub covered_artwork: usize,
}

impl ViewArtworkCoverage {
    pub fn missing_artwork(&self) -> usize {
        self.required_artwork.saturating_sub(self.covered_artwork)
    }

    pub fn percent(&self) -> Option<u8> {
        (self.required_artwork > 0).then(|| {
            ((self.covered_artwork.min(self.required_artwork) * 100)
                / self.required_artwork) as u8
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct MultiViewAuthoringState {
    pub target_angle_degrees: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MultiViewAction {
    ActivateView { view_id: String },
    RequestCreateView { kind: MultiViewKind, angle_degrees: f32 },
}

/// Multi-view authoring rail. The host owns view persistence and supplies
/// coverage snapshots; this panel emits semantic intents only.
pub fn show_multi_view_authoring(
    ui: &mut Ui,
    views: &[CharacterView],
    coverage: &[ViewArtworkCoverage],
    state: &mut MultiViewAuthoringState,
    actions: &mut Vec<MultiViewAction>,
) {
    ui.heading("Multi-view authoring");
    ui.label(RichText::new(
        "Register consistent turn views and track required artwork coverage per angle.",
    ).weak());

    let coverage_by_view: BTreeMap<&str, &ViewArtworkCoverage> = coverage
        .iter()
        .map(|entry| (entry.view_id.as_str(), entry))
        .collect();

    ui.horizontal_wrapped(|ui| {
        for kind in MultiViewKind::ALL {
            let angle = kind.target_angle();
            // A preset card represents an authored slot only when a view is
            // close enough to that canonical angle; otherwise it must remain creatable.
            let view = nearest_view(views, angle)
                .filter(|view| angular_distance(view.angle_degrees, angle) <= 22.5);
            ui.group(|ui| {
                ui.strong(kind.label());
                ui.label(format!("{angle:.0}°"));
                if let Some(view) = view {
                    ui.label(&view.label);
                    show_coverage(ui, coverage_by_view.get(view.id.as_str()).copied());
                    if ui.button("Activate view").clicked() {
                        actions.push(MultiViewAction::ActivateView { view_id: view.id.clone() });
                    }
                } else {
                    ui.label(RichText::new("No authored view").weak());
                    if ui.button("Create view").clicked() {
                        actions.push(MultiViewAction::RequestCreateView {
                            kind,
                            angle_degrees: angle,
                        });
                    }
                }
            });
        }
    });

    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Target angle");
        ui.add(egui::DragValue::new(&mut state.target_angle_degrees)
            .speed(1.0)
            .range(-180.0..=180.0)
            .suffix("°"));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if let Some(view) = nearest_view(views, state.target_angle_degrees) {
                if ui.button(format!("Activate nearest · {}", view.label)).clicked() {
                    actions.push(MultiViewAction::ActivateView { view_id: view.id.clone() });
                }
            } else {
                ui.label(RichText::new("No registered views").weak());
            }
        });
    });

    if views.is_empty() {
        ui.label(RichText::new(
            "Add Front, Three-quarter, Side and Back views to establish the character turn set.",
        ).weak());
    } else {
        let total_required: usize = coverage.iter().map(|item| item.required_artwork).sum();
        let total_covered: usize = coverage.iter()
            .map(|item| item.covered_artwork.min(item.required_artwork))
            .sum();
        let missing: usize = coverage.iter().map(ViewArtworkCoverage::missing_artwork).sum();
        ui.group(|ui| {
            ui.strong("Artwork coverage");
            if total_required == 0 {
                ui.label(RichText::new("Coverage not reported by host").weak());
            } else {
                ui.label(format!(
                    "{total_covered}/{total_required} required elements covered · {} missing",
                    missing,
                ));
                ui.add(egui::ProgressBar::new(
                    total_covered as f32 / total_required as f32,
                ).show_percentage());
            }
        });
    }
}

fn show_coverage(ui: &mut Ui, coverage: Option<&ViewArtworkCoverage>) {
    match coverage {
        Some(report) if report.required_artwork == 0 => {
            ui.label(RichText::new("No coverage requirements").weak());
        }
        Some(report) => {
            let covered = report.covered_artwork.min(report.required_artwork);
            let percent = report.percent().unwrap_or(0);
            ui.label(format!(
                "{covered}/{} · {percent}% covered",
                report.required_artwork,
            ));
            ui.add(egui::ProgressBar::new(percent as f32 / 100.0)
                .text(format!("{} missing", report.missing_artwork())));
        }
        None => {
            ui.label(RichText::new("Coverage unavailable").weak());
        }
    }
}

fn angular_distance(a: f32, b: f32) -> f32 {
    let delta = (a - b).rem_euclid(360.0);
    delta.min(360.0 - delta)
}

fn nearest_view(views: &[CharacterView], angle_degrees: f32) -> Option<&CharacterView> {
    if !angle_degrees.is_finite() {
        return None;
    }
    views.iter()
        .filter(|view| view.angle_degrees.is_finite())
        .min_by(|a, b| {
            angular_distance(a.angle_degrees, angle_degrees)
                .total_cmp(&angular_distance(b.angle_degrees, angle_degrees))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ViewOrigin, ViewStatus};

    fn view(id: &str, angle: f32) -> CharacterView {
        CharacterView {
            id: id.into(),
            label: id.into(),
            angle_degrees: angle,
            status: ViewStatus::Ready,
            origin: ViewOrigin::Authored,
        }
    }

    #[test]
    fn angle_selection_chooses_nearest_registered_view_and_wraps() {
        let views = vec![view("front", 0.0), view("side", 90.0), view("back", 180.0)];
        assert_eq!(nearest_view(&views, 72.0).map(|v| v.id.as_str()), Some("side"));
        assert_eq!(nearest_view(&views, 359.0).map(|v| v.id.as_str()), Some("front"));
        assert!(nearest_view(&views, f32::NAN).is_none());
    }

    #[test]
    fn preset_slots_do_not_claim_distant_views_as_authored() {
        let views = vec![view("front", 0.0)];
        assert!(nearest_view(&views, 45.0)
            .filter(|view| angular_distance(view.angle_degrees, 45.0) <= 22.5)
            .is_none());
        assert!(nearest_view(&views, 20.0)
            .filter(|view| angular_distance(view.angle_degrees, 20.0) <= 22.5)
            .is_some());
    }

    #[test]
    fn coverage_is_clamped_and_reports_missing_elements() {
        let report = ViewArtworkCoverage {
            view_id: "front".into(),
            required_artwork: 8,
            covered_artwork: 10,
        };
        assert_eq!(report.percent(), Some(100));
        assert_eq!(report.missing_artwork(), 0);
        let partial = ViewArtworkCoverage { covered_artwork: 5, ..report };
        assert_eq!(partial.percent(), Some(62));
        assert_eq!(partial.missing_artwork(), 3);
    }

    #[test]
    fn empty_coverage_has_no_percentage() {
        let report = ViewArtworkCoverage {
            view_id: "front".into(),
            required_artwork: 0,
            covered_artwork: 0,
        };
        assert_eq!(report.percent(), None);
    }
}
