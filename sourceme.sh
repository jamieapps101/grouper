#! /usr/bin/env bash


if [ ! -d .venv ]; then
    python3 -m venv .venv
    source .venv/bin/activate
    python3 -m pip install -r requirements.txt
    pre-commit --install
else
    source .venv/bin/activate
fi
