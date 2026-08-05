# rust-tune

Lecteur audio simple écrit en Rust, avec interface graphique.

Projet étudiant pour l'ESIG.

## Fonctionnalités

- Lecture des formats : **MP3**, **FLAC**, **WAV**, **OGG**
- Commandes de base : Jouer / Pause / Suivant / Dernière
- Affichage des métadonnées (titre, artiste, album…)
- Navigation dans la bibliothèque locale
- Différents thèmes

## Prérequis

- Rust (édition 2024)
- Linux (testé sur Debian 13 et Ubuntu 26.04 LTS)

## Installation

```bash
git clone https://github.com/xsmvn/rust-tune.git
cd rust-tune
cargo run --release

## Appimage
L'ajout de nouveau fichiers audio n'est pas fonctionnel, l'appimage est en lecture seule donc entrevoir la sauvegarde des fichiers de musique dans un autre emplacement
