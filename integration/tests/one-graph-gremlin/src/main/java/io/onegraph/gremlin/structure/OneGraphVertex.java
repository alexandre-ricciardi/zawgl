package io.onegraph.gremlin.structure;

import org.apache.tinkerpop.gremlin.structure.*;

import java.util.Iterator;

public class OneGraphVertex extends OneGraphElement implements Vertex {

    public OneGraphVertex(OneGraph oneGraph, Vertex detachedElement) {
        super(oneGraph, detachedElement);
    }

    @Override
    public Edge addEdge(String label, Vertex inVertex, Object... keyValues) {
        return null;
    }

    @Override
    public <V> VertexProperty<V> property(VertexProperty.Cardinality cardinality, String key, V value, Object... keyValues) {
        return null;
    }

    @Override
    public Iterator<Edge> edges(Direction direction, String... edgeLabels) {
        return null;
    }

    @Override
    public Iterator<Vertex> vertices(Direction direction, String... edgeLabels) {
        return null;
    }

    @Override
    public <V> Iterator<VertexProperty<V>> properties(String... propertyKeys) {
        return null;
    }
}
