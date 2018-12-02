(ns aoc2018.core
  (:require [clojure.java.io :as io]))

(defn parse-int
  "Apparently java functions can't be (easily) used as closures, so wrap it"
  [s]
  (Integer/parseInt s))

(defn load-input
  "Read the input file for the given day [1-25] and return an array of lines"
  [day]
  (with-open [rdr (io/reader (str "inputs/day" day ".txt"))]
    ;; must use doall to force invocation; `map` is lazy and if we dont
    ;; force invocation it will be evaluated later, after the reader is closed.
    (doall (line-seq rdr))))

(defn in?
  "true if coll contains elm"
  [coll elm]
  (some #(= elm %) coll))
