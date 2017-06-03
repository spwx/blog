from ...extensions import db

from datetime import datetime, timezone


def utcnow():
    return datetime.now(timezone.utc)


class Blog(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    title = db.Column(db.String(180), nullable=False)
    text = db.Column(db.Text, nullable=False)
    created_at = db.Column(db.TIMESTAMP(timezone=True), default=utcnow)
    last_updated = db.Column(db.TIMESTAMP(timezone=True), onupdate=utcnow)

    def __init__(self, title, text):
        self.title = title
        self.text = text

    def __repr__(self):
        return '<Blog %r>' % self.title
