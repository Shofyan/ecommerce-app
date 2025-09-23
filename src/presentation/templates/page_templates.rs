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

/// Generate product detail page
pub fn product_detail_page(product: &ProductResponse) -> String {
    let template = load_template("product_detail.html");
    
    template
        .replace("{{PRODUCT_NAME}}", &product.name)
        .replace("{{PRODUCT_ID}}", &product.id.to_string())
        .replace("{{PRODUCT_DESCRIPTION}}", product.description.as_deref().unwrap_or("No description available for this product."))
        .replace("{{PRODUCT_PRICE}}", &format!("{:.2}", product.price))
        .replace("{{PRODUCT_STOCK}}", &product.stock.to_string())
        .replace("{{STOCK_STATUS_CLASS}}", if product.stock > 0 { "text-green-600" } else { "text-red-600" })
        .replace("{{STOCK_STATUS_TEXT}}", if product.stock > 0 { "In Stock" } else { "Out of Stock" })
        .replace("{{CREATED_AT}}", &product.created_at.format("%B %d, %Y at %H:%M UTC").to_string())
        .replace("{{UPDATED_AT}}", &product.updated_at.format("%B %d, %Y at %H:%M UTC").to_string())
        .replace("{{PRODUCT_SKU}}", &format!("{:06}", product.id))
}

/// Generate error page
#[allow(dead_code)]
pub fn error_page(error_code: u16, message: &str) -> String {
    let template = load_template("error.html");
    
    template
        .replace("{{ERROR_CODE}}", &error_code.to_string())
        .replace("{{ERROR_MESSAGE}}", message)
}