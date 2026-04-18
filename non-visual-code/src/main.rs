use non_visual_code::run;

fn main() {
    if let Err(err) = run() {
        eprintln!("NVCode failed to start: {err}");
        std::process::exit(1);
    }
}
