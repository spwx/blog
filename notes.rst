todo:

move the database url to an s3 bucket
automatically detect and load extensions

Problem
-------

Unable to invoke remote commands with zappa.

Solution
--------

1. in the root directory of the package, import the module with the function I
   need:

   .. code:: python

     import devapp

2. Use the zappa command: 

   .. code:: sh

     zappa invoke 'from app.devapp import function; function()' --raw

-------------------------------------------------------------------------------

Problem
-------

Invocation of `fake_blogs` function fails

Solution
--------

1. Remove __pycache__ directories:

   .. code:: sh

     find . | grep -E "(__pycache__|\.pyc|\.pyo$)" | xargs rm -rf

Notes
-----

Start cleaning the application directory of all \__pycache__, \*.pyc and \*.pyo
files before using zappa to update or deploy. Alias the above command to
pyclean.

-------------------------------------------------------------------------------

Problem
-------

lambda doesn't serve static files

Solution
--------

Use Flask-S3 to:

1. Gather static files from registered blueprints
2. Upload files to an S3 bucket
3. Use the FLASK_DEBUG environment variable to determine if the files should be
   served locally or from the bucket
