Native HCL root spans now include leading whitespace in both packages, while
child spans still follow their actual tokens. Wrapped zero-width scanner tokens
retain their grammar-owned padding, preserving CSS descendant selectors and
Lean layout tokens. HCL collection comprehensions have an identity distinct
from imperative loops, with every existing source alias retained. Reviewed
concept definitions and distinctions now apply on every import regeneration.
The broader native catalog and multi source grammar work remain open.
