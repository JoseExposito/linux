// SPDX-License-Identifier: GPL-2.0

use core::{
    cell::UnsafeCell,
    ptr,
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

pub trait GetLinks {
    type EntryType: ?Sized;
    fn get_links(data: &Self::EntryType) -> &Links<Self::EntryType>;
}

pub struct Links<T: ?Sized>(UnsafeCell<ListEntry<T>>);

impl<T: ?Sized> Links<T> {
    pub fn new() -> Self {
        Self(UnsafeCell::new(ListEntry::new()))
    }
}

struct ListEntry<T: ?Sized> {
    next: Option<NonNull<T>>,
    prev: Option<NonNull<T>>,
    inserted: AtomicBool,
}

impl<T: ?Sized> ListEntry<T> {
    fn new() -> Self {
        Self {
            next: None,
            prev: None,
            inserted: AtomicBool::new(false),
        }
    }

    fn acquire_for_insertion(&mut self) -> bool {
        self.inserted
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    fn release_after_removal(&mut self) {
        self.inserted.store(false, Ordering::Release);
    }
}

pub struct RawList<G: GetLinks> {
    head: Option<NonNull<G::EntryType>>,
}

impl<G: GetLinks> RawList<G> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn insert_after_priv(
        &mut self,
        existing: &G::EntryType,
        new_entry: &mut ListEntry<G::EntryType>,
        new_ptr: Option<NonNull<G::EntryType>>,
    ) -> bool {
        if !new_entry.acquire_for_insertion() {
            // Nothing to do if already inserted.
            return false;
        }

        {
            let existing_links = unsafe { &mut *G::get_links(existing).0.get() };
            new_entry.next = existing_links.next;
            existing_links.next = new_ptr;
        }

        new_entry.prev = Some(NonNull::from(existing));
        let next_links = unsafe { &mut *G::get_links(new_entry.next.unwrap().as_ref()).0.get() };
        next_links.prev = new_ptr;
        true
    }

    pub fn insert_after(&mut self, existing: &G::EntryType, new: &G::EntryType) -> bool {
        let new_entry = unsafe { &mut *G::get_links(new).0.get() };
        self.insert_after_priv(existing, new_entry, Some(NonNull::from(new)))
    }

    fn push_back_internal(&mut self, new: &G::EntryType) -> bool {
        let new_entry = unsafe { &mut *G::get_links(new).0.get() };
        let new_ptr = Some(NonNull::from(new));
        match self.back() {
            Some(back) => self.insert_after_priv(unsafe { back.as_ref() }, new_entry, new_ptr),
            None => {
                if !new_entry.acquire_for_insertion() {
                    // Nothing to do if already inserted.
                    return false;
                }
                self.head = new_ptr;
                new_entry.next = new_ptr;
                new_entry.prev = new_ptr;
                true
            }
        }
    }

    pub unsafe fn push_back(&mut self, new: &G::EntryType) -> bool {
        self.push_back_internal(new)
    }

    fn remove_internal(&mut self, data: &G::EntryType) -> bool {
        let links = unsafe { &mut *G::get_links(data).0.get() };
        let next = if let Some(next) = links.next {
            next
        } else {
            // Nothing to do if the entry is not on the list.
            return false;
        };

        if ptr::eq(data, next.as_ptr()) {
            // We're removing the only element.
            self.head = None
        } else {
            // Update the head if we're removing it.
            if let Some(raw_head) = self.head {
                if ptr::eq(data, raw_head.as_ptr()) {
                    self.head = Some(next);
                }
            }

            unsafe { &mut *G::get_links(links.prev.unwrap().as_ref()).0.get() }.next = links.next;
            unsafe { &mut *G::get_links(next.as_ref()).0.get() }.prev = links.prev;
        }

        // Reset the links of the element we're removing so that we know it's not on any list.
        links.next = None;
        links.prev = None;
        links.release_after_removal();
        true
    }

    pub unsafe fn remove(&mut self, data: &G::EntryType) -> bool {
        self.remove_internal(data)
    }

    fn pop_front_internal(&mut self) -> Option<NonNull<G::EntryType>> {
        let head = self.head?;
        unsafe { self.remove(head.as_ref()) };
        Some(head)
    }

    pub fn pop_front(&mut self) -> Option<NonNull<G::EntryType>> {
        self.pop_front_internal()
    }

    pub fn front(&self) -> Option<NonNull<G::EntryType>> {
        self.head
    }

    pub fn back(&self) -> Option<NonNull<G::EntryType>> {
        unsafe { &*G::get_links(self.head?.as_ref()).0.get() }.prev
    }

    pub fn cursor_front(&self) -> Cursor<'_, G> {
        Cursor::new(self, self.front())
    }

    pub fn cursor_front_mut(&mut self) -> CursorMut<'_, G> {
        CursorMut::new(self, self.front())
    }
}

struct CommonCursor<G: GetLinks> {
    cur: Option<NonNull<G::EntryType>>,
}

impl<G: GetLinks> CommonCursor<G> {
    fn new(cur: Option<NonNull<G::EntryType>>) -> Self {
        Self { cur }
    }

    fn move_next(&mut self, list: &RawList<G>) {
        match self.cur.take() {
            None => self.cur = list.head,
            Some(cur) => {
                if let Some(head) = list.head {
                    // SAFETY: We have a shared ref to the linked list, so the links can't change.
                    let links = unsafe { &*G::get_links(cur.as_ref()).0.get() };
                    if links.next.unwrap() != head {
                        self.cur = links.next;
                    }
                }
            }
        }
    }

    fn move_prev(&mut self, list: &RawList<G>) {
        match list.head {
            None => self.cur = None,
            Some(head) => {
                let next = match self.cur.take() {
                    None => head,
                    Some(cur) => {
                        if cur == head {
                            return;
                        }
                        cur
                    }
                };
                // SAFETY: There's a shared ref to the list, so the links can't change.
                let links = unsafe { &*G::get_links(next.as_ref()).0.get() };
                self.cur = links.prev;
            }
        }
    }
}

pub struct Cursor<'a, G: GetLinks> {
    cursor: CommonCursor<G>,
    list: &'a RawList<G>,
}

impl<'a, G: GetLinks> Cursor<'a, G> {
    fn new(list: &'a RawList<G>, cur: Option<NonNull<G::EntryType>>) -> Self {
        Self {
            list,
            cursor: CommonCursor::new(cur),
        }
    }

    pub fn current(&self) -> Option<&'a G::EntryType> {
        let cur = self.cursor.cur?;
        // SAFETY: Objects must be kept alive while on the list.
        Some(unsafe { &*cur.as_ptr() })
    }

    pub fn move_next(&mut self) {
        self.cursor.move_next(self.list);
    }
}

pub struct CursorMut<'a, G: GetLinks> {
    cursor: CommonCursor<G>,
    list: &'a mut RawList<G>,
}

impl<'a, G: GetLinks> CursorMut<'a, G> {
    fn new(list: &'a mut RawList<G>, cur: Option<NonNull<G::EntryType>>) -> Self {
        Self {
            list,
            cursor: CommonCursor::new(cur),
        }
    }

    pub fn current(&mut self) -> Option<&mut G::EntryType> {
        let cur = self.cursor.cur?;
        // SAFETY: Objects must be kept alive while on the list.
        Some(unsafe { &mut *cur.as_ptr() })
    }

    /// Removes the entry the cursor is pointing to and advances the cursor to the next entry. It
    /// returns a raw pointer to the removed element (if one is removed).
    pub fn remove_current(&mut self) -> Option<NonNull<G::EntryType>> {
        let entry = self.cursor.cur?;
        self.cursor.move_next(self.list);
        unsafe { self.list.remove(entry.as_ref()) };
        Some(entry)
    }

    pub fn peek_next(&mut self) -> Option<&mut G::EntryType> {
        let mut new = CommonCursor::new(self.cursor.cur);
        new.move_next(self.list);
        // SAFETY: Objects must be kept alive while on the list.
        Some(unsafe { &mut *new.cur?.as_ptr() })
    }

    pub fn peek_prev(&mut self) -> Option<&mut G::EntryType> {
        let mut new = CommonCursor::new(self.cursor.cur);
        new.move_prev(self.list);
        // SAFETY: Objects must be kept alive while on the list.
        Some(unsafe { &mut *new.cur?.as_ptr() })
    }

    pub fn move_next(&mut self) {
        self.cursor.move_next(self.list);
    }
}
