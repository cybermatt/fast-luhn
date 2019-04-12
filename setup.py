import sys

from setuptools import setup

try:
    from setuptools_rust import RustExtension, Binding, Strip
except ImportError:
    import subprocess
    errno = subprocess.call([
        sys.executable, '-m', 'pip', 'install', 'setuptools-rust'
    ])
    if errno:
        print('Please install the setuptools-rust package.')
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension, Binding, Strip


def read(fname):
    return open('./README.rst').read()


setup(
    name='fast-luhn',
    version='0.1.3',
    description='Fast Luhn algorithm',
    long_description=read('README.rst'),
    keywords='luhn validation',
    url='https://github.com/cybermatt/fast-luhn',
    author='Matt Stroganov',
    author_email='strmatvey@gmail.com',
    license='MIT',
    packages=[
        'fast_luhn'
    ],
    rust_extensions=[
        RustExtension(
            'fast_luhn.fast_luhn',
            'Cargo.toml',
            binding=Binding.PyO3,
            strip=Strip.Debug
        )
    ],
    classifiers=[
        'Development Status :: 4 - Beta',
        'License :: OSI Approved :: MIT License',
        'Intended Audience :: Developers',
        'Operating System :: POSIX :: Linux',
        'Operating System :: MacOS :: MacOS X',
        'Programming Language :: Rust',
        'Programming Language :: Python',
        'Programming Language :: Python :: 2',
        'Programming Language :: Python :: 2.7',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.5',
        'Programming Language :: Python :: 3.6',
        'Programming Language :: Python :: 3.7',
    ],
    setup_requires=[
        'setuptools',
        'setuptools_rust',
        'wheel==0.31.1',
    ],
    zip_safe=False,
)
