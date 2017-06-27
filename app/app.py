from flask import Flask
from werkzeug.utils import find_modules, import_string

from .extensions import db, s3


def register_extensions(app):
    # dict([(name, cls) for name, cls in mod.__dict__.items()
    #     if isinstance(cls, type)])

    db.init_app(app)
    s3.init_app(app)


def register_blueprints(app):
    for name in find_modules('app.blueprints', include_packages=True):
        mod = import_string(name)
        if hasattr(mod, 'blueprint'):
            app.register_blueprint(mod.blueprint)


def create_app():
    app = Flask(__name__)

    app.config.from_object('app.config')

    register_blueprints(app)
    register_extensions(app)

    return app
