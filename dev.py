#!/usr/bin/env python3.8
import asyncio
from typing import List, Optional
import webbrowser
from os import path


async def run(program: str, *args: str):
    proc = await asyncio.create_subprocess_exec(program, *args, stdin=None, stderr=None)
    await proc.wait()


async def sass_watch(watch: str, render: str):
    print("Compiling Sass files in", watch, "to", render)
    await run("sass", "--watch", f"{watch}:{render}")


async def serve_static(root: str = "."):
    print("Serving static files in ", path.realpath(root))
    await run("sfz", "--render-index", "--bind", "127.0.0.1", root)


async def open_browser(address: str):
    print("Opening", address, "in your browser")
    webbrowser.open_new_tab(address)


async def amain():
    await asyncio.gather(
        serve_static("."),
        sass_watch("plugin/highlight/rustconf.sass", "plugin/highlight/rustconf.css"),
        sass_watch("css/theme/source", "dist/theme"),
        open_browser("http://localhost:5000/"),
    )


def main():
    try:
        asyncio.run(amain())
    except KeyboardInterrupt:
        print("Bye!")


if __name__ == "__main__":
    main()
