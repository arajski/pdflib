#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi(constructor)]
pub struct ThePdfLib {
  pub page_size: String,
  pub pages: u32,
}

#[napi]
impl ThePdfLib {
    pub async fn print(&self, text: &str) -> napi::Result<Self> {
        println!("{0} printed shit", text);
        return Ok(Self {
            page_size: "A4".to_string(),
            pages: self.pages
        });
    }
}

