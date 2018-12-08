(ns aoc2018.day8
  (:require [aoc2018.core :refer :all]
            [clojure.string :as str]))

(defn day8-input
  []
  (let [data (first (load-input 8))
        parts (str/split data #"\s+")]
    (map parse-int parts)))


(defn read-node
  "Given list of integers, read a node from the list,
  Returning that node and the remaining input after consuming"
  [input]
  (let [[child-count metadata-count] (take 2 input)
        input (drop 2 input)]
    (let [[child-list remaining]
          (reduce (fn [[children inp] _]
                    (let [[next-child next-input] (read-node inp)]
                      [(conj children next-child) next-input]
                      )
                    ) [[] input] (range 0 child-count))
          metadata-list (take metadata-count remaining)
          remaining (drop metadata-count remaining)
          ]
      [ { :children child-list :metadata metadata-list } remaining ]
      )))


(defn sum-metadata
  "Add up all the metadata entries in the tree under and including node"
  [node]
  (+
   (reduce + 0 (:metadata node))
   (apply + (map sum-metadata (:children node)))))


;; test data
;; (sum-metadata (first (read-node '(2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2))))
;; p1 answer
;; (let [tree (first (read-node (day8-input))) sum (sum-metadata tree)] sum)
