/// A [`Property`] denotes a key/value pair associated with an [`Edge`]. A property is like an
/// [Optional] in that a property can be not present. The key of a property is always a string and
/// the value of a property is an arbitrary type. Each underlying graph engine will typically have
/// constraints on what is allowed to be used as values.
pub trait Property<V>
where
    V: Clone,
{
    type Element;

    /// The key of the property.
    fn key(&self) -> String;

    /// The value of the property.
    fn value(&self) -> Option<V>;

    /// Whether the property is empty or not.
    fn is_present(&self) -> bool;

    /// If the property is present, then act on the value as specified by the `func`.
    fn if_present<F>(&self, func: F)
    where
        F: Fn(V),
    {
        if self.is_present() {
            func(self.value().unwrap());
        }
    }

    /// If the value is present, return the value, else return the provided value.
    fn or_else(&self, other_value: V) -> V {
        if self.is_present() {
            self.value().unwrap()
        } else {
            other_value
        }
    }

    /// If the value is present, return the value, else generate a value given the `func`.
    fn or_else_get<F>(&self, func: F) -> V
    where
        F: Fn() -> V,
    {
        if self.is_present() {
            self.value().unwrap()
        } else {
            func()
        }
    }

    /// If the value is present, return the value, else panic.
    fn unwrap(&self) -> V {
        if self.is_present() {
            self.value().unwrap()
        } else {
            panic!("Expected property to be present.");
        }
    }

    /// Get the element this property is associated with (i.e. [`Vertex`], [`Edge`], or [`VertexProperty`]).
    fn element(&self) -> Self::Element;

    // TODO: this probably needs mut self.
    /// Remove the property from the associated element.
    fn remove(&self);
}
