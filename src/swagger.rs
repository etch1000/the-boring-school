use rocket_okapi::{settings::UrlObject, swagger_ui::SwaggerUIConfig};

pub fn swag_config() -> SwaggerUIConfig {
    SwaggerUIConfig {
        urls: vec![UrlObject::new("The Boring School", "/openapi.json")],
        deep_linking: true,
        display_request_duration: true,
        ..Default::default()
    }
}
