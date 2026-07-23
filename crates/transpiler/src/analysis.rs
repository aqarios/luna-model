//! Typed analysis keys and analysis storage.

use std::{
    any::Any,
    collections::HashMap,
    fmt::Debug,
    marker::PhantomData,
    sync::{Arc, OnceLock, RwLock},
};

use crate::error::{TranspileErrorKind, TranspileKindResult};

/// Typed key for accessing analysis results.
/// The type parameter ensures compile-time type safety.
#[derive(Debug)]
pub struct AnalysisKey<T: 'static> {
    pub(crate) name: String,
    _marker: PhantomData<fn() -> T>,
}

impl<T: 'static> AnalysisKey<T> {
    /// Creates a new typed analysis key.
    pub const fn new(name: String) -> Self {
        Self {
            name,
            _marker: PhantomData,
        }
    }
}

/// A serialized analysis result deposited by a **plugin** analysis, retrievable by name across the
/// C-ABI.
///
/// Native analyses store their result as a typed value (see [`AnalysisManager::insert`]) and native
/// passes read it back typed with zero copies. A plugin analysis produces a result whose Rust type
/// the host does not know, so it deposits `(type_tag, bytes)` here instead; a plugin pass reads the
/// bytes back by name and deserializes into its own copy of the type.
#[derive(Clone, Debug)]
pub struct SerializedAnalysis {
    /// Stable type tag of the producing result type (verified by the consumer before decoding).
    pub type_tag: String,
    /// The result's serialized bytes.
    pub bytes: Vec<u8>,
}

/// Type-safe storage for analysis results keyed by [`AnalysisKey`].
#[derive(Clone, Default, Debug)]
pub struct AnalysisManager {
    results: HashMap<String, Arc<dyn Any + Send + Sync>>,
}

impl AnalysisManager {
    /// Get an analysis result (returns None if not computed)
    pub fn get<T: Send + Sync + 'static>(&self, key: &AnalysisKey<T>) -> Option<&T> {
        self.results
            .get(&key.name)
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    /// Get an analysis result (error if not available)
    pub fn require<T: Send + Sync + 'static>(
        &self,
        key: &AnalysisKey<T>,
    ) -> TranspileKindResult<&T> {
        self.get(key)
            .ok_or_else(|| TranspileErrorKind::MissingAnalysis {
                name: key.name.clone(),
            })
    }

    /// Stores an analysis result under its typed key.
    pub fn insert<T: Send + Sync + 'static>(&mut self, key: &AnalysisKey<T>, value: T) {
        self.results.insert(key.name.clone(), Arc::new(value));
    }

    /// Deposits a plugin analysis result as serialized bytes under `name`.
    ///
    /// The bytes are opaque to the host; a plugin pass retrieves them with [`get_serialized`] and
    /// decodes into its own result type after checking `type_tag`.
    ///
    /// [`get_serialized`]: Self::get_serialized
    pub fn insert_serialized(&mut self, name: &str, type_tag: String, bytes: Vec<u8>) {
        self.results.insert(
            name.to_string(),
            Arc::new(SerializedAnalysis { type_tag, bytes }),
        );
    }

    /// Retrieves a plugin analysis result deposited by [`insert_serialized`], if present.
    ///
    /// [`insert_serialized`]: Self::insert_serialized
    pub fn get_serialized(&self, name: &str) -> Option<&SerializedAnalysis> {
        self.results
            .get(name)
            .and_then(|boxed| boxed.downcast_ref::<SerializedAnalysis>())
    }

    /// Invalidates analysis entries by name.
    pub fn invalidate_many(&mut self, keys: &[String]) {
        for key in keys {
            self.results.remove(key);
        }
    }
}

/// Serializes a native (typed) analysis result in value for cross-ABI consumption by a plugin.
///
/// Receives the type-erased result stored in an [`AnalysisManager`]. Downcasts to its concrete type
/// and encodes it (as protobuf, by the registering crate), or returns `None` on a type mismatch.
pub type AnalysisResultSerializer = fn(&(dyn Any + Send + Sync)) -> Option<Vec<u8>>;
fn analysis_serializers()
-> &'static RwLock<HashMap<String, (&'static str, AnalysisResultSerializer)>> {
    static REGISTRY: OnceLock<RwLock<HashMap<String, (&'static str, AnalysisResultSerializer)>>> =
        OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Registers how to serialize the **native** analysis result stored under `name`, so a plugin can
/// consume a built-in analysis across the C-ABI. Built-ins keep storing typed results for native consumers.
/// This only adds an on-demand byte encoding. `type_tag` identifies the result type so a consumer
/// decoding into the wrong type is rejected. Idempotent per `name`.
pub fn register_analysis_serializer(
    name: impl Into<String>,
    type_tag: &'static str,
    serializer: AnalysisResultSerializer,
) {
    analysis_serializers()
        .write()
        .expect("analysis serializer registry poisoned")
        .insert(name.into(), (type_tag, serializer));
}

/// Serializes the analysis result stored under `name` using a serializer registered with
/// [`register_analysis_serializer`], returning `(type_tag, bytes)`. `None` if no serializer is
/// registered for `name` or no result is stored under it.
pub fn serialize_named_analysis(
    manager: &AnalysisManager,
    name: &str,
) -> Option<(&'static str, Vec<u8>)> {
    let (type_tag, serializer) = *analysis_serializers()
        .read()
        .expect("analysis serializer registry poisoned")
        .get(name)?;
    let value = manager.results.get(name)?;
    Some((type_tag, serializer(value.as_ref())?))
}
