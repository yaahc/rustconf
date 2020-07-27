import html
import re
import sys
from copy import copy
from dataclasses import dataclass
from functools import cached_property
from typing import Iterator

from bs4 import BeautifulSoup, Tag

USE_STMT = re.compile(r"^(\s*use )([a-z0-9_]+)(.*)$")


def prepend_class(class_: str, tag: Tag, append=False) -> None:
    if tag.has_attr("class"):
        classes = tag.get_attribute_list("class")
        if class_ not in classes:
            if append:
                classes.append(class_)
            else:
                classes.insert(0, class_)
            tag["class"] = classes
    else:
        tag["class"] = class_


@dataclass
class RevealTweaks:
    """Tweaks for fixing up HTML for a Reveal.js presentation.
    """

    soup: BeautifulSoup

    @classmethod
    def from_file(cls, fh) -> "RevealTweaks":
        soup = BeautifulSoup(fh.read(), "html.parser")
        return cls(soup)

    @cached_property
    def reveal_root(self) -> Tag:
        return self.soup.body.find("div", "reveal").find("div", "slides")

    @property
    def code_tags(self) -> Iterator[Tag]:
        for pre in self.reveal_root.find_all("pre"):
            if code := pre.find("code"):
                yield code

    @property
    def slides(self) -> Iterator[Tag]:
        for slide in self.reveal_root.find_all("section", recursive=False):
            yield slide

    def process_code(self):
        for code in self.code_tags:
            if classes := code.get("class"):
                # Reverse the escaping process in md-compiler.
                # This lets us preserve the class list from the markdown source.
                # For classes with an `=` (key=val), we set the corresponding element attribute.
                # For a class starting with `[`, we do the highlight line numbers thing.
                new_class = []
                class_list = html.unescape(classes[0]).split("\U00101312")
                for class_ in class_list:
                    if class_.startswith("["):
                        code["data-line-numbers"] = class_.lstrip("[").rstrip("]")
                    elif "=" in class_:
                        name, val = class_.split("=", maxsplit=1)
                        class_[name] = val
                    else:
                        new_class.append(class_)
                code["class"] = " ".join(new_class)

            max_line_len = max(map(len, code.string.splitlines()))
            if max_line_len > 50:
                prepend_class("left", code, append=True)

            # Find `use ...` lines and link the first component to docs.rs.
            if (
                classes := code.get_attribute_list("class")
            ) and "language-rust" in classes:
                snippet = code.string
                new_snip = []
                for line in snippet.splitlines():
                    if match := USE_STMT.match(line):
                        indent, crate, rest = match.groups()
                        new_snip.append(indent)
                        link = self.soup.new_tag("a", href=f"https://docs.rs/{crate}")
                        link.string = crate
                        new_snip.append(link)
                        new_snip.append(rest)
                    else:
                        new_snip.append(line)
                    new_snip.append("\n")
                # data-noescape means highlight.js / reveal preserve our links.
                code["data-noescape"] = ""
                code.clear()
                code.extend(new_snip)

    def fragments_lists(self):
        # If we have <list fragments>, turn the next <ul> or <li> into a
        # fragmented list (i.e. reveal one item at a time)
        for list_tag in self.reveal_root.find_all("list", fragments=True):
            for li in list_tag.find(["ul", "li"]).find_all("li"):
                prepend_class("fragment", li)
            list_tag.unwrap()

    def fragments_media(self):
        # Roughly the same thing as fragments_lists but for other media;
        # for this one, you do need to wrap all the media in <media fragments></media>
        # -- the closing tag isn't optional.
        #
        # If the <media> tag has a fade-out-old attribute, previous elements
        # will fade out while the new one fades in. That can be distracting,
        # though.
        for media_tag in self.reveal_root.find_all("media", fragments=True):
            media_tag.name = "div"
            prepend_class("r-stack", media_tag)
            del media_tag["fragments"]

            fade_out_old = media_tag.has_attr("fade-out-old")

            is_first = True
            fragment_index = 0
            for item in filter(lambda item: isinstance(item, Tag), media_tag.children):
                item["data-fragment-index"] = str(fragment_index)
                if is_first:
                    if fade_out_old:
                        prepend_class("fade-out", item)
                        prepend_class("fragment", item)
                else:
                    if fade_out_old:
                        prepend_class("fade-in-then-out", item)
                    else:
                        prepend_class("fade-in", item)
                    prepend_class("fragment", item)
                    # We want the first and second elements to have index 0
                    fragment_index += 1

                is_first = False
            # if we iterated at least once...
            if not is_first and fade_out_old:
                item.get_attribute_list("class")[1] = "fade-in"

    def next_slide_hints(self):
        # Turn 'Next slide:' in slide notes into an italic paragraph.
        for notes in self.reveal_root.find_all("aside", class_="notes"):
            maybe_p = notes.previous_sibling
            if maybe_p.name == "p" and not maybe_p.contents:
                maybe_p.decompose()

            for p in notes.find_all("p", recursive=False):
                for s in p.stripped_strings:
                    if s.startswith("Next slide: "):
                        wrapper = self.soup.new_tag("div")
                        wrapper["class"] = "next-slide"
                        p.wrap(wrapper)
                    break

    def slide_tags(self):
        # If we have a <slide> tag, copy its attributes onto the surrounding
        # slide's <section> tag.
        for slide in self.slides:
            if extra_attrs := slide.find("slide"):
                for name, val in extra_attrs.attrs.items():
                    slide[name] = val
                extra_attrs.unwrap()

    def footers(self):
        # If we have a <slide-footer> tag anywhere in the document, append it
        # to every slide that doesn't have a 'no-footer' attribute.
        if footer := self.reveal_root.find("slide-footer"):

            for tag in footer.find_all(["left", "center", "right"]):
                prepend_class(f"slide-footer-{tag.name}", tag)
                tag.name = "div"

            footer = footer.extract()
            footer.name = "div"
            prepend_class("slide-footer", footer)

            slides = self.slides
            # skip first (title) slide
            next(slides)

            for slide in slides:
                if not slide.has_attr("no-footer"):
                    slide.append(copy(footer))
                else:
                    del slide.attrs["no-footer"]

    def font_awesome(self):
        # Transform elements like <fab fa-twitter> into their corresponding
        # font-awesome elements.
        for icon in self.reveal_root.find_all(["fab", "far", "fas"]):
            prepend_class(icon.name, icon)

            delete_attrs = []
            for name, val in icon.attrs.items():
                if name.startswith("fa"):
                    prepend_class(name, icon)
                    delete_attrs.append(name)

            for name in delete_attrs:
                del icon.attrs[name]

            icon.name = "i"
            new_icon = copy(icon)
            new_icon.clear()

            icon.insert_before(new_icon)
            icon.unwrap()

    def slide_wrappers(self):
        # Wrap each slide's contents in <div class=slide-wrapper>, which helps
        # with things like vertical centering.
        for slide in self.slides:
            wrapper = self.soup.new_tag("div")
            wrapper["class"] = "slide-wrapper"
            slide.wrap(wrapper)  # <div><section>...</section></div>
            slide = slide.unwrap()  # <div>...</div>
            wrapper.wrap(slide)  # <section><div>...</div></section>
            # ... there's no "wrap_inner" or anything

    def media_slide_fix(self):
        # A Markdown quirk -- if a slide has a <p> containing only an <img>,
        # <iframe>, or <video> element, remove the <p> and keep the inner
        # element -- this helps keep media accurately sized.
        for slide in self.slides:
            for p in slide.find_all("p", recursive=False):
                if (
                    len(p.contents) == 1
                    and isinstance(tag := p.contents[0], Tag)
                    and tag.name in ["img", "iframe", "video"]
                ):
                    p.unwrap()

    def crate_links(self):
        for crate in self.reveal_root.find_all("crate"):
            name = next(iter(crate.attrs.keys()))

            code = self.soup.new_tag("code")
            code.string = name
            a = self.soup.new_tag("a", href=f"https://docs.rs/{name}")

            crate.insert_before(code)
            code.wrap(a)
            crate.unwrap()

    def process(self) -> BeautifulSoup:
        self.crate_links()
        self.font_awesome()
        self.process_code()
        self.fragments_lists()
        self.fragments_media()
        self.next_slide_hints()
        self.slide_tags()
        self.media_slide_fix()
        self.slide_wrappers()
        self.footers()
        return self.soup


def main():
    import time

    start = time.process_time()
    tweaks = RevealTweaks.from_file(sys.stdin)
    print(tweaks.process())
    end = time.process_time()
    delta = end - start
    print(f"Done touching up HTML in {delta:.03} seconds!", file=sys.stderr)


if __name__ == "__main__":
    main()
