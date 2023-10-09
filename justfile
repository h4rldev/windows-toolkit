set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set positional-arguments

@default:
    just --list

@build:
    if [ ! -d "build/" ]; then \
        @mkdir build; \
    fi
    cross build --target x86_64-pc-windows-gnu
    cp target/x86_64-pc-windows-gnu/debug/windows-toolkit.exe build/windows-toolkit.exe

@build-release:
    if [ ! -d "build/" ]; then \
        mkdir build; \
    fi
    cross build --release --target x86_64-pc-windows-gnu
    cp target/x86_64-pc-windows-gnu/release/windows-toolkit.exe build/windows-toolkit.exe

@build-windows:
    cargo build --target x86_64-pc-windows-msvc
    if [ ! -d "build\\" ]; then \
        mkdir build; \
    fi
    cp target\\x86_64-pc-windows-msvc\\debug\\windows-toolkit.exe build\\windows-toolkit.exe

@build-windows-release:
    cargo build --release --target x86_64-pc-windows-msvc
    if [ ! -d "build\\" ]; then \
        mkdir build; \
    fi
    cp target\\x86_64-pc-windows-msvc\\release\\windows-toolkit.exe build\\windows-toolkit.exe

@test:
    cargo test --target x86_64-pc-windows-gnu

@run-release args:
    cross run --release --target x86_64-pc-windows-gnu -- $1

@run args:
    echo $1
    cross run --target x86_64-pc-windows-gnu -- $1

@run-windows-release args:
    cargo run --release --target x86_64-pc-windows-gnu -- $1

@run-windows args:
    cargo run --target x86_64-pc-windows-gnu -- $1