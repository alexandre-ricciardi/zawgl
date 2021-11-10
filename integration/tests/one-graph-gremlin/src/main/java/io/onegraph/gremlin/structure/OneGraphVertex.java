package io.onegraph.gremlin.structure;

import org.apache.tinkerpop.gremlin.structure.Direction;
import org.apache.tinkerpop.gremlin.structure.Edge;
import org.apache.tinkerpop.gremlin.structure.Vertex;
import org.apache.tinkerpop.gremlin.structure.VertexProperty;
import org.apache.tinkerpop.gremlin.structure.util.ElementHelper;
import org.apache.tinkerpop.gremlin.structure.util.wrapped.WrappedVertex;

import java.util.Iterator;

public class OneGraphVertex extends OneGraphElement implements Vertex, WrappedVertex<Vertex> {

    public OneGraphVertex(OneGraph oneGraph, Vertex detachedElement) {
        super(oneGraph, detachedElement);
    }

    @Override
    public Edge addEdge(String label, Vertex inVertex, Object... keyValues) {
        ElementHelper.validateLabel(label);
        ElementHelper.legalPropertyKeyValueArray(keyValues);
        if (ElementHelper.getIdValue(keyValues).isPresent())
            throw Edge.Exceptions.userSuppliedIdsNotSupported();
        var e = getOneGraph().traversal().addE(label).from(this).to(inVertex).next();
        ElementHelper.attachProperties(e, keyValues);
        return e;
    }

    @Override
    public <V> VertexProperty<V> property(VertexProperty.Cardinality cardinality, String key, V value, Object... keyValues) {
        return getBaseVertex().property(cardinality, key, value, keyValues);
    }

    @Override
    public Iterator<Edge> edges(Direction direction, String... edgeLabels) {
        return getBaseVertex().edges(direction, edgeLabels);
    }

    @Override
    public Iterator<Vertex> vertices(Direction direction, String... edgeLabels) {
        return getBaseVertex().vertices(direction, edgeLabels);
    }

    @Override
    public <V> Iterator<VertexProperty<V>> properties(String... propertyKeys) {
        return getBaseVertex().properties(propertyKeys);
    }

    @Override
    public Vertex getBaseVertex() {
        return (Vertex) getBaseElement();
    }
}
