package io.onegraph.gremlin.structure;

import org.apache.tinkerpop.gremlin.structure.*;
import org.apache.tinkerpop.gremlin.structure.util.wrapped.WrappedEdge;

import java.util.Iterator;

public class OneGraphEdge extends OneGraphElement implements Edge, WrappedEdge<Edge> {
    public OneGraphEdge(OneGraph oneGraph, Element detachedElement) {
        super(oneGraph, detachedElement);
    }

    @Override
    public Iterator<Vertex> vertices(Direction direction) {
        return getBaseEdge().vertices(direction);
    }

    @Override
    public <V> Property<V> property(String key, V value) {
        return getBaseEdge().property(key, value);
    }

    @Override
    public <V> Iterator<Property<V>> properties(String... propertyKeys) {
        return getBaseEdge().properties(propertyKeys);
    }

    @Override
    public Edge getBaseEdge() {
        return (Edge) getBaseElement();
    }
}
