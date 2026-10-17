Cache cleanup writes inspectable LiNo reports and bounded, scoped histories
before npm dependencies are installed. Existing histories migrate automatically;
the explicit JSON CLI output and active-process coordination protocols retain
their existing formats. The shared structured data writer and validator remain
the package implementation used by the bootstrap path.
