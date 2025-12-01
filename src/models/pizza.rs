// pizza.rs
use serde::{ Deserialize, Serialize };
use validator:: {Validate};

#[derive(Validate, Deserialize, Serialize)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 4, message="pizza name required"))]
    pub pizza_name: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdatePizzaURL {
    pub uuid: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct Pizza {
    
}