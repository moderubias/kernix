import importlib.util
from pathlib import Path
import subprocess
import tempfile
import unittest

spec=importlib.util.spec_from_file_location('evidence',Path(__file__).with_name('evidence.py'))
evidence=importlib.util.module_from_spec(spec);spec.loader.exec_module(evidence)


class EvidenceTests(unittest.TestCase):
    def test_true_p95_is_not_the_maximum_and_samples_keep_order(self):
        samples=list(range(100,0,-1));result=evidence.distribution(samples)
        self.assertEqual(result['p50_ns'],50)
        self.assertEqual(result['p95_ns'],95)
        self.assertEqual(result['p99_ns'],99)
        self.assertEqual(result['max_ns'],100)
        self.assertEqual(result['samples_ns'],samples)
        with self.assertRaises(ValueError):evidence.distribution([])

    def test_seeded_schedule_is_a_permutation_and_retains_every_library(self):
        expected={(w,lib) for w,libs in evidence.CASES.items() for lib in libs}
        schedule=evidence.measurement_schedule(123)
        self.assertEqual(set(schedule),expected)
        self.assertEqual(len(schedule),len(expected))
        self.assertEqual(schedule,evidence.measurement_schedule(123))
        self.assertNotEqual(schedule,evidence.measurement_schedule(124))
        self.assertFalse(any(w.startswith('training_') for w,_ in evidence.measurement_schedule(123,True)))

    def test_dirty_and_untracked_sources_are_refused_but_ignored_notes_are_allowed(self):
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary)
            def git(*args):subprocess.run(['git',*args],cwd=root,check=True,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
            git('init');(root/'.gitignore').write_text('/scratch/\n', encoding='utf-8');git('add','.gitignore')
            git('-c','user.name=Harness Test','-c','user.email=test@example.invalid','commit','-m','baseline')
            revision=evidence.clean_revision(root)
            (root/'scratch').mkdir();(root/'scratch/note').write_text('private', encoding='utf-8')
            self.assertEqual(evidence.clean_revision(root),revision)
            (root/'untracked').write_text('changed', encoding='utf-8')
            with self.assertRaises(ValueError):evidence.clean_revision(root)
            (root/'untracked').unlink();(root/'.gitignore').write_text('changed', encoding='utf-8')
            with self.assertRaises(ValueError):evidence.clean_revision(root)


if __name__=='__main__':unittest.main()
