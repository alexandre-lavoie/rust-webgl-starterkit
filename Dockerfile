# Starting with Debian + Rust
FROM rust:buster

# Make root /app to prevent files to be places in actual root
WORKDIR /app

# NodeJS 14
RUN curl -sL https://deb.nodesource.com/setup_14.x | sh -s -- -y

# Install Nodejs
RUN apt-get update && apt-get install -y nodejs

# Install Wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -y

# Copies this folder inside the app
COPY . .

# Build wasm
RUN cd www && npm run build-wasm

# Install dependencies
RUN cd www && npm install

# Expose web port
EXPOSE 8080

# Set default launch command
CMD cd www && npm start 