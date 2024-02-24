=========
fast-luhn
=========

Luhn-algorithm_. Generate and validate strings of numbers. 
It is used for account number validation, credit card validation, data verification, cryptography, decoding, etcetera.
Rust realization for speed-up python code. Thread-safe. See benchmark_ for details.

.. _Luhn-algorithm: https://en.wikipedia.org/wiki/Luhn_algorithm

:Author: Matt Stroganov
:Version: 0.2.0


Installation
============
Install with pip:

::

   $ pip install fast-luhn

Usage
=====
Import package:

.. code:: python

   >>> import fast_luhn as fl

Validate
--------
Check if *string* is valid by luhn algorithm. Return *bool*.

.. code:: python

   >>> fl.validate("471629309440")
   False

Digit
--------
Calculate next digit for string of numbers. Return *int*.

.. code:: python

   >>> fl.digit("47162930944")
   7

Complete
--------
Add luhn-check digit to string of numbers. Return *string*.

.. code:: python

   >>> fl.complete("2398560146")
   '23985601469'

Generate
--------
Generate luhn-valid string of numbers with *length*. Return *string*.

.. code:: python

   >>> fl.generate(50)
   '58126333877729238938910323395262199130041545367401'

Build
=====
Install from sources:

::

   $ git clone https://github.com/cybermatt/fast-luhn
   $ cd fast-luhn
   $ pip install -r requirements-dev.txt
   $ python ./setup.py develop


Benchmark
=========

Pure python realization (from here_) was very slow.

.. _here: https://stackoverflow.com/questions/21079439/implementation-of-luhn-formula

Comparison with popular python Luhn modules:

+--------------+------------+-----------+------------+
| method \ lib |    luhn_   |    LAP_   | fast-luhn  |
+==============+============+===========+============+
| validate     |   4.65 µs  |  13.3 µs  | **0.2 µs** |
+--------------+------------+-----------+------------+
| generate     |   48.1 µs  |  94.4 µs  | **3.17 µs**|
+--------------+------------+-----------+------------+

.. _luhn: https://github.com/mmcloughlin/luhn
.. _LAP: https://github.com/garwoodpr/LuhnAlgorithmProof

Environment:

* CPU i5-6500 3.20GHz, 16gb RAM
* Linux 4.19
* Python 3.7.3


Tests
=====

::

   $ pytest -v tests.py 


License
=======

This project is licensed under the MIT License - see the LICENSE.txt_ file for details

.. _LICENSE.txt: LICENSE.txt
