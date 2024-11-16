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
@clean: cleanbin
    cargo clean

[doc('Cleans the final bins')]
@cleanbin:
    if ( Test-Path -path shape3d.exe ) { rm shape3d.exe }
    if ( Test-Path -path file_import.dll ) { rm file_import.dll }
    if ( Test-Path -path autosave.pc ) { rm autosave.pc }
