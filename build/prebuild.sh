#!/bin/bash

# Function to check npm version
npm_version() {
    echo "NPM Version: $(npm -v)" || { echo "npm -v failed"; exit 1; }
}

# Function to install npm dependencies
npm_install() {
    cd "$1" || exit
    npm install || { echo "npm install failed"; exit 1; }
}

# Function to build npm project
npm_build() {
    cd "$1" || exit
    npm run build || { echo "npm run build failed"; exit 1; }
}

# Main script execution
FRONTEND_DIR="../frontend"

npm_version
npm_install "$FRONTEND_DIR"
npm_build "$FRONTEND_DIR"
