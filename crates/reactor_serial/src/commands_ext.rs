use crate::prelude::*;
use bevy::prelude::*;

/// An extension trait for [`Commands`] and [`World`]
/// to allow easy initialization of a save slot
pub trait SaveSlotExt {
    /// If you want saves to be per-user, you should set the user id here
    /// If this is set all save slots will be relative to a folder
    /// named after this id
    fn set_user_id(&mut self, _: &str);

    /// Sets the current save slot to the given name, relative to the save root
    /// or the user id folder if one is set
    /// If no slot is set, the default save slot (`default_slot`) will be used
    fn set_save_slot(&mut self, slot_name: &str);
}

impl<'w, 's> SaveSlotExt for Commands<'w, 's> {
    fn set_save_slot(&mut self, slot_name: &str) {
        self.insert_resource(CurrentSaveSlot(SaveSlot {
            name: slot_name.to_string(),
        }));
    }

    fn set_user_id(&mut self, id: &str) {
        self.insert_resource(CurrentUserID { id: id.to_string() });
    }
}

impl SaveSlotExt for World {
    fn set_save_slot(&mut self, slot_name: &str) {
        self.insert_resource(CurrentSaveSlot(SaveSlot {
            name: slot_name.to_string(),
        }));
    }

    fn set_user_id(&mut self, id: &str) {
        self.insert_resource(CurrentUserID { id: id.to_string() });
    }
}
