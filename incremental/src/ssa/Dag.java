package ssa;

import java.util.Collections;
import java.util.LinkedHashMap;
import java.util.LinkedHashSet;
import java.util.Map;
import java.util.Set;

/**
 * A directed acyclic graph. See http://en.wikipedia.org/wiki/Directed_acyclic_graph
 *
 * @since 3.3
 */
public final class Dag<T> {
    /**
     * Multimap, supports <code>null key, but not null values.
     */
    private static final class MultiMap<K, V> {
        private final Map<K, Set<V>> fMap = new LinkedHashMap<>();

        /**
         * Adds <code>val to the values mapped to by key. If
         * <code>val is null, key is added to the key set of
         * the multimap.
         *
         * @param key the key
         * @param val the value
         */
        public void put(K key, V val) {
            Set<V> values = fMap.computeIfAbsent(key, k -> new LinkedHashSet<>());
            if (val != null)
                values.add(val);
        }

        /**
         * Returns all mappings for the given key, an empty set if there are no mappings.
         *
         * @param key the key
         * @return the mappings for <code>key
         */
        public Set<V> get(K key) {
            Set<V> values = fMap.get(key);
            return values == null ? new LinkedHashSet<>() : values;
        }

        public Set<K> keySet() {
            return fMap.keySet();
        }

        /**
         * Removes all mappings for <code>key and removes key from the key
         * set.
         *
         * @param key the key to remove
         * @return the removed mappings
         */
        public Set<V> removeAll(K key) {
            Set<V> values = fMap.remove(key);
            return values == null ? new LinkedHashSet<>() : values;
        }

        /**
         * Removes a mapping from the multimap, but does not remove the <code>key from the
         * key set.
         *
         * @param key the key
         * @param val the value
         */
        public void remove(K key, V val) {
            Set<V> values = fMap.get(key);
            if (values != null)
                values.remove(val);
        }

        /*
         * @see java.lang.Object#toString()
         */
        public String toString() {
            return fMap.toString();
        }
    }

    private final MultiMap<T, T> fOut = new MultiMap<>();
    private final MultiMap<T, T> fIn = new MultiMap<>();

    /**
     * Adds a directed edge from <code>origin to target. The vertices are not
     * required to exist prior to this call - if they are not currently contained by the graph, they are
     * automatically added.
     *
     * @param origin the origin vertex of the dependency
     * @param target the target vertex of the dependency
     * @return <code>true if the edge was added, false if the
     *         edge was not added because it would have violated the acyclic nature of the
     *         receiver.
     */
    public boolean addEdge(T origin, T target) {
        if (origin == null) throw new IllegalArgumentException();
        if (target == null) throw new IllegalArgumentException();

        if (hasPath(target, origin))
            return false;

        fOut.put(origin, target);
        fOut.put(target, null);
        fIn.put(target, origin);
        fIn.put(origin, null);
        return true;
    }

    /**
     * Adds a vertex to the graph. If the vertex does not exist prior to this call, it is added with
     * no incoming or outgoing edges. Nothing happens if the vertex already exists.
     *
     * @param vertex the new vertex
     */
    public void addVertex(T vertex) {
        if (vertex == null) throw new IllegalArgumentException();
        fOut.put(vertex, null);
        fIn.put(vertex, null);
    }

    /**
     * Removes a vertex and all its edges from the graph.
     *
     * @param vertex the vertex to remove
     */
    public void removeVertex(T vertex) {
        Set<T> targets= fOut.removeAll(vertex);
        for (T target : targets) fIn.remove(target, vertex);
        Set<T> origins = fIn.removeAll(vertex);
        for (T origin : origins) fOut.remove(origin, vertex);
    }

    /**
     * Returns the sources of the receiver. A source is a vertex with no incoming edges. The
     * returned set's iterator traverses the nodes in the order they were added to the graph.
     *
     * @return the sources of the receiver
     */
    public Set<T> getSources() {
        return computeZeroEdgeVertices(fIn);
    }

    /**
     * Returns the sinks of the receiver. A sink is a vertex with no outgoing edges. The returned
     * set's iterator traverses the nodes in the order they were added to the graph.
     *
     * @return the sinks of the receiver
     */
    public Set<T> getSinks() {
        return computeZeroEdgeVertices(fOut);
    }

    private Set<T> computeZeroEdgeVertices(MultiMap<T, T> map) {
        Set<T> candidates= map.keySet();
        Set<T> roots= new LinkedHashSet<>(candidates.size());
        for (T candidate : candidates) {
            if (map.get(candidate).isEmpty())
                roots.add(candidate);
        }
        return roots;
    }

    /**
     * Returns the direct children of a vertex. The returned {@link Set} is unmodifiable.
     *
     * @param vertex the parent vertex
     * @return the direct children of <code>vertex
     */
    public Set<T> getChildren(T vertex) {
        return Collections.unmodifiableSet(fOut.get(vertex));
    }

    private boolean hasPath(T start, T end) {
        // break condition
        if (start == end)
            return true;

        Set<T> children = fOut.get(start);
        // recursion
        for (T child : children)
            if (hasPath(child, end))
                return true;
        return false;
    }

    /*
     * @see java.lang.Object#toString()
     * @since 3.3
     */
    public String toString() {
        return "Out: " + fOut.toString() + " In: " + fIn.toString(); //$NON-NLS-1$ //$NON-NLS-2$
    }
}
