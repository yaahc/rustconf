import sys
from dataclasses import dataclass
from typing import Iterator
from functools import cached_property
import re
from itertools import chain

from bs4 import BeautifulSoup, Tag

USE_STMT = re.compile(r'^(\s*use )([a-z0-9_]+)(.*)$')

@dataclass
class RevealTweaks:
    """Tweaks for fixing up HTML for a Reveal.js presentation.
    """

    soup: BeautifulSoup

    @cached_property
    def reveal_root(self) -> Tag:
        return self.soup.body.find("div", "reveal").find("div", "slides")

    @property
    def code_tags(self) -> Iterator[Tag]:
        for pre in self.reveal_root.find_all("pre"):
            if code := pre.find('code'):
                yield code

    @classmethod
    def from_file(cls, fh) -> 'RevealTweaks':
        soup = BeautifulSoup(fh.read(), "html5lib")
        return cls(soup)

    def link_imports(self):
        for code in self.code_tags:
            if (classes := code.get('class')) and 'language-rust' in classes:
                snippet = code.string
                new_snip = []
                for line in snippet.splitlines():
                    if match := USE_STMT.match(line):
                        indent, crate, rest = match.groups()
                        new_snip.append(indent)
                        link = self.soup.new_tag('a', href=f'https://docs.rs/{crate}')
                        link.string = crate
                        new_snip.append(link)
                        new_snip.append(rest)
                    else:
                        new_snip.append(line)
                    new_snip.append('\n')
                code['data-noescape'] = ""
                code.clear()
                code.extend(new_snip)

    def fragments_lists(self):
        for list_tag in self.reveal_root.find_all('list', fragments=True):
            for li in list_tag.find(['ul', 'li']).find_all('li'):
                old_class = li.get('class') or ''
                li['class'] = ('fragment ' + old_class).rstrip()
            list_tag.unwrap()

    def next_slide_hints(self):
        for notes in self.reveal_root.find_all('aside', class_='notes'):
            maybe_p = notes.previous_sibling
            if maybe_p.name == 'p' and not maybe_p.contents:
                maybe_p.decompose()

            for p in notes.find_all('p', recursive=False):
                for s in p.stripped_strings:
                    if s.startswith('Next slide: '):
                        wrapper = self.soup.new_tag('div')
                        wrapper['class'] = 'next-slide'
                        p.wrap(wrapper)
                    break

    def slide_tags(self):
        for slide in self.reveal_root.find_all('section', recursive=False):
            if extra_attrs := slide.find('slide', recursive=False):
                for name, val in extra_attrs.attrs.items():
                    slide[name] = val

    def process(self) -> BeautifulSoup:
        self.link_imports()
        self.fragments_lists()
        self.next_slide_hints()
        self.slide_tags()
        return self.soup


def main():
    tweaks = RevealTweaks.from_file(sys.stdin)
    print(tweaks.process().prettify())


if __name__ == "__main__":
    main()
