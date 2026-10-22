"""Process-level uncertainty keeps pairing and refuses bogus samples."""
import unittest
from compare import paired_interval

class PairedTests(unittest.TestCase):
    def test_exact_scaling_and_seed_reproducibility(self):
        value=paired_interval([(2*x,x) for x in range(1,10)],123,1000)
        self.assertEqual(value,dict(median_ratio=2,ci95_low=2,ci95_high=2))
        pairs=[(x+1,x) for x in range(1,10)]
        value=paired_interval(pairs,32,1000)
        self.assertEqual(value,paired_interval(pairs,32,1000))
        self.assertLessEqual(value['ci95_low'],value['median_ratio'])
        self.assertGreaterEqual(value['ci95_high'],value['median_ratio'])
    def test_invalid_and_unpaired_inputs_are_rejected(self):
        for pairs in [[(1,1)]*4,[(1,0)]*5,[(float('nan'),1)]*5]:
            with self.assertRaises(ValueError):paired_interval(pairs)

if __name__=='__main__':unittest.main()
