const MemoryReader = require('./lib')

const mr = new MemoryReader('Dungeons.exe')

mr.run()

mr.on('tick', ({ count }) => console.log(count));

setTimeout(() => {
    mr.shutdown()
}, 5000)