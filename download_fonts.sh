#!/bin/bash

mkdir -p fonts

ROBOTO_FONT_URL="https://github.com/google/fonts/raw/refs/heads/main/ofl/roboto/Roboto%5Bwdth,wght%5D.ttf"
ROBOTO_MONO_FONT_URL="https://github.com/google/fonts/raw/refs/heads/main/apache/robotomono/RobotoMono%5Bwght%5D.ttf"
./download_font.sh "./fonts" "Roboto.ttf" "$ROBOTO_FONT_URL"
./download_font.sh "./fonts" "RobotoMono.ttf" "$ROBOTO_MONO_FONT_URL"
