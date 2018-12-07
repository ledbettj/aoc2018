(ns aoc2018.day7
  (:require [aoc2018.core :refer :all]
            [clojure.string :as str]))


(defn day7-input
  "return a sequence of pairs, where the first item is the prerequisite for the second"
  []
  (map (fn [line]
         (rest (re-matches #"Step (.) must be finished before step (.) can begin\." line))
         ) (load-input 7)))

(defn build-map
  " Return a map where the keys are each step, and the value is a set of the pre-reqs for each step"
  [input]
  (reduce (fn [graph [prereq step]]
            (let [new-graph (if (contains? graph step)
                              (update graph step conj prereq)
                              (conj graph { step (set [prereq]) }))]
              (if (contains? new-graph prereq)
                new-graph
                (conj new-graph { prereq (set [])}))))
          {} input))


;; (pick-next-step (build-map (day7-input)))




(defn available-steps
  "Return the list of steps that have no pre-reqs remaining"
  [dep-map]
  (map first (filter (fn [[step prereqs]] (empty? prereqs)) dep-map)))

(defn pick-next-step
  [dep-map]
  "Pick the first alphabetical available next step"
  (first (sort (available-steps dep-map))))

(defn remove-dep
  "Remove the given step from the map and from all dependency lists"
  [dep-map step]
  (reduce (fn [new-map [s prset]]
            (if (= s step)
              new-map
              (conj new-map { s (disj prset step) }))) {} dep-map))

;; strategy:
;; find the entries where the pre-reqs are empty.
;; pick the earliest value.
;; remove it from all pre-req lists and add it to the result.
;; remove it from the map.
;; repeat until map is empty.
(defn solve-order-internal
  [dep-map answer]
  (if (empty? dep-map)
    answer
    (let [next-step (pick-next-step dep-map)
          answer (conj answer next-step)
          dep-map (remove-dep dep-map next-step)]
      (solve-order-internal dep-map answer)
      )))


(defn solve-order
  [dep-map]
  (solve-order-internal dep-map []))


(def test-input
  (map (fn [line]
         (rest (re-matches #"Step (.) must be finished before step (.) can begin\." line)))
       ["Step C must be finished before step A can begin."
        "Step C must be finished before step F can begin."
        "Step A must be finished before step B can begin."
        "Step A must be finished before step D can begin."
        "Step B must be finished before step E can begin."
        "Step D must be finished before step E can begin."
        "Step F must be finished before step E can begin."]))


;; p1 answer:
;; (str/join (solve-order (build-map (day7-input))))
;; Value: "ADEFKLBVJQWUXCNGORTMYSIHPZ"
