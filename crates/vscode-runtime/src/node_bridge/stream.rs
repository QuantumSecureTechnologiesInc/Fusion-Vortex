//! Stream module for Node.js compatibility
// __FU_COMPAT_START__
#![allow(missing_docs)]
#[allow(missing_docs, dead_code)] type FString = String;
// __FU_COMPAT_END__
pub fn get_stream_module() -> FString {
    r#"
    (function() {
        const { EventEmitter } = require('events');
        
        class Readable extends EventEmitter {
            constructor(options) {
                super();
                this.readable = true;
                this.reading = false;
                this._queue = [];
                this._ended = false;
            }
            
            read(size) {
                if (this._queue.length === 0) {
                    if (this._ended) {
                        this.readable = false;
                        this.emit('end');
                    }
                    return null;
                }
                const chunk = this._queue.shift();
                this.emit('data', chunk);
                return chunk;
            }

            push(chunk) {
                if (chunk === null || chunk === undefined) {
                    this._ended = true;
                    this.emit('end');
                    return false;
                }
                this._queue.push(chunk);
                if (this.reading) {
                    this.emit('readable');
                }
                return this._queue.length > 0;
            }
            
            pipe(destination) {
                this.on('data', (chunk) => {
                    destination.write(chunk);
                });
                this.on('end', () => {
                    destination.end();
                });
                return destination;
            }
            
            pause() {
                this.emit('pause');
                return this;
            }
            
            resume() {
                this.emit('resume');
                return this;
            }
        }
        
        class Writable extends EventEmitter {
            constructor(options) {
                super();
                this.writable = true;
            }
            
            write(chunk, encoding, callback) {
                if (typeof encoding === 'function') {
                    callback = encoding;
                    encoding = 'utf8';
                }
                if (callback) callback();
                return true;
            }
            
            end(chunk, encoding, callback) {
                if (chunk) this.write(chunk, encoding);
                this.emit('finish');
                if (callback) callback();
            }
        }
        
        class Duplex extends Readable {
            constructor(options) {
                super(options);
                this.writable = true;
            }
            
            write(chunk, encoding, callback) {
                if (typeof encoding === 'function') {
                    callback = encoding;
                }
                if (callback) callback();
                return true;
            }
        }
        
        class Transform extends Duplex {
            constructor(options) {
                super(options);
            }
            
            _transform(chunk, encoding, callback) {
                callback(null, chunk);
            }
        }
        
        return {
            Readable,
            Writable,
            Duplex,
            Transform
        };
    })()
    "#
        .to_string()
}
