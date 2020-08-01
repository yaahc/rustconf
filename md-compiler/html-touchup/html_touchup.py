import sys
from dataclasses import dataclass
from typing import Iterator
from functools import cached_property
import re
from copy import copy
import html

from bs4 import BeautifulSoup, Tag

USE_STMT = re.compile(r'^(\s*use )([a-z0-9_]+)(.*)$')

def prepend_class(class_: str, tag: Tag) -> None:
    old_class = tag.get('class')
    if old_class is None:
        tag['class'] = class_
    else:
        tag['class'] = class_ + ' ' + old_class

@dataclass
class RevealTweaks:
    """Tweaks for fixing up HTML for a Reveal.js presentation.
    """

    soup: BeautifulSoup

    @classmethod
    def from_file(cls, fh) -> 'RevealTweaks':
        soup = BeautifulSoup(fh.read(), "html5lib")
        return cls(soup)

    @cached_property
    def reveal_root(self) -> Tag:
        return self.soup.body.find("div", "reveal").find("div", "slides")

    @property
    def code_tags(self) -> Iterator[Tag]:
        for pre in self.reveal_root.find_all("pre"):
            if code := pre.find('code'):
                yield code

    @property
    def slides(self) -> Iterator[Tag]:
        for slide in self.reveal_root.find_all('section', recursive=False):
            yield slide

    def process_code(self):
        for code in self.code_tags:
            if classes := code.get('class'):
                new_class = []
                class_list = html.unescape(classes[0]).split('\U00101312')
                for class_ in class_list:
                    if class_.startswith('['):
                        code['data-line-numbers'] = class_.lstrip('[').rstrip(']')
                    elif '=' in class_:
                        name, val = class_.split('=', maxsplit=1)
                        class_[name] = val
                    else:
                        new_class.append(class_)
                code['class'] = ' '.join(new_class)

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
                prepend_class('fragment', li)
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
        for slide in self.slides:
            if extra_attrs := slide.find('slide'):
                for name, val in extra_attrs.attrs.items():
                    slide[name] = val
                extra_attrs.unwrap()

    def footers(self):
        if footer := self.reveal_root.find('slide-footer'):
            print('Attaching footer to slides', file=sys.stderr)

            for tag in footer.find_all(['left', 'center', 'right']):
                prepend_class(f'slide-footer-{tag.name}', tag)
                tag.name = 'div'

            footer = footer.extract()
            footer.name = 'div'
            prepend_class('slide-footer', footer)

            slides = self.slides
            # skip first (title) slide
            next(slides)

            for slide in slides:
                if not slide.has_attr('no-footer'):
                    slide.append(copy(footer))
                else:
                    del slide.attrs['no-footer']

    def font_awesome(self):
        for icon in self.reveal_root.find_all(['fab', 'far', 'fas']):
            prepend_class(icon.name, icon)

            delete_attrs = []
            for name, val in icon.attrs.items():
                if name.startswith('fa'):
                    prepend_class(name, icon)
                    delete_attrs.append(name)

            for name in delete_attrs:
                del icon.attrs[name]

            icon.name = 'i'
            new_icon = copy(icon)
            new_icon.clear()

            icon.insert_before(new_icon)
            icon.unwrap()

    def slide_wrappers(self):
        for slide in self.slides:
            wrapper = self.soup.new_tag('div')
            wrapper['class'] = 'slide-wrapper'
            slide.wrap(wrapper) # <div><section>...</section></div>
            slide = slide.unwrap() # <div>...</div>
            wrapper.wrap(slide) # <section><div>...</div></section>
            # ... there's no "wrap_inner" or anything

    def media_slide_fix(self):
        for slide in self.slides:
            for p in slide.find_all('p', recursive=False):
                if (len(p.contents) == 1
                        and isinstance(tag := p.contents[0], Tag)
                        and tag.name in ['img', 'iframe', 'video']):
                    p.unwrap()

    def process(self) -> BeautifulSoup:
        self.font_awesome()
        self.process_code()
        self.fragments_lists()
        self.next_slide_hints()
        self.slide_tags()
        self.media_slide_fix()
        self.slide_wrappers()
        self.footers()
        return self.soup


def main():
    tweaks = RevealTweaks.from_file(sys.stdin)
    print(tweaks.process())


if __name__ == "__main__":
    main()
