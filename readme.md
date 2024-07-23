# Présentation

Ce dépot de code contient des app en rust toutes exécutables présentant des concepts plus ou moins basique en rust pour
commencer à coder.

La liste des examples n'est pas exhaustive et de nombreux exemples sont également présent dans la documentation
officielle : https://doc.rust-lang.org/book/

## Installation

### Installer Rust

La doc, officielle (trés court et simple !) : https://www.rust-lang.org/tools/install

### Installer node JS (Exemple 7 uniquement)

* Installer windows : https://nodejs.org/en/download/prebuilt-installer
* Binaires: https://nodejs.org/en/download/prebuilt-binaries
* Package manager : https://nodejs.org/en/download/package-manager

Chaque sous dossier commençant par un numéro de 0 à 6 est une app rust standard.
Une fois rust installer elle peux etre démarré de cette manière : 

Pour l'exemple 0 (sans Cargo) 
```shell
mkdir 0_hello_world
rustc .\hello_world.rs
.\hello_world.exe
```
Pour l'exemple 1 à 6 (sans Cargo)
```shell
mkdir <path to exo>
cargo run
```

Pour l'exemple 7 (sans Cargo)
```shell
mkdir 7_GUI/commet
npm run tauri dev
```