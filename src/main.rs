#![allow(unused_must_use)]
use eframe::egui;
use xml_builder::{XMLBuilder, XMLElement};

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([300.0, 200.0]).with_resizable(false),
        
        ..Default::default()
    };

    eframe::run_native(
        "Asset Builder",
        options,
        Box::new(|_cc| Ok(Box::<AssetBuilder>::default())),
    );
}


struct AssetBuilder {
    name: String,
    cube_scale_x: f32,
    cube_scale_y: f32,
    cube_scale_z: f32,
    id: String,
}

impl Default for AssetBuilder {
    fn default() -> Self {
        Self {
            name: String::new(),
            cube_scale_x: 1.0,
            cube_scale_y: 1.0,
            cube_scale_z: 1.0,
            id: String::new(),  
        }
    }
}

impl eframe::App for AssetBuilder {
    
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Asset Builder");

            ui.horizontal(|ui| {
                ui.label("Name:");

                ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Enter asset name"));
            });

            ui.horizontal(|ui| {
                // slider

                ui.label("Cube Scale:");

                ui.vertical(|ui| {

                    ui.add(egui::Slider::new(&mut self.cube_scale_x, 0.1..=10.0).text("X"));
                    ui.add(egui::Slider::new(&mut self.cube_scale_y, 0.1..=10.0).text("Y"));
                    ui.add(egui::Slider::new(&mut self.cube_scale_z, 0.1..=10.0).text("Z"));
                });
            
                
            });

            ui.horizontal(|ui| {
                ui.label("ID:");

                ui.add(egui::TextEdit::singleline(&mut self.id).hint_text("Enter asset ID"));
            });

            if ui.button("Criar Arquivo").clicked() {
                let path = rfd::FileDialog::new()
                    .set_title("Salvar Arquivo")
                    .add_filter("Arquivo .xtyp", &["xtyp"])
                    .save_file();

                if let Some(mut path) = path {
                    if path.extension().and_then(|s| s.to_str()) != Some("xtyp") {
                        // Se não tiver ou for diferente, adiciona/corrige para .xtyp
                        path.set_extension("xtyp");
                    }

                    let mut xml = XMLBuilder::new()
                    .version(xml_builder::XMLVersion::XML1_0)
                    .encoding("utf-8".into())
                    .build();

                    let mut cmaptypes = XMLElement::new("CMapTypes");

                    let mut archetypes = XMLElement::new("archetypes");
                    let mut item = XMLElement::new("Item");

                    let mut name_element = XMLElement::new("name");
                    name_element.add_text(self.name.clone());
                    let mut id_element = XMLElement::new("AssetId");
                    id_element.add_text(self.id.clone());

                    item.add_child(name_element);
                    item.add_child(id_element);

                    archetypes.add_child(item);
                    cmaptypes.add_child(archetypes);

                    xml.set_root_element(cmaptypes);

                    let mut writer: Vec<u8> = Vec::new();
                    xml.generate(&mut writer).unwrap();

                    std::fs::write(path, writer).expect("Failed to write XML file");
                }
            }
        });
    }
}