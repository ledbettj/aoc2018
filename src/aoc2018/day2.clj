(ns aoc2018.day2
  (:require [aoc2018.core :refer :all]))


(defn day2-input [] (load-input 2))

(defn char-count-in-str
  [str]
  (reduce (fn [state chr]
            (if (contains? state chr)
              (update state chr inc)
              (conj state { chr 1 })))
          {} str))

(defn count-box-types
  [input]
  (reduce (fn [state str]
            (let [counts (vals (char-count-in-str str))]
              (cond
                (and (in? counts 2) (in? counts 3))
                  (update (update state 2 inc) 3 inc)
                (in? counts 2)
                  (update state 2 inc)
                (in? counts 3)
                  (update state 3 inc)
                true state
                )))
            { 2 0 3 0 }
   input))

; (reduce * (vals (count-box-types (load-input 2))))
