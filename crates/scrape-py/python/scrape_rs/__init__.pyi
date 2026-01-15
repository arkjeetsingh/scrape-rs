"""Type stubs for scrape_rs."""

from collections.abc import Iterator
from typing import overload

class SoupConfig:
    """Configuration options for HTML parsing."""

    def __init__(
        self,
        max_depth: int = 256,
        strict_mode: bool = False,
    ) -> None: ...
    @property
    def max_depth(self) -> int:
        """Maximum nesting depth for DOM tree."""
        ...
    @property
    def strict_mode(self) -> bool:
        """Whether strict parsing mode is enabled."""
        ...

class Tag:
    """An HTML element in the DOM tree."""

    @property
    def name(self) -> str:
        """Returns the tag name (e.g., 'div', 'span')."""
        ...
    @property
    def text(self) -> str:
        """Returns the text content of this element."""
        ...
    @property
    def inner_html(self) -> str:
        """Returns the inner HTML of this element."""
        ...
    @property
    def parent(self) -> Tag | None:
        """Returns the parent element, if any."""
        ...
    @property
    def children(self) -> Iterator[Tag]:
        """Returns an iterator over direct child elements."""
        ...
    @property
    def next_sibling(self) -> Tag | None:
        """Returns the next sibling element."""
        ...
    @property
    def prev_sibling(self) -> Tag | None:
        """Returns the previous sibling element."""
        ...
    def get(self, attr: str) -> str | None:
        """Returns the value of an attribute, if present."""
        ...
    @overload
    def __getitem__(self, attr: str) -> str:
        """Gets attribute value. Raises KeyError if not found."""
        ...
    def has_class(self, class_name: str) -> bool:
        """Checks if this element has the specified class."""
        ...
    def find(self, selector: str) -> Tag | None:
        """Finds the first descendant matching the selector."""
        ...
    def find_all(self, selector: str) -> list[Tag]:
        """Finds all descendants matching the selector."""
        ...
    def select(self, selector: str) -> list[Tag]:
        """Selects descendants using a CSS selector."""
        ...

class Soup:
    """A parsed HTML document."""

    def __init__(
        self,
        html: str,
        config: SoupConfig | None = None,
    ) -> None:
        """Parses an HTML string into a Soup document."""
        ...
    @classmethod
    def from_file(cls, path: str) -> Soup:
        """Parses HTML from a file."""
        ...
    def find(self, selector: str) -> Tag | None:
        """Finds the first element matching the selector."""
        ...
    def find_all(self, selector: str) -> list[Tag]:
        """Finds all elements matching the selector."""
        ...
    def select(self, selector: str) -> list[Tag]:
        """Selects elements using a CSS selector."""
        ...
    @property
    def title(self) -> str | None:
        """Returns the document's title, if present."""
        ...
    @property
    def text(self) -> str:
        """Returns the document's text content with tags stripped."""
        ...

def parse_batch(
    documents: list[str],
    n_threads: int | None = None,
) -> list[Soup]:
    """Parse multiple HTML documents in parallel."""
    ...
