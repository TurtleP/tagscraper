import math
from typing import Union

__pretty_config = {
    "indent_char": " "
}


"""
{ "3 Doors Down": {"3 Doors Down (Deluxe Edition)": ["It's Not My Time"]}}

iteration 0:
  value is dict!
iteration 1:
  indent = 2
  value is list!
"""

_list_buffer = list()
_dict_buffer = list()


def pprint(input) -> str:
    if type(input) is dict:
        return pprint_dict(input)
    elif type(input) is list:
        return pprint_list(input)


def pprint_list(input: list, indent: int = 0, as_list: bool = False, clear: bool = False) -> Union[str, list]:
    _indent = __pretty_config["indent_char"] * indent

    if clear:
        _list_buffer.clear()

    for _, value in enumerate(input):
        if type(value) is list:
            return pprint_list(value)

        _list_buffer.append(f"{_indent}{value}")

    if as_list:
        return _list_buffer

    return "\n".join(_list_buffer)


def pprint_dict(input: dict, indent: int = 0) -> str:
    _indent = __pretty_config["indent_char"] * indent

    for key, value in input.items():
        _dict_buffer.append(f"{_indent}{key}")
        if type(value) is dict:
            pprint_dict(value, indent + 2)
        elif type(value) is list:
            list_buffer = pprint_list(value, indent + 2, True, True)
            _dict_buffer.extend(list_buffer)

    return "\n".join(_dict_buffer)
