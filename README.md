Straight answer: no_std with esp-hal, for this project. Here's the reasoning rather than just the verdict.

no_std + esp-hal

Pros

- Pure cargo build. One toolchain, one cargo build. That matters a lot here specifically: your whole setup is nix-flake-reproducible with just ci as the gate, and the no_std path adds nothing that can break that. What you fought through today is the entire setup cost.
- It's what the repo already is. esp-hal 1.1.2 is the 1.x stable API. esp-println, esp-backtrace, esp-alloc 0.10, embedded-graphics 0.8, embassy-executor 0.10 are all already in your local registry cache as transitive deps.
- Small and fast — image in the low hundreds of KB, boot in milliseconds, and RAM left over for framebuffers, which is the resource a
  touch LCD actually eats.
- Deterministic. No RTOS scheduler underneath you. Concurrency, if you want it, is embassy tasks — cooperative, no hidden preemption.

Cons

- No File, no threads, no std::time, no sockets. Heap only if you add esp-alloc.
- Your config story dies. toml + dirs + src/config/mod.rs don't survive; you bake settings in as consts or read NVS/flash. serde still works with default-features = false.
- Networking is the real cost. esp-wifi + embassy-net works, but HTTP/TLS means assembling reqwless/embedded-tls yourself. Days, not hours, the first time.
- Many peripherals sit behind esp-hal's unstable feature — you already enabled it, but the API can move between minor versions.

std + ESP-IDF

Pros

- Real std: threads, TcpStream, files on SPIFFS/FAT, std::time.
- esp-idf-svc hands you WiFi, HTTP, TLS (mbedtls), OTA, NVS as solved problems. For a network-fed widget this is a genuinely large head start.
- Your existing serde/toml/config code survives nearly untouched.

Cons

- esp-idf-sys runs cmake + ninja + python as part of cargo build. In a nix flake that's the fiddliest thing you could invite in — offline IDF, ESP_IDF_TOOLS_INSTALL_DIR, ldproxy on PATH. Your flake ships esp-idf-full, so it's achievable, but it would become the least reproducible part of just ci. (I haven't wired this one up in your flake, so treat the effort estimate as judgment, not measurement.)
- Megabyte-plus images, minutes-long first builds, FreeRTOS eating RAM.
- Peripheral access goes through C shims; timing is less predictable.

Why no_std wins here

Your description is "always-on desk widget for environment and status info" — the hot path is reading sensors and drawing pixels, not being a general-purpose computer. The one argument that would flip me is if the data comes over WiFi from web APIs with TLS. If canary is mostly pulling HTTPS JSON, esp-idf-svc saves you more than nix reproducibility costs you.

On reversibility, since you asked earlier in spirit: switching later is a rewrite of the HAL layer only — init, pin setup, SPI wiring — if you keep display and logic behind embedded-hal 1.0 traits instead of naming esp_hal:: types throughout. embedded-graphics, mipidsi, serde, and all your rendering code port unchanged, because both stacks implement the same traits. Structure it that way and the decision costs you a day later, not a rewrite.

So: does canary pull its data over WiFi, or from sensors wired to the board? That's the one input that could change my recommendation.

✻ Baked for 54s
