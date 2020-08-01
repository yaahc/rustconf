#!/usr/bin/env python3.8
import asyncio
from subprocess import CalledProcessError
from typing import List, Optional
import webbrowser
from os import path
import os
import argparse


async def run(program: str, *args: str):
    proc = await asyncio.create_subprocess_exec(program, *args, stdin=None, stderr=None)
    rc = await proc.wait()
    if rc != 0:
        raise ValueError(program)


async def sass_watch(watch: str, render: str):
    print("Compiling Sass files in", watch, "to", render)
    await run("sass", "--watch", f"{watch}:{render}")


async def serve_static(root: str = "."):
    print("Serving static files in ", path.realpath(root))
    await run("sfz", "--no-ignore", "--render-index", "--bind", "127.0.0.1", root)


async def open_browser(address: str):
    print("Opening", address, "in your browser")
    webbrowser.open_new_tab(address)


async def update_md_compiler():
    print("Ensuring md-compiler is up-to-date")
    os.chdir("md-compiler")
    await run("cargo", "build", "--release")
    os.chdir("..")


async def md_watch(watch: str):
    print("Compiling", watch, "to index.html")
    await run("./md-compiler/target/release/md-compiler", "--watch", watch)


async def nop():
    pass


async def amain(args):
    await update_md_compiler()
    await asyncio.gather(
        serve_static("."),
        sass_watch("plugin/highlight/rustconf.sass", "plugin/highlight/rustconf.css"),
        sass_watch("css/theme/source", "dist/theme"),
        md_watch("error-handling-isnt-all-about-errors.md"),
        open_browser("http://localhost:5000/") if args.open else nop(),
    )


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--no-open",
        dest="open",
        action="store_false",
        help="Don't open a browser window",
    )

    try:
        asyncio.run(amain(parser.parse_args()))
    except KeyboardInterrupt:
        print("Bye!")


if __name__ == "__main__":
    main()
