//! AniCraft-native panel adapter. Embed with `anicraft_host::show(ui, &mut app.s)`.
//! This accepts the host's live Session; it never creates a second document.
use anicraft_engine::{Session, doc::{ElementId, LayerId}};
use egui::{self, ColorImage, RichText, Sense, TextureOptions, Vec2};
use serde_json::json;

/// Draw Studio against AniCraft's live document, selection, evaluated stage and timeline.
pub fn show(ui: &mut egui::Ui, session: &mut Session) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("ANICRAFT 360 STUDIO").strong());
        ui.separator();
        ui.label(session.path.as_ref().map_or_else(|| "Untitled document".to_owned(), |p| p.file_name().unwrap_or_default().to_string_lossy().into_owned()));
        ui.separator();
        ui.label(format!("Scene {} · Frame {} / {}", session.scene + 1, session.frame + 1, session.frame_count()));
        if session.is_dirty() { ui.label(RichText::new("● Modified").color(egui::Color32::from_rgb(230, 170, 80))); }
    });
    ui.separator();
    let avail = ui.available_size();
    let left = 180.0_f32.min(avail.x * 0.23).max(130.0);
    let right = 205.0_f32.min(avail.x * 0.25).max(155.0);
    ui.horizontal_top(|ui| {
        ui.allocate_ui_with_layout(Vec2::new(left, avail.y), egui::Layout::top_down(egui::Align::Min), |ui| hierarchy(ui, session));
        ui.separator();
        ui.allocate_ui_with_layout(Vec2::new((avail.x-left-right-24.0).max(220.0), avail.y), egui::Layout::top_down(egui::Align::Min), |ui| { stage(ui, session); ui.add_space(8.0); timeline(ui, session); });
        ui.separator();
        ui.allocate_ui_with_layout(Vec2::new(right, avail.y), egui::Layout::top_down(egui::Align::Min), |ui| properties(ui, session));
    });
}

fn hierarchy(ui: &mut egui::Ui, s: &mut Session) {
    ui.label(RichText::new("DOCUMENT / LAYERS").strong().size(11.0));
    let layers: Vec<(LayerId,String,bool,bool)> = s.doc.timeline(s.scene).layers.iter().map(|l|(l.id,l.name.clone(),l.visible,l.locked)).collect();
    egui::ScrollArea::vertical().show(ui, |ui| for (id,name,visible,locked) in layers {
        if ui.selectable_label(s.layer==id,format!("{}  {}{}",if visible {"◉"} else {"○"},name,if locked {"  🔒"} else {""})).clicked() { s.layer=id; s.selection.clear(); }
        let n=s.selection.on(id).len(); if n>0 { ui.label(RichText::new(format!("   {n} selected")).size(10.0)); }
    });
    ui.separator(); ui.label(RichText::new("LIVE SELECTION").strong().size(10.0));
    let selected:Vec<(LayerId,ElementId)>=s.selection.iter().collect();
    if selected.is_empty(){ui.label(RichText::new("Nothing selected").color(egui::Color32::GRAY));}
    for (l,e) in selected { ui.label(RichText::new(format!("Layer {} · Element {}",l.0,e.0)).monospace().size(10.0)); }
}

fn stage(ui: &mut egui::Ui, s: &mut Session) {
    ui.horizontal(|ui| { ui.label(RichText::new("STAGE").strong().size(11.0)); ui.separator(); ui.label(format!("{} × {} · {:.0} fps",s.doc.stage.width,s.doc.stage.height,s.doc.stage.fps)); });
    let avail=ui.available_size();
    let scale=(avail.x/s.doc.stage.width.max(1) as f32).min((avail.y-145.0).max(120.0)/s.doc.stage.height.max(1) as f32).clamp(0.05,2.0);
    let view=anicraft_engine::render::View::stage(&s.doc,scale);
    let display=s.display_list();
    match anicraft_engine::render::render(&display,&view) {
        Ok((pixmap,_))=>{
            let rgba=pixmap.to_rgba8();
            let image=ColorImage::from_rgba_unmultiplied([pixmap.width as usize,pixmap.height as usize],&rgba);
            let texture=ui.ctx().load_texture("anicraft-360-live-stage",image,TextureOptions::LINEAR);
            let size=Vec2::new(pixmap.width as f32,pixmap.height as f32);
            let (rect,response)=ui.allocate_exact_size(size,Sense::click());
            ui.painter().image(texture.id(),rect,egui::Rect::from_min_max(egui::Pos2::ZERO,egui::Pos2::new(1.0,1.0)),egui::Color32::WHITE);
            if response.clicked() { if let Some(pos)=response.interact_pointer_pos() {
                let p=anicraft_engine::geom::Vec2::new((pos.x-rect.left())/scale,(pos.y-rect.top())/scale);
                if let Some((layer,element))=s.hit_test(p,4.0/scale) { s.layer=layer; if ui.input(|i|i.modifiers.shift){s.selection.toggle(layer,element);}else{s.selection=anicraft_engine::Selection::only(layer,[element]);} }
                else if !ui.input(|i|i.modifiers.shift){s.selection.clear();}
            }}
        },
        Err(e)=>{ui.colored_label(egui::Color32::LIGHT_RED,format!("Stage render failed: {e}"));}
    }
}

fn timeline(ui: &mut egui::Ui,s: &mut Session){
    ui.horizontal(|ui|{
        ui.label(RichText::new("TIMELINE").strong().size(11.0));ui.separator();
        if ui.button("◀").clicked(){let _=s.execute("timeline.prev_frame",&json!({}));}
        if ui.button("▶").clicked(){let _=s.execute("timeline.next_frame",&json!({}));}
        if ui.button("+ Keyframe").clicked(){let _=s.execute("timeline.insert_keyframe",&json!({"advance":true}));}
        ui.label(format!("Frame {} / {}",s.frame+1,s.frame_count()));
    });
    let layers=s.doc.timeline(s.scene).layers.clone();egui::ScrollArea::both().max_height(118.0).show(ui,|ui|{
        egui::Grid::new("anicraft_360_live_timeline").striped(true).show(ui,|ui|{
            ui.label("Layer");let first=s.frame.saturating_sub(4);for f in first..first.saturating_add(16){ui.label(RichText::new(format!("{}",f+1)).size(9.0));}ui.end_row();
            for layer in &layers{
                ui.label(if layer.id==s.layer{RichText::new(&layer.name).strong()}else{RichText::new(&layer.name)});
                for f in first..first.saturating_add(16){
                    let key=layer.keyframes.iter().find(|k|k.start<=f&&f<k.start.saturating_add(k.duration));
                    let marker=key.map_or("·",|k|if k.start==f{"◆"}else if k.tween.is_some()||k.motion.is_some(){"━"}else{"○"});
                    if ui.add(egui::Button::new(RichText::new(marker).size(10.0)).selected(s.frame==f).min_size(Vec2::new(18.0,18.0))).clicked(){let _=s.execute("timeline.goto",&json!({"frame":f}));}
                }ui.end_row();
            }
        });
    });
}

fn properties(ui:&mut egui::Ui,s:&Session){
    ui.label(RichText::new("PROPERTIES").strong().size(11.0));ui.separator();
    ui.label("Document");ui.label(s.path.as_ref().map_or("Unsaved", |_|"Open document"));
    ui.label(format!("Stage {} × {}",s.doc.stage.width,s.doc.stage.height));ui.label(format!("{:.2} fps",s.doc.stage.fps));
    ui.separator();ui.label("Active layer");ui.label(s.active_layer().map_or("(none)",|l|l.name.as_str()));
    ui.label("Selection");ui.label(format!("{} element(s)",s.selection.len()));
    ui.separator();ui.label("History");ui.label(if s.is_dirty(){"Unsaved changes"}else{"Saved state"});
    ui.label(RichText::new("Edits use AniCraft Session commands and undo history.").size(10.0).color(egui::Color32::GRAY));
}
