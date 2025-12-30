mod index;
mod post;
mod search;
mod static_files;

pub use index::index;
pub use post::post;
pub use search::search;
pub use static_files::serve_static;
