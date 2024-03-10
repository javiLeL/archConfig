use serde::{Deserialize, Serialize};
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Structure, Type};

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize)]
pub struct SessionPath {
    id: String,
    /// Name of session user
    path: OwnedObjectPath,
}

impl SessionPath {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn path(&self) -> &OwnedObjectPath {
        &self.path
    }
}

impl TryFrom<OwnedValue> for SessionPath {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let value = <Structure<'_>>::try_from(value)?;
        Ok(Self {
            id: <String>::try_from(value.fields()[0].clone())?,
            path: <OwnedObjectPath>::try_from(value.fields()[1].clone())?,
        })
    }
}
