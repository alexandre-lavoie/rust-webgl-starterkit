# Rust-WebGL Staterkit

WebGl + Rust 🦀.

## Install

This project has mutliple options for getting setup with the code.

### Barebone

You will need `rustup` (which will provide `rustc` + `cargo`), `wasm-pack`, and `nodejs` (used for hosting/packing website). Additionally, most popular light IDEs have a plugin for Rust (if you are unfortunate to not have any - the CLI is very clean). The following are basically all the commands you should need for Linux.

```bash
# Rustup Install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Wasm-pack Install
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Example Nodejs install for Linux (makes sure to have both nodejs and npm)
pacman -S nodejs npm
```

You will want to also install the `npm` dependencies in `www`. This can be done with either:

```bash
make install
```

or 

```bash
cd www && npm install
```

### Virtualized

There are `Docker` environments setup to run each environment. You can find the command(s) to run each environment in each section. The normal gist is:

```bash
# Build
sudo docker-compose build

# Run
sudo docker-compose up

# Remove environment (only necessary for erasing)
sudo docker-compose down
```

You can also force quit with the keyboard shortcut cariant to `ctrl-c` on your OS.

### NixOS/Nix

Doubt most people will use this, but if you happen to be using Nix, a `shell.nix` is provided in the root directory.

## Hot-Reload Server

This project uses webpack + nodemon to give a hot-reload of both the Rust engine and web renderer. One of the follow script should start the web dev server. You should find the script at `http://localhost:8080`.

```bash
# Make + Node
make clean start
```

or

```bash
# Node
cd www
npm install
npm start
```

or

```bash
#Docker
sudo docker-compose up
```

## Build

This builds the webpack project into `dist`. The `dist folder can then be shared through any basic HTML, JS, CSS hoster.

```bash
# Make + Node
make clean build
```

or

```bash
# Node
cd www
npm run build
```

or

```bash
#Docker
sudo docker-compose -f docker-compose-prod.yml up
```