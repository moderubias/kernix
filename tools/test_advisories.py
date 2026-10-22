"""Temporary comparator exceptions cannot exempt runtime or future findings."""
import datetime
import unittest
from check_advisories import unexpected

def report(identifier='RUSTSEC-2026-0367',version='8.6.0',name='zip'):
    return {'vulnerabilities':{'list':[dict(advisory={'id':identifier},package=dict(name=name,version=version))]}}

class AdvisoryTests(unittest.TestCase):
    def test_scope_identity_and_expiry(self):
        date=datetime.date(2026,10,22)
        self.assertFalse(unexpected(report(),True,date))
        for data,benchmark,today in [(report(),False,date),(report(version='8.7.0'),True,date),
              (report(identifier='NEW'),True,date),(report(name='other'),True,date),
              (report(),True,datetime.date(2026,11,23))]:
            self.assertTrue(unexpected(data,benchmark,today))

if __name__=='__main__':unittest.main()
