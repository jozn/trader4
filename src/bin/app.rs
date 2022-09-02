use trader4;
use trader4::app;

fn main() {
    write_settings_def();
    load_sttings();
}

fn load_sttings() {
    let js = std::fs::read_to_string("./settings.json").unwrap();
    let set: app::sim::Setting = serde_json::from_str(&js).unwrap();
    println!("{:#?}", set);
}

fn write_settings_def() {
    let set = app::sim::Setting::default();

    let js = serde_json::to_string_pretty(&set).unwrap();
    std::fs::write("./debug/settings_def.js", js);
}
