package io.onegraph.gremlin;

import org.apache.commons.configuration2.BaseConfiguration;
import org.apache.commons.configuration2.Configuration;
import org.apache.tinkerpop.gremlin.process.computer.GraphComputer;
import org.apache.tinkerpop.gremlin.structure.Edge;
import org.apache.tinkerpop.gremlin.structure.Graph;
import org.apache.tinkerpop.gremlin.structure.Transaction;
import org.apache.tinkerpop.gremlin.structure.Vertex;

import java.util.Iterator;
import java.util.Optional;

public class OneGraph implements Graph {

    private static final Configuration EMPTY_CONFIGURATION = new BaseConfiguration() {{
        this.setProperty(Graph.GRAPH, OneGraph.class.getName());
    }};

    protected final OneGraphConfiguration configuration;

    public OneGraph(Configuration configuration) {
        this.configuration = new OneGraphConfiguration(configuration);
    }

    public static OneGraph open() {
        return OneGraph.open(EMPTY_CONFIGURATION);
    }

    public static OneGraph open(final Configuration configuration) {
        return new OneGraph(Optional.ofNullable(configuration).orElse(EMPTY_CONFIGURATION));
    }

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
