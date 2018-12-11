(ns aoc2018.core-test
  (:require [clojure.test :refer :all]
            [aoc2018.core :refer :all]))

(deftest load-input-test
  (testing "it can read the file"
    (let [ input (load-input 1) ]
      (is (= 959 (count input)))
      (is (= "+12" (first input)))
      )))
