/*
 * This file is part of goliath.
 * goliath is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * any later version.
 * goliath is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License
 * along with goliath. If not, see <https://www.gnu.org/licenses/>.
 */

use core::cell::UnsafeCell;
use core::ops::Deref;
use core::ops::DerefMut;
use core::ops::Drop;
use core::sync::atomic::{AtomicBool, Ordering};

pub struct Spinlock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct SpinlockGuard<'a, T: 'a> {
    spinlock: &'a Spinlock<T>,
}

impl<T> Spinlock<T> {
    pub const fn new(value: T) -> Spinlock<T> {
        Spinlock {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> SpinlockGuard<'_, T> {
        loop {
            if self
                .locked
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                break SpinlockGuard { spinlock: self };
            }
        }
    }

    pub unsafe fn force<'a>(&'a self) -> &'a T {
        &*self.data.get()
    }
}

unsafe impl<T> Sync for Spinlock<T> {}

unsafe impl<T> Send for Spinlock<T> {}

impl<'a, T: 'a> Drop for SpinlockGuard<'a, T> {
    fn drop(&mut self) {
        self.spinlock.locked.store(false, Ordering::Release);
    }
}

impl<'a, T: 'a> Deref for SpinlockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.spinlock.data.get() }
    }
}

impl<'a, T: 'a> DerefMut for SpinlockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.spinlock.data.get() }
    }
}
