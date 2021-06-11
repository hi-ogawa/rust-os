import unittest
import subprocess
import yaml
import os.path


class Test(unittest.TestCase):
    @classmethod
    def define_cases(cls, cases):
        for case in cases:
            cls.define_case(case)

    @classmethod
    def define_case(cls, case):
        def test_method(self):
            process = subprocess.run(
                case["command"], shell=True, capture_output=True, text=True, timeout=10
            )
            actual = process.stdout
            expected = case["stdout"]
            self.assertEqual(actual, expected)

        setattr(cls, f"test_{case['name']}", test_method)


if __name__ == "__main__":
    infile = os.path.join(os.path.dirname(__file__), "test.yml")
    cases = yaml.load(open(infile), Loader=yaml.SafeLoader)
    Test.define_cases(cases)
    unittest.main()
