# frozen_string_literal: true

Node = Struct.new(:edges, :key, :lock, keyword_init: true)
Treasure = Class.new

def solve(tree, keys: [], locks: {})
  tree.edges.each do |edge|
    locks.add(edge) if edge.lock
  end
end

if __FILE__ == 'dungeon.rb'
  Node.new(
    edges: [
      Node.new(
        edges: [
          Node.new(lock: :a,
                   edges: [
                     Node.new(
                       edges: [
                         Node.new,
                         Node.new(lock: :b,
                                  edges: [
                                    Node.new(
                                      edges: [
                                        Treasure
                                      ]
                                    )
                                  ])
                       ]
                     )
                   ])
        ]
      ),
      Node.new(edges: [
                 Node.new(key: :b)
               ])
    ]
  )

  solve(
    tree
  )
end
