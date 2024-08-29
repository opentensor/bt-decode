from typing import Any, Callable, Dict, Optional, Union


def get_file_bytes(filename: str) -> bytes:
    with open(filename, "r") as f:
        hex_str = f.read()

    return bytes.fromhex(hex_str)


def fix_field(
    FIELD_FIXES: Dict[str, Union[Dict[str, Callable], Callable]],
    key: str,
    value: Any,
    parent_key: Optional[str] = None,
) -> Any:
    root_fixes = FIELD_FIXES
    if parent_key is not None and FIELD_FIXES.get(parent_key) is not None:
        root_fixes = FIELD_FIXES[parent_key]

    if root_fixes.get(key) is not None:
        return root_fixes[key](value)

    return value


def fix_attr_name(
    root_attr_fixes: Dict[str, str],
    attr_name: str,
    parent_name: str = None,
) -> str:
    if parent_name is not None and parent_name in root_attr_fixes:
        root_attr_fixes = root_attr_fixes.get(parent_name, {})

    if attr_name in root_attr_fixes:
        return root_attr_fixes[attr_name]

    return attr_name


def py_getattr(
    ATTR_NAME_FIXES: Dict[str, str], obj: Any, attr: str, parent_name: str = None
) -> Any:
    return getattr(obj, fix_attr_name(ATTR_NAME_FIXES, attr, parent_name))


# TODO: remove?
def get_metadata() -> bytes:
    with open("tests/metadata.hex", "r") as f:
        encoded_metadata_hex = f.read()

    metadata_bytes = bytes.fromhex(encoded_metadata_hex)

    return metadata_bytes
