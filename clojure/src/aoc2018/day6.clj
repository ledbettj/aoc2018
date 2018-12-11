(ns aoc2018.day6
  (:require [aoc2018.core :refer :all])
  (:require [clojure.string :as str]))

(defn day6-input
  []
  (let [input (load-input 6)]
    (map
     (fn [line] (map parse-int (str/split line #", ")))
     input)))


;; (day6-input)

(defn build-grid
  [points]
  (let [xmax (apply max (map first points))
        ymax (apply max (map last points))]
    (make-array Integer/TYPE (+ 1 xmax) (+ 1 ymax))))

;; (build-grid (day6-input))

(defn manhattan [p1 p2]
  (let [[x1 y1] p1
        [x2 y2] p2]
    (+ (Math/abs (- x2 x1)) (Math/abs (- y2 y1)))))

(defn plot-points
  [points]
  (let [grid (build-grid points)]
    (map (fn [[x y]]
           (println "setting" x y)
           (aset grid x y 1)
           ) points)
    grid))


(defn print-grid
  [grid]
  (str/join "\n" (map str/join (map vec grid))))

(print-grid (plot-points (day6-input)))


(map vec (plot-points '((0 0) (3 3))))
