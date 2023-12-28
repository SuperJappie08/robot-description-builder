"""Generate the code reference pages."""
from pathlib import Path

import mkdocs_gen_files

src = Path(__file__).parent.parent.parent / "python"

for path in sorted(src.rglob("*.py")):
    module_path = path.relative_to(src).with_suffix("")
    doc_path = path.relative_to(src).with_suffix(".md")
    full_doc_path = Path("api-reference", doc_path)

    # TODO: Move some virtual files around to get better TOC

    parts = list(module_path.parts)

    if parts[-1] == "__init__":
        parts = parts[:-1]
    elif parts[-1] == "__main__":
        continue

    with mkdocs_gen_files.open(full_doc_path, "w") as fd:
        identifier = ".".join(parts)
        print(
            "#"
            + parts[-1]
            + "\n::: "
            + identifier
            + "\n    options:\n      show_if_no_docstring: true\n      show_submodules: false\n      show_source: false\n      filters: [\"!^__all__\", \"!^__doc__\"]",
            file=fd,
        )

    mkdocs_gen_files.set_edit_path(full_doc_path, path)
