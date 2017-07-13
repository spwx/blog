import os

SQLALCHEMY_TRACK_MODIFICATIONS = False

mode = os.environ.get('MODE')

if mode == 'dev':
    SQLALCHEMY_DATABASE_URI = 'postgresql://blogdb:blogdb123@blogdb.cqxklygxzkdp.us-east-1.rds.amazonaws.com:5432/blogdb'
    FLASKS3_BUCKET_NAME = 'dev-cdn.wall.ninja'
    FLASKS3_FORCE_MIMETYPE = True
    FLASKS3_URL_STYLE = 'path'
else:
    # SQLALCHEMY_DATABASE_URI = 'sqlite:////tmp/dev.db'
    SQLALCHEMY_DATABASE_URI = 'postgresql://localdb:localdb@localhost:5432/localdb'
