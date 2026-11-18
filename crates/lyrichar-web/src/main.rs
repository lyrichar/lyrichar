use lyrichar_web::routes::App;

fn main() {
    #[cfg(feature = "server")]
    pid1::relaunch_if_pid1().expect("failed to relaunch");

    dioxus_sdk::storage::set_dir!();

    dioxus::launch(App);
}
