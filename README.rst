=========
fast-luhn
=========

Luhn algorithm_. Generate and validate strings of numbers. Rust realization for speed-up python code.

.. _algorithm: https://en.wikipedia.org/wiki/Luhn_algorithm

:Author: Matt Stroganov
:Version: 0.1.2


Installation
============
Install with pip:

::

   $ pip install fast-luhn

Usage
=====
Import package:

.. code:: python

   >>> import fast_luhn as luhn

Validate
--------
Check if *string* is valid by luhn algorithm. Return *bool*.

.. code:: python

   >>> luhn.validate("471629309440")
   False

Digit
--------
Calculate next digit for string of numbers. Return *int*.

.. code:: python

   >>> luhn.digit("47162930944")
   7

Complete
--------
Add luhn-check digit to string of numbers. Return *string*.

.. code:: python

   >>> luhn.complete("2398560146")
   '23985601469'

Generate
--------
Generate luhn-valid string of numbers with *length*. Return *string*.

.. code:: python

   >>> luhn.generate(50)
   '58126333877729238938910323395262199130041545367401'

Build
=====
Install from sources:

::

   $ git clone https://github.com/cybermatt/fast-luhn
   $ cd fast-luhn
   $ pip install -r requirements-dev.txt
   $ python ./setup.py develop

Tests
=====

::

   $ pytest -v tests.py 
