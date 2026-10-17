Added reusable native scanner descriptors for line-anchored here-document
labels and queued bodies, nested quoted interpolation, offside block
boundaries and Markdown fenced blocks. They compile into the shared Links
Notation operation language and use the existing interpreters in both
packages. Shared lexical fixtures cover concrete source preservation,
delimiter rejection and context isolation; these fixtures do not declare
complete native grammar support for the associated languages.
