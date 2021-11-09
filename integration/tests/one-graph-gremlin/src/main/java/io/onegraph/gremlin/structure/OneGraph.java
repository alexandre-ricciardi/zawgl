package io.onegraph.gremlin;

import org.apache.commons.configuration2.BaseConfiguration;
import org.apache.commons.configuration2.Configuration;
import org.apache.tinkerpop.gremlin.driver.Cluster;
import org.apache.tinkerpop.gremlin.driver.remote.DriverRemoteConnection;
import org.apache.tinkerpop.gremlin.driver.ser.Serializers;
import org.apache.tinkerpop.gremlin.process.computer.GraphComputer;
import org.apache.tinkerpop.gremlin.process.traversal.AnonymousTraversalSource;
import org.apache.tinkerpop.gremlin.process.traversal.dsl.graph.GraphTraversalSource;
import org.apache.tinkerpop.gremlin.structure.Edge;
import org.apache.tinkerpop.gremlin.structure.Graph;
import org.apache.tinkerpop.gremlin.structure.Transaction;
import org.apache.tinkerpop.gremlin.structure.Vertex;

import java.util.Collections;
import java.util.Iterator;
import java.util.Optional;

@Graph.OptIn(Graph.OptIn.SUITE_STRUCTURE_STANDARD)
@Graph.OptIn(Graph.OptIn.SUITE_PROCESS_STANDARD)
@Graph.OptIn(Graph.OptIn.SUITE_PROCESS_COMPUTER)
public class OneGraph implements Graph {

    private static final Configuration EMPTY_CONFIGURATION = new BaseConfiguration() {{
        this.setProperty(Graph.GRAPH, OneGraph.class.getName());
    }};

    protected final Configuration configuration;

    public OneGraph(Configuration configuration) {
        this.configuration = configuration;
    }

    public static OneGraph open() {
        return OneGraph.open(EMPTY_CONFIGURATION);
    }

    public static OneGraph open(final Configuration configuration) {
        return new OneGraph(Optional.ofNullable(configuration).orElse(EMPTY_CONFIGURATION));
    }

    @Override
    public Vertex addVertex(Object... keyValues) {
        throw Exceptions.vertexAdditionsNotSupported();
    }

    @Override
    public void close() throws Exception {
        configuration.clear();
    }

    @Override
    public GraphComputer compute() throws IllegalArgumentException {
        throw Exceptions.graphComputerNotSupported();
    }

    @Override
    public <C extends GraphComputer> C compute(Class<C> graphComputerClass) throws IllegalArgumentException {
        throw Exceptions.graphComputerNotSupported();
    }

    @Override
    public Configuration configuration() {
        return configuration;
    }

    @Override
    public Iterator<Edge> edges(Object... edgeIds) {
        return Collections.emptyIterator();
    }

    @Override
    public Transaction tx() {
        throw Exceptions.transactionsNotSupported();
    }

    @Override
    public Variables variables() {
        throw Exceptions.variablesNotSupported();
    }

    @Override
    public Iterator<Vertex> vertices(Object... vertexIds) {
        return Collections.emptyIterator();
    }

    @Override
    public GraphTraversalSource traversal() {
        return createSource(createCluster());
    }

    private Cluster createCluster() {
        final Cluster cluster = Cluster.build("localhost")
                .port(8182)
                .maxInProcessPerConnection(32)
                .maxSimultaneousUsagePerConnection(32)
                .serializer(Serializers.GRAPHSON_V3D0)
                .create();
        return cluster;
    }
    private GraphTraversalSource createSource(final Cluster cluster) {
        final GraphTraversalSource g = AnonymousTraversalSource.traversal().withRemote(DriverRemoteConnection.using(cluster));
        return g;
    }


}
