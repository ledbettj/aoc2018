(ns aoc2018.day2
  (:require [aoc2018.core :refer :all]
            [clojure.string :refer (join)]))


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

(defn find-single-letter-diffs
  [input]
  (map-indexed (fn [index str]
                 [str (single-letter-diffs-for str (drop index input)) ])
               input))

(defn single-letter-diff?
  [s1 s2]
  (and (= (count s1) (count s2))
       (= 1
          (reduce (fn [mismatch-count pair]
                    (let [ [c1 c2] pair ]
                      (cond
                        (= c1 c2) mismatch-count
                        (= 0 mismatch-count) 1
                        true (reduced 2))))
                  0
                  (map (fn [a b] [a b]) s1 s2)))))

(defn single-letter-diffs-for
  [str candidates]
  (filter (fn [candidate] (single-letter-diff? str candidate)) candidates))


(defn solve-day2
  [input]
  (let [results (find-single-letter-diffs input)
        filtered (first (filter (fn [row] (not-empty (last row)) ) results))
        [s1 [s2]] filtered]

    (join (map (fn [c1 c2]
           (if (= c1 c2)
             c1
             "")) s1 s2)
    )))

; (single-letter-diff? "abc" "abc")
; (solve-day2 (day2-input))
