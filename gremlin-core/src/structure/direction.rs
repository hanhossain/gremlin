use std::str::FromStr;

/// [`Direction`] is used to denote the direction of an [`Edge`] or location of a [`Vertex`] on an
/// [`Edge`]. For example:
/// ```text
/// gremlin--knows-->rexter
/// ```
/// is a [`Direction::Out`] [`Edge`] for Gremlin and a [`Direction::In`] edge for Rexster. Moreover,
/// given that [`Edge`], Gremlin is the [`Direction::Out`] [`Vertex`] and Rexster is the
/// [`Direction::In`] [`Vertex`].
pub enum Direction {
    /// Refers to an outgoing direction.
    Out,
    /// Refers to an incoming direction.
    In,
    /// Refers to either direction ([`Direction::In`] or [`Direction::Out`]).
    Both,
}

impl Direction {
    /// The actual direction of an [`Edge`] may only be [`Direction::In`] or [`Direction::Out`], as defined in this array.
    pub fn proper() -> [Direction; 2] {
        [Direction::Out, Direction::In]
    }

    /// Friendly alias to [`Direction::Out`].
    pub fn from() -> Direction {
        Direction::Out
    }

    /// Friendly alias to [`Direction::In`].
    pub fn to() -> Direction {
        Direction::In
    }

    /// Produce the opposite representation of the current `Direction` enum.
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Out => Direction::In,
            Direction::In => Direction::Out,
            Direction::Both => Direction::Both,
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_uppercase().as_str() {
            "TO" => Direction::to(),
            "FROM" => Direction::from(),
            "IN" => Direction::In,
            "OUT" => Direction::Out,
            _ => panic!("Invalid direction"),
        })
    }
}
