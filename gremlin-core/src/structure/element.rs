use crate::structure::property::Property;
use std::collections::HashSet;

/// An [`Element`] is the base class for both [`Vertex`] and [`Edge`]. An element has an identifier
/// that must be unique to its inheriting structs ([`Vertex`] or [`Edge`]). An element can maintain
/// a collection of [`Property`] objects. Typically, objects are primitives.
pub trait Element {
    type Graph;
    type Property<V: Clone>: Property<V, Element = Self>;

    // TODO: the java def was `public Object id()`, so this might need a different return type.
    /// Gets the unique identifier for the graph `Element`.
    fn id(&self) -> String;

    /// Gets the label for the graph `Element` which helps categorize it.
    fn label(&self) -> String;

    /// Get the graph that this element is within.
    fn graph(&self) -> Self::Graph;

    /// Get the keys of the properties associated with this element.
    fn keys(&self) -> HashSet<String>;

    /// Get a [`Property`] for the `Element` given its key. The default implementation calls
    /// the raw [`Element::properties`].
    fn property<V>(&self, key: &str) -> Option<Self::Property<V>>
    where
        V: Clone,
    {
        let properties = self.properties(vec![key]);
        properties.into_iter().next()
    }

    /// Add or set a property value for the `Element` given its key.
    fn set_property<V>(&mut self, key: &str, value: V) -> Self::Property<V>
    where
        V: Clone;

    /// Get the value of a [`Property`] given it's key. The default implementation calls
    /// [`Element::property`] and then returns the associated value.
    fn value<V>(&self, key: &str) -> V
    where
        V: Clone,
    {
        self.property(key).unwrap().unwrap()
    }

    /// Removes the `Element` from the graph.
    fn remove(&mut self);

    /// Get the values of properties as a [Vec].
    fn values<V>(&self, property_keys: Vec<&str>) -> Vec<V>
    where
        V: Clone,
    {
        self.properties(property_keys)
            .into_iter()
            .map(|property| property.value().unwrap())
            .collect()
    }

    // TODO: this should return an iterator instead of a vec
    /// Get a [Vec] of properties where the `propertyKeys` is meant to be a filter on the available
    /// keys. If no keys are provide then return all the properties.
    fn properties<V>(&self, property_keys: Vec<&str>) -> Vec<Self::Property<V>>
    where
        V: Clone;
}
