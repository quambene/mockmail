mod email;
mod mock_client;
mod real_client;
mod send_email;

pub use async_trait;
pub use email::Email;
pub use mock_client::MockClient;
pub use real_client::RealClient;
pub use send_email::{send_mock_email, send_real_email};
