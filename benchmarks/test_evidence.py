import importlib.util
from pathlib import Path
import subprocess
import tempfile
import unittest

spec=importlib.util.spec_from_file_location('evidence',Path(__file__).with_name('evidence.py'))
evidence=importlib.util.module_from_spec(spec);spec.loader.exec_module(evidence)


class EvidenceTests(unittest.TestCase):
    def test_legacy_rank_apis_are_omitted_without_substitute_numbers(self):
        legacy=evidence.measurement_schedule(123,False,False,True)
        self.assertNotIn(('cholesky_rank_update_256_f64','kernix'),legacy)
        self.assertIn(('cholesky_rank_update_256_f64','refactor'),legacy)
        self.assertIn(('cholesky_rank_update_256_f64','nalgebra'),legacy)
        self.assertIn(('solve_cholesky_256x64_f64','kernix'),legacy)

    def test_legacy_sparse_only_omits_missing_kernix_api(self):
        schedule=evidence.measurement_schedule(123,legacy_sparse=True)
        self.assertNotIn(('sparse_product_256_f64','kernix'),schedule)
        self.assertIn(('sparse_product_256_f64','kernix-dense'),schedule)
        self.assertIn(('sparse_product_256_f64','scipy'),schedule)
        self.assertIn(('matmul_128_f64','kernix'),schedule)

    def test_parallel_affinity_never_invents_unavailable_cpus(self):
        self.assertIsNone(evidence.parallel_affinity(None))
        self.assertEqual(evidence.parallel_affinity({7}),[7])
        selected=evidence.parallel_affinity({1,3,5})
        self.assertEqual(len(selected),2)
        self.assertLessEqual(set(selected),{1,3,5})
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
        self.assertFalse(any(w.startswith(('pivoted_qr_','min_norm_')) for w,_ in evidence.measurement_schedule(123,False,True)))
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



class BackendPolicyTests(unittest.TestCase):
    def test_non_accelerate_needs_no_shim_and_cannot_claim_api_verification(self):
        value=evidence.configure_numpy_threads({'Build Dependencies':{'blas':{'name':'scipy-openblas'}}},Path('/unused'))
        self.assertEqual(value,dict(method='backend environment variables',requested_threads=1))

    def test_accelerate_cannot_fall_back_silently_on_missing_controller(self):
        from unittest.mock import patch
        import os
        with patch.object(evidence.sys,'platform','darwin'),patch.dict(os.environ,{},clear=True):
            with self.assertRaisesRegex(ValueError,'explicit Accelerate controller'):
                evidence.apply_accelerate_policy()

if __name__=='__main__':unittest.main()
