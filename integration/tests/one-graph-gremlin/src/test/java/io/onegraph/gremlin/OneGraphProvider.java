package org.onegraph.gremlin.integration.test;

import java.util.Map;
import java.util.Set;

import org.apache.commons.configuration2.Configuration;
import org.apache.tinkerpop.gremlin.GraphProvider;
import org.apache.tinkerpop.gremlin.LoadGraphWith;
import org.apache.tinkerpop.gremlin.LoadGraphWith.GraphData;
import org.apache.tinkerpop.gremlin.structure.Graph;

public class OneGraphProvider implements GraphProvider {

    public void clear(Graph graph, Configuration configuration) {

    }

    @Override
    public Set<Class> getImplementations() {
        return null;
    }

    @Override
    public void loadGraphData(Graph graph, LoadGraphWith loadGraphWith, Class testClass, String testName) {
        
    }

    @Override
    public Configuration newGraphConfiguration(String graphName, Class<?> test, String testMethodName,
            Map<String, Object> configurationOverrides, GraphData loadGraphWith) {
        return null;
    }
    
}