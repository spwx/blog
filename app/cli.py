import click

import flask_s3

from .app import create_app
from .extensions import db as database


app = create_app()


@app.cli.command()
def s3():
    """ Gather and upload static files to AWS S3"""
    app.config.update(
        FLASKS3_BUCKET_NAME='dev-cdn.wall.ninja',
        FLASKS3_FORCE_MIMETYPE=True,
        FLASKS3_USE_HTTPS=False,
    )

    flask_s3.create_all(app)


@app.cli.command()
def routes():
    """
    List all of the available routes.

    :return: str
    """
    output = {}

    for rule in app.url_map.iter_rules():
        route = {
            'path': rule.rule,
            'methods': '({0})'.format(', '.join(rule.methods))
        }

        output[rule.endpoint] = route

    endpoint_padding = max(len(endpoint) for endpoint in output.keys()) + 2

    for key in sorted(output):
        if 'debugtoolbar' not in key and 'debug_toolbar' not in key:
            click.echo('{0: >{1}}: {2}'.format(key, endpoint_padding,
                                               output[key]))


# Database commands
@app.cli.group()
def db():
    """ Commands to modify the database """
    pass


@db.command()
def init():
    """ Initialize the database """
    database.create_all()
    click.echo('Added tables to the database.')


@db.command()
def reset():
    """ Reset the database """
    database.drop_all()
    database.create_all()
    click.echo('Reset the database.')


@db.command()
def fake():
    """ Generate fake blog posts """
    from faker import Faker
    from app.blueprints.blog.models import Blog

    fake = Faker()

    for x in range(100):
        entry = Blog(fake.sentence(), fake.text())
        database.session.add(entry)

    database.session.commit()
    click.echo('Created fake blog entries.')
