Native grammar extras now retain scanner and rule action state through the
following syntax token in both interpreters. Trivia keeps state at each leaf
boundary, so immediate tokens and explicitly requested extra tokens use the
state of the prefix they retain. This prevents a completed here-document body
from leaving its closing label queued and producing a second phantom body.
Shared fixtures cover named nodes, token rules and external scanner extras,
rejections, Unicode spans and applying a retaken extra's action once.
