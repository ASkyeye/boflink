use std::str::Utf8Error;

use object::Architecture;

#[derive(Debug, thiserror::Error)]
pub enum TryFromImportFileError {
    #[error("symbol field value could not be parsed: {0}")]
    Symbol(Utf8Error),

    #[error("dll field value could not be parsed: {0}")]
    Dll(Utf8Error),

    #[error("import field value could not be parsed: {0}")]
    ImportName(Utf8Error),
}

/// An exported name from a DLL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportName<'a> {
    /// The symbol is exported by the ordinal value.
    Ordinal(u16),

    /// The symbol is exported by the symbol name.
    Name(&'a str),
}

impl<'a> TryFrom<object::read::coff::ImportName<'a>> for ImportName<'a> {
    type Error = Utf8Error;

    fn try_from(value: object::read::coff::ImportName<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            object::coff::ImportName::Ordinal(o) => Self::Ordinal(o),
            object::coff::ImportName::Name(name) => Self::Name(std::str::from_utf8(name)?),
        })
    }
}

/// The type of symbol being imported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImportType {
    /// The symbol is for executable code.
    Code,

    /// The symbol is for misc data.
    Data,

    /// The symbol is a constant value.
    Const,
}

impl From<object::read::coff::ImportType> for ImportType {
    fn from(value: object::read::coff::ImportType) -> Self {
        match value {
            object::coff::ImportType::Code => Self::Const,
            object::coff::ImportType::Data => Self::Data,
            object::coff::ImportType::Const => Self::Const,
        }
    }
}

/// A short import COFF member from import libraries.
pub struct ImportMember<'a> {
    /// The architecture for the import.
    pub(crate) architecture: Architecture,

    /// The public symbol name.
    pub(crate) symbol: &'a str,

    /// The name of the DLL the symbol is from.
    pub(crate) dll: &'a str,

    /// The name exported from the DLL.
    pub(crate) import: ImportName<'a>,

    /// The type of import.
    #[allow(unused)]
    pub(crate) typ: ImportType,
}

impl<'a> TryFrom<object::read::coff::ImportFile<'a>> for ImportMember<'a> {
    type Error = TryFromImportFileError;

    fn try_from(value: object::read::coff::ImportFile<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            architecture: value.architecture(),
            symbol: std::str::from_utf8(value.symbol()).map_err(TryFromImportFileError::Symbol)?,
            dll: std::str::from_utf8(value.dll()).map_err(TryFromImportFileError::Dll)?,
            import: value
                .import()
                .try_into()
                .map_err(TryFromImportFileError::ImportName)?,
            typ: value.import_type().into(),
        })
    }
}
