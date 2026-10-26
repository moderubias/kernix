"""Regression checks for inventory drift and explicit incomplete VJP contracts."""
import copy
import json
import unittest
import check_operation_support as support


class CoverageTests(unittest.TestCase):
    def setUp(self):
        self.data = json.loads(support.DATA.read_text(encoding='utf-8'))
        self.names, self.tape = support.discover()

    def test_current_exports_and_tape_are_fully_classified(self):
        support.validate(self.data, self.names, self.tape)
        rows = {r['operation']: r for r in self.data['operations']}
        self.assertEqual(rows['matmul']['vjp'], 'supported')
        self.assertEqual(rows['neural::layer_norm']['vjp'], 'supported')
        self.assertEqual(rows['linalg::solve_refined']['vjp'], 'unsupported')
        self.assertIn('linalg::RefinedSolution::termination', self.names)
        self.assertEqual(support.methods('impl<T> Tape<T> { binary!(add, Add); pub fn solve() {} }', 'Tape'), {'add','solve'})

    def test_new_forward_export_cannot_escape_review(self):
        with self.assertRaisesRegex(ValueError, 'inventory drift'):
            support.validate(self.data, self.names | {'new_op'}, self.tape)

    def test_new_or_removed_tape_method_is_rejected(self):
        for tape in [self.tape | {'new_recording_op'}, self.tape - {'matmul'}]:
            with self.assertRaisesRegex(ValueError, 'Tape drift'):
                support.validate(self.data, self.names, tape)

    def test_supported_entry_requires_real_test_and_recording_method(self):
        for change in [{'tests':[]}, {'tape':[]}, {'tests':['crates/kernix/tests/autograd_tests.rs::missing_test']}]:
            data = copy.deepcopy(self.data)
            next(r for r in data['operations'] if r['operation']=='matmul').update(change)
            with self.assertRaises(ValueError):
                support.validate(data, self.names, self.tape)

    def test_duplicates_fail(self):
        data = copy.deepcopy(self.data)
        data['operations'].append(data['operations'][0])
        with self.assertRaisesRegex(ValueError, 'duplicate'):
            support.validate(data, self.names, self.tape)

    def test_installation_versions_check_git_tag_and_every_declaration(self):
        simple = '```toml\n[dependencies]\nkernix = "0.12.0"\n```'
        git = '```toml\nkernix = { version = "0.12.0", git = "https://github.com/moderubias/kernix", tag = "v0.12.0", default-features = false }\n```'
        self.assertTrue(support.installation_versions_match(simple, '0.12.0'))
        self.assertTrue(support.installation_versions_match(git, '0.12.0'))
        self.assertFalse(support.installation_versions_match(git.replace('v0.12.0', 'v0.11.1'), '0.12.0'))
        self.assertFalse(support.installation_versions_match(simple + '\n' + git.replace('version = "0.12.0"', 'version = "0.11.1"'), '0.12.0'))
        self.assertFalse(support.installation_versions_match('no dependency declaration', '0.12.0'))

    def test_stale_release_series_fails(self):
        data = copy.deepcopy(self.data)
        data['series'] = '0.4'
        with self.assertRaisesRegex(ValueError, 'series'):
            support.validate(data, self.names, self.tape)


if __name__ == '__main__':
    unittest.main()
