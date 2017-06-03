from flask import Blueprint, render_template, redirect, url_for
from .models import Blog

blog = Blueprint('blog', __name__, template_folder='templates')


# @blog.route('/')
# def index():
#     return redirect(url_for('blog.home'))


@blog.route('/blog/', defaults={'page': 1}, methods=["GET", "POST"])
@blog.route('/blog/<int:page>/', methods=["GET", "POST"])
def home(page):
    paginated = Blog.query.paginate(page, 10)
    return render_template("blog/index.html", paginated=paginated)
