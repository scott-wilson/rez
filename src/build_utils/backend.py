import sys
from setuptools import build_meta as _orig

prepare_metadata_for_build_wheel = _orig.prepare_metadata_for_build_wheel
build_wheel = _orig.build_wheel
build_sdist = _orig.build_sdist

USE_RUST = sys.version_info >= (3, 7)


def get_requires_for_build_wheel(self, config_settings=None):
    requires = ["maturin>=0.12,<0.13"] if USE_RUST else []
    return _orig.get_requires_for_build_wheel(config_settings) + requires


def get_requires_for_build_sdist(self, config_settings=None):
    requires = ["maturin>=0.12,<0.13"] if USE_RUST else []
    return _orig.get_requires_for_build_sdist(config_settings) + requires