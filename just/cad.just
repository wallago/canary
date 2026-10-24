# Export each enclosure part to an STL in target/enclosure/
[group('hardware')]
stl:
    mkdir -p target/enclosure
    for part in shell cover; do \
        nix run nixpkgs#openscad -- -D "part=\"$part\"" -o "target/enclosure/$part.stl" hardware/enclosure/canary.scad; \
    done
