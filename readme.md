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
Une fois rust installé, l'exemple peut être démarré de cette manière : 

Pour l'exemple 0 (sans Cargo) 
```shell
mkdir 0_hello_world
rustc .\hello_world.rs
.\hello_world.exe
```
Pour l'exemple 1 à 6
```shell
mkdir <path to exo>
cargo run
```

Pour l'exemple 7 (attention le premier build sera plus long pour télécharger les dépendances)
```shell
mkdir 7_GUI/comet
npm run tauri dev
```

## Documentation

* Site officiel de RUST : https://www.rust-lang.org/
* Site d'Angular :  https://angular.dev/
* Site d'Angular material : https://material.angular.io/
* Site de Tauri : https://tauri.app/

## TroubleShooting

Il est possible que l'exercice 7 ne compile pas à cause d'un programme windows non trouvé :

Attention il y a des pre-requis supplémentaire que rust et node : https://tauri.app/v1/guides/getting-started/prerequisites

si apres l'installation des pré-requis la commande `run npm tauri dev` ne fonctionne toujours pas, désisntaller rust avec la commande 
```shell
rustup self uninstall
```
Redémarrer la machine. Re-installer Rust. 