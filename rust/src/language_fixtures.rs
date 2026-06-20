//! Executable language-target fixtures, extracted from `parity` to keep
//! `src/parity.rs` under the repository file-size limit.

use crate::parity::LanguageFixture;

/// Executable fixtures for every language target requested by the founding issue.
pub const LANGUAGE_FIXTURES: &[LanguageFixture] = &[
    LanguageFixture {
        language: "txt",
        source: "Plain text region\ncafe au lait\nUTF-8 line: café\n",
        description: "Plain-text UTF-8 prose with trailing newline",
    },
    LanguageFixture {
        language: "Markdown",
        source: "# Title\n\n```rust\nfn main() {}\n```\n",
        description: "Markdown document with embedded fenced code",
    },
    LanguageFixture {
        language: "HTML",
        source: "<script>const x = 1;</script><style>.x { color: red; }</style><p style=\"color: blue\">text</p>\n",
        description: "HTML document with script, style, and style-attribute regions",
    },
    LanguageFixture {
        language: "PDF",
        source: "%PDF-1.7\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R /F2 5 0 R /F3 6 0 R >> >> /Contents 7 0 R >>\nendobj\n4 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>\nendobj\n5 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold >>\nendobj\n6 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Oblique >>\nendobj\n7 0 obj\n<< /Length 163 >>\nstream\n/H1 BDC\nBT\n72 720 Td\n/F1 24 Tf\n(Status Report) Tj\nET\nEMC\n/P BDC\nBT\n72 698 Td\n/F1 12 Tf\n(The system is ) Tj\n/F2 12 Tf\n(ready) Tj\n/F1 12 Tf\n( for launch.) Tj\nET\nEMC\nendstream\nendobj\nxref\n0 8\n0000000000 65535 f \n0000000009 00000 n \n0000000058 00000 n \n0000000115 00000 n \n0000000261 00000 n \n0000000331 00000 n \n0000000406 00000 n \n0000000484 00000 n \ntrailer\n<< /Size 8 /Root 1 0 R >>\nstartxref\n697\n%%EOF\n",
        description: "Text PDF profile with bold + heading + paragraph (issue #84)",
    },
    LanguageFixture {
        language: "DOCX",
        source: "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<w:document xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"><w:body><w:p><w:pPr><w:pStyle w:val=\"Heading1\"/></w:pPr><w:r><w:t xml:space=\"preserve\">Status Report</w:t></w:r></w:p><w:p><w:r><w:t xml:space=\"preserve\">The system is </w:t></w:r><w:r><w:rPr><w:b/></w:rPr><w:t xml:space=\"preserve\">ready</w:t></w:r><w:r><w:t xml:space=\"preserve\"> for launch.</w:t></w:r></w:p><w:p><w:pPr><w:numPr><w:ilvl w:val=\"0\"/><w:numId w:val=\"1\"/></w:numPr></w:pPr><w:r><w:t xml:space=\"preserve\">First item</w:t></w:r></w:p><w:p><w:pPr><w:numPr><w:ilvl w:val=\"0\"/><w:numId w:val=\"1\"/></w:numPr></w:pPr><w:r><w:t xml:space=\"preserve\">Second </w:t></w:r><w:r><w:rPr><w:b/></w:rPr><w:t xml:space=\"preserve\">strong</w:t></w:r><w:r><w:t xml:space=\"preserve\"> item</w:t></w:r></w:p><w:sectPr/></w:body></w:document>\n",
        description: "OOXML document.xml profile with heading + bold + bullet list (issue #85)",
    },
    LanguageFixture {
        language: "Python",
        source: "def f(x):\n    return x + 1\n",
        description: "Python function with indentation",
    },
    LanguageFixture {
        language: "C",
        source: "int main(void) { return 0; }\n",
        description: "C entry point",
    },
    LanguageFixture {
        language: "Java",
        source: "class Main { public static void main(String[] args) {} }\n",
        description: "Java class entry point",
    },
    LanguageFixture {
        language: "C++",
        source: "int main() { return 0; }\n",
        description: "C++ entry point",
    },
    LanguageFixture {
        language: "C#",
        source: "class C { static void Main() {} }\n",
        description: "C# class entry point",
    },
    LanguageFixture {
        language: "JavaScript",
        source: "const value = 1;\n",
        description: "JavaScript binding",
    },
    LanguageFixture {
        language: "Visual Basic",
        source: "Module Program\nEnd Module\n",
        description: "Visual Basic module",
    },
    LanguageFixture {
        language: "R",
        source: "value <- 1\n",
        description: "R assignment",
    },
    LanguageFixture {
        language: "sql-ansi",
        source: "SELECT id, name FROM users WHERE active = TRUE;\n",
        description: "ANSI SQL select statement",
    },
    LanguageFixture {
        language: "Delphi/Object Pascal",
        source: "unit DemoUnit;\n\ninterface\n\ntype\n  TBox<T> = class\n  private\n    FValue: T;\n  public\n    [Stored]\n    property Value: T read FValue write FValue;\n  end;\n\nimplementation\n\nend.\n",
        description: "Delphi/Object Pascal unit with a generic class property",
    },
    LanguageFixture {
        language: "English",
        source: "Hawaii is a state.\n",
        description: "English formalization sentence",
    },
    LanguageFixture {
        language: "Mandarin Chinese",
        source: "你好。\n",
        description: "Mandarin Chinese sentence",
    },
    LanguageFixture {
        language: "Hindi",
        source: "नमस्ते।\n",
        description: "Hindi sentence",
    },
    LanguageFixture {
        language: "Spanish",
        source: "Hawaii es un estado.\n",
        description: "Spanish reconstruction sentence",
    },
    LanguageFixture {
        language: "French",
        source: "Hawaii est un etat.\n",
        description: "French reconstruction sentence",
    },
    LanguageFixture {
        language: "Modern Standard Arabic",
        source: "مرحبا.\n",
        description: "Modern Standard Arabic sentence",
    },
    LanguageFixture {
        language: "Bengali",
        source: "নমস্কার।\n",
        description: "Bengali sentence",
    },
    LanguageFixture {
        language: "Russian",
        source: "Гавайи это штат.\n",
        description: "Russian reconstruction sentence",
    },
    LanguageFixture {
        language: "Portuguese",
        source: "Hawaii e um estado.\n",
        description: "Portuguese reconstruction sentence",
    },
    LanguageFixture {
        language: "Urdu",
        source: "سلام۔\n",
        description: "Urdu sentence",
    },
    LanguageFixture {
        language: "JSON",
        source: "{\n  \"name\": \"café\",\n  \"items\": [1, 2, 3]\n}\n",
        description: "JSON object with UTF-8 string and array",
    },
    LanguageFixture {
        language: "YAML",
        source: "name: café\nitems:\n  - 1\n  - 2\n",
        description: "YAML mapping with UTF-8 value and sequence",
    },
    LanguageFixture {
        language: "TOML",
        source: "title = \"café\"\n\n[owner]\nname = \"Tom\"\n",
        description: "TOML document with a UTF-8 value and a table",
    },
    LanguageFixture {
        language: "XML",
        source: "<note lang=\"en\">\n  <body>café</body>\n</note>\n",
        description: "XML element tree with attribute and UTF-8 text",
    },
    LanguageFixture {
        language: "INI",
        source: "; comment\n[owner]\nname = café\n",
        description: "INI section with a comment and a UTF-8 value",
    },
    LanguageFixture {
        language: "protobuf",
        source: "syntax = \"proto3\";\n\nmessage Person {\n  string name = 1;\n}\n",
        description: "Protocol Buffers message definition",
    },
    LanguageFixture {
        language: "GraphQL",
        source: "type Person {\n  name: String!\n}\n",
        description: "GraphQL schema-definition type",
    },
    LanguageFixture {
        language: "CSV",
        source: "name,city\n\"Ana, Maria\",Lisbon\nBao,北京\n",
        description: "CSV records with a quoted comma and UTF-8 field",
    },
    LanguageFixture {
        language: "JSON5",
        source: "{\n  // JSON5 accepts comments and unquoted keys\n  name: 'café',\n  items: [1, 2, 3,],\n}\n",
        description: "JSON5 object with comments, unquoted keys, single quotes, and trailing commas",
    },
    LanguageFixture {
        language: "PHP",
        source: "<?php\nfunction greet($name) {\n    return \"café \" . $name;\n}\n",
        description: "PHP function returning a UTF-8 string",
    },
    LanguageFixture {
        language: "Swift",
        source: "func greet(_ name: String) -> String {\n    return \"café \\(name)\"\n}\n",
        description: "Swift function with a UTF-8 interpolated string",
    },
    LanguageFixture {
        language: "Kotlin",
        source: "fun greet(name: String): String {\n    return \"café $name\"\n}\n",
        description: "Kotlin function with a UTF-8 template string",
    },
    LanguageFixture {
        language: "Scala",
        source: "object Demo {\n  def greet(name: String): String = s\"café $name\"\n}\n",
        description: "Scala object with a UTF-8 interpolated method",
    },
    LanguageFixture {
        language: "Lua",
        source: "local function greet(name)\n  return \"café \" .. name\nend\n",
        description: "Lua function concatenating a UTF-8 string",
    },
    LanguageFixture {
        language: "Perl",
        source: "use utf8;\nsub greet {\n    my ($name) = @_;\n    return \"café $name\";\n}\n",
        description: "Perl subroutine returning a UTF-8 interpolated string",
    },
];
