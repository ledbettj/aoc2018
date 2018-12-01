(ns day1.core-test
  (:require [clojure.test :refer :all]
            [day1.core :refer :all]))

(deftest day1-part1
  (testing "Day 1 Part 1"
    (let [ [input] [(load-input "inputs/day1.txt")]
          [results] [(run1 0 input)]]
      (is (= 408 results)))))

(deftest day1-part2
  (testing "Day 1 Part 2"
    (let [ [input] [(load-input "inputs/day1.txt")]
          [results] [(run2 0 input)]]
      (is (= 55250 results)))))
