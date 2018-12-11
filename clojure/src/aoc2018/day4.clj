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

(defn ensure-entry
  "Ensure that there is an en entry in the state map for the given guard id"
  [state guard-id]
  (if (contains? state guard-id)
    state
    (conj state { guard-id { :events [] :sleep-map {} :asleep-for 0 } })))


(defn ensure-min-in-sleep-map
  [sleep-map minute]
  (if (contains? sleep-map minute)
    sleep-map
    (conj sleep-map { minute 0 })))

(defn update-sleep-map
  [sleep-map day minute]

  (update
   (ensure-min-in-sleep-map sleep-map minute)
   minute
   inc))

(defn mark-asleep
  [data day minute]
  (let [data (update data :asleep-for inc)]
    (update data :sleep-map update-sleep-map day minute)))

(defn update-guard-data
  [data event]
  (let [last-event (last (:events data))
        new-data (update data :events conj event)]
    (if (and last-event (= (:event last-event) :asleep) (= (:event event) :awake))
      ;; need to record time from (:time last-event) to (:time event) as asleep
      (let [start-time (:time last-event) duration (t/in-minutes (t/interval start-time (:time event)))]
        (reduce (fn [data n]
                  (let [when (t/plus start-time (t/minutes n))
                        day (t/day when)
                        min (t/minute when)]
                    (mark-asleep data day min))
                  ) new-data (range 0 duration)))
      ;; otherwise nothing
      new-data)))

(defn process-event-for-guard
  "Update the state map with the given event for the given guard"
  [state guard-id prev-event event]
  (let [new-state (update state guard-id update-guard-data event)]
    (if (not= guard-id (:id prev-event))
      (update new-state (:id prev-event) update-guard-data event)
      new-state)))


(defn process-event
  [state [prev-event event]]
  (let [id (:id event)
        prev-id (:id prev-event)
        state (ensure-entry state (:id event))]
    (process-event-for-guard state id prev-event event)))

(defn process-event-list
  [event-list]
  (let [first-event (first event-list)]
    (reduce process-event
            { (:id first-event) { :events [first-event] :sleep-map {} :asleep-for 0 } }
            (partition 2 1 event-list))))


(defn solve-this-garbage
  []
  (let [results (vals (process-event-list (day4-input)))
        sleepiest (last (sort-by (fn [entry] (:asleep-for entry)) results))
        id (:id (first (:events sleepiest)))
        sleep-map (:sleep-map sleepiest)
        ]
    id
    ;; sleep-map
    ))


;; (solve-this-garbage)
;; most asleep is at m40
;; id is 2351
;; (* 2351 40)


(defn solve-this-garbage-pt2
  []
  (let [results (vals (process-event-list (day4-input)))
        sleepiest (last (sort-by (fn [entry]
                                   (if (= (count (:sleep-map entry)) 0)
                                     0
                                   (reduce max (vals (:sleep-map entry)))))

                                 results))
        id (:id (first (:events sleepiest)))
        ]
    (:sleep-map sleepiest)
    ))

;; (solve-this-garbage-pt2)
;; id is 1997
;; most asleep at m20
;; (* 20 1997)
