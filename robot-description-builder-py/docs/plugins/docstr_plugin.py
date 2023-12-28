import ast
import logging
from typing import Optional

from griffe import get_logger
from griffe.agents.nodes import ObjectNode
from griffe.dataclasses import Module
from griffe.extensions import Extension

logger: logging.Logger = get_logger(__name__)  # type: ignore #get_logger("rdb-ext")


class RDBExtenion(Extension):
    # # def on_module_members(self, *, node: AST | ObjectNode, mod: Module) -> None:
    # #     to_add = []
    # #     for import_from in node.body:
    # #         if (
    # #             isinstance(import_from, ast.ImportFrom)
    # #             and import_from.module is not None
    # #             and "_internal" in import_from.module
    # #         ):
    # #             for ident in import_from.names:
    # #                 logger.info(ident.name)
    # #                 to_add.append(
    # #                     ast.dump(
    # #                         ast.parse(f"class {ident.name}:\n    pass\n"),
    # #                         indent=4,
    # #                     )
    # #                 )
    # #             del import_from
    # #     node.extend(to_add)
    # #     return super().on_module_members(node=node, mod=mod)

    def on_package_loaded(self, *, pkg: Module) -> None:
        logger.debug("RDB-Extension is Loaded!")
        return super().on_package_loaded(pkg=pkg)

    def on_module_node(self, *, node: ast.AST | ObjectNode) -> None:
        if isinstance(node, ast.Module):
            remove_idxs: Optional[int] = None
            # add_code = []

            for idx, code in enumerate(node.body):
                if (
                    isinstance(code, ast.ImportFrom)
                    and code.module is not None
                    and code.module == "_internal"
                ):
                    remove_idxs = idx
                    logger.info(ast.unparse(code))

            if remove_idxs is not None:
                node.body.pop(remove_idxs)
        return super().on_module_node(node=node)
