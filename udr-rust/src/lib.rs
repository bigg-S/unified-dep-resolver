use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, js_sys};
use reqwest;
use scraper::{Html, Selector};
use js_sys::Promise;

#[wasm_bindgen]
pub fn fetch_package_data(package_name: String) -> Promise {
    let future = async move {
        // Construct URL for fetching data
        let url = format!("https://pypi.org/project/{}/", package_name);
        
        // Perform HTTP GET request
        let body = reqwest::get(&url).await.unwrap().text().await.unwrap();
        
        // Parse the HTML content
        let document = Html::parse_document(&body);
        let version_selector = Selector::parse("h1").unwrap();
        let dependency_selector = Selector::parse("a.package-header__dependency").unwrap();

        // Extract version from HTML
        let version = document.select(&version_selector).next().unwrap().inner_html().split(' ').last().unwrap().to_string();
        
        // Extract dependencies from HTML
        let dependencies: Vec<String> = document.select(&dependency_selector).map(|d| d.inner_html()).collect();

        // Create a result as a JsValue
        let result = JsValue::from_serde(&(version, dependencies)).unwrap();
        
        Ok(result)
    };

    future_to_promise(future)
}
