use std::{rc::Rc, sync::Arc};

use parking_lot::{Mutex, ReentrantMutex};

/**
 * Deadlock is a situation where two or more locks are waiting for one another
 * Threads become "stuck" and are unable to continue
 *
 * Deadlocks can occur when:
 * Using multiple locks
 * Recursing while taking a lock
 * Locking the same lock twice
 *
 *
 * ReentrantMutex allows multiple locks from the same thread
 *  Use for recursive functions
 *  Anytime you need to lock the same lock more than once
 * try_lock() can prevent deadlocks
 *  Drop all locks used in function and try again after a short period
 * Use backoff crate for optimized performance
 */

// This function will cause a deadlock as it is trying to lock the same lock twice
pub fn recurse(data: Rc<Mutex<u32>>, remaining: usize) -> usize {
    let mut locked = data.lock();
    match remaining {
        rem if rem == 0 => 0,
        _ => recurse(Rc::clone(&data), remaining - 1),
    }
}

// This function will not cause a deadlock as it is using a ReentrantMutex
pub fn fix_recurse(data: Rc<ReentrantMutex<u32>>, remaining: usize) -> usize {
    let mut locked = data.lock();
    match remaining {
        rem if rem == 0 => 0,
        _ => fix_recurse(Rc::clone(&data), remaining - 1),
    }
}

type ArcAccount = Arc<Mutex<Account>>;

pub struct Account {
    pub balance: i64,
}

pub fn transfer_funds(sender: ArcAccount, reciever: ArcAccount, amount: i64) {
    let mut lock_a = sender.lock();
    let mut lock_b = reciever.lock();

    lock_a.balance -= amount;
    lock_b.balance += amount;
}
