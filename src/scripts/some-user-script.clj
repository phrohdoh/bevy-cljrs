;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; things missing from clojure.core of cljrs
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
(def constantly (fn [v] (fn [& _] v))) ;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(def on-startup (fn [& args] {:systems "operational"}))
;todo
;(defn on-startup [& args] {:systems "operational"})

;(def toggle-selection? (constantly true))
(def toggle-selection?
  (fn [unit]
    (= (get unit :unit/player-id) 1)))
