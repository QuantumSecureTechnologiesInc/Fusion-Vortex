//! EventEmitter implementation for Node.js compatibility

pub fn get_events_module() -> String {
    r#"
    (function() {
        class EventEmitter {
            constructor() {
                this._events = {};
            }
            
            on(event, listener) {
                if (!this._events[event]) {
                    this._events[event] = [];
                }
                this._events[event].push(listener);
                return this;
            }
            
            once(event, listener) {
                const onceWrapper = (...args) => {
                    listener(...args);
                    this.off(event, onceWrapper);
                };
                return this.on(event, onceWrapper);
            }
            
            off(event, listener) {
                if (!this._events[event]) return this;
                this._events[event] = this._events[event].filter(l => l !== listener);
                return this;
            }
            
            removeListener(event, listener) {
                return this.off(event, listener);
            }
            
            removeAllListeners(event) {
                if (event) {
                    delete this._events[event];
                } else {
                    this._events = {};
                }
                return this;
            }
            
            emit(event, ...args) {
                if (!this._events[event]) return false;
                this._events[event].forEach(listener => {
                    try {
                        listener(...args);
                    } catch (error) {
                        console.error('Error in event listener:', error);
                    }
                });
                return true;
            }
            
            listeners(event) {
                return this._events[event] || [];
            }
            
            listenerCount(event) {
                return this.listeners(event).length;
            }
            
            eventNames() {
                return Object.keys(this._events);
            }
        }
        
        return { EventEmitter };
    })()
    "#
    .to_string()
}
