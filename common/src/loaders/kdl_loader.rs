use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::{Asset, Resource},
    reflect::TypePath,
    utils::thiserror::Error,
};

use kdl::{KdlDocument, KdlError};

#[derive(Asset, TypePath, Debug, Resource)]
pub struct KdlAsset(pub KdlDocument);

#[derive(Default)]
pub struct KdlLoader;

#[derive(Error, Debug)]
pub enum KdlLoaderError {
    #[error("Failed to read UTF-8 text")]
    FailedToReadText,
    #[error("Failed to read bytes")]
    ReaderFailure,
    #[error("Invalid KDL document")]
    InvalidKdlDocument,
}

impl AssetLoader for KdlLoader {
    type Asset = KdlAsset;
    type Error = KdlLoaderError;
    type Settings = ();

    fn extensions(&self) -> &[&str] {
        &["kdl"]
    }

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            let reader_result = reader.read_to_end(&mut bytes).await;
            if reader_result.is_err() {
                return Err(KdlLoaderError::ReaderFailure);
            }

            let text = String::from_utf8(bytes);
            if text.is_err() {
                return Err(KdlLoaderError::FailedToReadText);
            }

            let document: Result<KdlDocument, KdlError> = text.unwrap().parse();
            if document.is_err() {
                return Err(KdlLoaderError::InvalidKdlDocument);
            }

            Ok(KdlAsset(document.unwrap()))
        })
    }
}
