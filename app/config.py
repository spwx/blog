import os

SQLALCHEMY_TRACK_MODIFICATIONS = False

mode = os.environ.get('MODE')

# if mode == 'awsdev':
#     SQLALCHEMY_DATABASE_URI = 'postgresql://blogdb:blogdb123@blogdb.cqxklygxzkdp.us-east-1.rds.amazonaws.com:5432/blogdb'
# elif mode == 'localdev':
#     SQLALCHEMY_DATABASE_URI = 'sqlite:////tmp/dev.db'

SQLALCHEMY_DATABASE_URI = 'postgresql://blogdb:blogdb123@blogdb.cqxklygxzkdp.us-east-1.rds.amazonaws.com:5432/blogdb'
