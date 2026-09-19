//! Build script: compiles the Slint UI into Rust.

/// Turns `assets/ui.slint` into generated Rust under `OUT_DIR`.
// A build script has no caller to hand an error to: aborting the build *is*
// the error path, so the workspace's `expect_used` lint does not apply here.
#[allow(clippy::expect_used)]
fn main() {
    // Glyphs are rasterised here, at the sizes the UI asks for, into embedded
    // anti-aliased bitmaps — no font engine runs on the chip.
    slint_build::compile_with_config(
        "../assets/ui.slint",
        slint_build::CompilerConfiguration::new()
            .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer),
    )
    .expect("Slint UI failed to compile");
}
