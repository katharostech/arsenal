// File modified from: https://github.com/amethyst/amethyst/blob/v0.10/amethyst_assets/src/prefab/mod.rs
//
// Copyright 2016 The Amethyst Project Developers

// Licensed under the Apache License, Version 2.0, <docs/LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <docs/LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::marker::PhantomData;

use amethyst::core::specs::prelude::{
    Component, DenseVecStorage, Entity, FlaggedStorage, Read, WriteExpect, ReadExpect, SystemData, WriteStorage,
};

use amethyst::assets::{Asset, AssetStorage, Format, Handle, Loader, Progress, ProgressCounter, PrefabData, Directory};

pub use amethyst::core::specs::error::Error as PrefabError;

/// Loads an asset relative to the working directory.
///
/// This is kind of haphazard right now and will be completely reworked later.
#[derive(Clone, Deserialize, Serialize)]
pub enum RelativeAssetPrefab<A, F>
where
    A: Asset,
    F: Format<A>,
{
    /// From existing handle
    #[serde(skip)]
    Handle(Handle<A>),

    /// From file, (name, format, format options)
    File(String, F, F::Options),
}

impl<'a, A, F> PrefabData<'a> for RelativeAssetPrefab<A, F>
where
    A: Asset,
    F: Format<A> + Clone,
    F::Options: Clone,
{
    type SystemData = (
        WriteExpect<'a, Loader>,
        WriteStorage<'a, Handle<A>>,
        Read<'a, AssetStorage<A>>,
    );

    type Result = Handle<A>;

    fn add_to_entity(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        _: &[Entity],
    ) -> Result<Handle<A>, PrefabError> {
        let handle = match *self {
            RelativeAssetPrefab::Handle(ref handle) => handle.clone(),
            RelativeAssetPrefab::File(..) => unreachable!(),
        };
        system_data.1.insert(entity, handle.clone()).map(|_| handle)
    }

    fn load_sub_assets(
        &mut self,
        progress: &mut ProgressCounter,
        system_data: &mut Self::SystemData,
    ) -> Result<bool, PrefabError> {
        system_data.0.add_source(
            "current_dir",
            Directory::new(std::env::current_dir().expect("Could not get current working dir"))
        );
        let handle = if let RelativeAssetPrefab::File(ref name, ref format, ref options) = *self {
            Some(system_data.0.load_from(
                name.as_ref(),
                format.clone(),
                options.clone(),
                "current_dir",
                progress,
                &system_data.2,
            ))
        } else {
            None
        };
        if let Some(handle) = handle {
            *self = RelativeAssetPrefab::Handle(handle);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
