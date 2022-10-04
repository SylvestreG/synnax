use reqwest::blocking::Client;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Lcd {
    pub lcd_address: String,
    client: Client,
}

impl PartialEq<Self> for Lcd {
    fn eq(&self, other: &Self) -> bool {
        self.lcd_address == other.lcd_address
    }
}

impl Eq for Lcd {}

impl Lcd {
    pub fn new(lcd_address: String) -> Result<Self, anyhow::Error> {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()?;

        Ok(Lcd {
            lcd_address,
            client,
        })
    }

    pub fn get<T: serde::de::DeserializeOwned + Clone>(
        &self,
        endpoint: String,
    ) -> Result<T, anyhow::Error> {
        log::debug!("GET {}", endpoint);
        let request = self
            .client
            .get(format!("{}{}", self.lcd_address, endpoint))
            .send()?;

        let data = &request.text()?;
        log::trace!("-> payload\n{}", data);
        Ok(serde_json::from_str::<T>(data)?)
    }
}
