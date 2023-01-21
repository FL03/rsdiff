#!/usr/bin/env bash
sudo apt update -y && sudo apt upgrade -y && sudo apt autoremove -y
sudo apt install -y protobuf-compiler
sh scripts/rustup.sh