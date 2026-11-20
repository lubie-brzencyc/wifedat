use eframe::egui;

/// To, co okno ma powiedzieć.
const WIADOMOSC: &str =
    "Kocham Rusta, Copilota i Gemini <333333333333333,\nale to tylko pionki #żonujto";

/// Puste okno z jednym napisem — bez ozdóbek i bez stylowania.
struct Okno;

impl eframe::App for Okno {
    fn ui(&mut self, interfejs: &mut egui::Ui, _ramka: &mut eframe::Frame) {
        egui::CentralPanel::default_margins().show(interfejs, |interfejs| {
            interfejs.centered_and_justified(|interfejs| {
                interfejs.add(egui::Label::new(egui::RichText::new(WIADOMOSC).size(18.0)).wrap());
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let opcje = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Kocham Rusta")
            .with_inner_size([820.0, 200.0])
            .with_min_inner_size([360.0, 120.0]),
        ..Default::default()
    };

    eframe::run_native("Kocham Rusta", opcje, Box::new(|_| Ok(Box::new(Okno))))
}
