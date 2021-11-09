package io.onegraph.gremlin;

import java.util.Collections;
import java.util.Map;
import java.util.Set;

import io.onegraph.gremlin.structure.OneGraph;
import org.apache.commons.configuration2.BaseConfiguration;
import org.apache.commons.configuration2.Configuration;
import org.apache.tinkerpop.gremlin.GraphProvider;
import org.apache.tinkerpop.gremlin.LoadGraphWith;
import org.apache.tinkerpop.gremlin.LoadGraphWith.GraphData;
import org.apache.tinkerpop.gremlin.structure.Graph;

public class OneGraphProvider implements GraphProvider {

    public void clear(Graph graph, Configuration configuration) throws Exception {
        if (graph != null)
            graph.close();
    }

    @Override
    public Set<Class> getImplementations() {
        return Collections.emptySet();
    }

    @Override
    public void loadGraphData(Graph graph, LoadGraphWith loadGraphWith, Class testClass, String testName) {
        
    }

    @Override
    public Configuration newGraphConfiguration(String graphName, Class<?> test, String testMethodName,
            Map<String, Object> configurationOverrides, GraphData loadGraphWith) {
        var conf = new BaseConfiguration();
        conf.setProperty(Graph.GRAPH, OneGraph.class.getName());
        return conf;
    }
    
}