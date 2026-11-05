use crate::jdplus_main_ws_v1::{
    TsMonikerDto,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsMoniker {
    pub source: std::string::String,
    pub id: std::string::String,
}

impl Default for TsMoniker {
    fn default() -> Self {
        Self {
            source: std::string::String::from("Default"),
            id: std::string::String::from("Default"),
        }
    }
}

impl From<TsMonikerDto> for TsMoniker {
    fn from(dto: TsMonikerDto) -> Self {
        Self {
            source: dto.source,
            id: dto.id,
        }
    }
}

impl From<TsMoniker> for TsMonikerDto {
    fn from(moniker: TsMoniker) -> Self {
        Self {
            source: moniker.source,
            id: moniker.id,
        }
    }
}