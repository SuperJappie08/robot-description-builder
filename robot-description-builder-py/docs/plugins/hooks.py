# Inspired by: https://github.com/samuelcolvin/watchfiles/blob/9f19a18da5323d0e63a2ee16c47ed6c819f89be5/docs/plugins.py
from mkdocs.config import Config
from mkdocs.structure.files import Files
from mkdocs.plugins import get_plugin_logger

logger = get_plugin_logger("rdb-hooks")


def on_files(files: Files, config: Config) -> Files:
    return remove_files(files)


def remove_files(files: Files) -> Files:
    to_remove = []
    for file in files:
        if file.src_path in {"plugins.py", "cli_help.txt"}:
            to_remove.append(file)
        elif file.src_path.startswith("__pycache__/") or file.src_path.startswith(
            "plugins/"
        ):
            to_remove.append(file)

    logger.debug("removing files: %s", [f.src_path for f in to_remove])
    for f in to_remove:
        files.remove(f)

    return files
