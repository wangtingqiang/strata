use serde::Serialize;

use crate::pagination::{CursorPagination, PagePagination};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub pagination: PagePagination,
}

impl<T> Paginated<T> {
    pub fn map_items<U>(self, f: impl FnMut(T) -> U) -> Paginated<U> {
        Paginated {
            items: self.items.into_iter().map(f).collect(),
            pagination: self.pagination,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CursorPaginated<T, C> {
    pub items: Vec<T>,
    pub pagination: CursorPagination<C>,
}

impl<T, C> CursorPaginated<T, C> {
    pub fn map_items<U>(self, f: impl FnMut(T) -> U) -> CursorPaginated<U, C> {
        CursorPaginated {
            items: self.items.into_iter().map(f).collect(),
            pagination: self.pagination,
        }
    }

    pub fn map_cursor<U>(self, f: impl FnOnce(C) -> U) -> CursorPaginated<T, U> {
        CursorPaginated {
            items: self.items,
            pagination: self.pagination.map_cursor(f),
        }
    }
}
