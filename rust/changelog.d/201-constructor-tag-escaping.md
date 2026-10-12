Escape constructor tags with the existing JSON string encoders in both JavaScript target emitters. Backslashes, quotes and line breaks retain their data value during construction and pattern dispatch.

Constructor fields use computed keys and retain their exact names, including reserved words and `__proto__`, as own data properties.

JavaScript-origin tagged objects retain mutable property descriptors, with explicit constructor freezing stored in the linked checked representation. Native algebraic-data origins retain their documented frozen JavaScript encoding. Other targets refuse explicit JavaScript constructor freezing until they can represent its property descriptors.
