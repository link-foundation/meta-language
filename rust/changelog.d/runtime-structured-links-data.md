Both runtime packages now load their structured concept, foundation, language,
Unicode and text metadata from inspectable Links Notation. The automatic
conversion preserves decoded values and collection order, and the data
builders reproduce that representation. Text metadata has a separate filename
from the executable text grammar. Language identification also loads identical
packaged Links data in both runtimes instead of an embedded JavaScript object.
