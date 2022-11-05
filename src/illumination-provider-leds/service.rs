use tonic::Response;

use self::org::freedesktop::illumination::v1::entities_service_server::EntitiesService;
use self::org::freedesktop::illumination::v1::{
    EntitiesServiceEventsRequest, GetSettingsRequest, GetSettingsResponse, SetSettingsRequest,
    SetSettingsResponse, Settings,
};
use crate::dirs::LedDir;
use core::future::Future;
use core::marker::Send;
use core::pin::Pin;
use std::collections::HashMap;

pub mod org {
    pub mod freedesktop {
        pub mod illumination {
            pub mod v1 {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/org.freedesktop.illumination.v1.rs"
                ));
            }
        }
    }
}

#[derive(Default)]
pub struct Entities {
    known: HashMap<u64, LedDir>,
}

#[tonic::async_trait]
impl EntitiesService for Entities {
    async fn events(
        &self,
        request: tonic::Request<EntitiesServiceEventsRequest>,
    ) -> Result<tonic::Response<Self::EventsStream>, tonic::Status> {
        let r = request.into_inner();
        let led_dir = self.known[&r.entity_id];

    }

    fn get_settings(
        &self,
        request: tonic::Request<GetSettingsRequest>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<tonic::Response<GetSettingsResponse>, tonic::Status>> + Send,
        >,
    > {
        let led_dir = self.known[&0];
        let settings = Settings {
            on: led_dir.get_on(),
            brightness: led_dir.get_brightness(),
            color_temperature: led_dir.get_color_temperature(),
        };
        Ok(Response::new(GetSettingsResponse {
            current: Some(settings),
        }))
    }

    fn set_settings<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<SetSettingsRequest>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<tonic::Response<SetSettingsResponse>, tonic::Status>>
                + Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
    }
}
