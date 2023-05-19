use sp_inherents::{InherentIdentifier, InherentData};
use node_template_runtime::{ExternalDataType};
use node_template_runtime::pallet_template::{INHERENT_IDENTIFIER};
use async_trait;
use std::fmt::Debug;
use sp_core::Encode;

//#[derive(Debug, Clone, Default)]
#[derive(Debug, Clone)]
pub struct ExternalDataInherentProvider(pub Option<ExternalDataType>);

#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for ExternalDataInherentProvider {
	async fn provide_inherent_data(
		&self,
		inherent_data: &mut InherentData,
	) -> Result<(), sp_inherents::Error> {
        if let Some(ref data) = self.0 {
            inherent_data.put_data(INHERENT_IDENTIFIER, &data.encode())
        } else {
            Ok(())
        }
	}

	async fn try_handle_error(
		&self,
		identifier: &InherentIdentifier,
		error: &[u8],
	) -> Option<Result<(), sp_inherents::Error>> {
		if *identifier != INHERENT_IDENTIFIER {
            Some(Err(sp_inherents::Error::Application(Box::from(std::format!("Error processing inherent: {:?}", error)))))
		} else {
            None
        }
	}
}
