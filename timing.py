#!/usr/bin/env python3.8
from __future__ import annotations

import sys
from dataclasses import dataclass
from functools import cached_property
from typing import Iterator, Dict, Union, List

from bs4 import BeautifulSoup, Tag


@dataclass
class RevealTiming:
    """Class for extracting timing information from a Reveal presentation
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
    def slides(self) -> Iterator[Tag]:
        for slide in self.reveal_root.find_all("section", recursive=False):
            yield slide

    def timing_info(self) -> TimingInfo:
        untimed_slides = []
        timed_slides = []
        times = {}
        for i, slide in enumerate(self.slides):
            if slide.has_attr("data-timing"):
                timed_slides.append(i)
                times[i] = int(slide['data-timing'])
            else:
                untimed_slides.append(i)
        return TimingInfo(
            untimed_slides=untimed_slides,
            timed_slides=timed_slides,
            times=times,
        )

@dataclass
class TimingInfo:
    untimed_slides: List[int]
    timed_slides: List[int]
    times: Dict[int, int]

    def __str__(self):
        total_time = sum(self.times.values())
        remaining_time = (30 * 60) - total_time

        untimed_slides = (f"Untimed slides: {len(self.untimed_slides)}: {', '.join(map(str, self.untimed_slides))}" if self.untimed_slides else '')
        timed_slides = (f"Timed slides:   {len(self.timed_slides)}: {', '.join(map(str, self.timed_slides))}" if self.timed_slides else '')

        time_for_untimed_slides = (f"Time for untimed slides: {remaining_time / len(self.untimed_slides):.2f} seconds each" if self.untimed_slides else '')

        total_time = f"Total time:     {total_time / 60:.2f} minutes"
        remaining_time_line = f"Remaining time: {remaining_time / 60:.2f} minutes"

        return '\n'.join(line for line in (untimed_slides, timed_slides, total_time, remaining_time_line, time_for_untimed_slides) if line)

def main():
    with open('index.html') as f:
        reveal = RevealTiming.from_file(f)
        times = reveal.timing_info()
        print(times)

if __name__ == '__main__':
    main()
