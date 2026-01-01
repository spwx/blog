mod index;
mod not_found;
mod post;
mod search;
mod sitemap;
mod static_files;

pub use index::index;
pub use not_found::not_found;
pub use post::post;
pub use search::search;
pub use sitemap::{robots, sitemap};
pub use static_files::serve_static;
