import click
from .app import create_app
from .extensions import db


app = create_app()


@app.cli.command()
def initdb():
    """ Initialize the database """
    db.create_all()
    click.echo('Added tables to the database.')


@app.cli.command()
def resetdb():
    """ Reset the database """
    db.drop_all()
    db.create_all()
    click.echo('Reset the database.')


@app.cli.command()
def fake_blogs():
    """ Generate fake blog posts. """
    from faker import Faker
    from .blueprints.blog.models import Blog

    fake = Faker()

    for x in range(100):
        entry = Blog(fake.sentence(), fake.text())
        db.session.add(entry)

    db.session.commit()


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
