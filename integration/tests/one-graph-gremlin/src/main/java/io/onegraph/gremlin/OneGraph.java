package org.onegraph.gremlin.integration.test;

import java.util.Iterator;

import org.apache.commons.configuration2.Configuration;
import org.apache.tinkerpop.gremlin.process.computer.GraphComputer;
import org.apache.tinkerpop.gremlin.structure.Edge;
import org.apache.tinkerpop.gremlin.structure.Graph;
import org.apache.tinkerpop.gremlin.structure.Transaction;
import org.apache.tinkerpop.gremlin.structure.Vertex;

public class OneGraph implements Graph {

    @Override
    public Vertex addVertex(Object... keyValues) {
        return null;
    }

    @Override
    public void close() throws Exception {
        
    }

    @Override
    public GraphComputer compute() throws IllegalArgumentException {
        return null;
    }

    @Override
    public <C extends GraphComputer> C compute(Class<C> graphComputerClass) throws IllegalArgumentException {
        return null;
    }

    @Override
    public Configuration configuration() {
        return null;
    }

    @Override
    public Iterator<Edge> edges(Object... edgeIds) {
        return null;
    }

    @Override
    public Transaction tx() {
        return null;
    }

    @Override
    public Variables variables() {
        return null;
    }

    @Override
    public Iterator<Vertex> vertices(Object... vertexIds) {
        // TODO Auto-generated method stub
        return null;
    }
    
}
