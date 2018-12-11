(ns aoc2018.day3
  (:require [aoc2018.core :refer :all]))

(defrecord Point [x y])
(defrecord Rectangle [position width height])

(defrecord Claim [id rect])

(defn parse-claim
  "String representing a claim:  #id @ x,y: WxH"
  [str]
  (let [matches (rest (re-matches #"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)" str))
        [id x y w h] (map parse-int matches)]
    (Claim. id (Rectangle. (Point. x y) w h))))

(defn day3-input
  []
  (map parse-claim (load-input 3)))

(defn update-claims
  [state claim]
  (let [xstart (:x (:position (:rect claim)))
        xend   (+ xstart (:width (:rect claim)))
        ystart (:y (:position (:rect claim)))
        yend   (+ ystart (:height (:rect claim)))
        xrange (range xstart xend)
        yrange (range ystart yend)]
    (reduce (fn [state x]
              (reduce (fn [smap y]
                        (if (contains? smap [x y])
                          (update smap [x y] inc)
                          (conj smap { [x y] 1 })
                          )) state yrange)
              ) state xrange)))

(defn build-overlap-set
  [claims]
  (reduce update-claims {} claims))

(defn count-overlaping-points
  [claim-map]
  (count (filter (fn [value] (> value 1)) (vals claim-map))))

(defn no-overlaps?
  [claim claim-map]
  (let [xstart (:x (:position (:rect claim)))
        xend   (+ xstart (:width (:rect claim)))
        ystart (:y (:position (:rect claim)))
        yend   (+ ystart (:height (:rect claim)))
        xrange (range xstart xend)
        yrange (range ystart yend)]
    (reduce (fn [state x]
              (reduce (fn [state y]
                        (and state (= (claim-map [x y]) 1))
                        ) state yrange))
            true xrange)))

(defn find-non-overlapping-claims
  [claims claim-map]
  (first (filter (fn [claim]
                   (no-overlaps? claim claim-map))
                 claims )))

;; answer to part 1
;; (count-overlaping-points (build-overlap-set (day3-input)))

;; answer to part 2
;; (:id (find-non-overlapping-claims (day3-input) (build-overlap-set (day3-input))))

