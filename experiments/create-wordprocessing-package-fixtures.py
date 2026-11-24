"""Write the plain inputs for the shared ZIP package regression fixture."""

import io
import json
import sys
import zipfile


namespace = "http://schemas.openxmlformats.org/wordprocessingml/2006/main"
main = (
    f'<a:document xmlns:a="{namespace}"><a:p><a:r><a:t>Кошка &amp; '
    "<![CDATA[dog<s>]]></a:t><a:tab/><a:t>tab</a:t><a:br/><a:t>line</a:t>"
    "</a:r></a:p></a:document>"
)
parts = {
    "word/header1.xml": f'<hdr xmlns="{namespace}"><p><r><t>A header</t></r></p></hdr>',
    "word/document.xml": main,
    "word/footnotes.xml": f'<w:footnotes xmlns:w="{namespace}"><w:p><w:r><w:t>A note</w:t></w:r></w:p></w:footnotes>',
    "word/footer2.xml": f'<w:ftr xmlns:w="{namespace}"><w:p><w:r><w:t>A footer</w:t></w:r></w:p></w:ftr>',
}
names = ["word/document.xml", "word/header1.xml", "word/footer2.xml", "word/footnotes.xml"]


def package(method):
    output = io.BytesIO()
    with zipfile.ZipFile(output, "w") as archive:
        manifest = '<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"/>'
        archive.writestr(zipfile.ZipInfo("[Content_Types].xml"), manifest, compress_type=method)
        for name, xml in parts.items():
            archive.writestr(zipfile.ZipInfo(name), xml, compress_type=method)
    return output.getvalue()


cases = []
for method in (zipfile.ZIP_STORED, zipfile.ZIP_DEFLATED):
    data = package(method)
    with zipfile.ZipFile(io.BytesIO(data)) as archive:
        ranges = []
        for name in names:
            entry = archive.getinfo(name)
            start = entry.header_offset + 30 + len(name.encode())
            ranges.append([start, start + entry.compress_size])
    cases.append({"archive": data.hex(), "compression": "stored" if method == 0 else "deflate", "ranges": ranges})

corrupt = bytearray.fromhex(cases[0]["archive"])
corrupt[cases[0]["ranges"][0][0] + 2] ^= 1
json.dump({"cases": cases, "names": names, "parts": [parts[name] for name in names],
           "expectedText": "Кошка & dog<s>\ttab\nline\nA header\nA footer\nA note",
           "corrupt": corrupt.hex()}, sys.stdout, ensure_ascii=False)
