from __future__ import print_function
from setuptools import setup
import os


def build_native(spec):
    build = spec.add_external_build(
        cmd=['cargo', 'build', '--release'],
        path='./rust'
    )

    spec.add_cffi_module(
        module_path='sourmash_lib._lowlevel',
        dylib=lambda: build.find_dylib('sourmash_lib', in_path='target/release'),
        header_filename=lambda: build.find_header('sourmash.h', in_path='target')
    )

# retrieve VERSION from sourmash_lib/VERSION.
thisdir = os.path.dirname(__file__)
version_file = open(os.path.join(thisdir, 'sourmash_lib', 'VERSION'))
VERSION = version_file.read().strip()

CLASSIFIERS = [
    "Environment :: Console",
    "Environment :: MacOS X",
    "Intended Audience :: Science/Research",
    "License :: OSI Approved :: BSD License",
    "Natural Language :: English",
    "Operating System :: POSIX :: Linux",
    "Operating System :: MacOS :: MacOS X",
    "Programming Language :: C++",
    "Programming Language :: Python :: 2.7",
    "Programming Language :: Python :: 3.5",
    "Programming Language :: Python :: 3.6",
    "Topic :: Scientific/Engineering :: Bio-Informatics",
]

CLASSIFIERS.append("Development Status :: 5 - Production/Stable")

SETUP_METADATA = \
               {
    "name": "sourmash",
    "version": VERSION,
    "description": "tools for comparing DNA sequences with MinHash sketches",
    "url": "https://github.com/dib-lab/sourmash",
    "author": "C. Titus Brown",
    "author_email": "titus@idyll.org",
    "license": "BSD 3-clause",
    "packages": ["sourmash_lib"],
    "zip_safe": False,
    "platforms": 'any',
    "entry_points": {'console_scripts': [
        'sourmash = sourmash_lib.__main__:main'
        ]
    },
    "install_requires": ["screed>=0.9", "ijson", "khmer>2.0<3.0"],
    "setup_requires": ["setuptools>=18.0", 'milksnake'],
    "extras_require": {
        'test' : ['pytest', 'pytest-cov', 'numpy', 'matplotlib', 'scipy'],
        'demo' : ['jupyter', 'jupyter_client', 'ipython'],
        'doc' : ['sphinx'],
        },
    "include_package_data": True,
    'milksnake_tasks': [build_native],
    "classifiers": CLASSIFIERS
    }

setup(**SETUP_METADATA)

