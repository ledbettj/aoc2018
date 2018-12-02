(ns aoc2018.day2-test
  (:require [clojure.test :refer :all]
            [aoc2018.day2 :refer :all]))

(deftest day2-part1
  (testing "Day 2 Part 1"
    (let [ answer
          (reduce * (vals (count-box-types (day2-input)))) ]
      (is (= 5904 answer)))))
