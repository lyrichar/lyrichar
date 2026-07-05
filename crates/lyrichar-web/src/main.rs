use lyrichar_web::routes::App;

fn main() {
    dioxus_sdk::storage::set_dir!();

    dioxus::launch(App);
}
