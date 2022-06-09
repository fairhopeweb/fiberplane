use clap::ArgEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Sorting<T: SortField> {
    #[serde(default = "T::default_sort_field")]
    pub sort_by: T,

    #[serde(default = "T::default_sort_direction")]
    pub sort_direction: SortDirection,
}

impl<T: SortField> Default for Sorting<T> {
    fn default() -> Self {
        Self {
            sort_by: T::default_sort_field(),
            sort_direction: T::default_sort_direction(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, ArgEnum, strum_macros::IntoStaticStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl SortDirection {
    #[inline]
    pub fn to_sql(&self) -> &'static str {
        match self {
            SortDirection::Ascending => "ASC",
            SortDirection::Descending => "DESC",
        }
    }
}

pub trait SortField {
    fn default_sort_field() -> Self;

    #[inline]
    fn default_sort_direction() -> SortDirection {
        SortDirection::Ascending
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, ArgEnum, strum_macros::IntoStaticStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TemplateListSortFields {
    Title,
    CreatedAt,
    UpdatedAt,
}

impl TemplateListSortFields {
    #[inline]
    pub fn to_sql(&self) -> &'static str {
        match self {
            TemplateListSortFields::Title => "title",
            TemplateListSortFields::UpdatedAt => "updated_at",
            TemplateListSortFields::CreatedAt => "created_at",
        }
    }
}

impl SortField for TemplateListSortFields {
    #[inline]
    fn default_sort_field() -> Self {
        Self::Title
    }
}
