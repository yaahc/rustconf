#! /usr/bin/env python3.8

from dataclasses import dataclass
import sys

from bs4 import BeautifulSoup, Tag


@dataclass
class GnomeFix:
    """Tweaks for fixing "Copy as HTML" output from gnome-terminal
    """

    soup: BeautifulSoup

    @classmethod
    def from_file(cls, fh) -> "GnomeFix":
        soup = BeautifulSoup(fh.read(), "html.parser")
        return cls(soup)

    def process(self) -> BeautifulSoup:
        self.soup.find('pre')['class'] = 'term'
        for tag in self.soup.find_all("font"):
            tag.name = "span"
            color = tag.attrs.pop('color')
            tag['style'] = f"color: {color}"
        return self.soup


def main():
    tweaks = GnomeFix.from_file(sys.stdin)
    print(tweaks.process())


if __name__ == "__main__":
    main()
