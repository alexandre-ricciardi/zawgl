/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package org.onegraph.gremlin.integration.test;

import java.io.IOException;
import java.util.List;

import org.apache.tinkerpop.gremlin.driver.Cluster;
import org.apache.tinkerpop.gremlin.driver.remote.DriverRemoteConnection;
import org.apache.tinkerpop.gremlin.driver.ser.Serializers;
import org.apache.tinkerpop.gremlin.process.traversal.AnonymousTraversalSource;
import org.apache.tinkerpop.gremlin.process.traversal.P;
import org.apache.tinkerpop.gremlin.process.traversal.dsl.graph.GraphTraversalSource;
import org.apache.tinkerpop.gremlin.process.traversal.dsl.graph.__;
import org.apache.tinkerpop.gremlin.structure.Vertex;
import org.apache.tinkerpop.gremlin.structure.io.graphson.GraphSONIo;
import org.apache.tinkerpop.gremlin.tinkergraph.structure.TinkerFactory;
import org.junit.Assert;
import org.junit.Test;

public class GremlinTest {
    public void testAppHasAGreeting() {

        final Cluster cluster = createCluster();
        try {
            final GraphTraversalSource g = createSource(cluster);
            List<Object> verticesWithNamePumba = g.V().has("name", "pumba").out("friendOf").id().toList();
            System.out.println(verticesWithNamePumba);
        } finally {
            cluster.close();
        }

    }

    public void testMatch() {
        final Cluster cluster = createCluster();
        try {
            final GraphTraversalSource g = createSource(cluster);
            var v1 = g.V().match(
                __.as("a").out("knows").as("b"),
                __.as("a").out("created").as("c"),
                __.as("b").out("created").as("c")).
              addE("friendlyCollaborator").from("a").to("b").
                property("id",23).property("project", __.select("c").values("name")).iterate();
            System.out.println(v1);
        } finally {
            cluster.close();
        }
    }

    @Test
    public void testMatchEdge() {
        final Cluster cluster = createCluster();
        try {
            final GraphTraversalSource g = createSource(cluster);
            createVertexAndEdge(g);
            var v1 = g.V().as("source").outE("knows").V().addE("testEdge").to("source").next();
            System.out.println(v1);
        } finally {
            cluster.close();
        }
    }

    @Test
    public void testCreateVertex() {
        final Cluster cluster = createCluster();
        try {
            final GraphTraversalSource g = createSource(cluster);
            createVertexAndEdge(g);
        } finally {
            cluster.close();
        }
    }

    @Test
    public void testCreateRetrieveVertex() {
        final Cluster cluster = createCluster();
        try {
            final GraphTraversalSource g = createSource(cluster);
            createVertexAndEdge(g);
            var res = g.V().out("knows").next();
            System.out.println(res);
        } finally {
            cluster.close();
        }
    }

    private GraphTraversalSource createSource(final Cluster cluster) {
        final GraphTraversalSource g = AnonymousTraversalSource.traversal().withRemote(DriverRemoteConnection.using(cluster));
        return g;
    }

    private void createVertexAndEdge(final GraphTraversalSource g) {
        Vertex v1 = g.addV("person").property("name","marko").next();
        Assert.assertEquals("person", v1.label());
        Vertex v2 = g.addV("person").property("name","stephen").next();
        Assert.assertEquals("person", v2.label());
        var res = g.V(v1).addE("knows").from(v2).property("weight",0.75).next();
        Assert.assertEquals("knows", res.label());
    }

    @Test
    public void testCreateEdge() {
        final Cluster cluster = createCluster();
        try {
            final GraphTraversalSource g = createSource(cluster);
            createVertexAndEdge(g);
            var v1 = g.V().has("name", P.within("marko", "stephen")).as("person").
            V().has("name", P.within("stephen")).addE("uses").from("person").next();
            Assert.assertEquals("uses", v1.label());
        } finally {
            cluster.close();
        }
    }

    public void printGraph() throws IOException {
        var graph = TinkerFactory.createModern();
        var w = graph.io(GraphSONIo.build()).writer();
        w.create().writeGraph(System.out, graph);
        System.out.println(graph.traversal().V().has("name", P.within("marko", "stephen")).next().property("name"));        
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
}

