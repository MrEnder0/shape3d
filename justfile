set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

default: build-full

alias b := build-base
#alias bp := build-plugin
alias bf := build-full
alias c := clean

[doc('Build the base binary')]
@build-base:
    cargo build --release
    mv target/release/shape3d.exe shape3d.exe

[doc('Build the base binary and all the plugins')]
@build-full: build-base
    cd plugins/file_import; cargo build --release; mv target/release/file_import.dll ../../file_import.dll

[doc('Clean the project')]
@clean:
    rm shape3d.exe
    rm file_import.dll
    rm autosave.pc
    cargo clean