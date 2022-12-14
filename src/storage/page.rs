// SOURCES + USEFUL LINKS
// https://github.com/cmu-db/bustub/blob/master/src/include/storage/page/page.h

#![allow(dead_code, unused_imports)]
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::concurrency::{
    rw_acquire_excl, rw_acquire_shared, rw_acquire_upgradable, rw_release_excl, rw_release_shared,
    rw_release_upgradable, rw_upgrade_shared, RwSynchronized,
};
use crate::shared::{PageId, PAGE_SIZE};

#[serde_as]
#[derive(Derivative, Deserialize, Serialize)]
#[derivative(Default)]
pub struct Page {
    #[serde_as(as = "[_; PAGE_SIZE]")]
    #[derivative(Default(value = "[0u8; PAGE_SIZE]"))]
    data: [u8; PAGE_SIZE],
    #[derivative(Default(value = "-1"))]
    id: PageId,
    #[derivative(Default(value = "0"))]
    pin_count: usize,
    #[derivative(Default(value = "false"))]
    dirty: bool,
}

impl Page {
    const PAGE_HEADER_SIZE: usize = 8;

    pub fn new(id: PageId, data: &[u8; PAGE_SIZE]) -> Self {
        Page {
            data: *data,
            id,
            pin_count: 0,
            dirty: false,
        }
    }

    #[inline]
    pub fn get_data(&self) -> [u8; PAGE_SIZE] {
        self.data
    }

    #[inline]
    pub fn get_id(&self) -> PageId {
        self.id
    }

    #[inline]
    pub fn get_pin_count(&self) -> usize {
        self.pin_count
    }

    #[inline]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_data(mut self, data: &[u8; PAGE_SIZE]) {
        self.data = *data;
    }
}

// pub type Page = RwSynchronized<PageInternal>;

// #[inline]
// pub fn w_latch(page: &Page) {
//     unsafe {
//         rw_acquire_excl(&page);
//     }
// }

// #[inline]
// pub fn w_unlatch(page: &Page) {
//     unsafe {
//         rw_release_excl(&page);
//     }
// }

// #[inline]
// pub fn r_latch(page: &Page) {
//     unsafe {
//         rw_acquire_shared(&page);
//     }
// }

// #[inline]
// pub fn r_unlatch(page: &Page) {
//     unsafe {
//         rw_release_shared(&page);
//     }
// }

// #[inline]
// pub fn u_latch(page: &Page) {
//     unsafe {
//         rw_acquire_upgradable(&page);
//     }
// }

// #[inline]
// pub fn u_unlatch(page: &Page) {
//     unsafe {
//         rw_release_upgradable(&page);
//     }
// }

// #[inline]
// pub fn u_upgrade_latch(page: &Page) {
//     unsafe {
//         rw_upgrade_shared(&page);
//     }
// }
