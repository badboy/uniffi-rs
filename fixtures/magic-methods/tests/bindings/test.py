# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import unittest
from magic import *

class TestMagic(unittest.TestCase):
    def test_str(self):
        m = Magic("yo")
        self.assertEqual(str(m), "Magic(yo)")

    def test_repr(self):
        m = Magic("yo")
        self.assertEqual(repr(m), 'Magic { val: "yo" }')

    def test_eq(self):
        m = Magic("yo")
        self.assertEqual(m, Magic("yo"))
        self.assertNotEqual(m, Magic("yoyo"))

    def test_hash(self):
        d = {}
        m = Magic("m")
        d[m] = "m"
        self.assertTrue(m in d)

if __name__=='__main__':
    unittest.main()
