(ns aoc2018.day1-test
  (:require [clojure.test :refer :all]
            [aoc2018.day1 :refer :all]))

(deftest day1-part1
  (testing "Day 1 Part 1"
    (let [ [input] [(day1-input)]
          [results] [(run1 0 input)]]
      (is (= 408 results)))))

(deftest day1-part2
  (testing "Day 1 Part 2"
    (let [ [input] [(day1-input)]
          [results] [(run2 0 input)]]
      (is (= 55250 results)))))
