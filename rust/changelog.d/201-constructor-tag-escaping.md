Escape constructor tags with the existing JSON string encoders in both JavaScript target emitters. Backslashes, quotes and line breaks retain their data value during construction and pattern dispatch.

Constructor fields use computed keys and retain their exact names, including reserved words and `__proto__`, as own data properties.
