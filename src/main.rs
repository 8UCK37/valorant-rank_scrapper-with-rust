use isahc::prelude::*;
use select::document::Document;
use select::predicate::Class;

fn main() {
    // Download the raw HTML from the URL
    let url = "https://tracker.gg/valorant/profile/riot/Ez4aCE%237365/overview"; // Replace with the actual URL
    let mut response = isahc::get(url).expect("Failed to send request");
    let html = response.text().expect("Failed to read response");

    // Parse the HTML using select
    let document = Document::from(html.as_str());

    // Find the desired <div> element
    let div_element = document
        .find(Class("rating-entry__rank-info"))
        .next()
        .expect("Element not found");

    // Extract the text inside the <div class="value">
    let value_element = div_element
        .find(Class("value"))
        .next()
        .expect("Element not found");
    let value_text = value_element.text();

    println!("{}", value_text); // Print the text inside the div class="value"
}
