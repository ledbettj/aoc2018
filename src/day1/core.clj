(ns day1.core
  (:require [clojure.java.io :as io]))

(defn parse-int [s] (Integer/parseInt s))

(defn load-input
  "Read the given input file and return a list of integers"
  [file-name]
  (with-open [rdr (io/reader file-name)]
    (doall (map parse-int (line-seq rdr)))))

; day1 part 1
(defn run1 [initial-value inputs]
  (reduce + initial-value inputs))

(defn run2-iter
  [state]
  (let [{inputs :inputs} state]
    (reduce (fn [history value]
              (let [{ seen :seen sum :sum} history
                    new (+ sum value)]
                (if (contains? seen new)
                  (reduced new)
                  { :seen (conj seen new) :sum new :inputs inputs })))
            state inputs)))


(defn run2
  [initial-value inputs]
  (first (filter #(instance? Long %)
              (iterate run2-iter {:inputs inputs :seen (hash-set initial-value) :sum initial-value }))))
