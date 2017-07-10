//! # Scrupy
//!
//! Scrupy is a fast, modern spider framework written in and for Rust. The framework implements the
//! functionalities of [Scrapy](https://scrapy.org/), but is low-level and typesafe. It exposes an
//! elegant API and uses zero unsafe code.
//!
//! # Components
//! ## Engine
//! The engine is responsible for controlling the data flow between all components of the system,
//! and triggering events when certain actions occur.
//! ## Scheduler
//! The Scheduler receives requests from the engine and enqueues them for feeding them later (also to the engine) when the engine requests them.
//!
//! ## Downloader
//! The Downloader is responsible for fetching web pages and feeding them to the engine which, in turn, feeds them to the spiders.
//!
//! ##Spiders
//! Spiders are custom classes written by Scrapy users to parse responses and extract items (aka scraped items) from them or additional requests to follow. For more information see Spiders.
//!
//! ##Item Pipeline
//! The Item Pipeline is responsible for processing the items once they have been extracted (or scraped) by the spiders. Typical tasks include cleansing, validation and persistence (like storing the item in a database). For more information see Item Pipeline.
//!
//! ##Downloader middlewares
//! Downloader middlewares are specific hooks that sit between the Engine and the Downloader and process requests when they pass from the Engine to the Downloader, and responses that pass from Downloader to the Engine.
//!
//! ###Use a Downloader middleware if you need to do one of the following:
//!
//! * process a request just before it is sent to the Downloader (i.e. right before Scrupy sends the request to the website);
//! * change received response before passing it to a spider;
//! * send a new Request instead of passing received response to a spider;
//! * pass response to a spider without fetching a web page;
//! * silently drop some requests.


pub mod engine;
pub mod spider;
pub mod scheduler;
pub mod downloader;
pub mod item_pipeline;
pub mod downloader_middleware;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
