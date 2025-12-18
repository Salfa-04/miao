//!
//! Persistent memory cell for non-zeroed SRAM.
//!
//! This module provides [`MemCell`], a low-level abstraction for memory regions
//! that survive resets (for example, Backup SRAM).
//!
//! # Safety Model
//!
//! `MemCell<T>` provides raw access to a memory region that is assumed to:
//!
//! - Survive system resets
//! - Not be automatically zero-initialized
//! - Exist for the entire lifetime of the program
//!
//! This type deliberately does **not** participate in Rust's ownership,
//! aliasing, or drop semantics. All safety guarantees must be upheld
//! externally by the caller.
//!
//! In particular:
//!
//! - Initialization state is tracked manually via a magic value
//! - Interior mutability is provided through raw pointers
//! - No attempt is made to synchronize concurrent access
//!
//! Misuse of this type can easily result in undefined behavior.
//!
//! # Note:
//!
//! This abstraction does not provide any memory ordering guarantees beyond
//! volatile access. On systems with caches, DMA, or multiple cores, additional
//! synchronization may be required.
//!
//! This abstraction does not provide atomicity guarantees.
//! Partial writes may be observable if a reset occurs mid-operation.
//!

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{Ordering, compiler_fence};

///
/// A persistent, uninitialized memory cell.
///
/// This type is intended for use with memory regions that are **not cleared**
/// on reset (for example, Backup SRAM).
///
#[repr(C)]
pub struct MemCell<T: Sized> {
    magic: UnsafeCell<MaybeUninit<u64>>,
    value: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T: Sized + Send> Send for MemCell<T> {}
unsafe impl<T: Sized + Sync> Sync for MemCell<T> {}

impl<T> MemCell<T> {
    const ABI_VERSION: u16 = 0x0001;

    ///
    /// Creates a new uninitialized memory cell.
    ///
    /// This function does **not** write anything to memory. When placed in a
    /// non-zeroed memory section, existing contents are preserved.
    ///
    #[inline(always)]
    pub const fn uninit() -> Self {
        Self {
            magic: UnsafeCell::new(MaybeUninit::uninit()),
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    ///
    /// Returns a pointer to the magic number.
    ///
    /// The returned pointer may reference uninitialized or stale memory.
    /// No validity checks are performed.
    ///
    #[inline(always)]
    fn magic(&self) -> *mut u64 {
        self.magic.get().cast()
    }

    ///
    /// Returns a pointer to the contained value.
    ///
    /// The memory pointed to by the returned pointer may be uninitialized
    /// or contain data from a previous boot.
    ///
    #[inline(always)]
    fn value(&self) -> *mut T {
        self.value.get().cast()
    }
}

impl<T> MemCell<T> {
    ///
    /// Magic value indicating that the stored value is valid for this firmware version.
    ///
    /// The lower 16 bits encode an ABI version. A mismatch causes the cell to be
    /// treated as uninitialized.
    ///
    const MAGIC: u64 = 0xCAFA_DEAD_BEEF_0000 | ((Self::ABI_VERSION as u64) & 0xFFFF);

    ///
    /// Initialize the memory cell with a value.
    ///
    /// This function writes `val` into the backing memory and then marks the
    /// cell as initialized by writing a magic value.
    ///
    /// If a system reset occurs before the magic value is written, the cell
    /// will be treated as uninitialized on the next boot.
    ///
    /// Calling `init()` overwrites any existing stored value without dropping it.
    ///
    /// # Safety
    ///
    /// The caller must ensure **all** of the following:
    ///
    /// ## Exclusive access
    ///
    /// - No concurrent access to this `MemCell` occurs during the call,
    ///   including from interrupts, other CPU cores, or DMA.
    /// - No other live pointers (raw or reference) exist to the same memory.
    ///
    /// ## Type requirements
    ///
    /// - `T` must be valid to persist across system resets.
    /// - `T` must not contain:
    ///   - References or pointers that become invalid after reset
    ///   - Heap-allocated data (`Box`, `Vec`, `String`, etc.)
    ///   - Interior assumptions about initialization or drop order
    /// - `T` must not rely on its `Drop` implementation being executed.
    ///
    /// In practice, `T` should be a plain data type composed only of
    /// integers, booleans, or other reset-stable values.
    ///
    /// ## Memory validity
    ///
    /// - The memory backing this `MemCell` must remain valid and mapped for
    ///   the entire lifetime of the program.
    /// - The memory must not be repurposed or reinitialized by the runtime,
    ///   linker, or hardware.
    ///
    /// Violating any of these requirements may result in undefined behavior.
    ///
    pub unsafe fn init(&self, val: T) -> *mut T {
        unsafe {
            self.value().write_volatile(val);
            compiler_fence(Ordering::SeqCst);
            self.magic().write_volatile(Self::MAGIC);
        }

        self.value()
    }

    ///
    /// Obtain a mutable pointer to the stored value if initialized.
    ///
    /// Returns `None` if the memory cell is not currently marked as initialized.
    ///
    /// This function performs a volatile read of the magic value to determine
    /// initialization state.
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    ///
    /// ## Aliasing and exclusivity
    ///
    /// - No mutable or immutable references exist to the value while the
    ///   returned pointer is in use.
    /// - The returned pointer is not used concurrently from multiple contexts
    ///   unless such access is externally synchronized.
    ///
    /// ## Lifetime
    ///
    /// - The returned pointer must not be used after:
    ///   - `invalidate()` is called
    ///   - `init()` is called again
    ///   - The backing memory becomes invalid for any reason
    ///
    /// ## Semantics
    ///
    /// - This function does **not** establish Rust-level exclusive access.
    /// - The pointer provides only *logical* mutability.
    ///
    /// Failure to uphold these conditions may result in undefined behavior.
    ///
    pub unsafe fn get(&self) -> Option<*mut T> {
        let magic = unsafe { self.magic().read_volatile() };
        if magic == Self::MAGIC {
            Some(self.value())
        } else {
            None
        }
    }
}

impl<T> MemCell<T> {
    ///
    /// Mark the memory cell as uninitialized.
    ///
    /// After this call, `get()` will return `None` until the cell is
    /// reinitialized via `init()`.
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    ///
    /// - No pointers previously obtained from `get()` or `init()` are still
    ///   being used.
    /// - No concurrent access to this `MemCell` occurs during the call.
    ///
    /// Violating these requirements may result in undefined behavior.
    ///
    pub unsafe fn invalidate(&self) {
        unsafe { self.magic().write_volatile(0) }
    }

    pub const fn as_ptr(&self) -> *const Self {
        &raw const *self
    }
}
