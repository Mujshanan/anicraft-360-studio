use eframe::egui::{self, Align, Color32, FontId, Layout, Pos2, Rect, RichText, Sense, Shape, Stroke, Vec2};

const BG: Color32 = Color32::from_rgb(25, 27, 31);
const CHROME: Color32 = Color32::from_rgb(31, 34, 39);
const PANEL: Color32 = Color32::from_rgb(38, 41, 47);
const PANEL_ALT: Color32 = Color32::from_rgb(46, 50, 57);
const FIELD: Color32 = Color32::from_rgb(27, 30, 35);
const BORDER: Color32 = Color32::from_rgb(62, 67, 76);
const TEXT: Color32 = Color32::from_rgb(229, 232, 237);
const MUTED: Color32 = Color32::from_rgb(151, 158, 169);
const ACCENT: Color32 = Color32::from_rgb(65, 126, 232);
const MINT: Color32 = Color32::from_rgb(89, 185, 153);
const VIOLET: Color32 = Color32::from_rgb(145, 119, 214);
const RED: Color32 = Color32::from_rgb(220, 87, 92);

#[derive(Clone, Copy, PartialEq, Eq)]
enum ViewTab { Front, ThreeQuarter, Side, BackThreeQuarter, Back }
impl ViewTab {
    fn all() -> [(Self, &'static str, i32); 5] {
        [(Self::Front, "Front", 0), (Self::ThreeQuarter, "3/4", 45), (Self::Side, "Side", 90),
         (Self::BackThreeQuarter, "3/4", 135), (Self::Back, "Back", 180)]
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Selection { Character, Head, Body, SmartBone, SwitchGroup, View }
struct StudioApp {
    active_view: ViewTab,
    angle: f32,
    selection: Selection,
    show_landmarks: bool,
    show_silhouette: bool,
    show_rig: bool,
    playing: bool,
    status: &'static str,
}
impl Default for StudioApp {
    fn default() -> Self {
        Self { active_view: ViewTab::Front, angle: 0.0, selection: Selection::Head,
            show_landmarks: true, show_silhouette: false, show_rig: true, playing: false,
            status: "All changes are local to this prototype" }
    }
}
impl eframe::App for StudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals {
            dark_mode: true,
            panel_fill: PANEL,
            window_fill: PANEL,
            override_text_color: Some(TEXT),
            selection: egui::style::Selection { bg_fill: ACCENT, stroke: Stroke::new(1.0, Color32::WHITE) },
            widgets: egui::style::Widgets {
                noninteractive: egui::style::WidgetVisuals { bg_fill: PANEL, weak_bg_fill: PANEL, bg_stroke: Stroke::new(1.0, BORDER), fg_stroke: Stroke::new(1.0, MUTED), corner_radius: 3.into(), expansion: 0.0 },
                inactive: egui::style::WidgetVisuals { bg_fill: PANEL_ALT, weak_bg_fill: PANEL_ALT, bg_stroke: Stroke::NONE, fg_stroke: Stroke::new(1.0, TEXT), corner_radius: 3.into(), expansion: 0.0 },
                hovered: egui::style::WidgetVisuals { bg_fill: Color32::from_rgb(59, 65, 75), weak_bg_fill: Color32::from_rgb(59, 65, 75), bg_stroke: Stroke::new(1.0, ACCENT), fg_stroke: Stroke::new(1.0, TEXT), corner_radius: 3.into(), expansion: 0.0 },
                active: egui::style::WidgetVisuals { bg_fill: ACCENT, weak_bg_fill: ACCENT, bg_stroke: Stroke::new(1.0, ACCENT), fg_stroke: Stroke::new(1.0, Color32::WHITE), corner_radius: 3.into(), expansion: 0.0 },
                open: egui::style::WidgetVisuals { bg_fill: PANEL_ALT, weak_bg_fill: PANEL_ALT, bg_stroke: Stroke::new(1.0, ACCENT), fg_stroke: Stroke::new(1.0, TEXT), corner_radius: 3.into(), expansion: 0.0 },
            },
            ..Default::default()
        });
        egui::TopBottomPanel::top("topbar").frame(egui::Frame::new().fill(CHROME).inner_margin(egui::Margin::symmetric(12, 8))).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("◈  AniCraft").strong().size(15.0));
                ui.separator();
                ui.label(RichText::new("360 Studio").strong().color(Color32::from_rgb(133, 174, 255)));
                ui.separator();
                for item in ["File", "Edit", "View", "Character", "Rig", "Help"] { ui.label(RichText::new(item).color(MUTED)); }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new("●  LOCAL PROTOTYPE").size(10.0).color(MINT));
                    ui.label(RichText::new("character_turn_v03").monospace().color(MUTED));
                });
            });
        });
        egui::TopBottomPanel::bottom("status").frame(egui::Frame::new().fill(CHROME).inner_margin(egui::Margin::symmetric(12, 5))).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("●").color(MINT));
                ui.label(RichText::new(self.status).size(11.0).color(MUTED));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new("100%").monospace().color(MUTED));
                    ui.separator();
                    ui.label(RichText::new("24 fps").monospace().color(MUTED));
                    ui.separator();
                    ui.label(RichText::new("550 × 400").monospace().color(MUTED));
                });
            });
        });
        egui::TopBottomPanel::bottom("timeline").resizable(true).default_height(142.0).min_height(110.0).frame(egui::Frame::new().fill(PANEL).inner_margin(egui::Margin::same(8))).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("TIMELINE").strong().size(11.0).color(MUTED));
                ui.separator();
                if ui.button(if self.playing { "Ⅱ" } else { "▶" }).clicked() { self.playing = !self.playing; }
                ui.button("◀");
                ui.button("▶|");
                ui.label(RichText::new("Frame  001").monospace().color(TEXT));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| { ui.label(RichText::new("AniCraft timeline · host-owned").size(10.0).color(MUTED)); });
            });
            ui.add_space(6.0);
            let rows = [("Head", MINT), ("Body", MINT), ("Smart Bone · Blink", VIOLET), ("Turn views", ACCENT)];
            let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 78.0), Sense::hover());
            let p = ui.painter_at(rect);
            for i in 0..7 { let x = rect.left() + 104.0 + i as f32 * 74.0; p.line_segment([Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())], Stroke::new(1.0, BORDER)); p.text(Pos2::new(x + 4.0, rect.top() + 1.0), egui::Align2::LEFT_TOP, format!("{:02}", i * 5 + 1), FontId::monospace(9.0), MUTED); }
            for (i, (name, color)) in rows.iter().enumerate() {
                let y = rect.top() + 20.0 + i as f32 * 14.0;
                p.text(Pos2::new(rect.left() + 4.0, y), egui::Align2::LEFT_CENTER, *name, FontId::proportional(10.0), TEXT);
                let x = rect.left() + 104.0;
                p.rect_filled(Rect::from_min_size(Pos2::new(x + 3.0, y - 5.0), Vec2::new(260.0 - i as f32 * 18.0, 10.0)), 2.0, *color);
            }
            let play_x = rect.left() + 108.0;
            p.line_segment([Pos2::new(play_x, rect.top()), Pos2::new(play_x, rect.bottom())], Stroke::new(2.0, RED));
        });
        egui::SidePanel::left("hierarchy").resizable(true).default_width(218.0).min_width(185.0).frame(egui::Frame::new().fill(PANEL).inner_margin(egui::Margin::same(10))).show(ctx, |ui| {
            section_title(ui, "CHARACTER HIERARCHY", "＋");
            ui.add_space(8.0);
            ui.horizontal(|ui| { ui.label("▾"); ui.label(RichText::new("Mira · Character").strong()); });
            ui.indent("parts", |ui| {
                tree_item(ui, "▣", "Head", self.selection == Selection::Head, || self.selection = Selection::Head);
                tree_item(ui, "▣", "Body", self.selection == Selection::Body, || self.selection = Selection::Body);
                tree_item(ui, "▾", "Rig", false, || {});
                ui.indent("rig", |ui| {
                    tree_item(ui, "◇", "Root", false, || {});
                    tree_item(ui, "◇", "Head turn", self.selection == Selection::SmartBone, || self.selection = Selection::SmartBone);
                });
                tree_item(ui, "▾", "Switch Groups", self.selection == Selection::SwitchGroup, || self.selection = Selection::SwitchGroup);
                ui.indent("switch", |ui| { tree_item(ui, "◈", "Eye shape", false, || {}); tree_item(ui, "◈", "Mouth", false, || {}); });
            });
            ui.add_space(14.0);
            section_title(ui, "WORKSPACE", "⋯");
            for (icon, label) in [("▧", "Views"), ("◇", "Smart Bones"), ("◈", "Switch Groups"), ("▤", "Asset Library")] {
                ui.horizontal(|ui| { ui.label(RichText::new(icon).color(MUTED)); ui.label(label); });
            }
            ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                ui.separator();
                ui.label(RichText::new("DOCUMENT").size(10.0).strong().color(MUTED));
                ui.label(RichText::new("Mira_turn_v03.anicraft").size(11.0).monospace());
            });
        });
        egui::SidePanel::right("properties").resizable(true).default_width(250.0).min_width(215.0).frame(egui::Frame::new().fill(PANEL).inner_margin(egui::Margin::same(10))).show(ctx, |ui| {
            section_title(ui, "PROPERTIES", "⋯");
            ui.add_space(8.0);
            let name = match self.selection { Selection::Head => "Head", Selection::Body => "Body", Selection::SmartBone => "Head turn", Selection::SwitchGroup => "Eye shape", Selection::View => "Active view", Selection::Character => "Mira" };
            ui.label(RichText::new(name).size(15.0).strong());
            ui.label(RichText::new(match self.selection { Selection::SmartBone => "SMART BONE", Selection::SwitchGroup => "SWITCH GROUP", Selection::View => "VIEW REGISTRATION", _ => "CHARACTER PART" }).size(9.0).color(ACCENT).strong());
            ui.separator();
            match self.selection {
                Selection::SmartBone => {
                    property_label(ui, "Controller reference");
                    ui.label(RichText::new("forma.controller.head-turn").monospace().size(10.0).color(MUTED));
                    property_label(ui, "Trigger range");
                    ui.horizontal(|ui| { ui.add(egui::DragValue::new(&mut self.angle).speed(1.0).suffix("°")); ui.label("−90° to +90°"); });
                    property_label(ui, "Evaluation");
                    status_line(ui, "On-demand preview only", VIOLET);
                }
                Selection::SwitchGroup => {
                    property_label(ui, "Default option");
                    ui.radio_value(&mut self.angle, 0.0, "Open");
                    ui.radio_value(&mut self.angle, 1.0, "Closed");
                    property_label(ui, "Group state");
                    status_line(ui, "Draft · not persisted", MINT);
                }
                _ => {
                    property_label(ui, "Position");
                    two_values(ui, "X", "0.0", "Y", "0.0");
                    property_label(ui, "Scale");
                    two_values(ui, "W", "100%", "H", "100%");
                    property_label(ui, "Rotation");
                    ui.label(RichText::new("0.0°").monospace());
                    property_label(ui, "Overlays");
                    ui.checkbox(&mut self.show_rig, "Rig controls");
                    ui.checkbox(&mut self.show_landmarks, "Landmarks");
                    ui.checkbox(&mut self.show_silhouette, "Silhouette guide");
                }
            }
            ui.add_space(14.0);
            section_title(ui, "FORMA DEFORM", "↗");
            ui.label(RichText::new("Deformation preview").strong().size(11.0));
            status_line(ui, "Not connected", MUTED);
            ui.label(RichText::new("Controller and target handoff will be enabled when the host exposes a verified bridge.").size(10.0).color(MUTED));
            ui.add_space(10.0);
            ui.add_enabled(false, egui::Button::new("Open in Forma Deform"));
        });
        egui::CentralPanel::default().frame(egui::Frame::new().fill(BG).inner_margin(egui::Margin::same(10))).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("STAGE").strong().size(11.0).color(MUTED));
                ui.separator();
                ui.label(RichText::new("Mira /").color(MUTED));
                ui.label(RichText::new(match self.active_view { ViewTab::Front => "Front", ViewTab::ThreeQuarter => "Front 3/4", ViewTab::Side => "Side", ViewTab::BackThreeQuarter => "Back 3/4", ViewTab::Back => "Back" }).strong());
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| { ui.label(RichText::new("▣  Fit").color(MUTED)); ui.label(RichText::new("⌖  Select").color(MUTED)); });
            });
            ui.add_space(8.0);
            let available = ui.available_size();
            let stage_h = (available.y - 132.0).max(230.0);
            let (stage, response) = ui.allocate_exact_size(Vec2::new(available.x, stage_h), Sense::click_and_drag());
            paint_stage(ui, stage, self.active_view, self.show_landmarks, self.show_silhouette, self.show_rig);
            if response.clicked() { self.selection = Selection::Character; }
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("TURN VIEWS").strong().size(10.0).color(MUTED));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| { ui.label(RichText::new("5 authored views").size(10.0).color(MUTED)); });
            });
            ui.horizontal(|ui| {
                for (view, label, angle) in ViewTab::all() {
                    let selected = self.active_view == view;
                    let text = format!("{}   {}°", label, angle);
                    if ui.add_sized([78.0, 30.0], egui::Button::selectable(selected, text)).clicked() {
                        self.active_view = view; self.angle = angle as f32; self.selection = Selection::View;
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label(RichText::new("TURN ANGLE").size(9.0).strong().color(MUTED));
                ui.label(RichText::new(format!("{:.0}°", self.angle)).monospace().color(TEXT));
                let changed = ui.add(egui::Slider::new(&mut self.angle, 0.0..=180.0).show_value(false)).changed();
                if changed { self.status = "Angle changed · view interpolation is not yet evaluated"; }
            });
        });
    }
}
fn section_title(ui: &mut egui::Ui, title: &str, action: &str) {
    ui.horizontal(|ui| { ui.label(RichText::new(title).size(10.0).strong().color(MUTED)); ui.with_layout(Layout::right_to_left(Align::Center), |ui| { ui.label(RichText::new(action).color(MUTED)); }); });
    ui.separator();
}
fn tree_item(ui: &mut egui::Ui, icon: &str, label: &str, selected: bool, action: impl FnOnce()) {
    let response = ui.add_sized([ui.available_width(), 27.0], egui::Button::selectable(selected, format!("{}   {}", icon, label)).wrap());
    if response.clicked() { action(); }
}
fn property_label(ui: &mut egui::Ui, label: &str) { ui.add_space(10.0); ui.label(RichText::new(label).size(10.0).color(MUTED)); }
fn two_values(ui: &mut egui::Ui, a: &str, av: &str, b: &str, bv: &str) {
    ui.horizontal(|ui| { for (label, value) in [(a, av), (b, bv)] { ui.vertical(|ui| { ui.label(RichText::new(label).size(9.0).color(MUTED)); ui.label(RichText::new(value).monospace()); }); } });
}
fn status_line(ui: &mut egui::Ui, label: &str, color: Color32) { ui.horizontal(|ui| { ui.label(RichText::new("●").color(color)); ui.label(RichText::new(label).size(10.0)); }); }
fn paint_stage(ui: &egui::Ui, rect: Rect, view: ViewTab, landmarks: bool, silhouette: bool, rig: bool) {
    let p = ui.painter_at(rect);
    p.rect_filled(rect, 2.0, Color32::from_rgb(20, 22, 26));
    let center = rect.center();
    let scale = (rect.height() / 390.0).min(rect.width() / 410.0).max(0.55);
    let w = 100.0 * scale;
    let h = 150.0 * scale;
    for x in 0..((rect.width() / 24.0) as i32) { let xx = rect.left() + x as f32 * 24.0; p.line_segment([Pos2::new(xx, rect.top()), Pos2::new(xx, rect.bottom())], Stroke::new(0.5, Color32::from_gray(35))); }
    for y in 0..((rect.height() / 24.0) as i32) { let yy = rect.top() + y as f32 * 24.0; p.line_segment([Pos2::new(rect.left(), yy), Pos2::new(rect.right(), yy)], Stroke::new(0.5, Color32::from_gray(35))); }
    p.line_segment([Pos2::new(center.x, rect.top() + 12.0), Pos2::new(center.x, rect.bottom() - 12.0)], Stroke::new(1.0, Color32::from_gray(52)));
    let side_factor = match view { ViewTab::Front => 1.0, ViewTab::ThreeQuarter => 0.82, ViewTab::Side => 0.58, ViewTab::BackThreeQuarter => 0.82, ViewTab::Back => 1.0 };
    let head = Pos2::new(center.x, center.y - h * 0.63);
    let body = Rect::from_center_size(Pos2::new(center.x, center.y + h * 0.34), Vec2::new(w * side_factor, h * 0.95));
    let head_color = Color32::from_rgb(224, 174, 139);
    let body_color = Color32::from_rgb(91, 135, 184);
    p.circle_filled(head, w * 0.48 * side_factor, head_color);
    p.circle_stroke(head, w * 0.48 * side_factor, Stroke::new(1.5, Color32::from_rgb(246, 210, 181)));
    p.rect_filled(body, 22.0, body_color);
    p.rect_stroke(body, 22.0, Stroke::new(1.5, Color32::from_rgb(128, 167, 207)), egui::StrokeKind::Inside);
    let shoulder_y = center.y + h * 0.02;
    for dir in [-1.0, 1.0] {
        let start = Pos2::new(center.x + dir * w * 0.35 * side_factor, shoulder_y);
        let end = Pos2::new(center.x + dir * w * 0.74 * side_factor, center.y + h * 0.62);
        p.line_segment([start, end], Stroke::new(15.0 * scale, head_color));
        p.line_segment([start, end], Stroke::new(1.0, Color32::from_rgb(246, 210, 181)));
    }
    let eye_offset = if view == ViewTab::Side { w * 0.18 } else { w * 0.16 * side_factor };
    if !matches!(view, ViewTab::Back) {
        p.circle_filled(Pos2::new(head.x - eye_offset, head.y - 2.0), 3.2 * scale, Color32::from_rgb(55, 45, 43));
        if view != ViewTab::Side { p.circle_filled(Pos2::new(head.x + eye_offset, head.y - 2.0), 3.2 * scale, Color32::from_rgb(55, 45, 43)); }
        p.line_segment([Pos2::new(head.x - 8.0 * scale, head.y + 18.0 * scale), Pos2::new(head.x + 8.0 * scale, head.y + 18.0 * scale)], Stroke::new(1.5, Color32::from_rgb(150, 75, 70)));
    } else {
        p.line_segment([Pos2::new(head.x, head.y - 12.0 * scale), Pos2::new(head.x, head.y + 22.0 * scale)], Stroke::new(2.0, Color32::from_rgb(150, 75, 70)));
    }
    if silhouette { p.circle_stroke(head, w * 0.53, Stroke::new(1.0, VIOLET)); p.rect_stroke(body.expand(5.0), 24.0, Stroke::new(1.0, VIOLET), egui::StrokeKind::Inside); }
    if landmarks {
        for (name, pos) in [("Eye", Pos2::new(head.x, head.y - 2.0)), ("Chin", Pos2::new(head.x, head.y + w * 0.38)), ("Shoulder", Pos2::new(center.x, shoulder_y))] {
            p.circle_filled(pos, 4.0, ACCENT); p.circle_stroke(pos, 6.5, Stroke::new(1.0, Color32::WHITE));
            p.text(pos + Vec2::new(9.0, -7.0), egui::Align2::LEFT_CENTER, name, FontId::proportional(9.0), Color32::from_rgb(170, 194, 245));
        }
    }
    if rig {
        let root = Pos2::new(center.x, center.y + h * 0.68);
        p.circle_filled(root, 4.5, MINT);
        p.line_segment([root, Pos2::new(center.x, center.y + h * 0.02)], Stroke::new(1.5, MINT));
        p.circle_filled(Pos2::new(center.x, center.y + h * 0.02), 4.0, MINT);
        p.line_segment([Pos2::new(center.x, center.y + h * 0.02), head], Stroke::new(1.5, MINT));
    }
    p.text(rect.left_top() + Vec2::new(12.0, 10.0), egui::Align2::LEFT_TOP, "STAGE PREVIEW  ·  SAMPLE ARTWORK", FontId::monospace(9.0), MUTED);
    p.text(rect.right_bottom() + Vec2::new(-12.0, -10.0), egui::Align2::RIGHT_BOTTOM, "550 × 400", FontId::monospace(9.0), MUTED);
}
fn main() -> eframe::Result {
    let options = eframe::NativeOptions { viewport: egui::ViewportBuilder::default().with_inner_size([1440.0, 900.0]).with_min_inner_size([1050.0, 700.0]), ..Default::default() };
    eframe::run_native("AniCraft 360 Studio · UI Prototype", options, Box::new(|_cc| Ok(Box::<StudioApp>::default())))
}
