(ns aoc2018.day4
  (:require [aoc2018.core :refer :all])
  (:require [clojure.string :as str])
  (:require [clj-time.core :as t])
  (:require [clj-time.format :as tf]))

(defn parse-event-line
  "Parse a given event line into a map for easier processing."
  [line]
  (let [[_ ts event id] (re-find #"^\[(.+)\] (\w+) #?([^\s]+)" line)
        timestamp (tf/parse (tf/formatter :date-hour-minute) (str/replace ts #" " "T"))]
    (cond
      (= event "Guard") { :event :awake  :time timestamp :id (parse-int id) }
      (= event "falls") { :event :asleep :time timestamp }
      (= event "wakes") { :event :awake  :time timestamp })))


(defn day4-input
  "Load the input file and parse each event into the proper order and include IDs"
  []
  (let [sorted-list (sort-by (fn [event] (:time event)) (map parse-event-line (load-input 4)))]
    (reduce
     (fn [results e]
       (let [id (if (contains? e :id) (:id e) (:id (last results)))]
         (conj results (conj e {:id id })))) [] sorted-list)))


(defn new-guard-state [first-event]
  { :state (:event first-event) :asleep-for 0 :list (vector first-event) })

(defn update-guard-state [guard-state event]
  (println "update-guard-state on " guard-state "with" event)
  (let [new-state (:event event) current-state (:state guard-state) last-event (last (:list guard-state))]
    (update
     (let [updated-state (conj guard-state { :state new-state })]
       (println "moving from " current-state "to" new-state)
       (if (and (= new-state :awake) (= current-state :asleep))
         (update updated-state :asleep-for + (t/in-minutes (t/interval (:time last-event) (:time event))))
         updated-state)
       ) :list conj event)))

(defn process-guard-event-list
  [event-list]
  (let [first-event (first event-list)
        initial-state { (:id first-event) (new-guard-state first-event) }]
    (reduce (fn [state [prev-event event]]
              (if (not= (:id event) (:id prev-event))
                ;;  mark last guard as awake
                (update 
                 ;; start new guard 
                 (conj state { (:id event) (new-guard-state event) })
                 (:id prev-event) update-guard-state event)
                ;; else
                (update state (:id event) update-guard-state event)))
               initial-state (partition 2 1 event-list))))


;; day4-input
;; (process-guard-event-list (day4-input))
;;
(defn try-to-solve-this-madness []
  (let [sleepy-guard-state (last (sort-by (fn [item] (:asleep-for item)) (vals (process-guard-event-list (day4-input)))))
        duration (:asleep-for sleepy-guard-state)]
    sleepy-guard-state
    ))

(try-to-solve-this-madness)
