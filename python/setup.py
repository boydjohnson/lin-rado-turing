from setuptools import setup, find_namespace_packages
from setuptools_rust import Binding, RustExtension

setup(
    name="py-lin-rado-turing",
    version="0.0.1",
    rust_extensions=[RustExtension("py_lin_rado_turing.tm", binding=Binding.PyO3, path="Cargo.toml")],
    packages=find_namespace_packages(include="py_lin_rado_turing.*"),
    zip_safe=False,
)
