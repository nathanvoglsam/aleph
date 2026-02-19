use std::collections::HashMap;
use std::sync::LazyLock;

use init_list::InitList;

use crate::ObjectDescription;

/// This is the head of the object system registry. All objects that interact with the object system
/// will push an entry onto this list during the static init phase.
///
/// # Warning
///
/// Do not use this. This is only public for macros to consume. Please use [`TYPES`] instead.
#[doc(hidden)]
pub static __UNSAFE_REGISTRY_HEAD: InitList<&'static ObjectDescription> = InitList::new();

/// A lazily initialized table of all types registered into the object system.
pub static TYPES: LazyLock<HashMap<uuid::Uuid, &'static ObjectDescription>> = LazyLock::new(|| {
    // We don't care if someone's sealed the list before, only that it has been sealed.
    let _ = __UNSAFE_REGISTRY_HEAD.seal();
    assert_no_duplicate_ids_registered();
    HashMap::from_iter(__UNSAFE_REGISTRY_HEAD.iter().copied().map(|v| (v.id, v)))
});

/// Utility function that will walk the list of registered types and assert if there are any
/// duplicate type IDs registered.
fn assert_no_duplicate_ids_registered() {
    let mut types = HashMap::new();
    for object in __UNSAFE_REGISTRY_HEAD.iter().copied() {
        let existing = types.insert(object.id, object);
        if let Some(existing) = existing {
            assert_eq!(object.id, existing.id); // Just being careful
            panic!(
                "Colliding IObject type IDs detected. '{}' and '{}' have the same ID of '{}'!",
                existing.name, object.name, existing.id
            );
        }
    }
}

/// Type alias made available for macros.
#[doc(hidden)]
pub type ListEntry = init_list::ListItem<&'static ObjectDescription>;
