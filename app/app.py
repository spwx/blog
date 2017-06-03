from flask import Flask
from werkzeug.utils import find_modules, import_string

from .extensions import db


def register_extensions(app):
    db.init_app(app)


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
