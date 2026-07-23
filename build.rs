const COMMANDS: &[&str] = &[
    "print_thermal_printer",
    "list_thermal_printers",
    "test_thermal_printer",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
