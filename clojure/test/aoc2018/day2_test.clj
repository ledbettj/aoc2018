(ns aoc2018.day2-test
  (:require [clojure.test :refer :all]
            [aoc2018.day2 :refer :all]))

(deftest single-letter-diff-test
  (testing "It works"
    (is (single-letter-diff? "abc" "adc"))
    (is (not (single-letter-diff? "abc" "abc")))
    (is (not (single-letter-diff? "abc" "add")))))

(deftest single-letter-diffs-for-test
  (testing "It works"
    (is (= (single-letter-diffs-for "abc" ["zaz" "add" "bbc" "abz"])
           ["bbc" "abz"]))))

(deftest day2-part1
  (testing "Day 2 Part 1"
    (let [ answer
          (reduce * (vals (count-box-types (day2-input)))) ]
      (is (= 5904 answer)))))

(deftest day2-part2
  (testing "Day 2 Part 2"
    (let [ answer (solve-day2 (day2-input)) ]
      (is (= "jiwamotgsfrudclzbyzkhlrvp" answer))
      )))
