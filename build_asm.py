#!/usr/bin/env python

import argparse
parser = argparse.ArgumentParser()
parser.add_argument("INPUT_PATH", help = "Path for the input 6502 assembly file")
args = parser.parse_args()

import os
import shutil
from urllib.request import urlretrieve
BIN_NAME = "vasm6502_oldstyle"
BUILD_PATH = "vasm-main"
SOURCE_FILE_PATH = f"{BUILD_PATH}/vasm.zip"
if not os.path.exists(BIN_NAME):
    print(BIN_NAME, "not found")
    if input(f"Do you want to download and build {BIN_NAME} from source?\n[Y/n]:").lower() != 'y':
        exit(0)
    if not os.path.exists(BUILD_PATH):
        print(f"Creating build directory: {BUILD_PATH}")
        os.makedirs(BUILD_PATH)
    if not os.path.exists(SOURCE_FILE_PATH):
        url = "https://codeload.github.com/ArchUsr64/vasm/zip/refs/heads/main"
        print(f"Downloading vasm source from:{url}")
        urlretrieve(url, filename=SOURCE_FILE_PATH)
    os.system(f"unzip {SOURCE_FILE_PATH}")
    os.system(f"make CPU=6502 SYNTAX=oldstyle -C {BUILD_PATH}")
    shutil.move(f"{BUILD_PATH}/{BIN_NAME}", BIN_NAME)

INPUT_PATH = args.INPUT_PATH
os.system(f"./{BIN_NAME} -Fbin -dotdir {INPUT_PATH}")
print(f"Compiled '{INPUT_PATH}' to 'a.out' and debug symbols at 'symbols.dbg'")
