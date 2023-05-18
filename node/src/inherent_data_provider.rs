use sp_inherents::{InherentIdentifier, IsFatalError, InherentData};
use node_template_runtime::{ExternalDataType};
use node_template_runtime::external_data_inherent::{InherentType, INHERENT_IDENTIFIER};
use async_trait;
use std::fmt::Debug;
use sp_core::Encode;
use std::sync::{Arc, Mutex};
//pub type DataType = crate::external_data_inherent::ExternalDataType;

//#[derive(Debug, Clone, Default)]
#[derive(Debug, Clone)]
pub struct ExternalDataInherentProvider(pub Option<Arc<Mutex<ExternalDataType>>>);
//pub struct ExternalDataInherentProvider(pub InherentType);

impl ExternalDataInherentProvider {   
    fn get_data(&self) -> Option<ExternalDataType> {
        let data = self.0.as_ref().map(|arc| *(*arc).lock().unwrap());
        log::info!("--------------------- data: {:?}", data);
        data
    }
    
    fn set_data(&self, data: ExternalDataType) {
        if let Some(ref arc) = self.0 {
            *(*arc).lock().unwrap() = data;
        }
    }
}

#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for ExternalDataInherentProvider {
	async fn provide_inherent_data(
		&self,
		inherent_data: &mut InherentData,
	) -> Result<(), sp_inherents::Error> {
//		inherent_data.put_data(INHERENT_IDENTIFIER, &self.0)
        log::info!("--------------------- provide_inherent_data, get_data: {:?}", self.get_data());
        if let Some(data) = self.get_data() {
            log::info!("--------------------- provide_inherent_data: inside:{:?}", data);
            self.set_data(data + 1);
            inherent_data.put_data(INHERENT_IDENTIFIER, &data.encode())
        } else {
            panic!("Data holder is None")
        }
//                Err(sp_inherents::Error::Application("Data holder is None".into()))
	}

	async fn try_handle_error(
		&self,
		identifier: &InherentIdentifier,
		_error: &[u8],
	) -> Option<Result<(), sp_inherents::Error>> {
		// Dont' process modules from other inherents
		if *identifier != INHERENT_IDENTIFIER {
			return None
		}

		// All errors with the author inehrent are fatal
		Some(Err(sp_inherents::Error::Application(Box::from(String::from("Error processing author inherent")))))
	}
}
