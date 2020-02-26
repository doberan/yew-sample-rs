extern crate yew_sample_rs as ysr;

fn main() {
    // yew::App::<yew_sample_rs::Model>::new().mount_to_body();
    // yew::run_loop();
    // println!("hello world");
    yew::start_app::<ysr::Model>();
}