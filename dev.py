#!/usr/bin/env python3.8
import asyncio
from subprocess import CalledProcessError
from typing import List, Optional
import webbrowser
from os import path
import os
import argparse

RESET = "\x1b[0m"
BOLD = "\x1b[1m"
UNDERLINE = "\x1b[4m"
RED = "\x1b[91m"
GREEN = "\x1b[92m"
CYAN = "\x1b[96m"

GOOD = f"{BOLD}{GREEN}"
INFO = f"{BOLD}{CYAN}"
ERROR = f"{BOLD}{RED}"


async def run(program: str, *args: str):
    try:
        proc = await asyncio.create_subprocess_exec(
            program, *args, stdin=None, stderr=None
        )
    except FileNotFoundError as e:
        raise ValueError(
            f"Executing `{program} {' '.join(args)}` failed."
            + f"\nDo you have {program} installed?"
            + f"\nRunning under `nix-shell` may help."
            + f"\nCaused by: {e}",
            e,
        )

    try:
        rc = await proc.wait()
    except asyncio.CancelledError:
        print(f"{INFO}Quitting `{program} {' '.join(args)}`{RESET}")
        try:
            proc.terminate()
            await proc.wait()
        except ProcessLookupError:
            # who cares!
            pass
        raise

    if rc != 0:
        raise ValueError(
            f"Program failed (returned {rc}): `{program} {' '.join(args)}`"
        )


async def sass_watch(watch: str, render: str):
    print(f"{INFO}Compiling Sass files in", watch, "to", render + RESET)
    await run("sass", "--watch", f"{watch}:{render}")


async def serve_static(root: str = "."):
    print(f"{INFO}Serving static files in ", path.realpath(root) + RESET)
    await run("devd", "--color",
              "--livereload", "--livewatch",
              "--ignore", "/favicon.*",
              "--port", "5000",
              "--address", "127.0.0.1",
              root)


async def open_browser(address: str):
    # It's annoying to open a browser window if e.g. the static file server
    # fails to start; we wait 0.05 seconds so that if anything else crashes on
    # start / fails to start, this coroutine gets cancelled before opening a
    # new tab.
    await asyncio.sleep(0.05)
    print(f"{INFO}Opening", address, f"in your browser{RESET}")
    webbrowser.open_new_tab(address)


async def update_md_compiler():
    print(f"{INFO}Ensuring md-compiler is up-to-date{RESET}")
    os.chdir("md-compiler")
    await run("cargo", "build", "--release")
    os.chdir("..")


async def md_watch(watch: str):
    print(f"{INFO}Compiling", watch, f"to index.html{RESET}")
    await run("./md-compiler/target/release/md-compiler", "--watch", watch)


async def nop():
    pass


async def amain(args):
    await update_md_compiler()
    awaitables = [
        serve_static("."),
        sass_watch("plugin/highlight/rustconf.sass", "plugin/highlight/rustconf.css"),
        sass_watch("css/theme/source", "dist/theme"),
        md_watch("error-handling-isnt-all-about-errors.md"),
        open_browser("http://localhost:5000/") if args.open else nop(),
    ]
    try:
        gather = asyncio.gather(*awaitables)
        await gather
    except ValueError as e:
        print(ERROR + " ".join(map(str, e.args)) + RESET)
    except KeyboardInterrupt:
        print(f"{INFO}Received ^C. Quitting.")
        gather.cancel()


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
        print(f"{INFO}Bye!{RESET}")


if __name__ == "__main__":
    main()
