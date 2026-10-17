# SPDX-License-Identifier: MIT
import unittest
from unittest.mock import patch
import caller_run as caller


class CallerAdmissionControls(unittest.TestCase):
    def test_original_header_pin(self):
        self.assertEqual(caller.HEADER, "libs/base/pxtbase.h")
        self.assertEqual(
            caller.HEADER_SHA,
            "87c459c4d8fefa5bd851f862be8e83659185825c41448155caed34a22fe0bd17",
        )

    def test_exact_fragment_contract(self):
        self.assertEqual(caller.FRAGMENTS, {
            "caller.inc": "9443b0d8e64b57daf36068a0ca8a50a2c58094452573c8dabe7522a3e7a40236",
            "image-header.inc": "23056e13fc8341d15f3754024000f35f8928b2dd5ea7aa916f1fd47072cdac4b",
            "image-accessors.inc": "8198b2664d5627e90b54d477f11751c7788b06e1d9abfe0d345bf02da5f26436",
            "method.inc": caller.direct.FRAGMENTS["sendIndexedImage444"],
        })

    def test_changed_fragment_cannot_be_admitted(self):
        with patch.object(caller.direct, "fragment", return_value=b"changed"):
            with self.assertRaisesRegex(RuntimeError, "pinned caller/accessor fragment mismatch"):
                caller.extract(b"synthetic", b"synthetic")


if __name__ == "__main__":
    unittest.main()
