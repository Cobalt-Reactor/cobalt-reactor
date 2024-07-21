use crate::prelude::*;
use bevy::prelude::*;
use std::marker::PhantomData;

/// The context of the save (i.e. global, user, or slot)
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Context {
    /// The global save context, saved in the root folder
    Global,
    /// The user save context, saved in the user folder (set by `set_user_id`)
    User,
    /// The slot save context, saved in the slot folder (set by `set_save_slot`), default
    #[default]
    Slot,
}

/// An event used to request saving of data local to the `SaveSlot`
/// non-ascii characters are replaced with underscores
#[derive(Event, Debug, Clone)]
pub struct SaveRequest<T>
where
    T: SaveData,
{
    /// The subdirectory to save the file to relative to the `SaveSlot`
    pub(crate) sub_directory: Option<String>,
    /// The name of the file
    pub(crate) file_name: String,
    /// The data to save
    pub(crate) save_data: T,
    /// The context of the save
    pub(crate) context: Context,
}

impl<T> SaveRequest<T>
where
    T: SaveData,
{
    /// Creates a new `SaveRequest`
    pub fn new(file_name: &str, save_data: T) -> Self {
        Self {
            save_data,
            file_name: file_name.to_string(),
            sub_directory: None,
            context: Context::Slot,
        }
    }

    /// Sets the subdirectory relative to the `SaveSlot`
    pub fn with_sub_directory(mut self, sub_directory: &str) -> Self {
        self.sub_directory = Some(sub_directory.replace(|c: char| !c.is_ascii(), "_"));
        self
    }

    /// Sets the context of the save, `Context::Slot` by default
    pub fn with_context(mut self, context: Context) -> Self {
        self.context = context;
        self
    }
}

/// An event used to request loading of data local to the `SaveSlot`
/// non-ascii characters are replaced with underscores
#[derive(Event, Debug, Clone)]
pub struct LoadRequest<T>
where
    T: SaveData,
{
    /// The subdirectory to save the file to relative to the `SaveSlot`
    pub(crate) sub_directory: Option<String>,
    /// The name of the file
    pub(crate) file_name: String,
    /// The context of the save
    pub(crate) context: Context,
    /// The data to save
    marker: PhantomData<T>,
}

impl<T> LoadRequest<T>
where
    T: SaveData,
{
    /// Creates a new `LoadRequest`
    pub fn new(file_name: &str) -> Self {
        Self {
            sub_directory: None,
            context: Context::Slot,
            file_name: file_name.to_string(),
            marker: PhantomData,
        }
    }

    /// Sets the subdirectory relative to the `SaveSlot`
    pub fn with_sub_directory(mut self, sub_directory: &str) -> Self {
        self.sub_directory = Some(sub_directory.replace(|c: char| !c.is_ascii(), "_"));
        self
    }

    /// Sets the context of the save, `Context::Slot` by default
    pub fn with_context(mut self, context: Context) -> Self {
        self.context = context;
        self
    }
}

/// An event sent when a `LoadRequest` has completed
#[derive(Event, Debug, Clone)]
pub struct LoadComplete<T>
where
    T: SaveData,
{
    /// The outcome of the load
    pub outcome: LoadOutcome,
    /// The name of the file
    pub file_name: String,
    pub(crate) _marker: PhantomData<T>,
}

/// An event sent when a `SaveRequest` has completed
#[derive(Event, Debug, Clone)]
pub struct SaveComplete<T>
where
    T: SaveData,
{
    /// The outcome of the save
    pub outcome: SaveOutcome,
    /// The name of the file
    pub file_name: String,
    pub(crate) _marker: PhantomData<T>,
}
