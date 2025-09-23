use crate::application::ProductResponse;
use std::fs;

/// Load template from file
fn load_template(template_name: &str) -> String {
    fs::read_to_string(format!("static/html/{}", template_name))
        .unwrap_or_else(|_| {
            eprintln!("Warning: Could not load template {}", template_name);
            String::new()
        })
}

/// Generate the main products page with search and add product form
pub fn products_page(products: &[ProductResponse]) -> String {
    let product_cards = products.iter().map(product_card).collect::<Vec<_>>().join("");
    let template = load_template("products.html");
    
    template.replace("{{PRODUCT_CARDS}}", &product_cards)
}

/// Generate a single product card
pub fn product_card(product: &ProductResponse) -> String {
    let template = load_template("product_card.html");
    let stock_badge_class = if product.stock > 0 {
        "bg-green-100 text-green-800"
    } else {
        "bg-red-100 text-red-800"
    };

    template
        .replace("{{PRODUCT_ID}}", &product.id.to_string())
        .replace("{{PRODUCT_NAME}}", &product.name)
        .replace("{{STOCK_BADGE_CLASS}}", stock_badge_class)
        .replace("{{PRODUCT_STOCK}}", &product.stock.to_string())
        .replace("{{PRODUCT_DESCRIPTION}}", product.description.as_deref().unwrap_or("No description provided"))
        .replace("{{PRODUCT_PRICE}}", &format!("{:.2}", product.price))
        .replace("{{CREATED_AT}}", &product.created_at.format("%Y-%m-%d %H:%M").to_string())
        .replace("{{UPDATED_AT}}", &product.updated_at.format("%Y-%m-%d %H:%M").to_string())
}

/// Generate product list partial for HTMX updates
pub fn product_list_partial(products: &[ProductResponse]) -> String {
    products.iter().map(product_card).collect::<Vec<_>>().join("")
}