(ns aoc2018.day5
  (:require [aoc2018.core :refer :all])
  (:require [clojure.string :as str]))


(defn day5-input [] (load-input 5))

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



;; (count (fully-reduce-polymer (day5-input)))

