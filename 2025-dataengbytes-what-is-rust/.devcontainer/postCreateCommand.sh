#!/bin/bash
set -e

echo "Installing Python data science packages..."
pip install --no-cache-dir polars pandas matplotlib jupyter ipykernel numpy scikit-learn

echo "Installing uv package manager..."
curl -LsSf https://astral.sh/uv/install.sh | sh

echo "Setting up Rust tooling..."
rustup component add clippy rustfmt
cargo install cargo-watch cargo-edit cargo-update

echo "Creating python-analysis venv with uv..."
cd /workspaces/$(basename $(pwd))/python-analysis || mkdir -p /workspaces/$(basename $(pwd))/python-analysis
cd /workspaces/$(basename $(pwd))/python-analysis
uv venv
. .venv/bin/activate
pip install -e .

echo "Setup complete!"