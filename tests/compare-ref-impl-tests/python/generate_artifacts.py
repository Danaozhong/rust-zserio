import json
import os
from pathlib import Path
from typing import Protocol

import zserio


class ZserioPackableObject(Protocol):
    def bitsizeof(self, bitposition: int = 0) -> int:
        pass

    def bitsizeof_packed(
        self, zserio_context_node: zserio.array.PackingContextNode, bitposition: int = 0
    ) -> int:
        pass

    def write(self, zserio_writer: zserio.BitStreamWriter) -> None:
        pass


def get_test_artifacts_dir() -> Path:
    return Path(__file__).parent.parent.resolve() / "artifacts"


def generate_artifacts(name: str, zserio_object: ZserioPackableObject) -> None:
    """
    Serializes the zserio-serializable object to a binary, and stores it in the artifacts
    directory. Each file is accompanied with a json file, containing properties of the
    object (such as the zserio bitsize).
    """
    os.makedirs(get_test_artifacts_dir(), exist_ok=True)

    # Serialize the object itself.
    zserio.serialize_to_file(
        zserio_object, str(get_test_artifacts_dir() / f"{name}_from_python.bin")
    )

    # Store the bitsizes.
    bitsize = zserio_object.bitsizeof()

    packing_context_node = zserio.array.PackingContextNode()
    bitsize_packed = zserio_object.bitsizeof_packed(packing_context_node)

    json_attributes = {
        "bitsize": bitsize,
        "bitsize_packed": bitsize_packed,
    }

    with open(get_test_artifacts_dir() / f"{name}_stats.json", "w") as file_handle:
        json.dump(json_attributes, file_handle)
