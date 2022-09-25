#![allow(dead_code, unused_imports)]

use std::borrow::BorrowMut;
use std::collections::{HashMap, LinkedList};
use std::sync::Arc;

use crate::concurrency::{RwSynchronized, Synchronized};
use crate::shared::{FrameId, PageId, PAGE_SIZE};
use crate::storage::diskmgr::DiskMgr;
use crate::storage::free_list::FreeList;
use crate::storage::page::Page;
use crate::storage::page_table::PageTable;

use super::diskmgr::DiskMgrInternal;

pub struct BufferPoolFrameInternal {
    frame_id: FrameId,
    page: Page,
}

impl BufferPoolFrameInternal {
    fn new(frame_id: FrameId) -> Self {
        Self {
            frame_id,
            page: Page::new(),
        }
    }
}

pub type BufferPoolFrame = Synchronized<BufferPoolFrameInternal>;
pub type BufferPoolFrames = RwSynchronized<Vec<BufferPoolFrame>>;

pub struct BufferPoolInternal {
    pool_size: usize,
    replacer_k: usize,
    diskmgr: DiskMgr,
    page_table: PageTable,
    free_list: FreeList<FrameId>,
    frames: BufferPoolFrames,
}

impl BufferPoolInternal {
    pub fn new(pool_size: usize, replacer_k: usize, diskmgr: DiskMgr) -> Self {
        let mut free_list_internal: LinkedList<FrameId> = LinkedList::new();
        let mut frames_internal = Vec::new();
        for i in 0..pool_size {
            free_list_internal.push_back(i as isize);
            frames_internal.push(Arc::new(parking_lot::Mutex::new(
                BufferPoolFrameInternal::new(i as isize),
            )));
        }
        Self {
            pool_size,
            replacer_k,
            diskmgr,
            page_table: Arc::new(parking_lot::RwLock::new(HashMap::new())),
            free_list: Arc::new(parking_lot::RwLock::new(free_list_internal)),
            frames: Arc::new(parking_lot::RwLock::new(frames_internal)),
        }
    }
}

pub type BufferPool = RwSynchronized<BufferPoolInternal>;

#[cfg(test)]
mod tests {
    #![allow(unused_variables)]
    use lazy_static::lazy_static;
    use std::sync::Arc;

    use crate::shared::Song;
    use crate::storage::diskmgr::{DiskMgr, DiskMgrInternal};
    use crate::storage::ioutil;
    use crate::storage::page::Page;

    use super::{BufferPool, BufferPoolFrameInternal, BufferPoolInternal};

    lazy_static! {
        static ref BUFMGR_TEST_FILE: String =
            crate::shared::cwd() + "/data/test/__bufmgr__/bufmgr.bin";
        static ref BUFMGR: BufferPool =
            Arc::new(parking_lot::RwLock::new(BufferPoolInternal::new(
                10,
                1,
                Arc::new(parking_lot::RwLock::new(DiskMgrInternal::new(
                    &BUFMGR_TEST_FILE,
                ))),
            )));
        static ref STATE: Arc<(parking_lot::Mutex<bool>, parking_lot::Condvar)> =
            Arc::new((parking_lot::Mutex::new(true), parking_lot::Condvar::new()));
    }

    #[test]
    fn create() {
        let buffer_pool = Arc::new(parking_lot::RwLock::new(BufferPoolInternal::new(
            10,
            1,
            Arc::new(parking_lot::RwLock::new(DiskMgrInternal::new(
                &BUFMGR_TEST_FILE,
            ))),
        )));

        let internal = (*buffer_pool).data_ptr();
        let frames = unsafe { (*internal).frames.read() };
        assert!(frames.len() == 10);
    }

    #[test]
    fn setup_full_bufmgr_test() {
        let (mutex, condvar) = &**STATE;
        let diskmgr = unsafe { &(*BUFMGR.data_ptr()).diskmgr };
        let diskmgr_handle = diskmgr.read();
        let afraid = Song::new(1, "Afraid", "The Neighbourhood");
        let reflections = Song::new(2, "Reflections", "The Neighbourhood");
        let chlorine = Song::new(3, "Chlorine", "21 Pilots");
        let nervous = Song::new(4, "Nervous", "The Neighbourhood");

        let mut prepared = mutex.lock();
        assert!(!diskmgr_handle.clear().is_err());

        assert!(!diskmgr_handle
            .write_page(afraid.id as isize, &ioutil::to_buffer(afraid).unwrap())
            .is_err());
        assert!(!diskmgr_handle
            .write_page(
                reflections.id as isize,
                &ioutil::to_buffer(reflections).unwrap()
            )
            .is_err());
        assert!(!diskmgr_handle
            .write_page(chlorine.id as isize, &ioutil::to_buffer(chlorine).unwrap())
            .is_err());
        assert!(!diskmgr_handle
            .write_page(nervous.id as isize, &ioutil::to_buffer(nervous).unwrap())
            .is_err());

        *prepared = true;
        condvar.notify_one();
    }

    #[test]
    fn full_bufmgr_test() {
        let (mutex, condvar) = &**STATE;
        let mut prepared = mutex.lock();
        while !*prepared {
            condvar.wait(&mut prepared);
        }
    }
}
