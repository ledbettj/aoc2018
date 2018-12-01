(ns aoc2018.day1
  (:require [clojure.java.io :as io]))

(defn parse-int
  "Apparently java functions can't be (easily) used as closures, so wrap it"
  [s] (Integer/parseInt s))

(defn load-input
  "Read the given input file and parse each line into an integer"
  [file-name]
  (with-open [rdr (io/reader file-name)]
    ;; must use doall to force invocation; `map` is lazy and if we dont
    ;; force invocation it will be evaluated later, after the reader is closed.
    (doall (map parse-int (line-seq rdr)))))

(defn run1
  "Solve Day 1 Part 1 for the given inputs, assuming a starting value of initial-value"
  [initial-value inputs]
  (reduce + initial-value inputs))

(defn run2-iter
  "Perform a single iteration on the given state, returning either the next state, or if
   we finally encountered a duplicate value, a single integer representing the duplicate
   value. `state` here looks like
  { :seen <set of seen values> :sum <current running sum> :inputs <list of inputs to iterate over> }"
  [state]
  (let [{inputs :inputs} state]
    (reduce (fn [history value]
              (let [{ seen :seen sum :sum} history
                    new (+ sum value)]
                (if (contains? seen new)
                  (reduced new) ; stops reduction and returns the found value
                  { :seen (conj seen new) :sum new :inputs inputs }))) ; returns the next state
            state inputs)))

(defn run2
  "Solve Day 2 part 2 for the given inputs and initial-value"
  [initial-value inputs]
  (first (filter #(instance? Long %)
              (iterate run2-iter {:inputs inputs :seen (hash-set initial-value) :sum initial-value }))))
