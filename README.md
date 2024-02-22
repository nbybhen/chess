# Description

A regular game of chess using SDL2 in Rust.

# Installation

In order to install the program, you must have the `SDL2_Image` library installed, as well as Rust.

On `macOS`, you can follow these installation steps using `Homebrew`:

```bash
# Installs required SDL_Image files
brew install sdl2_image

git clone <https://github.com/nbybhen/chess.git>
cd chess
cargo run
```

On `linux`, you can follow these installation steps:

```bash
# Installs required SDL_Image files (use desired package manager)
sudo apt install libsdl2-image-dev

git clone <https://github.com/nbybhen/chess.git>
cd chess
cargo run
```

*You must have `Cargo` installed on your computer to run the application.*

If linking errors occur to `-lSDL2` when attempting to build, try installing the SDL2 library using via your package manager as well.
