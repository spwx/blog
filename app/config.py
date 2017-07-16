import os

SQLALCHEMY_TRACK_MODIFICATIONS = False

mode = os.environ.get('MODE')

if mode == 'dev':
    FLASKS3_BUCKET_NAME = 'dev-cdn.wall.ninja'
    FLASKS3_FORCE_MIMETYPE = True
    FLASKS3_URL_STYLE = 'path'


SQLALCHEMY_DATABASE_URI = os.environ.get('DBSTRING')
