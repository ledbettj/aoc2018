(ns day1.core
  (:require [clojure.java.io :as io]))

(defn parse-int [s] (Integer/parseInt s))

(defn load-input
  "Read the given input file and return a list of integers"
  [file-name]
  (with-open [rdr (io/reader file-name)]
    (doall (map parse-int (line-seq rdr)))))


(defn run [initial-value inputs]
  (reduce + initial-value inputs))
