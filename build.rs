fn main() {
    let sound_enabled = std::env::var_os("CARGO_FEATURE_SOUND").is_some();
    if !sound_enabled {
        return;
    }

    // Probe ALSA early and emit a clear warning with alternatives.
    match pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("alsa")
    {
        Ok(_) => {
            println!("cargo:rustc-cfg=alsa_available");
        }
        Err(err) => {
            println!("cargo:warning=ALSA development files not found ({err}).");
            println!(
                "cargo:warning=Install ALSA dev libs (e.g., `libasound2-dev`) or build without sound: \
                 `cargo build --no-default-features` or run `cargo run -- --mute`."
            );
        }
    }
}
