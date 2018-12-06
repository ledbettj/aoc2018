(ns aoc2018.day5
  (:require [aoc2018.core :refer :all])
  (:require [clojure.string :as str]))


(defn day5-input [] (first (load-input 5)))

(defn reduce-polymer
  [polymer]
  (str/join (reduce
   (fn [result ch]
     (let [last-ch (last result)]
       (cond
         ;; no previous char, just append
         (nil? last-ch) [ch]
         ;; characters are same but different case, drop them
         (and (not= last-ch ch) (= (str/upper-case last-ch) (str/upper-case ch))) (pop result)
         ;; otherwise just append
         true (conj result ch)
         ))) [] polymer)))

(defn fully-reduce-polymer
  [polymer]
  (let [reduced (reduce-polymer polymer)]
    (if (= (count reduced) (count polymer))
      reduced
      (fully-reduce-polymer reduced))))


(defn remove-polymer-unit
  [polymer unit]
  (str/replace polymer
               (re-pattern (str "[" (str/lower-case unit) (str/upper-case unit) "]"))
               ""))

(defn a-to-z
  []
  (map char (range (int \a) (inc (int \z)))))

(defn solve-part2
  []
  (let [input (day5-input)
        results (map (fn [ch]
                       (let [cnt (count (fully-reduce-polymer (remove-polymer-unit input ch)))]
                         (println ch "reduced to" cnt)
                         cnt))
                     (a-to-z))]
    (reduce min results)))

;; p1:
;; (count (fully-reduce-polymer (day5-input)))
;; p2:
;; (solve-part2)
