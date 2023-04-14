// /**
//  * An {@link Edge} links two {@link Vertex} objects. Along with its {@link Property} objects, an {@link Edge} has both
//  * a {@link Direction} and a {@code label}. The {@link Direction} determines which {@link Vertex} is the tail
//  * {@link Vertex} (out {@link Vertex}) and which {@link Vertex} is the head {@link Vertex}
//  * (in {@link Vertex}). The {@link Edge} {@code label} determines the type of relationship that exists between the
//  * two vertices.
//  * <p/>
//  * Diagrammatically:
//  * <pre>
//  * outVertex ---label---&gt; inVertex.
//  * </pre>
//  *
//  * @author Marko A. Rodriguez (http://markorodriguez.com)
//  * @author Stephen Mallette (http://stephen.genoprime.com)
//  */
// public interface Edge extends Element {
pub trait Edge {}
//
//     /**
//      * The default label to use for an edge.
//      * This is typically never used as when an edge is created, an edge label is required to be specified.
//      */
//     public static final String DEFAULT_LABEL = "edge";
//
//     /**
//      * Retrieve the vertex (or vertices) associated with this edge as defined by the direction.
//      * If the direction is {@link Direction#BOTH} then the iterator order is: {@link Direction#OUT} then {@link Direction#IN}.
//      *
//      * @param direction Get the incoming vertex, outgoing vertex, or both vertices
//      * @return An iterator with 1 or 2 vertices
//      */
//     public Iterator<Vertex> vertices(final Direction direction);
//
//     /**
//      * Get the outgoing/tail vertex of this edge.
//      *
//      * @return the outgoing vertex of the edge
//      */
//     public default Vertex outVertex() {
//         return this.vertices(Direction.OUT).next();
//     }
//
//     /**
//      * Get the incoming/head vertex of this edge.
//      *
//      * @return the incoming vertex of the edge
//      */
//     public default Vertex inVertex() {
//         return this.vertices(Direction.IN).next();
//     }
//
//     /**
//      * Get both the outgoing and incoming vertices of this edge.
//      * The first vertex in the iterator is the outgoing vertex.
//      * The second vertex in the iterator is the incoming vertex.
//      *
//      * @return an iterator of the two vertices of this edge
//      */
//     public default Iterator<Vertex> bothVertices() {
//         return this.vertices(Direction.BOTH);
//     }
//
//     /**
//      * {@inheritDoc}
//      */
//     @Override
//     public <V> Iterator<Property<V>> properties(final String... propertyKeys);
// }
